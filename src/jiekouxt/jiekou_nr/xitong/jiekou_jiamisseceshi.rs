use actix_web::{HttpRequest, HttpResponse, web};
use crate::gongju::jiamigongju;
use crate::jiekouxt::jiekouxtzhuti::{self, Jiekoudinyi, Qingqiufangshi};
use crate::jiekouxt::jiamichuanshu::jiamichuanshuzhongjian;
use tokio_stream::StreamExt;

#[allow(non_upper_case_globals)]
pub const dinyi: Jiekoudinyi = Jiekoudinyi {
    lujing: "/jiamisseceshi",
    nicheng: "加密SSE测试",
    jieshao: "加密SSE流式推送测试，每条消息独立加密",
    fangshi: Qingqiufangshi::Post,
    jiami: true,
    xudenglu: false,
    xuyonghuzu: false,
    yunxuputong: false,
};

#[allow(non_upper_case_globals)]
const tuisongcishu: usize = 5;
#[allow(non_upper_case_globals)]
const tuisongjiange_haomiao: u64 = 500;

fn shengchengjiamiliushi(miyao: Vec<u8>) -> impl futures_core::Stream<Item = Result<actix_web::web::Bytes, actix_web::Error>> {
    let mut jishu = 0usize;
    tokio_stream::wrappers::IntervalStream::new(actix_web::rt::time::interval(std::time::Duration::from_millis(tuisongjiange_haomiao)))
        .take(tuisongcishu)
        .map(move |_| {
            jishu += 1;
            let mingwen = serde_json::json!({
                "xulie": jishu,
                "neirong": format!("第{}条加密SSE消息", jishu),
                "zongji": tuisongcishu
            }).to_string();
            let miwen = jiamigongju::jiami(mingwen.as_bytes(), &miyao)
                .map(|m| jiamigongju::zhuanbase64(&m))
                .unwrap_or_default();
            Ok(actix_web::web::Bytes::from(format!("data: {}\n\n", miwen)))
        })
}

pub async fn chuli(req: HttpRequest, _ti: web::Bytes) -> HttpResponse {
    let miyao = match jiamichuanshuzhongjian::paishengyao(&req).await {
        Some(m) => m,
        None => return jiekouxtzhuti::shibai(401, "加密会话无效"),
    };
    HttpResponse::Ok()
        .content_type("text/event-stream")
        .insert_header(("Cache-Control", "no-cache"))
        .insert_header(("Connection", "keep-alive"))
        .streaming(shengchengjiamiliushi(miyao))
}
