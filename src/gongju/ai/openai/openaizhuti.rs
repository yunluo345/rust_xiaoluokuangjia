use llm::builder::LLMBuilder;
use llm::chat::{ChatProvider, ChatResponse, MessageType};
use llm::LLMProvider;
use super::aipeizhi::Aipeizhi;
use super::aixiaoxiguanli::Xiaoxiguanli;

/// ReAct 单次调用结果
pub enum ReactJieguo {
    /// AI 返回了纯文本回复
    Wenben(String),
    /// AI 要求调用工具
    Gongjudiaoyong(Vec<llm::ToolCall>),
}

fn goujianshili(peizhi: &Aipeizhi, tishici: Option<&str>) -> Option<Box<dyn LLMProvider>> {
    let mut builder = LLMBuilder::new()
        .backend(peizhi.leixing.clone())
        .api_key(&peizhi.miyao)
        .model(&peizhi.moxing)
        .temperature(peizhi.wendu);
    if peizhi.zuida_token > 0 {
        builder = builder.max_tokens(peizhi.zuida_token);
    }
    if !peizhi.jiekoudizhi.is_empty() {
        builder = builder.base_url(&peizhi.jiekoudizhi);
    }
    if let Some(t) = tishici {
        builder = builder.system(t);
    }
    builder.build().ok()
}

/// 非流式调用
pub async fn putongqingqiu(peizhi: &Aipeizhi, guanli: &Xiaoxiguanli) -> Option<String> {
    let chaoshi = std::time::Duration::from_secs(peizhi.chaoshishijian);
    println!("[OpenAI] 非流式调用 模型:{} 接口:{}", peizhi.moxing, peizhi.jiekoudizhi);
    for cishu in 0..=peizhi.chongshicishu {
        let shili = match goujianshili(peizhi, guanli.huoqu_xitongtishici()) {
            Some(s) => s,
            None => {
                println!("[OpenAI] 第 {} 次构建实例失败", cishu + 1);
                continue;
            }
        };
        println!("[OpenAI] 尝试第 {} 次非流式调用", cishu + 1);
        let jieguo = actix_web::rt::time::timeout(
            chaoshi,
            shili.chat_with_tools(guanli.huoqu_xiaoxilie(), guanli.huoqu_gongjulie()),
        ).await;
        match &jieguo {
            Ok(Ok(xiangying)) => {
                if let Some(wenben) = xiangying.text() {
                    println!("[OpenAI] 非流式调用成功");
                    return Some(wenben);
                }
                println!("[OpenAI] 响应无文本内容");
            }
            Ok(Err(e)) => println!("[OpenAI] 调用失败: {:?}", e),
            Err(_) => println!("[OpenAI] 调用超时"),
        }
    }
    println!("[OpenAI] 所有重试均失败");
    None
}

/// 非流式 ReAct 单次调用，返回文本或工具调用
pub async fn putongqingqiu_react(peizhi: &Aipeizhi, guanli: &Xiaoxiguanli) -> Option<ReactJieguo> {
    let chaoshi = std::time::Duration::from_secs(peizhi.chaoshishijian);
    println!("[ReAct] 非流式调用 模型:{} 接口:{}", peizhi.moxing, peizhi.jiekoudizhi);
    for cishu in 0..=peizhi.chongshicishu {
        let shili = match goujianshili(peizhi, guanli.huoqu_xitongtishici()) {
            Some(s) => s,
            None => {
                println!("[ReAct] 第 {} 次构建实例失败", cishu + 1);
                continue;
            }
        };
        println!("[ReAct] 尝试第 {} 次调用", cishu + 1);
        let jieguo = actix_web::rt::time::timeout(
            chaoshi,
            shili.chat_with_tools(guanli.huoqu_xiaoxilie(), guanli.huoqu_gongjulie()),
        ).await;
        match jieguo {
            Ok(Ok(xiangying)) => {
                if let Some(diaoyong) = xiangying.tool_calls() {
                    if !diaoyong.is_empty() {
                        println!("[ReAct] AI 请求调用 {} 个工具", diaoyong.len());
                        return Some(ReactJieguo::Gongjudiaoyong(diaoyong));
                    }
                }
                if let Some(wenben) = xiangying.text() {
                    println!("[ReAct] AI 返回文本回复");
                    return Some(ReactJieguo::Wenben(wenben));
                }
                println!("[ReAct] 响应无文本也无工具调用");
            }
            Ok(Err(e)) => println!("[ReAct] 调用失败: {:?}", e),
            Err(_) => println!("[ReAct] 调用超时"),
        }
    }
    println!("[ReAct] 所有重试均失败");
    None
}

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

/// 流式调用，返回 reqwest 字节流响应
/// dai_gongju 控制是否携带工具定义，最终流式输出阶段应传 false
pub async fn liushiqingqiu(peizhi: &Aipeizhi, guanli: &Xiaoxiguanli, dai_gongju: bool) -> Option<reqwest::Response> {
    let wanzhengdizhi = format!("{}/chat/completions", peizhi.jiekoudizhi.trim_end_matches('/'));
    let xiaoxilie = goujian_xiaoxiti(guanli);
    let mut qingqiuti = serde_json::json!({
        "model": peizhi.moxing,
        "messages": xiaoxilie,
        "stream": true,
        "temperature": peizhi.wendu,
    });
    if peizhi.zuida_token > 0 {
        qingqiuti["max_tokens"] = serde_json::json!(peizhi.zuida_token);
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
            qingqiuti["tools"] = serde_json::json!(tools);
        }
    }
    let chaoshi = std::time::Duration::from_secs(peizhi.chaoshishijian);
    for cishu in 0..=peizhi.chongshicishu {
        println!("[OpenAI] 尝试第 {} 次流式调用 -> {}", cishu + 1, wanzhengdizhi);
        let jieguo = reqwest::Client::builder()
            .no_proxy()
            .build()
            .unwrap()
            .post(&wanzhengdizhi)
            .header("Authorization", format!("Bearer {}", peizhi.miyao))
            .header("Content-Type", "application/json")
            .timeout(chaoshi)
            .json(&qingqiuti)
            .send()
            .await;
        match jieguo {
            Ok(xiangying) if xiangying.status().is_success() => {
                println!("[OpenAI] 流式请求成功，状态码: {}", xiangying.status());
                return Some(xiangying);
            }
            Ok(xiangying) => {
                let zhuangtai = xiangying.status();
                let neirong = xiangying.text().await.unwrap_or_default();
                println!("[OpenAI] 请求失败，状态码: {}，响应: {}", zhuangtai, neirong);
            }
            Err(e) => {
                println!("[OpenAI] 请求异常: {}", e);
            }
        }
    }
    None
}
