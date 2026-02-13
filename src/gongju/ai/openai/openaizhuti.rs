use futures_core::Stream;
use llm::builder::LLMBuilder;
use llm::chat::{ChatProvider, StreamChunk};
use llm::LLMProvider;
use std::pin::Pin;
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
    for _ in 0..=peizhi.chongshicishu {
        let shili = goujianshili(peizhi, guanli.huoqu_xitongtishici())?;
        let jieguo = actix_web::rt::time::timeout(
            chaoshi,
            shili.chat_with_tools(guanli.huoqu_xiaoxilie(), guanli.huoqu_gongjulie()),
        ).await;
        if let Ok(Ok(xiangying)) = jieguo {
            if let Some(wenben) = xiangying.text() {
                return Some(wenben);
            }
        }
    }
    None
}

type Liushiliu = Pin<Box<dyn Stream<Item = Result<StreamChunk, llm::error::LLMError>> + Send>>;

/// 流式调用，返回 StreamChunk 流
pub async fn liushiqingqiu(peizhi: &Aipeizhi, guanli: &Xiaoxiguanli) -> Option<Liushiliu> {
    let chaoshi = std::time::Duration::from_secs(peizhi.chaoshishijian);
    for _ in 0..=peizhi.chongshicishu {
        let shili = goujianshili(peizhi, guanli.huoqu_xitongtishici())?;
        if let Ok(Ok(liu)) = actix_web::rt::time::timeout(
            chaoshi,
            shili.chat_stream_with_tools(guanli.huoqu_xiaoxilie(), guanli.huoqu_gongjulie()),
        ).await {
            return Some(liu);
        }
    }
    None
}
