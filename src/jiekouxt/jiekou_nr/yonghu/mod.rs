pub mod jiekou_denglu;

use actix_web::web;
use crate::jiekouxt::jiekouxtzhuti::huoqufangfa;

/// 注册用户相关接口
pub fn zhuce(cfg: &mut web::ServiceConfig, qianzhui: &str) {
    cfg.service(
        web::scope(qianzhui)
            .route(jiekou_denglu::dinyi.lujing, huoqufangfa(jiekou_denglu::dinyi.fangshi)().to(jiekou_denglu::chuli))
    );
}
