use crate::peizhixt::peizhi_nr::peizhi_ai::Ai;
use crate::peizhixt::peizhixitongzhuti;
use serde_json::Value;

use super::gongyong::{ai_putongqingqiu_wenben, jinghua_json_huifu};

/// 跨日报交流内容分析：输入交流内容列表，AI 输出结构化分析 JSON
pub async fn ai_jiaoliu_fenxi(jiaoliuneirong_lie: &[Value]) -> Option<String> {
    if jiaoliuneirong_lie.is_empty() {
        return None;
    }
    let peizhi = peizhixitongzhuti::duqupeizhi::<Ai>(Ai::wenjianming()).unwrap_or_default();
    let tishici = &peizhi.jiaoliu_fenxi_tishici;
    if tishici.is_empty() {
        println!("[交流分析] jiaoliu_fenxi_tishici 未配置");
        return None;
    }

    let mut neirong_duanlie: Vec<String> = Vec::new();
    for xiang in jiaoliuneirong_lie {
        let riqi = xiang.get("riqi").and_then(|v| v.as_str()).unwrap_or("未知日期");
        let neirong = xiang.get("neirong").and_then(|v| v.as_str()).unwrap_or("");
        if !neirong.is_empty() {
            neirong_duanlie.push(format!("[{}] {}", riqi, neirong));
        }
    }
    if neirong_duanlie.is_empty() {
        return None;
    }

    let yonghuxiaoxi = format!(
        "以下是按时间排列的交流内容记录（共{}条）：\n\n{}",
        neirong_duanlie.len(),
        neirong_duanlie.join("\n")
    );
    println!("[交流分析] 发送 {} 条交流记录给 AI", neirong_duanlie.len());

    let huifu = ai_putongqingqiu_wenben(tishici, yonghuxiaoxi, 120).await?;
    let jinghua = jinghua_json_huifu(&huifu);
    // 验证是合法 JSON
    serde_json::from_str::<Value>(jinghua).ok()?;
    Some(jinghua.to_string())
}

/// 深度分析：输入完整日报原文 + 分析维度，AI 输出该维度的深度分析 JSON
pub async fn ai_ribao_shendu_fenxi(ribao_neirong: &str, weidu: &str) -> Option<String> {
    if ribao_neirong.is_empty() {
        return None;
    }
    let peizhi = peizhixitongzhuti::duqupeizhi::<Ai>(Ai::wenjianming()).unwrap_or_default();
    let jichuci = &peizhi.jiaoliu_fenxi_tishici;
    let json_geshi = r#"输出JSON必须严格使用以下结构（只输出一个JSON对象，不要用Report等包裹）：
{"zhutihuizong":[{"zhuti":"主题名","cishu":数字,"miaoshu":"描述"}],"yanbianguiji":"演变轨迹描述","guanjianwenti":[{"wenti":"问题描述","yanzhongchengdu":"高/中/低"}],"jianyi":"1.建议一 2.建议二"}
字段说明：zhutihuizong=主题汇总,yanbianguiji=演变轨迹,guanjianwenti=关键问题,jianyi=建议。所有字段名必须用拼音，不要用英文。"#;
    let xitong_tishici = if jichuci.is_empty() {
        format!(
            "你是一名资深项目分析专家。你需要从「{}」这个维度对日报内容进行深度分析。\n\
            要求：1)输出必须是合法JSON; 2)所有分析必须基于日报原文事实; 3)内容要有实际价值，不要泛泛而谈; 4)使用中文。\n\n{}",
            weidu, json_geshi
        )
    } else {
        format!(
            "{}\n\n当前分析维度：{}。请专注于此维度进行深度分析。\n\n{}",
            jichuci, weidu, json_geshi
        )
    };

    let yonghuxiaoxi = format!(
        "分析维度：{}\n\n以下是相关日报原文，请从此维度进行深度分析：\n\n{}",
        weidu, ribao_neirong
    );
    println!("[深度分析] 维度={} 输入长度={}", weidu, ribao_neirong.len());

    let huifu = ai_putongqingqiu_wenben(&xitong_tishici, yonghuxiaoxi, 180).await?;
    let jinghua = jinghua_json_huifu(&huifu);
    serde_json::from_str::<Value>(jinghua).ok()?;
    Some(jinghua.to_string())
}

/// 跨日报项目关联分析：输入多项目标签聚合数据，AI 输出项目关联分析 JSON
pub async fn ai_xiangmu_guanlian_fenxi(xiangmu_shuju: &Value) -> Option<String> {
    let peizhi = peizhixitongzhuti::duqupeizhi::<Ai>(Ai::wenjianming()).unwrap_or_default();
    let tishici = &peizhi.xiangmu_guanlian_tishici;
    if tishici.is_empty() {
        println!("[项目关联分析] xiangmu_guanlian_tishici 未配置");
        return None;
    }

    let yonghuxiaoxi = format!(
        "以下是多个项目的标签聚合数据，请分析项目之间的关联关系：\n\n{}",
        serde_json::to_string_pretty(xiangmu_shuju).unwrap_or_default()
    );
    println!("[项目关联分析] 发送项目数据给 AI");

    let huifu = ai_putongqingqiu_wenben(tishici, yonghuxiaoxi, 120).await?;
    let jinghua = jinghua_json_huifu(&huifu);
    serde_json::from_str::<Value>(jinghua).ok()?;
    Some(jinghua.to_string())
}
