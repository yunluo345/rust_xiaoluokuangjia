use actix_web::HttpResponse;
use serde_json::json;
use crate::gongju::jichugongju;
use crate::jiekouxt::jiekouxtzhuti::{Jiekoudinyi, Qingqiufangshi};

#[allow(non_upper_case_globals)]
pub const dinyi: Jiekoudinyi = Jiekoudinyi {
    lujing: "/jiankang",
    nicheng: "健康检查",
    jieshao: "返回服务运行状态和当前时间戳",
    fangshi: Qingqiufangshi::Get,
    jiami: false,
    xudenglu: false,
    xuyonghuzu: false,
    yunxuputong: true,
};

/// 健康检查接口，返回服务运行状态和当前时间戳
pub async fn chuli() -> HttpResponse {
    HttpResponse::Ok().json(json!({
        "zhuangtai": "正常",
        "shijianchuo": jichugongju::huoqushijianchuo()
    }))
}
