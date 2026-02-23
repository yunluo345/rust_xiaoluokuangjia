pub mod jiekou_jiankang;
pub mod jiekou_jiamiceshi;
pub mod jiekou_sseceshi;
pub mod jiekou_jiamisseceshi;
pub mod jiekou_aiqudao;

use actix_web::web;
use crate::jiekouxt::jiekouxtzhuti::huoqufangfa;

/// 注册系统相关接口
pub fn zhuce(cfg: &mut web::ServiceConfig, qianzhui: &str) {
    cfg.service(
        web::scope(qianzhui)
            .route(jiekou_jiankang::dinyi.lujing, huoqufangfa(jiekou_jiankang::dinyi.fangshi)().to(jiekou_jiankang::chuli))
            .route(jiekou_jiamiceshi::dinyi.lujing, huoqufangfa(jiekou_jiamiceshi::dinyi.fangshi)().to(jiekou_jiamiceshi::chuli))
            .route(jiekou_sseceshi::dinyi.lujing, huoqufangfa(jiekou_sseceshi::dinyi.fangshi)().to(jiekou_sseceshi::chuli))
            .route(jiekou_jiamisseceshi::dinyi.lujing, huoqufangfa(jiekou_jiamisseceshi::dinyi.fangshi)().to(jiekou_jiamisseceshi::chuli))
            .route(jiekou_aiqudao::dinyi.lujing, huoqufangfa(jiekou_aiqudao::dinyi.fangshi)().to(jiekou_aiqudao::chuli))
    );
}
