pub mod xitong;
pub mod jiami;

use actix_web::web;
use super::jiekouxtzhuti::Jiekouzhucexinxi;

#[allow(non_upper_case_globals)]
const xitong_qianzhui: &str = "/jiekou/xitong";
#[allow(non_upper_case_globals)]
const jiami_qianzhui: &str = "/jiekou/jiami";

/// 注册所有接口模块的路由
pub fn zhuce(cfg: &mut web::ServiceConfig) {
    xitong::zhuce(cfg);
    jiami::zhuce(cfg);
}

/// 汇总所有接口定义，用于同步到数据库
pub fn huoqujiekoulie() -> Vec<Jiekouzhucexinxi> {
    vec![
        Jiekouzhucexinxi { qianzhui: xitong_qianzhui, dinyi: &xitong::jiekou_jiankang::dinyi },
        Jiekouzhucexinxi { qianzhui: xitong_qianzhui, dinyi: &xitong::jiekou_jiamiceshi::dinyi },
        Jiekouzhucexinxi { qianzhui: xitong_qianzhui, dinyi: &xitong::jiekou_sseceshi::dinyi },
        Jiekouzhucexinxi { qianzhui: xitong_qianzhui, dinyi: &xitong::jiekou_jiamisseceshi::dinyi },
        Jiekouzhucexinxi { qianzhui: jiami_qianzhui, dinyi: &jiami::jiekou_gongyao::dinyi },
    ]
}
