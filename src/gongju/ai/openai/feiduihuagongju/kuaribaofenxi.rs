use crate::peizhixt::peizhi_nr::peizhi_ai::Ai;
use crate::peizhixt::peizhixitongzhuti;
use serde_json::Value;

use super::ai_zhixingqi::zhixing_ai_json_jianbian;

/// 跨日报交流内容分析：输入交流内容列表，AI 输出结构化分析 JSON
pub async fn ai_jiaoliu_fenxi(jiaoliuneirong_lie: &[Value]) -> Option<String> {
    if jiaoliuneirong_lie.is_empty() {
        return None;
    }
    let peizhi = peizhixitongzhuti::duqupeizhi::<Ai>(Ai::wenjianming()).unwrap_or_default();

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

    zhixing_ai_json_jianbian(&peizhi.jiaoliu_fenxi_tishici, yonghuxiaoxi, 120, "交流分析").await
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

    zhixing_ai_json_jianbian(&xitong_tishici, yonghuxiaoxi, 180, &format!("深度分析-{}", weidu)).await
}

/// 跨日报项目关联分析：输入多项目标签聚合数据，AI 输出项目关联分析 JSON
pub async fn ai_xiangmu_guanlian_fenxi(xiangmu_shuju: &Value) -> Option<String> {
    let peizhi = peizhixitongzhuti::duqupeizhi::<Ai>(Ai::wenjianming()).unwrap_or_default();

    let yonghuxiaoxi = format!(
        "以下是多个项目的标签聚合数据，请分析项目之间的关联关系：\n\n{}",
        serde_json::to_string_pretty(xiangmu_shuju).unwrap_or_default()
    );

    zhixing_ai_json_jianbian(&peizhi.xiangmu_guanlian_tishici, yonghuxiaoxi, 120, "项目关联分析").await
}

/// 深度关联分析：标签聚合 + 日报原文 + 用户自定义提示，大幅提升分析深度
pub async fn ai_guanlian_shendu_fenxi(xiangmu_shuju: &Value, ribao_neirong: &str, yonghu_tishi: &str) -> Option<String> {
    let peizhi = peizhixitongzhuti::duqupeizhi::<Ai>(Ai::wenjianming()).unwrap_or_default();
    let jichu = &peizhi.xiangmu_guanlian_tishici;

    let json_geshi = r#"输出JSON必须严格使用以下结构（只输出一个JSON对象）：
{"xiangmuguanxi":[{"xm1":"实体1","xm2":"实体2","guanxi":"关系描述","gongxiangziyuan":["人/客户/技术"],"miaoshu":"详细分析"}],"fengxiantishi":["风险描述"],"zhutihuizong":[{"zhuti":"主题名","miaoshu":"描述","shejiXiangmu":["实体名"]}],"guanjianwenti":[{"neirong":"问题描述","shejiXiangmu":["实体名"],"yanzhongchengdu":"高/中/低"}],"jianyi":"1.建议一 2.建议二"}
所有字段名必须用拼音，不要用英文。内容必须基于日报原文事实，不要泛泛而谈。"#;

    let xitong_tishici = if jichu.is_empty() {
        format!(
            "你是一名资深项目分析专家。你需要对多个实体进行深度关联分析。\n\
            要求：1)输出必须是合法JSON; 2)分析必须基于日报原文事实，引用具体细节; 3)内容要有实际价值，切入要深; 4)使用中文。\n\n{}",
            json_geshi
        )
    } else {
        format!(
            "{}\n\n请对以下实体进行深度关联分析。\n\n{}",
            jichu, json_geshi
        )
    };

    let mut yonghuxiaoxi = format!(
        "以下是实体的标签聚合数据：\n{}\n",
        serde_json::to_string_pretty(xiangmu_shuju).unwrap_or_default()
    );
    if !ribao_neirong.is_empty() {
        yonghuxiaoxi.push_str(&format!("\n以下是各实体关联的日报原文，请基于原文事实进行深度分析：\n\n{}", ribao_neirong));
    }
    if !yonghu_tishi.is_empty() {
        yonghuxiaoxi.push_str(&format!("\n\n用户特别关注的分析角度：{}", yonghu_tishi));
    }

    zhixing_ai_json_jianbian(&xitong_tishici, yonghuxiaoxi, 240, "深度关联分析").await
}
