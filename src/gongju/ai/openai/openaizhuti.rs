use futures_core::Stream;
use llm::builder::LLMBuilder;
use llm::chat::{ChatProvider, ChatResponse, StreamChunk};
use llm::{LLMProvider, ToolCall};
use std::pin::Pin;
use std::future::Future;
use tokio::sync::mpsc;
use tokio_stream::StreamExt;
use super::aipeizhi::Aipeizhi;
use super::aixiaoxiguanli::Xiaoxiguanli;
use super::liushishijian::Liushishijian;

#[allow(non_upper_case_globals)]
const zuida_lingpaishu: u32 = 4096;
#[allow(non_upper_case_globals)]
const zuida_gongjuxunhuancishu: usize = 10;

pub enum Aijieguo {
    Wenben(String),
    Gongjudiaoyong(Vec<ToolCall>),
}

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

async fn daichaoshiqingqiu(peizhi: &Aipeizhi, guanli: &mut Xiaoxiguanli) -> Option<Box<dyn ChatResponse>> {
    guanli.yasuo(peizhi.zuidatoken);
    let chaoshi = std::time::Duration::from_secs(peizhi.chaoshishijian);
    for _ in 0..=peizhi.chongshicishu {
        let shili = goujianshili(peizhi, guanli.huoqu_xitongtishici())?;
        if let Ok(Ok(xiangying)) = actix_web::rt::time::timeout(
            chaoshi,
            shili.chat_with_tools(guanli.huoqu_xiaoxilie(), guanli.huoqu_gongjulie()),
        ).await {
            return Some(xiangying);
        }
    }
    None
}

pub async fn putongqingqiu(peizhi: &Aipeizhi, guanli: &mut Xiaoxiguanli) -> Option<String> {
    daichaoshiqingqiu(peizhi, guanli).await?.text()
}

pub async fn gongjuqingqiu(peizhi: &Aipeizhi, guanli: &mut Xiaoxiguanli) -> Option<Aijieguo> {
    let xiangying = daichaoshiqingqiu(peizhi, guanli).await?;
    if let Some(diaoyong) = xiangying.tool_calls() {
        if !diaoyong.is_empty() {
            return Some(Aijieguo::Gongjudiaoyong(diaoyong));
        }
    }
    xiangying.text().map(Aijieguo::Wenben)
}

pub async fn gongjuxunhuan<F, Fut>(
    peizhi: &Aipeizhi,
    guanli: &mut Xiaoxiguanli,
    zhixingqi: F,
) -> Option<String>
where
    F: Fn(Vec<ToolCall>) -> Fut,
    Fut: Future<Output = Vec<ToolCall>>,
{
    for _ in 0..zuida_gongjuxunhuancishu {
        match gongjuqingqiu(peizhi, guanli).await? {
            Aijieguo::Wenben(wenben) => return Some(wenben),
            Aijieguo::Gongjudiaoyong(diaoyong) => {
                guanli.zhuijia_zhushougongjudiaoyong(diaoyong.clone());
                let jieguo = zhixingqi(diaoyong).await;
                guanli.zhuijia_gongjujieguo(jieguo);
            }
        }
    }
    None
}

type Liushiliu = Pin<Box<dyn Stream<Item = Result<StreamChunk, llm::error::LLMError>> + Send>>;

async fn daichaoshiliushi(peizhi: &Aipeizhi, guanli: &mut Xiaoxiguanli) -> Option<Liushiliu> {
    guanli.yasuo(peizhi.zuidatoken);
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

pub async fn liushiqingqiu(peizhi: &Aipeizhi, guanli: &mut Xiaoxiguanli) -> Option<Liushiliu> {
    daichaoshiliushi(peizhi, guanli).await
}

fn zhuanhuan_liushikuai(kuai: StreamChunk) -> Liushishijian {
    match kuai {
        StreamChunk::Text(neirong) => Liushishijian::Wenbenkuai { neirong },
        StreamChunk::ToolUseStart { index, id, name } => {
            Liushishijian::Gongjukaishi { suoyin: index, gongjuid: id, gongjuming: name }
        }
        StreamChunk::ToolUseInputDelta { index, partial_json } => {
            Liushishijian::Gongjucanshu { suoyin: index, bufen_json: partial_json }
        }
        StreamChunk::ToolUseComplete { index: _, tool_call } => {
            Liushishijian::Gongjuwancheng {
                suoyin: 0,
                gongjuid: tool_call.id.clone(),
                gongjuming: tool_call.function.name.clone(),
                canshu: tool_call.function.arguments.clone(),
            }
        }
        StreamChunk::Done { stop_reason } => Liushishijian::Wancheng { yuanyin: stop_reason },
    }
}

async fn xiaofei_liushi(liu: Liushiliu, fasongqi: &mpsc::Sender<Liushishijian>) -> Vec<ToolCall> {
    let mut gongjulie: Vec<ToolCall> = Vec::new();
    let mut liu = liu;
    while let Some(Ok(kuai)) = liu.next().await {
        if let StreamChunk::ToolUseComplete { index: _, ref tool_call } = kuai {
            gongjulie.push(tool_call.clone());
        }
        let _ = fasongqi.send(zhuanhuan_liushikuai(kuai)).await;
    }
    gongjulie
}

pub async fn liushigongjuxunhuan<F, Fut>(
    peizhi: &Aipeizhi,
    guanli: &mut Xiaoxiguanli,
    zhixingqi: F,
    fasongqi: mpsc::Sender<Liushishijian>,
) where
    F: Fn(Vec<ToolCall>) -> Fut,
    Fut: Future<Output = Vec<ToolCall>>,
{
    for _ in 0..zuida_gongjuxunhuancishu {
        let liu = match daichaoshiliushi(peizhi, guanli).await {
            Some(l) => l,
            None => {
                let _ = fasongqi.send(Liushishijian::Cuowu {
                    xinxi: "获取AI流失败".to_string(),
                }).await;
                return;
            }
        };
        let gongjulie = xiaofei_liushi(liu, &fasongqi).await;
        if gongjulie.is_empty() {
            return;
        }
        guanli.zhuijia_zhushougongjudiaoyong(gongjulie.clone());
        let jieguo = zhixingqi(gongjulie).await;
        for gc in &jieguo {
            let _ = fasongqi.send(Liushishijian::Gongjujieguo {
                gongjuid: gc.id.clone(),
                gongjuming: gc.function.name.clone(),
                jieguo: gc.function.arguments.clone(),
            }).await;
        }
        guanli.zhuijia_gongjujieguo(jieguo);
    }
}
