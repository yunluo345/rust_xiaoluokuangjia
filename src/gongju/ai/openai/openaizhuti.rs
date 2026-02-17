use llm::builder::LLMBuilder;
use llm::chat::ChatProvider;
use llm::LLMProvider;
use super::aipeizhi::Aipeizhi;
use super::aixiaoxiguanli::Xiaoxiguanli;

#[allow(non_upper_case_globals)]
const zuida_lingpaishu: u32 = 4096;

fn goujianshili(peizhi: &Aipeizhi, tishici: Option<&str>) -> Option<Box<dyn LLMProvider>> {
    let mut builder = LLMBuilder::new()
        .backend(peizhi.leixing.clone())
        .api_key(&peizhi.miyao)
        .model(&peizhi.moxing)
        .temperature(peizhi.wendu)
        .max_tokens(zuida_lingpaishu);
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
        if !xiaoxi.content.is_empty() {
            xiaoxilie.push(serde_json::json!({"role": juese, "content": xiaoxi.content}));
        }
    }
    xiaoxilie
}

/// 流式调用，返回 reqwest 字节流响应
pub async fn liushiqingqiu(peizhi: &Aipeizhi, guanli: &Xiaoxiguanli) -> Option<reqwest::Response> {
    let wanzhengdizhi = format!("{}/chat/completions", peizhi.jiekoudizhi.trim_end_matches('/'));
    let xiaoxilie = goujian_xiaoxiti(guanli);
    let qingqiuti = serde_json::json!({
        "model": peizhi.moxing,
        "messages": xiaoxilie,
        "stream": true,
        "max_tokens": zuida_lingpaishu,
        "temperature": peizhi.wendu,
    });
    let chaoshi = std::time::Duration::from_secs(peizhi.chaoshishijian);
    for cishu in 0..=peizhi.chongshicishu {
        println!("[OpenAI] 尝试第 {} 次流式调用 -> {}", cishu + 1, wanzhengdizhi);
        let jieguo = reqwest::Client::new()
            .post(&wanzhengdizhi)
            .header("Authorization", format!("Bearer {}", peizhi.miyao))
            .header("Content-Type", "application/json")
            .timeout(chaoshi)
            .json(&qingqiuti)
            .send()
            .await;
        match jieguo {
            Ok(xiangying) if xiangying.status().is_success() => return Some(xiangying),
            Ok(xiangying) => {
                println!("[OpenAI] 请求失败，状态码: {}", xiangying.status());
            }
            Err(e) => {
                println!("[OpenAI] 请求异常: {}", e);
            }
        }
    }
    None
}
