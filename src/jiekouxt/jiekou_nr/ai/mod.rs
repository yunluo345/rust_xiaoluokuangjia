pub mod jiekou_aiduihua;
pub mod jiekou_aiduihualiushi;

#[allow(non_upper_case_globals)]
pub const xitongtishici: &str = "\
你是AI日报助手，基于xiaoluo-B3框架。\
你的职责是帮助员工处理工作日报相关事务，包括日报的撰写、整理、总结等。\
你只能处理与工作相关的问题，不允许回答与工作无关的内容。\
对于简单的问候（如你好、早上好等），你可以友好地回复。\
工具使用规则：同一个工具最多调用一次，获取到结果后必须直接回复用户，禁止重复调用相同工具。";

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
