use crate::peizhixt::peizhi_nr::peizhi_ai::Ai;
use crate::peizhixt::peizhi_nr::tishici_moban;
use crate::peizhixt::peizhixitongzhuti;
use serde_json::Value;

use super::ai_zhixingqi::zhixing_ai_json_jianbian;

#[allow(non_upper_case_globals)]
const jiaoliu_fenxi_chaoshi_miao: u64 = 120;
#[allow(non_upper_case_globals)]
const shendu_fenxi_chaoshi_miao: u64 = 180;
#[allow(non_upper_case_globals)]
const xiangmu_guanlian_chaoshi_miao: u64 = 120;
#[allow(non_upper_case_globals)]
const guanlian_shendu_chaoshi_miao: u64 = 240;

fn duqu_ai_peizhi() -> Ai {
    peizhixitongzhuti::duqupeizhi::<Ai>(Ai::wenjianming()).unwrap_or_default()
}

/// 跨日报交流内容分析：输入交流内容列表，AI 输出结构化分析 JSON
pub async fn ai_jiaoliu_fenxi(jiaoliuneirong_lie: &[Value]) -> Option<String> {
    if jiaoliuneirong_lie.is_empty() {
        return None;
    }
    let peizhi = duqu_ai_peizhi();

    let neirong_duanlie: Vec<String> = jiaoliuneirong_lie.iter().filter_map(|xiang| {
        let riqi = xiang.get("riqi").and_then(|v| v.as_str()).unwrap_or("未知日期");
        let neirong = xiang.get("neirong").and_then(|v| v.as_str()).unwrap_or("");
        (!neirong.is_empty()).then(|| format!("[{}] {}", riqi, neirong))
    }).collect();
    if neirong_duanlie.is_empty() {
        return None;
    }

    let yonghuxiaoxi = format!(
        "以下是按时间排列的交流内容记录（共{}条）：\n\n{}",
        neirong_duanlie.len(),
        neirong_duanlie.join("\n")
    );

    zhixing_ai_json_jianbian(&peizhi.jiaoliu_fenxi_tishici, yonghuxiaoxi, jiaoliu_fenxi_chaoshi_miao, "交流分析").await
}

/// 深度分析：输入完整日报原文 + 分析维度，AI 输出该维度的深度分析 JSON
pub async fn ai_ribao_shendu_fenxi(ribao_neirong: &str, weidu: &str) -> Option<String> {
    if ribao_neirong.is_empty() {
        return None;
    }
    let peizhi = duqu_ai_peizhi();
    let xitong_tishici = format!(
        "{}\n\n当前分析维度：{}。请专注于此维度进行深度分析。\n\n{}",
        peizhi.shendu_fenxi_tishici, weidu, tishici_moban::shendu_fenxi_json_geshi
    );

    let yonghuxiaoxi = format!(
        "分析维度：{}\n\n以下是相关日报原文，请从此维度进行深度分析：\n\n{}",
        weidu, ribao_neirong
    );

    zhixing_ai_json_jianbian(&xitong_tishici, yonghuxiaoxi, shendu_fenxi_chaoshi_miao, &format!("深度分析-{}", weidu)).await
}

/// 跨日报项目关联分析：输入多项目标签聚合数据，AI 输出项目关联分析 JSON
pub async fn ai_xiangmu_guanlian_fenxi(xiangmu_shuju: &Value) -> Option<String> {
    let peizhi = duqu_ai_peizhi();

    let yonghuxiaoxi = format!(
        "以下是多个项目的标签聚合数据，请分析项目之间的关联关系：\n\n{}",
        serde_json::to_string_pretty(xiangmu_shuju).unwrap_or_default()
    );

    zhixing_ai_json_jianbian(&peizhi.xiangmu_guanlian_tishici, yonghuxiaoxi, xiangmu_guanlian_chaoshi_miao, "项目关联分析").await
}

/// 深度关联分析：标签聚合 + 日报原文 + 用户自定义提示，大幅提升分析深度
pub async fn ai_guanlian_shendu_fenxi(xiangmu_shuju: &Value, ribao_neirong: &str, yonghu_tishi: &str) -> Option<String> {
    let peizhi = duqu_ai_peizhi();
    let xitong_tishici = format!(
        "{}\n\n请对以下实体进行深度关联分析。\n\n{}",
        peizhi.guanlian_shendu_tishici, tishici_moban::guanlian_shendu_json_geshi
    );

    let mut yonghuxiaoxi = format!(
        "以下是实体的标签聚合数据：\n{}",
        serde_json::to_string_pretty(xiangmu_shuju).unwrap_or_default()
    );
    if !ribao_neirong.is_empty() {
        yonghuxiaoxi.push_str(&format!("\n以下是各实体关联的日报原文，请基于原文事实进行深度分析：\n\n{}", ribao_neirong));
    }
    if !yonghu_tishi.is_empty() {
        yonghuxiaoxi.push_str(&format!("\n\n用户特别关注的分析角度：{}", yonghu_tishi));
    }

    zhixing_ai_json_jianbian(&xitong_tishici, yonghuxiaoxi, guanlian_shendu_chaoshi_miao, "深度关联分析").await
}
