use crate::peizhixt::peizhi_nr::peizhi_ai::Ai;
use crate::peizhixt::peizhixitongzhuti;
use serde_json::{json, Value};

use super::gongyong::{ai_putongqingqiu_wenben, jinghua_json_huifu};

#[allow(non_upper_case_globals)]
const biaoti_chaoshi_miao: u64 = 30;
#[allow(non_upper_case_globals)]
const zhaiyao_chaoshi_miao: u64 = 60;
#[allow(non_upper_case_globals)]
const siweidaotu_chaoshi_miao: u64 = 120;

fn duqu_ai_peizhi() -> Ai {
    peizhixitongzhuti::duqupeizhi::<Ai>(Ai::wenjianming()).unwrap_or_default()
}

/// AI生成日报标题
pub async fn ai_shengcheng_biaoti(neirong: &str) -> Option<String> {
    let peizhi = duqu_ai_peizhi();
    let huifu = ai_putongqingqiu_wenben(
        &peizhi.biaoti_shengcheng_tishici,
        format!("请为以下日报生成标题：\n\n{}", neirong),
        biaoti_chaoshi_miao,
    ).await?;
    let biaoti = huifu.trim().to_string();
    println!("[标题生成] {}", biaoti);
    (!biaoti.is_empty()).then_some(biaoti)
}

/// AI生成日报摘要
pub async fn ai_shengcheng_zhaiyao(neirong: &str) -> Option<String> {
    let peizhi = duqu_ai_peizhi();
    let huifu = ai_putongqingqiu_wenben(
        &peizhi.zhaiyao_shengcheng_tishici,
        format!("请为以下日报生成摘要：\n\n{}", neirong),
        zhaiyao_chaoshi_miao,
    ).await?;
    let zhaiyao = huifu.trim().to_string();
    println!("[摘要生成] {}", zhaiyao.chars().take(60).collect::<String>());
    (!zhaiyao.is_empty()).then_some(zhaiyao)
}

/// 构建思维导图提示词
fn goujian_siweidaotu_tishici(peizhi: &Ai) -> String {
    let zijiedian: Vec<Value> = peizhi.siweidaotu_weidu.iter().map(|wd| {
        let zi: Vec<Value> = wd.zijiedian.iter()
            .map(|zj| json!({"mingcheng": zj.mingcheng, "neirong": zj.neirong}))
            .collect();
        json!({"mingcheng": wd.mingcheng, "zijiedian": zi})
    }).collect();

    let lizi = serde_json::to_string_pretty(&json!({
        "mingcheng": "日报分析",
        "zijiedian": zijiedian
    })).unwrap_or_default();

    let mut zhuyixiang: Vec<String> = vec![
        "所有节点名称用中文".to_string(),
        "neirong 字段必须基于日报实际内容分析，不要编造".to_string(),
        "如果日报中没有某方面的信息，neirong 写\"日报未提及\"".to_string(),
    ];

    peizhi.siweidaotu_weidu.iter()
        .filter(|wd| !wd.beizhu.is_empty())
        .for_each(|wd| zhuyixiang.push(format!("{}节点{}", wd.mingcheng, wd.beizhu)));

    zhuyixiang.push("只返回JSON，不要返回其他内容".to_string());

    let zhuyi = zhuyixiang.iter()
        .enumerate()
        .map(|(i, x)| format!("{}. {}", i + 1, x))
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        "你是日报深度分析助手。根据日报内容生成一份思维导图JSON，对日报进行全面分析。\n\
        返回纯 JSON，不要包含 markdown 代码块标记。\n\
        结构要求：\n{}\n\
        注意：\n{}",
        lizi, zhuyi
    )
}

/// AI生成思维导图
pub async fn ai_shengcheng_siweidaotu(neirong: &str, peizhi: &Ai) -> Option<String> {
    let xitongtishici = goujian_siweidaotu_tishici(peizhi);
    let huifu = ai_putongqingqiu_wenben(
        &xitongtishici,
        format!("请对以下日报进行全面分析并生成思维导图：\n\n{}", neirong),
        siweidaotu_chaoshi_miao,
    ).await?;
    let jinghua = jinghua_json_huifu(&huifu);

    match serde_json::from_str::<Value>(jinghua) {
        Ok(_) => {
            println!("[思维导图] 生成成功 长度={}", jinghua.len());
            Some(jinghua.to_string())
        }
        Err(e) => {
            println!("[思维导图] JSON解析失败: {} 原文={}", e, jinghua.chars().take(80).collect::<String>());
            None
        }
    }
}
