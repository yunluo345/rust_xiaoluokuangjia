pub mod jiekou_gongyao;

use actix_web::web;
use crate::jiekouxt::jiekouxtzhuti::huoqufangfa;

/// 注册加密相关接口
pub fn zhuce(cfg: &mut web::ServiceConfig, qianzhui: &str) {
    cfg.service(
        web::scope(qianzhui)
            .route(jiekou_gongyao::dinyi.lujing, huoqufangfa(jiekou_gongyao::dinyi.fangshi)().to(jiekou_gongyao::chuli))
    );
}
