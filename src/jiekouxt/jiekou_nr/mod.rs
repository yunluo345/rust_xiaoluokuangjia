pub mod xitong;

use actix_web::web;
use super::jiekouxtzhuti::Jiekouzhucexinxi;

#[allow(non_upper_case_globals)]
const xitong_qianzhui: &str = "/xitong";

/// 注册所有接口模块的路由
pub fn zhuce(cfg: &mut web::ServiceConfig) {
    xitong::zhuce(cfg);
}

/// 汇总所有接口定义，用于同步到数据库
pub fn huoqujiekoulie() -> Vec<Jiekouzhucexinxi> {
    vec![
        Jiekouzhucexinxi { qianzhui: xitong_qianzhui, dinyi: &xitong::jiekou_jiankang::dinyi },
    ]
}
