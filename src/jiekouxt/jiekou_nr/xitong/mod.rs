pub mod jiekou_jiankang;
pub mod jiekou_jiamiceshi;
pub mod jiekou_sseceshi;
pub mod jiekou_jiamisseceshi;

use actix_web::web;
use crate::jiekouxt::jiekouxtzhuti::huoqufangfa;

/// 注册系统相关接口
pub fn zhuce(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/xitong")
            .route(jiekou_jiankang::dinyi.lujing, huoqufangfa(jiekou_jiankang::dinyi.fangshi)().to(jiekou_jiankang::chuli))
            .route(jiekou_jiamiceshi::dinyi.lujing, huoqufangfa(jiekou_jiamiceshi::dinyi.fangshi)().to(jiekou_jiamiceshi::chuli))
            .route(jiekou_sseceshi::dinyi.lujing, huoqufangfa(jiekou_sseceshi::dinyi.fangshi)().to(jiekou_sseceshi::chuli))
            .route(jiekou_jiamisseceshi::dinyi.lujing, huoqufangfa(jiekou_jiamisseceshi::dinyi.fangshi)().to(jiekou_jiamisseceshi::chuli))
    );
}
