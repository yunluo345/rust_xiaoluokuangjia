use actix_web::{HttpRequest, HttpResponse, web};
use serde::Deserialize;
use crate::gongju::jiamigongju;
use crate::jiekouxt::jiekouxtzhuti::{self, Jiekoudinyi, Qingqiufangshi};
use crate::jiekouxt::jiamichuanshu::jiamichuanshuzhongjian;
use crate::shujuku::psqlshujuku::shujubiao_nr::ai::shujucaozuo_aiqudao;
use crate::gongju::ai::openai::{aipeizhi, aixiaoxiguanli, openaizhuti};
use futures_core::Stream;
use std::pin::Pin;
use std::task::{Context, Poll};

#[allow(non_upper_case_globals)]
pub const dinyi: Jiekoudinyi = Jiekoudinyi {
    lujing: "/duihualiushi",
    nicheng: "AI对话流式",
    jieshao: "流式AI对话接口，自动选择可用渠道，实时推送响应",
    fangshi: Qingqiufangshi::Post,
    jiami: true,
    xudenglu: true,
    xuyonghuzu: false,
    yunxuputong: false,
};

#[derive(Deserialize)]
struct Xiaoxi {
    juese: String,
    neirong: String,
}

#[derive(Deserialize)]
struct Qingqiuti {
    xiaoxilie: Vec<Xiaoxi>,
}

fn jiami_sse(neirong: &str, miyao: &[u8]) -> String {
    let miwen = jiamigongju::jiami(neirong.as_bytes(), miyao)
        .map(|m| jiamigongju::zhuanbase64(&m))
        .unwrap_or_default();
    format!("data: {}\n\n", miwen)
}

fn cuowu_sse(xinxi: &str, miyao: &[u8]) -> HttpResponse {
    let neirong = serde_json::json!({"cuowu": xinxi}).to_string();
    HttpResponse::Ok()
        .content_type("text/event-stream")
        .insert_header(("Cache-Control", "no-cache"))
        .insert_header(("Connection", "keep-alive"))
        .body(jiami_sse(&neirong, miyao))
}

fn tiqu_wenben(json: &serde_json::Value) -> Option<&str> {
    json.pointer("/choices/0/delta/content")?.as_str()
}

struct Jiamiliushi {
    neiliu: Pin<Box<dyn Stream<Item = Result<actix_web::web::Bytes, reqwest::Error>> + Send>>,
    miyao: Vec<u8>,
    huanchong: String,
    jieshu: bool,
}

impl Stream for Jiamiliushi {
    type Item = Result<actix_web::web::Bytes, actix_web::Error>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();
        if this.jieshu {
            return Poll::Ready(None);
        }
        match this.neiliu.as_mut().poll_next(cx) {
            Poll::Ready(Some(Ok(shuju))) => {
                let wenben = String::from_utf8_lossy(&shuju);
                this.huanchong.push_str(&wenben);
                let mut shuchu = String::new();
                while let Some(weizhi) = this.huanchong.find("\n") {
                    let hang: String = this.huanchong.drain(..=weizhi).collect();
                    let hang = hang.trim();
                    if hang.is_empty() { continue; }
                    let shuju_str = hang.strip_prefix("data: ").unwrap_or(hang);
                    if shuju_str == "[DONE]" {
                        this.jieshu = true;
                        break;
                    }
                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(shuju_str) {
                        if let Some(neirong) = tiqu_wenben(&json) {
                            if !neirong.is_empty() {
                                let jiamishuju = serde_json::json!({"neirong": neirong}).to_string();
                                shuchu.push_str(&jiami_sse(&jiamishuju, &this.miyao));
                            }
                        }
                    }
                }
                if shuchu.is_empty() {
                    cx.waker().wake_by_ref();
                    Poll::Pending
                } else {
                    Poll::Ready(Some(Ok(actix_web::web::Bytes::from(shuchu))))
                }
            }
            Poll::Ready(Some(Err(e))) => {
                this.jieshu = true;
                let cuowu = serde_json::json!({"cuowu": format!("流式传输错误: {}", e)}).to_string();
                Poll::Ready(Some(Ok(actix_web::web::Bytes::from(jiami_sse(&cuowu, &this.miyao)))))
            }
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}

async fn chuliqingqiu(mingwen: &[u8], miyao: Vec<u8>) -> HttpResponse {
    let qingqiu: Qingqiuti = match serde_json::from_slice::<Qingqiuti>(mingwen) {
        Ok(q) if !q.xiaoxilie.is_empty() => q,
        Ok(_) => return cuowu_sse("消息列表不能为空", &miyao),
        Err(_) => return cuowu_sse("请求参数格式错误", &miyao),
    };

    let qudao = match shujucaozuo_aiqudao::suiji_huoqu_qudao("openapi").await {
        Some(q) => q,
        None => return cuowu_sse("暂无可用AI渠道", &miyao),
    };

    let peizhi = match aipeizhi::Aipeizhi::cong_qudaoshuju(&qudao) {
        Some(p) => p,
        None => return cuowu_sse("AI渠道配置错误", &miyao),
    };

    let mut guanli = aixiaoxiguanli::Xiaoxiguanli::xingjian();
    for xiaoxi in qingqiu.xiaoxilie {
        match xiaoxi.juese.as_str() {
            "user" => guanli.zhuijia_yonghuxiaoxi(xiaoxi.neirong),
            "assistant" => guanli.zhuijia_zhushouneirong(xiaoxi.neirong),
            _ => {}
        }
    }

    let xiangying = match openaizhuti::liushiqingqiu(&peizhi, &guanli).await {
        Some(x) => x,
        None => return cuowu_sse("AI服务调用失败", &miyao),
    };

    let liushi = Jiamiliushi {
        neiliu: Box::pin(xiangying.bytes_stream()),
        miyao,
        huanchong: String::new(),
        jieshu: false,
    };

    HttpResponse::Ok()
        .content_type("text/event-stream")
        .insert_header(("Cache-Control", "no-cache"))
        .insert_header(("Connection", "keep-alive"))
        .streaming(liushi)
}

pub async fn chuli(req: HttpRequest, ti: web::Bytes) -> HttpResponse {
    let miyao = match jiamichuanshuzhongjian::paishengyao(&req).await {
        Some(m) => m,
        None => return jiekouxtzhuti::shibai(401, "加密会话无效"),
    };
    match jiamichuanshuzhongjian::jiemiqingqiuti(&ti, &miyao) {
        Some(mingwen) => chuliqingqiu(&mingwen, miyao).await,
        None => jiekouxtzhuti::shibai(400, "解密请求体失败"),
    }
}
