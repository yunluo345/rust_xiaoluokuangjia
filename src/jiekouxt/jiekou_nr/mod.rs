pub mod xitong;
pub mod jiami;
pub mod yonghu;
pub mod ai;
pub mod ribao;

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
#[allow(non_upper_case_globals)]
const ribao_qianzhui: &str = "/ribao";

/// 注册所有接口模块的路由
pub fn zhuce(cfg: &mut web::ServiceConfig) {
    xitong::zhuce(cfg, xitong_qianzhui);
    jiami::zhuce(cfg, jiami_qianzhui);
    yonghu::zhuce(cfg, yonghu_qianzhui);
    ai::zhuce(cfg, ai_qianzhui);
    ribao::zhuce(cfg, ribao_qianzhui);
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
        Jiekouzhucexinxi { qianzhui: yonghu_qianzhui, dinyi: &yonghu::jiekou_yonghuguanli::dinyi },
        Jiekouzhucexinxi { qianzhui: ai_qianzhui, dinyi: &ai::jiekou_aiduihua::dinyi },
        Jiekouzhucexinxi { qianzhui: ai_qianzhui, dinyi: &ai::jiekou_aiduihualiushi::dinyi },
        Jiekouzhucexinxi { qianzhui: ai_qianzhui, dinyi: &ai::jiekou_aidiaoduqi::dinyi },
        Jiekouzhucexinxi { qianzhui: xitong_qianzhui, dinyi: &xitong::jiekou_aiqudao::dinyi },
        Jiekouzhucexinxi { qianzhui: ribao_qianzhui, dinyi: &ribao::jiekou_ribao::dinyi },
        Jiekouzhucexinxi { qianzhui: ribao_qianzhui, dinyi: &ribao::jiekou_ribao_yonghu::dinyi },
    ]
}
