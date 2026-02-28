use llm::chat::MessageType;
use super::aipeizhi::Aipeizhi;
use super::aixiaoxiguanli::Xiaoxiguanli;

/// ReAct 单次调用结果
pub enum ReactJieguo {
    /// AI 返回了纯文本回复（含可选思考过程）
    Wenben { neirong: String, sikao: Option<String> },
    /// AI 要求调用工具
    Gongjudiaoyong(Vec<llm::ToolCall>),
}

// ── 内部公共组件 ──

fn goujian_xiaoxiti(guanli: &Xiaoxiguanli) -> Vec<serde_json::Value> {
    let mut xiaoxilie = Vec::new();
    if let Some(tishici) = guanli.huoqu_xitongtishici() {
        xiaoxilie.push(serde_json::json!({"role": "system", "content": tishici}));
    }
    for xiaoxi in guanli.huoqu_xiaoxilie() {
        let juese = match xiaoxi.role {
            llm::chat::ChatRole::User => "user",
            llm::chat::ChatRole::Assistant => "assistant",
        };
        match &xiaoxi.message_type {
            MessageType::ToolUse(diaoyonglie) => {
                let tool_calls: Vec<serde_json::Value> = diaoyonglie.iter().map(|d| {
                    serde_json::json!({
                        "id": d.id,
                        "type": d.call_type,
                        "function": {
                            "name": d.function.name,
                            "arguments": d.function.arguments,
                        }
                    })
                }).collect();
                xiaoxilie.push(serde_json::json!({
                    "role": "assistant",
                    "content": null,
                    "tool_calls": tool_calls,
                }));
            }
            MessageType::ToolResult(jieguolie) => {
                for jieguo in jieguolie {
                    xiaoxilie.push(serde_json::json!({
                        "role": "tool",
                        "tool_call_id": jieguo.id,
                        "content": jieguo.function.arguments,
                    }));
                }
            }
            _ => {
                if !xiaoxi.content.is_empty() {
                    xiaoxilie.push(serde_json::json!({"role": juese, "content": xiaoxi.content}));
                }
            }
        }
    }
    xiaoxilie
}

/// 构建请求体（流式/非流式共用）
/// 使用 max_tokens（兼容第三方API）而非 max_completion_tokens（OpenAI专属）
/// assistant 工具调用消息显式包含 content: null（部分API要求）
fn goujian_qingqiuti(peizhi: &Aipeizhi, guanli: &Xiaoxiguanli, liushi: bool, dai_gongju: bool) -> serde_json::Value {
    let mut ti = serde_json::json!({
        "model": peizhi.moxing,
        "messages": goujian_xiaoxiti(guanli),
        "stream": liushi,
        "temperature": peizhi.wendu,
    });
    if peizhi.zuida_token > 0 {
        ti["max_tokens"] = serde_json::json!(peizhi.zuida_token);
    }
    if dai_gongju {
        if let Some(gongjulie) = guanli.huoqu_gongjulie() {
            let tools: Vec<serde_json::Value> = gongjulie.iter().map(|g| {
                serde_json::json!({
                    "type": g.tool_type,
                    "function": {
                        "name": g.function.name,
                        "description": g.function.description,
                        "parameters": g.function.parameters,
                    }
                })
            }).collect();
            ti["tools"] = serde_json::json!(tools);
        }
    }
    ti
}

/// 发送 HTTP 请求（含重试+退避），返回成功的原始响应
async fn fasong_qingqiu(peizhi: &Aipeizhi, ti: &serde_json::Value) -> Option<reqwest::Response> {
    // 调度器：获取全局AI并发许可，满则排队，超时返回None
    let _xukezheng = match super::diaoduqi::changshi_huoqu_xukezheng_moren().await {
        Ok(xk) => xk,
        Err(e) => {
            println!("[OpenAI] 调度器排队超时: {}", e);
            return None;
        }
    };

    let dizhi = format!("{}/chat/completions", peizhi.jiekoudizhi.trim_end_matches('/'));
    let chaoshi = std::time::Duration::from_secs(peizhi.chaoshishijian);
    for cishu in 0..=peizhi.chongshicishu {
        if super::diaoduqi::dangqian_yiquxiao() {
            return None;
        }
        match reqwest::Client::builder()
            .no_proxy()
            .build()
            .unwrap()
            .post(&dizhi)
            .header("Authorization", format!("Bearer {}", peizhi.miyao))
            .header("Content-Type", "application/json")
            .timeout(chaoshi)
            .json(ti)
            .send()
            .await
        {
            Ok(x) if x.status().is_success() => return Some(x),
            Ok(x) => {
                let zt = x.status();
                let nr = x.text().await.unwrap_or_default();
                println!("[OpenAI] 请求失败 状态: {} 响应: {}", zt, nr);
                if cishu < peizhi.chongshicishu {
                    let jichu = if zt.as_u16() == 429 {
                        5 * (cishu as u64 + 1)
                    } else {
                        2 * (cishu as u64 + 1)
                    };
                    let yanchi = jichu + fastrand::u64(0..4);
                    println!("[OpenAI] 等待{}秒后重试", yanchi);
                    tokio::time::sleep(std::time::Duration::from_secs(yanchi)).await;
                }
            }
            Err(e) => {
                println!("[OpenAI] 请求异常: {}", e);
                if cishu < peizhi.chongshicishu {
                    let yanchi = 2 * (cishu as u64 + 1) + fastrand::u64(0..3);
                    println!("[OpenAI] 等待{}秒后重试", yanchi);
                    tokio::time::sleep(std::time::Duration::from_secs(yanchi)).await;
                }
            }
        }
    }
    None
}

/// 从 AI 响应 JSON 中提取文本内容（过滤空文本）
fn tiqu_wenben(json: &serde_json::Value) -> Option<String> {
    json["choices"][0]["message"]["content"].as_str()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

/// 从 AI 响应 JSON 中提取思考过程（兼容 reasoning_content 和 reasoning 字段）
fn tiqu_sikao(json: &serde_json::Value) -> Option<String> {
    let msg = &json["choices"][0]["message"];
    msg["reasoning_content"].as_str()
        .or_else(|| msg["reasoning"].as_str())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

/// 非流式请求，返回解析后的 JSON
async fn feiliushi_json(peizhi: &Aipeizhi, guanli: &Xiaoxiguanli, dai_gongju: bool) -> Option<serde_json::Value> {
    let ti = goujian_qingqiuti(peizhi, guanli, false, dai_gongju);
    let xiangying = fasong_qingqiu(peizhi, &ti).await?;
    let neirong = xiangying.text().await.unwrap_or_default();
    match serde_json::from_str(&neirong) {
        Ok(json) => Some(json),
        Err(e) => { println!("[OpenAI] JSON解析失败: {}", e); None }
    }
}

// ── 对外接口 ──

/// 判断响应体是否为限流错误（某些API返回HTTP 200但body含限流状态码）
fn shifou_xianliu_xiangying(json: &serde_json::Value) -> bool {
    if let Some(xiaoxi) = json.get("msg").and_then(|v| v.as_str()) {
        if xiaoxi.to_lowercase().contains("rate limit") {
            return true;
        }
    }
    match json.get("status") {
        Some(serde_json::Value::String(s)) => s == "429" || s == "449",
        Some(serde_json::Value::Number(n)) => matches!(n.as_u64(), Some(429) | Some(449)),
        _ => false,
    }
}

/// 非流式调用内部实现（含响应体限流退避重试），返回 (wenben, sikao)
async fn putongqingqiu_neibu(peizhi: &Aipeizhi, guanli: &Xiaoxiguanli) -> Option<(String, Option<String>)> {
    let zuida_xianliu_chongshi: u32 = 3;
    for changshi in 0..=zuida_xianliu_chongshi {
        if super::diaoduqi::dangqian_yiquxiao() {
            return None;
        }
        let json = match feiliushi_json(peizhi, guanli, false).await {
            Some(j) => j,
            None => return None,
        };
        // 检测响应体内限流（HTTP 200 但 body 含 429/449）
        if json.get("choices").is_none() && shifou_xianliu_xiangying(&json) {
            if changshi < zuida_xianliu_chongshi {
                let yanchi = 5 * (changshi as u64 + 1) + fastrand::u64(0..5);
                println!("[OpenAI] 响应体限流，等待{}秒后重试（{}/{})", yanchi, changshi + 1, zuida_xianliu_chongshi);
                tokio::time::sleep(std::time::Duration::from_secs(yanchi)).await;
                continue;
            }
            println!("[OpenAI] 响应体限流，重试已耗尽");
            return None;
        }

        let wenben = tiqu_wenben(&json)?;
        let sikao = tiqu_sikao(&json);
        return Some((wenben, sikao));
    }
    None
}

/// 非流式调用（仅返回文本）
pub async fn putongqingqiu(peizhi: &Aipeizhi, guanli: &Xiaoxiguanli) -> Option<String> {
    putongqingqiu_neibu(peizhi, guanli).await.map(|(w, _)| w)
}

/// 非流式调用（返回文本 + 思考内容）
pub async fn putongqingqiu_daisikao(peizhi: &Aipeizhi, guanli: &Xiaoxiguanli) -> Option<(String, Option<String>)> {
    putongqingqiu_neibu(peizhi, guanli).await
}

/// 从 content 文本中解析 <tool_call> 格式的工具调用（兼容部分模型不走标准 tool_calls 字段的情况）
fn jiexi_wenben_gongjudiaoyong(neirong: &str) -> Option<Vec<llm::ToolCall>> {
    let mut jieguo = Vec::new();
    let mut sousuo = neirong;
    while let Some(kaishi) = sousuo.find("<tool_call>") {
        let houtui = &sousuo[kaishi + 11..];
        let jieshu = houtui.find("</tool_call>")?;
        let json_str = houtui[..jieshu].trim();
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(json_str) {
            let mingcheng = json["name"].as_str()
                .or_else(|| json["function"]["name"].as_str());
            let canshu = json["arguments"].as_object()
                .map(|o| serde_json::to_string(o).unwrap_or_default())
                .or_else(|| json["arguments"].as_str().map(|s| s.to_string()))
                .or_else(|| json["parameters"].as_object()
                    .map(|o| serde_json::to_string(o).unwrap_or_default()));
            if let (Some(ming), Some(can)) = (mingcheng, canshu) {
                jieguo.push(llm::ToolCall {
                    id: format!("textcall_{}", jieguo.len()),
                    call_type: "function".to_string(),
                    function: llm::FunctionCall {
                        name: ming.to_string(),
                        arguments: can,
                    },
                });
            }
        }
        sousuo = &houtui[jieshu + 12..];
    }
    if jieguo.is_empty() { None } else { Some(jieguo) }
}

/// 非流式 ReAct 单次调用，返回文本或工具调用
pub async fn putongqingqiu_react(peizhi: &Aipeizhi, guanli: &Xiaoxiguanli) -> Option<ReactJieguo> {
    let json = feiliushi_json(peizhi, guanli, true).await?;
    // 优先检查标准 tool_calls 字段
    if let Some(diaoyong_shuzu) = json["choices"][0]["message"]["tool_calls"].as_array() {
        if !diaoyong_shuzu.is_empty() {
            let diaoyong: Vec<llm::ToolCall> = diaoyong_shuzu.iter().filter_map(|tc| {
                Some(llm::ToolCall {
                    id: tc["id"].as_str()?.to_string(),
                    call_type: tc["type"].as_str().unwrap_or("function").to_string(),
                    function: llm::FunctionCall {
                        name: tc["function"]["name"].as_str()?.to_string(),
                        arguments: tc["function"]["arguments"].as_str()?.to_string(),
                    },
                })
            }).collect();
            if !diaoyong.is_empty() {
                return Some(ReactJieguo::Gongjudiaoyong(diaoyong));
            }
        }
    }
    // 兗底：从 content 中解析 <tool_call> 标签格式的工具调用
    if let Some(neirong) = tiqu_wenben(&json) {
        if neirong.contains("<tool_call>") {
            if let Some(diaoyong) = jiexi_wenben_gongjudiaoyong(&neirong) {
                return Some(ReactJieguo::Gongjudiaoyong(diaoyong));
            }
        }
        let sikao = tiqu_sikao(&json);
        return Some(ReactJieguo::Wenben { neirong, sikao });
    }
    // 兜底：移除工具重试
    println!("[ReAct] 响应无文本也无工具调用，移除工具做最终回复");
    if guanli.huoqu_gongjulie().is_some() {
        if let Some(json2) = feiliushi_json(peizhi, guanli, false).await {
            if let Some(wenben) = tiqu_wenben(&json2) {
                let sikao = tiqu_sikao(&json2);
                return Some(ReactJieguo::Wenben { neirong: wenben, sikao });
            }
        }
    }
    println!("[ReAct] 所有重试均失败");
    None
}

/// 流式调用，返回 reqwest 字节流响应
/// dai_gongju 控制是否携带工具定义，最终流式输出阶段应传 false
pub async fn liushiqingqiu(peizhi: &Aipeizhi, guanli: &Xiaoxiguanli, dai_gongju: bool) -> Option<reqwest::Response> {
    let ti = goujian_qingqiuti(peizhi, guanli, true, dai_gongju);
    fasong_qingqiu(peizhi, &ti).await
}
