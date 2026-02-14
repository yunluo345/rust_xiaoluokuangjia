pub mod jiekou_aiduihua;
pub mod jiekou_aiqudao;

use actix_web::web;
use crate::jiekouxt::jiekouxtzhuti::huoqufangfa;

pub fn zhuce(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/ai")
            .route(jiekou_aiduihua::dinyi.lujing, huoqufangfa(jiekou_aiduihua::dinyi.fangshi)().to(jiekou_aiduihua::chuli))
            .route(jiekou_aiqudao::dinyi.lujing, huoqufangfa(jiekou_aiqudao::dinyi.fangshi)().to(jiekou_aiqudao::chuli))
    );
}
