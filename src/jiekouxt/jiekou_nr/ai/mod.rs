pub mod jiekou_aiduihua;
pub mod jiekou_aiduihualiushi;
pub mod ceshi;

use serde::Deserialize;
use crate::gongju::ai::openai::{aipeizhi, aixiaoxiguanli, gongjuji, openaizhuti};
use crate::gongju::ai::openai::openaizhuti::ReactJieguo;
use crate::shujuku::psqlshujuku::shujubiao_nr::ai::shujucaozuo_aiqudao;
use crate::peizhixt::peizhixitongzhuti;
use crate::peizhixt::peizhi_nr::peizhi_ai::Ai;

#[allow(non_upper_case_globals)]
pub const xitongtishici: &str = "\
你是AI日报助手，基于xiaoluo-B3框架。\
你的职责是帮助员工处理工作日报相关事务，包括日报的撰写、整理、总结等。\
你只能处理与工作相关的问题，不允许回答与工作无关的内容。\
对于简单的问候（如你好、早上好等），你可以友好地回复。\
工具使用规则：同一个工具最多调用一次，获取到结果后必须直接回复用户，禁止重复调用相同工具。";

#[allow(non_upper_case_globals)]
pub const yitu_tishici: &str = "\
你是意图分析助手。根据用户消息判断意图类型。\
只返回JSON，不要返回其他任何内容。\
格式：{\"leixing\":\"gongjudiaoyong\"或\"putongduihua\",\"guanjianci\":\"提取的关键词\"}\
- gongjudiaoyong：用户需要查询数据、执行操作、管理系统、检查或验证或审核日报或文档等（如查时间、管理渠道、检查日报、验证日报是否合格等）\
- putongduihua：普通问候、闲聊、知识问答等\
注意：只要用户消息中包含检查、验证、审核、日报等操作性词语，优先判断为gongjudiaoyong。";

#[allow(non_upper_case_globals)]
const chongfu_yuzhi: u32 = 2;

#[derive(Deserialize)]
pub struct Xiaoxi {
    pub juese: String,
    pub neirong: String,
}

#[derive(Deserialize)]
pub struct Qingqiuti {
    pub xiaoxilie: Vec<Xiaoxi>,
}

pub struct YituJieguo {
    pub leixing: String,
    pub guanjianci: String,
    pub yuanwen: String,
}

/// 构建消息管理器：设置系统提示词、注册工具、填充历史消息
pub fn goujian_guanli(qingqiu: &Qingqiuti) -> aixiaoxiguanli::Xiaoxiguanli {
    let mut guanli = aixiaoxiguanli::Xiaoxiguanli::xingjian()
        .shezhi_xitongtishici(xitongtishici);
    for gongju in gongjuji::huoqu_suoyougongju() {
        guanli = guanli.tianjia_gongju(gongju);
    }
    for xiaoxi in &qingqiu.xiaoxilie {
        match xiaoxi.juese.as_str() {
            "user" => guanli.zhuijia_yonghuxiaoxi(&xiaoxi.neirong),
            "assistant" => guanli.zhuijia_zhushouneirong(&xiaoxi.neirong),
            _ => {}
        }
    }
    guanli
}

/// 根据意图构建消息管理器：设置系统提示词、注册筛选后的工具、填充历史消息
pub fn goujian_guanli_anyitu(qingqiu: &Qingqiuti, gongjulie: Vec<llm::chat::Tool>) -> aixiaoxiguanli::Xiaoxiguanli {
    let mut guanli = aixiaoxiguanli::Xiaoxiguanli::xingjian()
        .shezhi_xitongtishici(xitongtishici);
    for gongju in gongjulie {
        guanli = guanli.tianjia_gongju(gongju);
    }
    for xiaoxi in &qingqiu.xiaoxilie {
        match xiaoxi.juese.as_str() {
            "user" => guanli.zhuijia_yonghuxiaoxi(&xiaoxi.neirong),
            "assistant" => guanli.zhuijia_zhushouneirong(&xiaoxi.neirong),
            _ => {}
        }
    }
    guanli
}

/// 获取渠道并解析配置
pub async fn huoqu_peizhi() -> Option<aipeizhi::Aipeizhi> {
    let qudao = shujucaozuo_aiqudao::suiji_huoqu_qudao("openapi").await?;
    println!("获取到的渠道数据: {}", qudao);
    aipeizhi::Aipeizhi::cong_qudaoshuju(&qudao)
}

/// 意图分析：用AI判断用户本次消息的意图
async fn fenxi_yitu(peizhi: &aipeizhi::Aipeizhi, xiaoxilie: &[Xiaoxi]) -> Option<YituJieguo> {
    let mut guanli = aixiaoxiguanli::Xiaoxiguanli::xingjian()
        .shezhi_xitongtishici(yitu_tishici);
    for xiaoxi in xiaoxilie {
        match xiaoxi.juese.as_str() {
            "user" => guanli.zhuijia_yonghuxiaoxi(&xiaoxi.neirong),
            "assistant" => guanli.zhuijia_zhushouneirong(&xiaoxi.neirong),
            _ => {}
        }
    }
    let benci_neirong = xiaoxilie.last().map(|x| x.neirong.as_str()).unwrap_or("");
    println!("[意图分析] 开始分析: {}", benci_neirong);
    let huifu = openaizhuti::putongqingqiu(peizhi, &guanli).await?;
    println!("[意图分析] AI返回: {}", huifu);
    let jinghua = huifu.trim()
        .trim_start_matches("```json")
        .trim_start_matches("```")
        .trim_end_matches("```")
        .trim();
    if let Ok(json) = serde_json::from_str::<serde_json::Value>(jinghua) {
        let leixing = json["leixing"].as_str().unwrap_or("putongduihua").to_string();
        let guanjianci = json["guanjianci"].as_str().unwrap_or("").to_string();
        Some(YituJieguo { leixing, guanjianci, yuanwen: huifu })
    } else {
        println!("[意图分析] JSON解析失败");
        None
    }
}

/// 优先用 AI 关键词匹配工具，匹配不到再用原文兜底，都没有则返回全部工具
fn zhineng_tiqu_gongju_youxian(guanjianci: &str, yuanwen: &str) -> Vec<llm::chat::Tool> {
    let gongju = gongjuji::zhineng_tiqu_gongju(guanjianci);
    if !gongju.is_empty() {
        return gongju;
    }
    let gongju = gongjuji::zhineng_tiqu_gongju(yuanwen);
    if !gongju.is_empty() {
        return gongju;
    }
    gongjuji::huoqu_suoyougongju()
}

/// 意图分析 + 工具筛选：先AI分析，失败则降级关键词匹配，再失败则无工具
pub async fn huoqu_yitu_gongju(peizhi: &aipeizhi::Aipeizhi, xiaoxilie: &[Xiaoxi]) -> (Vec<llm::chat::Tool>, String) {
    let benci_neirong = xiaoxilie.last().map(|x| x.neirong.as_str()).unwrap_or("");
    // 1. 尝试AI意图分析
    if let Some(yitu) = fenxi_yitu(peizhi, xiaoxilie).await {
        if yitu.leixing == "gongjudiaoyong" {
            let gongju = zhineng_tiqu_gongju_youxian(&yitu.guanjianci, benci_neirong);
            println!("[意图] 工具调用，匹配到 {} 个工具", gongju.len());
            return (gongju, format!("工具调用: {}", yitu.guanjianci));
        }
        println!("[意图] 普通对话");
        return (vec![], "普通对话".to_string());
    }
    // 2. AI分析失败，降级为直接关键词匹配
    println!("[意图] AI分析失败，降级关键词匹配");
    let gongju = gongjuji::zhineng_tiqu_gongju(benci_neirong);
    if !gongju.is_empty() {
        println!("[意图] 降级匹配到 {} 个工具", gongju.len());
        return (gongju, "工具调用(降级匹配)".to_string());
    }
    // 3. 都失败，无工具直接对话
    println!("[意图] 降级无结果，普通对话");
    (vec![], "普通对话(降级)".to_string())
}

/// 并发执行工具调用
async fn zhixing_gongjudiaoyong(qz: &str, lie: &[llm::ToolCall], lingpai: &str) -> Vec<llm::ToolCall> {
    let renwu: Vec<_> = lie.iter().map(|d| {
        let mut d = d.clone();
        let qz = qz.to_string();
        let lingpai = lingpai.to_string();
        async move {
            println!("[{}] 执行工具: {} 参数: {}", qz, d.function.name, d.function.arguments);
            d.function.arguments = gongjuji::zhixing(&d.function.name, &d.function.arguments, &lingpai).await;
            d
        }
    }).collect();
    futures::future::join_all(renwu).await
}

/// 工具调用签名，用于重复检测
fn gongju_qianming(lie: &[llm::ToolCall]) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    for d in lie {
        d.function.name.hash(&mut h);
        d.function.arguments.hash(&mut h);
    }
    h.finish()
}

/// ReAct循环：处理工具调用，返回最终结果（成功为Some，失败为None）
pub async fn react_xunhuan(
    peizhi: &aipeizhi::Aipeizhi,
    guanli: &mut aixiaoxiguanli::Xiaoxiguanli,
    qz: &str,
    lingpai: &str,
    _qingqiu: &Qingqiuti,
) -> Option<ReactJieguo> {
    let zuida = peizhixitongzhuti::duqupeizhi::<Ai>(Ai::wenjianming())
        .map(|p| p.zuida_xunhuancishu).unwrap_or(20);
    let mut shangci_hash: u64 = 0;
    let mut chongfu: u32 = 0;

    for cishu in 1..=zuida {
        println!("[{}] 第 {} 轮循环", qz, cishu);
        match openaizhuti::putongqingqiu_react(peizhi, guanli).await {
            Some(ReactJieguo::Wenben(huifu)) => return Some(ReactJieguo::Wenben(huifu)),
            Some(ReactJieguo::Gongjudiaoyong(lie)) => {
                let hash = gongju_qianming(&lie);
                if hash == shangci_hash && shangci_hash != 0 {
                    chongfu += 1;
                    if chongfu >= chongfu_yuzhi {
                        println!("[{}] 工具重复调用 {} 次，终止", qz, chongfu + 1);
                        return None;
                    }
                } else {
                    chongfu = 0;
                }
                shangci_hash = hash;
                guanli.zhuijia_zhushou_gongjudiaoyong(lie.clone());
                guanli.zhuijia_gongjujieguo(zhixing_gongjudiaoyong(qz, &lie, lingpai).await);
            }
            None => return None,
        }
    }
    None
}

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
