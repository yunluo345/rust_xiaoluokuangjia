use crate::gongju::ai::openai::{aixiaoxiguanli, openaizhuti};
use crate::peizhixt::peizhi_nr::peizhi_ai::Ai;
use serde_json::Value;
use std::collections::{HashMap, HashSet};

use super::gongyong::jinghua_json_huifu;

/// AI标签提取
pub async fn ai_tiqu_biaoqian(neirong: &str, peizhi: &Ai, yiyou_biaoqian: Option<&str>) -> Option<Vec<(String, String)>> {
    let biaoqian_tishi = peizhi.ribao_biaoqian.iter()
        .map(|bq| {
            let biecheng_str = bq.biecheng.join("、");
            let qianzhui = [
                bq.bitian.then_some("【必填】"),
                bq.duozhi.then_some("【多值，用数组】"),
            ].into_iter().flatten().collect::<String>();
            format!("{}{}（{}，别名：{}）", qianzhui, bq.mingcheng, bq.miaoshu, biecheng_str)
        })
        .collect::<Vec<_>>()
        .join("；");

    let yiyou_tishi = yiyou_biaoqian
        .map(|s| format!("\n\n该日报已有以下标签：\n{}\n如果已有标签已经覆盖了某个分类，且内容准确，则不要重复返回该标签。只返回需要新增或更正的标签。", s))
        .unwrap_or_default();

    let xitongtishici = format!(
        "你是日报标签提取助手。从日报内容中提取以下标签信息：{}\n\
        请仔细阅读日报内容，提取所有相关标签。\n\
        返回JSON格式：{{\"标签名1\": \"值1\", \"标签名2\": \"值2\"}}\n\
        如果某个标签有多个值（如多个人名），必须使用数组：{{\"xxx\": [\"aaa\", \"bbb\"]}}\n\
        注意：\n\
        1. 标签名必须使用配置中的标准名称（不要使用别名）\n\
        2. 如果日报中没有某个标签的信息，不要返回该标签\n\
        3. 标记了【多值】的标签，每个值必须单独一个数组元素，不要用逗号拼接\n\
        4. 不要使用代词（如「我」「他」「她」「你」「对方」等）作为人名，必须提取实际姓名；如果日报中未提及真实姓名，则不要返回该标签\n\
        5. 即使标签标注了【必填】，如果日报中确实没有相关信息，也绝对不要编造或使用占位值（如Report 1、项目1、客户A等），直接不返回该标签\n\
        6. 只返回JSON，不要返回其他内容{}",
        biaoqian_tishi, yiyou_tishi
    );

    let aipeizhi = match crate::jiekouxt::jiekou_nr::ai::huoqu_peizhi().await {
        Some(p) => p.shezhi_chaoshi(60).shezhi_chongshi(1),
        None => {
            println!("[标签提取] AI配置获取失败");
            return None;
        }
    };

    let mut guanli = aixiaoxiguanli::Xiaoxiguanli::xingjian()
        .shezhi_xitongtishici(&xitongtishici);
    guanli.zhuijia_yonghuxiaoxi(format!("请从以下日报中提取标签：\n\n{}", neirong));

    let huifu = match openaizhuti::putongqingqiu(&aipeizhi, &guanli).await {
        Some(h) => {
            let xianshi: String = h.chars().take(100).collect();
            println!("[标签提取] AI返回: {}{}",  xianshi, if h.chars().count() > 100 { "..." } else { "" });
            h
        }
        None => {
            println!("[标签提取] AI调用失败，返回None");
            return None;
        }
    };

    let tiquxiang = tichubiaoqianxiang(&huifu, peizhi);
    println!("[标签提取] 解析结果: {} 个标签", tiquxiang.len());
    (!tiquxiang.is_empty()).then_some(tiquxiang)
}

/// 将 JSON 值拆分为多条文本（数组展开、分隔符拆分、数字转字符串）
fn chaifenzhi(zhi: &Value, duozhi: bool) -> Vec<String> {
    match zhi {
        Value::Array(shuzu) => shuzu.iter()
            .filter_map(|x| x.as_str().map(|s| s.trim().to_string()))
            .filter(|s| !s.is_empty())
            .collect(),
        Value::String(s) if duozhi => s.split(&[',', '、', '，'][..])
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(String::from)
            .collect(),
        _ => zhi.as_str().map(|s| s.trim().to_string())
            .or_else(|| zhi.as_i64().map(|n| n.to_string()))
            .or_else(|| zhi.as_u64().map(|n| n.to_string()))
            .filter(|s| !s.is_empty())
            .into_iter()
            .collect(),
    }
}

/// 从文本中提取标签项
pub fn tichubiaoqianxiang(neirong: &str, peizhi: &Ai) -> Vec<(String, String)> {
    let neirong = jinghua_json_huifu(neirong);
    let mut biezhuan_duoduiyi: HashMap<String, HashSet<String>> = HashMap::new();
    let mut duozhiji: HashSet<String> = HashSet::new();
    let mut zhuce_bieming = |biecheng: &str, biaozhunmingcheng: &str| {
        let jian = biecheng.trim();
        if jian.is_empty() {
            return;
        }
        biezhuan_duoduiyi
            .entry(jian.to_string())
            .or_default()
            .insert(biaozhunmingcheng.to_string());
    };
    for biaoqian in &peizhi.ribao_biaoqian {
        zhuce_bieming(&biaoqian.mingcheng, &biaoqian.mingcheng);
        zhuce_bieming(&biaoqian.miaoshu, &biaoqian.mingcheng);
        if biaoqian.duozhi {
            duozhiji.insert(biaoqian.mingcheng.clone());
        }
        for biecheng in &biaoqian.biecheng {
            zhuce_bieming(biecheng, &biaoqian.mingcheng);
        }
    }

    let mut biezhuan: HashMap<String, String> = HashMap::new();
    let mut chongtu_bieming: Vec<String> = Vec::new();
    for (biecheng, biaozhunjihe) in biezhuan_duoduiyi {
        match biaozhunjihe.len() {
            1 => {
                if let Some(biaozhun) = biaozhunjihe.into_iter().next() {
                    biezhuan.insert(biecheng, biaozhun);
                }
            }
            n if n > 1 => chongtu_bieming.push(biecheng),
            _ => {}
        }
    }
    if !chongtu_bieming.is_empty() {
        chongtu_bieming.sort();
        println!("[标签提取] 检测到别名冲突，已忽略: {}", chongtu_bieming.join("、"));
    }

    let mut jieguo: Vec<(String, String)> = Vec::new();
    let mut quchong: HashSet<String> = HashSet::new();

    let charu = |biaozhun: &str, wenzi: String, quchong: &mut HashSet<String>, jieguo: &mut Vec<(String, String)>| {
        let jian = format!("{}|{}", biaozhun, wenzi);
        if !wenzi.is_empty() && quchong.insert(jian) {
            jieguo.push((biaozhun.to_string(), wenzi));
        }
    };

    if let Ok(Value::Object(duixiang)) = serde_json::from_str::<Value>(neirong) {
        for (leixing, zhi) in duixiang {
            let biaozhun = match biezhuan.get(leixing.trim()) {
                Some(v) => v.clone(),
                None => continue,
            };
            for wenzi in chaifenzhi(&zhi, duozhiji.contains(&biaozhun)) {
                charu(&biaozhun, wenzi, &mut quchong, &mut jieguo);
            }
        }
        if !jieguo.is_empty() {
            return jieguo;
        }
    }

    for hang in neirong.lines().map(str::trim).filter(|s| !s.is_empty()) {
        if let Some((leixing, zhi)) = hang
            .split_once('：')
            .or_else(|| hang.split_once(':'))
            .map(|(l, z)| (l.trim(), z.trim()))
        {
            let biaozhun = match biezhuan.get(leixing) {
                Some(v) => v.clone(),
                None => continue,
            };
            let zhilie: Vec<&str> = match duozhiji.contains(&biaozhun) {
                true => zhi.split(&[',', '、', '，'][..]).map(str::trim).filter(|s| !s.is_empty()).collect(),
                false => vec![zhi].into_iter().filter(|s| !s.is_empty()).collect(),
            };
            for pian in zhilie {
                charu(&biaozhun, pian.to_string(), &mut quchong, &mut jieguo);
            }
        }
    }

    if !jieguo.is_empty() {
        return jieguo;
    }

    let mut qita: Vec<(String, String, usize)> = Vec::new();
    for (biecheng, biaozhun) in &biezhuan {
        let biaodianlie = ["：", ":"];
        for biaodian in biaodianlie {
            let qianzhui = format!("{}{}", biecheng, biaodian);
            let mut qidian = 0usize;
            while qidian < neirong.len() {
                let pianyi = match neirong[qidian..].find(&qianzhui) {
                    Some(v) => v,
                    None => break,
                };
                let kaishi = qidian + pianyi;
                let zhi_kaishi = kaishi + qianzhui.len();
                qita.push((biaozhun.clone(), biecheng.clone(), zhi_kaishi));
                qidian = kaishi + qianzhui.len();
            }
        }
    }

    qita.sort_by(|a, b| a.2.cmp(&b.2));

    for i in 0..qita.len() {
        let (biaozhun, _, zhi_kaishi) = &qita[i];
        let mut zhi_jieshu = neirong.len();
        if i + 1 < qita.len() {
            zhi_jieshu = qita[i + 1].2;
        }
        if *zhi_kaishi >= zhi_jieshu || zhi_jieshu > neirong.len() {
            continue;
        }
        let zhi = neirong[*zhi_kaishi..zhi_jieshu]
            .trim()
            .trim_start_matches('：')
            .trim_start_matches(':')
            .trim();
        if zhi.is_empty() {
            continue;
        }
        let zhilie: Vec<&str> = match duozhiji.contains(biaozhun) {
            true => zhi.split(&[',', '、', '，'][..]).map(str::trim).filter(|s| !s.is_empty()).collect(),
            false => vec![zhi],
        };
        for pian in zhilie {
            charu(biaozhun, pian.to_string(), &mut quchong, &mut jieguo);
        }
    }

    jieguo
}
