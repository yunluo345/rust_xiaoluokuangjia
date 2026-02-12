use actix_web::HttpResponse;
use crate::jiekouxt::jiekouxtzhuti::{Jiekoudinyi, Qingqiufangshi};
use tokio_stream::StreamExt;

#[allow(non_upper_case_globals)]
pub const dinyi: Jiekoudinyi = Jiekoudinyi {
    lujing: "/sseceshi",
    nicheng: "SSE测试",
    jieshao: "普通SSE流式推送测试，逐条发送5条消息",
    fangshi: Qingqiufangshi::Sse,
    jiami: false,
    xudenglu: false,
    xuyonghuzu: false,
    yunxuputong: false,
};

#[allow(non_upper_case_globals)]
const tuisongcishu: usize = 5;
#[allow(non_upper_case_globals)]
const tuisongjiange_haomiao: u64 = 500;

fn shengchengliushi() -> impl futures_core::Stream<Item = Result<actix_web::web::Bytes, actix_web::Error>> {
    let mut jishu = 0usize;
    tokio_stream::wrappers::IntervalStream::new(actix_web::rt::time::interval(std::time::Duration::from_millis(tuisongjiange_haomiao)))
        .take(tuisongcishu)
        .map(move |_| {
            jishu += 1;
            let shuju = serde_json::json!({
                "xulie": jishu,
                "neirong": format!("第{}条SSE消息", jishu),
                "zongji": tuisongcishu
            });
            Ok(actix_web::web::Bytes::from(format!("data: {}\n\n", shuju)))
        })
}

pub async fn chuli() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/event-stream")
        .insert_header(("Cache-Control", "no-cache"))
        .insert_header(("Connection", "keep-alive"))
        .streaming(shengchengliushi())
}
