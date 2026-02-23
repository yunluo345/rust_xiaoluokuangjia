pub mod jiekou_ribao_yonghu;
pub mod jiekou_ribao;

use actix_web::web;
use crate::jiekouxt::jiekouxtzhuti::huoqufangfa;

/// 注册日报相关接口
pub fn zhuce(cfg: &mut web::ServiceConfig, qianzhui: &str) {
    cfg.service(
        web::scope(qianzhui)
            .route(jiekou_ribao_yonghu::dinyi.lujing, huoqufangfa(jiekou_ribao_yonghu::dinyi.fangshi)().to(jiekou_ribao_yonghu::chuli))
            .route(jiekou_ribao::dinyi.lujing, huoqufangfa(jiekou_ribao::dinyi.fangshi)().to(jiekou_ribao::chuli))
    );
}
