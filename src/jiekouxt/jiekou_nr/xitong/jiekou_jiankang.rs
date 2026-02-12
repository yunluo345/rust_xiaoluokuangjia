use actix_web::HttpResponse;
use serde::Serialize;
use crate::jiekouxt::jiekouxtzhuti::{self, Jiekoudinyi, Qingqiufangshi};

#[allow(non_upper_case_globals)]
pub const dinyi: Jiekoudinyi = Jiekoudinyi {
    lujing: "/jiankang",
    nicheng: "健康检查",
    jieshao: "返回服务运行状态和当前时间戳",
    fangshi: Qingqiufangshi::Get,
    jiami: false,
    xudenglu: false,
    xuyonghuzu: false,
    yunxuputong: false,
};

#[derive(Serialize)]
struct Jiankangshuju {
    zhuangtai: &'static str,
}

/// 健康检查接口，返回服务运行状态
pub async fn chuli() -> HttpResponse {
    jiekouxtzhuti::chenggong("服务运行正常", Jiankangshuju { zhuangtai: "正常" })
}
