use llm::chat::MessageType;
use super::aipeizhi::Aipeizhi;
use super::aixiaoxiguanli::Xiaoxiguanli;

/// ReAct 单次调用结果
pub enum ReactJieguo {
    /// AI 返回了纯文本回复
    Wenben(String),
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

/// 发送 HTTP 请求（含重试），返回成功的原始响应
async fn fasong_qingqiu(peizhi: &Aipeizhi, ti: &serde_json::Value) -> Option<reqwest::Response> {
    let dizhi = format!("{}/chat/completions", peizhi.jiekoudizhi.trim_end_matches('/'));
    let chaoshi = std::time::Duration::from_secs(peizhi.chaoshishijian);
    for cishu in 0..=peizhi.chongshicishu {
        println!("[OpenAI] 尝试第 {} 次调用", cishu + 1);
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
            }
            Err(e) => println!("[OpenAI] 请求异常: {}", e),
        }
    }
    None
}

/// 从 AI 响应 JSON 中提取文本内容（过滤空文本）
fn tiqu_wenben(json: &serde_json::Value) -> Option<String> {
    json["choices"][0]["message"]["content"].as_str()
        .map(|s| s.to_string())
        .filter(|s| !s.trim().is_empty())
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

/// 非流式调用
pub async fn putongqingqiu(peizhi: &Aipeizhi, guanli: &Xiaoxiguanli) -> Option<String> {
    println!("[OpenAI] 非流式调用 模型:{} 接口:{}", peizhi.moxing, peizhi.jiekoudizhi);
    let json = feiliushi_json(peizhi, guanli, false).await?;
    let wenben = tiqu_wenben(&json)?;
    println!("[OpenAI] 非流式调用成功");
    Some(wenben)
}

/// 非流式 ReAct 单次调用，返回文本或工具调用
pub async fn putongqingqiu_react(peizhi: &Aipeizhi, guanli: &Xiaoxiguanli) -> Option<ReactJieguo> {
    println!("[ReAct] 非流式调用 模型:{} 接口:{}", peizhi.moxing, peizhi.jiekoudizhi);
    let json = feiliushi_json(peizhi, guanli, true).await?;
    // 优先检查工具调用
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
                println!("[ReAct] AI 请求调用 {} 个工具", diaoyong.len());
                return Some(ReactJieguo::Gongjudiaoyong(diaoyong));
            }
        }
    }
    // 检查文本回复
    if let Some(wenben) = tiqu_wenben(&json) {
        println!("[ReAct] AI 返回文本回复");
        return Some(ReactJieguo::Wenben(wenben));
    }
    // 兜底：移除工具重试
    println!("[ReAct] 响应无文本也无工具调用，移除工具做最终回复");
    if guanli.huoqu_gongjulie().is_some() {
        if let Some(json2) = feiliushi_json(peizhi, guanli, false).await {
            if let Some(wenben) = tiqu_wenben(&json2) {
                println!("[ReAct] 移除工具后成功获取文本回复");
                return Some(ReactJieguo::Wenben(wenben));
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
