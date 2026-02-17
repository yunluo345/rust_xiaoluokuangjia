pub mod jiekou_aiduihua;
pub mod jiekou_aiduihualiushi;

use actix_web::web;
use crate::jiekouxt::jiekouxtzhuti::huoqufangfa;
use crate::jiekouxt::quanxianyanzheng::quanxianyanzhengzhongjian::Quanxianyanzheng;

pub fn zhuce(cfg: &mut web::ServiceConfig, qianzhui: &str) {
    cfg.service(
        web::scope(qianzhui)
            .wrap(Quanxianyanzheng)
            .route(jiekou_aiduihua::dinyi.lujing, huoqufangfa(jiekou_aiduihua::dinyi.fangshi)().to(jiekou_aiduihua::chuli))
            .route(jiekou_aiduihualiushi::dinyi.lujing, huoqufangfa(jiekou_aiduihualiushi::dinyi.fangshi)().to(jiekou_aiduihualiushi::chuli))
    );
}
