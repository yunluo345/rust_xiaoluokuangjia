use actix_web::{HttpRequest, HttpResponse, web};
use crate::jiekouxt::jiekouxtzhuti::{self, Jiekoudinyi, Qingqiufangshi};
use crate::gongju::ai::openai::openaizhuti;
use futures_core::Stream;
use std::pin::Pin;
use std::task::{Context, Poll};

#[allow(non_upper_case_globals)]
pub const dinyi: Jiekoudinyi = Jiekoudinyi {
    lujing: "/duihualiushi",
    nicheng: "AI对话流式",
    jieshao: "流式AI对话接口，自动选择可用渠道，实时推送响应",
    fangshi: Qingqiufangshi::Post,
    jiami: false,
    xudenglu: true,
    xuyonghuzu: false,
    yunxuputong: false,
};


fn cuowu_sse(xinxi: &str) -> HttpResponse {
    let neirong = serde_json::json!({"cuowu": xinxi}).to_string();
    HttpResponse::Ok()
        .content_type("text/event-stream")
        .insert_header(("Cache-Control", "no-cache"))
        .insert_header(("Connection", "keep-alive"))
        .body(format!("data: {}\n\n", neirong))
}

fn tiqu_wenben(json: &serde_json::Value) -> Option<&str> {
    json.pointer("/choices/0/delta/content")?.as_str()
}

struct Liushi {
    neiliu: Pin<Box<dyn Stream<Item = Result<actix_web::web::Bytes, reqwest::Error>> + Send>>,
    huanchong: String,
    jieshu: bool,
    chushi: Option<String>,
}

impl Stream for Liushi {
    type Item = Result<actix_web::web::Bytes, actix_web::Error>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();
        if let Some(chushi) = this.chushi.take() {
            return Poll::Ready(Some(Ok(actix_web::web::Bytes::from(chushi))));
        }
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
                    let shuju_str = hang.strip_prefix("data:").unwrap_or(hang).trim_start();
                    if shuju_str == "[DONE]" {
                        this.jieshu = true;
                        break;
                    }
                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(shuju_str) {
                        if let Some(neirong) = tiqu_wenben(&json) {
                            if !neirong.is_empty() {
                                let shuju = serde_json::json!({"neirong": neirong}).to_string();
                                shuchu.push_str(&format!("data: {}\n\n", shuju));
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
                Poll::Ready(Some(Ok(actix_web::web::Bytes::from(format!("data: {}\n\n", cuowu)))))
            }
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}

async fn chuliqingqiu(ti: &[u8], lingpai: &str) -> HttpResponse {
    let qingqiu: super::Qingqiuti = match serde_json::from_slice::<super::Qingqiuti>(ti) {
        Ok(q) if !q.xiaoxilie.is_empty() => q,
        Ok(_) => return cuowu_sse("消息列表不能为空"),
        Err(_) => return cuowu_sse("请求参数格式错误"),
    };

    let peizhi = match super::huoqu_peizhi().await {
        Some(p) => p,
        None => return cuowu_sse("暂无可用AI渠道或配置错误"),
    };

    let benci_neirong = qingqiu.xiaoxilie.iter()
        .rev()
        .find(|x| x.juese == "user")
        .map(|x| x.neirong.as_str())
        .unwrap_or("");

    let (gongjulie, yitu_miaoshu) = super::huoqu_yitu_gongju(&peizhi, benci_neirong).await;
    println!("[AI对话流式] 意图: {} 工具数: {}", yitu_miaoshu, gongjulie.len());

    let mut guanli = super::goujian_guanli_anyitu(&qingqiu, gongjulie);

    if let Some(openaizhuti::ReactJieguo::Wenben(huifu)) =
        super::react_xunhuan(&peizhi, &mut guanli, "流式ReAct", lingpai, &qingqiu).await
    {
        let shuju = serde_json::json!({"neirong": huifu, "yitu": yitu_miaoshu}).to_string();
        return HttpResponse::Ok()
            .content_type("text/event-stream")
            .insert_header(("Cache-Control", "no-cache"))
            .insert_header(("Connection", "keep-alive"))
            .body(format!("data: {}\n\n", shuju));
    }

    let xiangying = match openaizhuti::liushiqingqiu(&peizhi, &guanli, false).await {
        Some(x) => x,
        None => return cuowu_sse("AI流式服务调用失败"),
    };

    let yitu_shuju = serde_json::json!({"yitu": yitu_miaoshu}).to_string();
    let chushi_sse = format!("data: {}\n\n", yitu_shuju);

    let liushi = Liushi {
        neiliu: Box::pin(xiangying.bytes_stream()),
        huanchong: String::new(),
        jieshu: false,
        chushi: Some(chushi_sse),
    };

    HttpResponse::Ok()
        .content_type("text/event-stream")
        .insert_header(("Cache-Control", "no-cache"))
        .insert_header(("Connection", "keep-alive"))
        .streaming(liushi)
}

pub async fn chuli(req: HttpRequest, ti: web::Bytes) -> HttpResponse {
    let lingpai = match jiekouxtzhuti::tiqulingpai(&req) {
        Some(l) => {
            println!("[AI对话流式] 用户令牌: {}", l);
            l
        }
        None => return jiekouxtzhuti::shibai(401, "缺少授权令牌"),
    };
    
    println!("[AI对话流式] 前端请求内容: {}", String::from_utf8_lossy(&ti));
    if let Ok(json) = serde_json::from_slice::<serde_json::Value>(&ti) {
        if let Some(zuihou) = json["xiaoxilie"].as_array().and_then(|arr| arr.last()) {
            println!("[AI对话流式] 本次发送内容: {}", zuihou["neirong"].as_str().unwrap_or(""));
        }
    }
    
    chuliqingqiu(&ti, &lingpai).await
}
