pub mod jiekou_denglu;
pub mod jiekou_aiqudao;

use actix_web::web;
use crate::jiekouxt::jiekouxtzhuti::huoqufangfa;

/// 注册用户相关接口
pub fn zhuce(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/yonghu")
            .route(jiekou_denglu::dinyi.lujing, huoqufangfa(jiekou_denglu::dinyi.fangshi)().to(jiekou_denglu::chuli))
            .route(jiekou_aiqudao::dinyi.lujing, huoqufangfa(jiekou_aiqudao::dinyi.fangshi)().to(jiekou_aiqudao::chuli))
    );
}
