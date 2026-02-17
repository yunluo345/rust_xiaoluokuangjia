pub mod xitong;
pub mod jiami;
pub mod yonghu;
pub mod ai;

use actix_web::web;
use super::jiekouxtzhuti::Jiekouzhucexinxi;

#[allow(non_upper_case_globals)]
const xitong_qianzhui: &str = "/xitong";
#[allow(non_upper_case_globals)]
const jiami_qianzhui: &str = "/jiami";
#[allow(non_upper_case_globals)]
const yonghu_qianzhui: &str = "/yonghu";
#[allow(non_upper_case_globals)]
const ai_qianzhui: &str = "/ai";

/// 注册所有接口模块的路由
pub fn zhuce(cfg: &mut web::ServiceConfig) {
    xitong::zhuce(cfg, xitong_qianzhui);
    jiami::zhuce(cfg, jiami_qianzhui);
    yonghu::zhuce(cfg, yonghu_qianzhui);
    ai::zhuce(cfg, ai_qianzhui);
}

/// 汇总所有接口定义，用于同步到数据库
pub fn huoqujiekoulie() -> Vec<Jiekouzhucexinxi> {
    vec![
        Jiekouzhucexinxi { qianzhui: xitong_qianzhui, dinyi: &xitong::jiekou_jiankang::dinyi },
        Jiekouzhucexinxi { qianzhui: xitong_qianzhui, dinyi: &xitong::jiekou_jiamiceshi::dinyi },
        Jiekouzhucexinxi { qianzhui: xitong_qianzhui, dinyi: &xitong::jiekou_sseceshi::dinyi },
        Jiekouzhucexinxi { qianzhui: xitong_qianzhui, dinyi: &xitong::jiekou_jiamisseceshi::dinyi },
        Jiekouzhucexinxi { qianzhui: jiami_qianzhui, dinyi: &jiami::jiekou_gongyao::dinyi },
        Jiekouzhucexinxi { qianzhui: yonghu_qianzhui, dinyi: &yonghu::jiekou_denglu::dinyi },
        Jiekouzhucexinxi { qianzhui: ai_qianzhui, dinyi: &ai::jiekou_aiduihua::dinyi },
        Jiekouzhucexinxi { qianzhui: ai_qianzhui, dinyi: &ai::jiekou_aiduihualiushi::dinyi },
    ]
}
