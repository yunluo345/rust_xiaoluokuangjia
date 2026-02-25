use crate::shujuku::psqlshujuku::shujubiao_nr::yonghu::{yonghuyanzheng, shujucaozuo_yonghuzu};
use crate::gongju::ai::openai::{aixiaoxiguanli, openaizhuti};
use crate::peizhixt::peizhi_nr::peizhi_ai::Ai;
use crate::peizhixt::peizhixitongzhuti;
use crate::shujuku::psqlshujuku::shujubiao_nr::ribao::{
    shujucaozuo_biaoqian,
    shujucaozuo_biaoqianleixing,
    shujucaozuo_ribao,
    shujucaozuo_ribao_biaoqian,
    shujucaozuo_ribao_biaoqianrenwu,
};
use llm::chat::Tool;
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};

use super::super::Gongjufenzu;

pub fn huoqu_guanjianci() -> Vec<String> {
    vec![
        "日报标签任务".to_string(),
        "处理日报任务".to_string(),
        "标签提取任务".to_string(),
    ]
}

pub fn huoqu_fenzu() -> Gongjufenzu {
    Gongjufenzu::Xitong
}

pub fn dinyi() -> Tool {
    let peizhi = peizhixitongzhuti::duqupeizhi::<Ai>(Ai::wenjianming()).unwrap_or_default();
    
    let biaoqian_tishi = peizhi.ribao_biaoqian.iter()
        .map(|bq| {
            let biecheng_str = bq.biecheng.join("、");
            format!("{}（{}，别名：{}）", bq.mingcheng, bq.miaoshu, biecheng_str)
        })
        .collect::<Vec<_>>()
        .join("；");
    
    let miaoshu = format!(
        "处理日报标签提取任务，从日报内容中提取标签并绑定。支持的标签：{}",
        biaoqian_tishi
    );
    
    Tool {
        tool_type: "function".to_string(),
        function: llm::chat::FunctionTool {
            name: "ribao_renwubiaoqian_chuli".to_string(),
            description: miaoshu,
            parameters: json!({
                "type": "object",
                "properties": {
                    "shuliang": {
                        "type": "integer",
                        "description": "本次处理任务数量，未传时使用系统配置并发数量"
                    }
                }
            }),
        },
    }
}

fn huoquzifuchuan(shuju: &Value, ziduan: &str) -> Option<String> {
    shuju.get(ziduan).and_then(|v| {
        v.as_str()
            .map(|s| s.to_string())
            .or_else(|| v.as_i64().map(|n| n.to_string()))
            .or_else(|| v.as_u64().map(|n| n.to_string()))
    })
}

fn yanzheng_bitian_biaoqian(tiquxiang: &[(String, String)], peizhi: &Ai) -> Option<Vec<String>> {
    let bitian_mingcheng: HashSet<String> = peizhi.ribao_biaoqian.iter()
        .filter(|bq| bq.bitian)
        .map(|bq| bq.mingcheng.clone())
        .collect();
    
    let yitiqumingcheng: HashSet<String> = tiquxiang.iter()
        .map(|(ming, _)| ming.clone())
        .collect();
    
    let queshi: Vec<String> = bitian_mingcheng.difference(&yitiqumingcheng)
        .cloned()
        .collect();
    
    (!queshi.is_empty()).then_some(queshi)
}

async fn ai_shengcheng_zhaiyao(neirong: &str) -> Option<String> {
    let aipeizhi = crate::jiekouxt::jiekou_nr::ai::huoqu_peizhi().await?
        .shezhi_chaoshi(60)
        .shezhi_chongshi(1);

    let mut guanli = aixiaoxiguanli::Xiaoxiguanli::xingjian()
        .shezhi_xitongtishici(
            "你是日报摘要生成助手。根据日报内容生成简洁摘要。\n\
            要求：\n\
            1. 控制在100字以内\n\
            2. 突出重点工作和关键成果\n\
            3. 只返回摘要文本，不要返回其他内容"
        );
    guanli.zhuijia_yonghuxiaoxi(format!("请为以下日报生成摘要：\n\n{}", neirong));

    let huifu = openaizhuti::putongqingqiu(&aipeizhi, &guanli).await?;
    let zhaiyao = huifu.trim().to_string();
    println!("[摘要生成] {}", zhaiyao.chars().take(60).collect::<String>());
    (!zhaiyao.is_empty()).then_some(zhaiyao)
}

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

async fn ai_shengcheng_siweidaotu(neirong: &str, peizhi: &Ai) -> Option<String> {
    let aipeizhi = crate::jiekouxt::jiekou_nr::ai::huoqu_peizhi().await?
        .shezhi_chaoshi(120)
        .shezhi_chongshi(1);

    let xitongtishici = goujian_siweidaotu_tishici(peizhi);
    let mut guanli = aixiaoxiguanli::Xiaoxiguanli::xingjian()
        .shezhi_xitongtishici(&xitongtishici);
    guanli.zhuijia_yonghuxiaoxi(format!("请对以下日报进行全面分析并生成思维导图：\n\n{}", neirong));

    let huifu = openaizhuti::putongqingqiu(&aipeizhi, &guanli).await?;
    let jinghua = huifu.trim()
        .trim_start_matches("```json").trim_start_matches("```")
        .trim_end_matches("```")
        .trim();

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

async fn ai_shengcheng_guanxifenxi(neirong: &str, peizhi: &Ai) -> Option<String> {
    let aipeizhi = crate::jiekouxt::jiekou_nr::ai::huoqu_peizhi().await?
        .shezhi_chaoshi(60)
        .shezhi_chongshi(1);

    let mut guanli = aixiaoxiguanli::Xiaoxiguanli::xingjian()
        .shezhi_xitongtishici(&peizhi.guanxifenxi_tishici);
    guanli.zhuijia_yonghuxiaoxi(format!("请分析以下日报中的人物关系：\n\n{}", neirong));

    let huifu = openaizhuti::putongqingqiu(&aipeizhi, &guanli).await?;
    let jinghua = huifu.trim()
        .trim_start_matches("```json").trim_start_matches("```")
        .trim_end_matches("```")
        .trim();

    match serde_json::from_str::<Value>(jinghua) {
        Ok(_) => {
            println!("[关系分析] 生成成功 长度={}", jinghua.len());
            Some(jinghua.to_string())
        }
        Err(e) => {
            println!("[关系分析] JSON解析失败: {} 原文={}", e, jinghua.chars().take(80).collect::<String>());
            None
        }
    }
}

/// 解析kuozhan为结构化JSON，兼容旧格式
fn jiexi_kuozhan(kuozhan_str: Option<&str>) -> Value {
    let raw = match kuozhan_str.filter(|s| !s.trim().is_empty()) {
        Some(s) => s,
        None => return json!({}),
    };
    match serde_json::from_str::<Value>(raw) {
        Ok(v) if v.get("siweidaotu").is_some() => v,
        Ok(v) if v.is_object() => json!({"siweidaotu": v}),
        _ => json!({}),
    }
}

async fn ai_tiqu_biaoqian(neirong: &str, peizhi: &Ai, yiyou_biaoqian: Option<&str>) -> Option<Vec<(String, String)>> {
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
        4. 只返回JSON，不要返回其他内容{}",
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

fn tichubiaoqianxiang(neirong: &str, peizhi: &Ai) -> Vec<(String, String)> {
    let mut biezhuan: HashMap<String, String> = HashMap::new();
    let mut duozhiji: HashSet<String> = HashSet::new();
    for biaoqian in &peizhi.ribao_biaoqian {
        biezhuan.insert(biaoqian.mingcheng.trim().to_string(), biaoqian.mingcheng.clone());
        biezhuan.insert(biaoqian.miaoshu.trim().to_string(), biaoqian.mingcheng.clone());
        if biaoqian.duozhi {
            duozhiji.insert(biaoqian.mingcheng.clone());
        }
        for biecheng in &biaoqian.biecheng {
            let jian = biecheng.trim();
            if !jian.is_empty() {
                biezhuan.insert(jian.to_string(), biaoqian.mingcheng.clone());
            }
        }
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
                qidian = kaishi + 1;
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


async fn huoquhuochuangjian_leixingid(
    mingcheng: &str,
    leixinghuancun: &mut HashMap<String, String>,
) -> Option<String> {
    if let Some(id) = leixinghuancun.get(mingcheng) {
        return Some(id.clone());
    }

    if let Some(cunzai) = shujucaozuo_biaoqianleixing::chaxun_mingcheng(mingcheng).await {
        if let Some(id) = huoquzifuchuan(&cunzai, "id") {
            leixinghuancun.insert(mingcheng.to_string(), id.clone());
            return Some(id);
        }
    }

    let id = shujucaozuo_biaoqianleixing::xinzeng(mingcheng).await?;
    leixinghuancun.insert(mingcheng.to_string(), id.clone());
    Some(id)
}

async fn huoquhuochuangjian_biaoqianid(
    leixingid: &str,
    zhi: &str,
    biaoqianhuancun: &mut HashMap<String, String>,
) -> Option<String> {
    let jian = format!("{}|{}", leixingid, zhi);
    if let Some(id) = biaoqianhuancun.get(&jian) {
        return Some(id.clone());
    }

    if let Some(cunzai) = shujucaozuo_biaoqian::chaxun_leixingid_zhi(leixingid, zhi).await {
        if let Some(id) = huoquzifuchuan(&cunzai, "id") {
            biaoqianhuancun.insert(jian, id.clone());
            return Some(id);
        }
    }

    let id = shujucaozuo_biaoqian::xinzeng(leixingid, zhi).await?;
    biaoqianhuancun.insert(jian, id.clone());
    Some(id)
}

async fn bangdingbiaoqian(ribaoid: &str, biaoqianid: &str) -> Option<bool> {
    if shujucaozuo_ribao_biaoqian::guanliancunzai(ribaoid, biaoqianid).await {
        return Some(false);
    }
    shujucaozuo_ribao_biaoqian::xinzeng(ribaoid, biaoqianid)
        .await
        .map(|n| n > 0)
}

async fn chuli_shibai(renwuid: &str, ribaoid: &str, xiaoxi: &str) -> Value {
    println!("[任务处理] ✗ 任务{}失败 日报={} 原因={}", renwuid, ribaoid, xiaoxi);
    let _ = shujucaozuo_ribao_biaoqianrenwu::biaojishibai(renwuid).await;
    json!({
        "chenggong": false,
        "renwuid": renwuid,
        "ribaoid": ribaoid,
        "xiaoxi": xiaoxi,
    })
}

async fn chuli_dange_renwu(renwu: Value, peizhi: &Ai) -> Value {
    let renwuid = match huoquzifuchuan(&renwu, "id") {
        Some(v) => v,
        None => return json!({"chenggong": false, "xiaoxi": "任务缺少ID"}),
    };

    let ribaoid = match huoquzifuchuan(&renwu, "ribaoid") {
        Some(v) => v,
        None => return chuli_shibai(&renwuid, "", "任务缺少日报ID").await,
    };
    println!("[任务处理] 开始处理 任务={} 日报={}", renwuid, ribaoid);

    let ribao = match shujucaozuo_ribao::chaxun_id(&ribaoid).await {
        Some(v) => v,
        None => return chuli_shibai(&renwuid, &ribaoid, "日报不存在").await,
    };

    let neirong = match ribao.get("neirong").and_then(|v| v.as_str()).map(str::trim) {
        Some(v) if !v.is_empty() => v,
        _ => return chuli_shibai(&renwuid, &ribaoid, "日报内容为空").await,
    };

    let yiyou_shuju = shujucaozuo_ribao_biaoqian::chaxun_ribaoid_daixinxi(&ribaoid).await
        .filter(|lie| !lie.is_empty());

    let yiyou = yiyou_shuju.as_ref().map(|lie| lie.iter()
        .filter_map(|b| {
            let lx = b["leixingmingcheng"].as_str()?;
            let zhi = b["zhi"].as_str()?;
            Some(format!("- {}：{}", lx, zhi))
        })
        .collect::<Vec<_>>()
        .join("\n")
    );
    if yiyou.is_some() {
        println!("[任务处理] 任务={} 已有 {} 个标签", renwuid, yiyou.as_ref().unwrap().lines().count());
    }

    let yiyou_biaoqianmingcheng: HashSet<String> = yiyou_shuju.as_ref()
        .map(|lie| lie.iter()
            .filter_map(|b| b["leixingmingcheng"].as_str().map(String::from))
            .collect())
        .unwrap_or_default();

    let biaoqianxiang = match ai_tiqu_biaoqian(neirong, peizhi, yiyou.as_deref()).await {
        Some(xiang) => {
            println!("[任务处理] 任务={} AI提取到 {} 个标签", renwuid, xiang.len());
            xiang
        }
        None => {
            println!("[任务处理] 任务={} AI提取失败，尝试字符串匹配", renwuid);
            let xiang = tichubiaoqianxiang(neirong, peizhi);
            if xiang.is_empty() {
                return chuli_shibai(&renwuid, &ribaoid, "AI和字符串匹配均未提取到标签").await;
            }
            println!("[任务处理] 任务={} 字符串匹配到 {} 个标签", renwuid, xiang.len());
            xiang
        }
    };

    let mut yanzheng_xiang: Vec<(String, String)> = biaoqianxiang.clone();
    for ming in &yiyou_biaoqianmingcheng {
        if !yanzheng_xiang.iter().any(|(m, _)| m == ming) {
            yanzheng_xiang.push((ming.clone(), String::new()));
        }
    }

    if let Some(queshi) = yanzheng_bitian_biaoqian(&yanzheng_xiang, peizhi) {
        let xiaoxi = format!("缺少必填标签: {}", queshi.join("、"));
        return chuli_shibai(&renwuid, &ribaoid, &xiaoxi).await;
    }

    let mut leixinghuancun: HashMap<String, String> = HashMap::new();
    let mut biaoqianhuancun: HashMap<String, String> = HashMap::new();
    let mut bangdingshu: u64 = 0;
    let mut jieguolie: Vec<Value> = Vec::new();
    
    let tiqujieguo: HashMap<String, String> = biaoqianxiang.iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();

    for (leixingmingcheng, zhi) in biaoqianxiang {
        let leixingid = match huoquhuochuangjian_leixingid(&leixingmingcheng, &mut leixinghuancun).await {
            Some(v) => v,
            None => return chuli_shibai(&renwuid, &ribaoid, "标签类型创建失败").await,
        };

        let biaoqianid = match huoquhuochuangjian_biaoqianid(&leixingid, &zhi, &mut biaoqianhuancun).await {
            Some(v) => v,
            None => return chuli_shibai(&renwuid, &ribaoid, "标签创建失败").await,
        };

        let xinbangding = match bangdingbiaoqian(&ribaoid, &biaoqianid).await {
            Some(v) => v,
            None => return chuli_shibai(&renwuid, &ribaoid, "标签绑定失败").await,
        };

        if xinbangding {
            bangdingshu += 1;
        }

        jieguolie.push(json!({
            "leixingmingcheng": leixingmingcheng,
            "zhi": zhi,
            "leixingid": leixingid,
            "biaoqianid": biaoqianid,
            "xinbangding": xinbangding,
        }));
    }

    let biaoqianjieguo = json!({
        "bangdingshu": bangdingshu,
        "biaoqianlie": jieguolie,
    })
    .to_string();

    if shujucaozuo_ribao_biaoqianrenwu::biaojichenggong(&renwuid, &biaoqianjieguo)
        .await
        .is_none()
    {
        return chuli_shibai(&renwuid, &ribaoid, "任务完成状态更新失败").await;
    }

    let xuyao_zhaiyao = ribao.get("zhaiyao").and_then(|v| v.as_str()).map_or(true, |s| s.trim().is_empty());
    let kuozhan_yuanshi = ribao.get("kuozhan").and_then(|v| v.as_str());
    let mut kuozhan_jiegou = jiexi_kuozhan(kuozhan_yuanshi);
    let xuyao_siweidaotu = kuozhan_jiegou.get("siweidaotu").is_none();
    let xuyao_guanxifenxi = kuozhan_jiegou.get("guanxifenxi").is_none();
    let mut kuozhan_yigengxin = false;

    let zhaiyao_jieguo = match xuyao_zhaiyao {
        true => match ai_shengcheng_zhaiyao(neirong).await {
            Some(zhaiyao) => {
                let _ = shujucaozuo_ribao::gengxin(&ribaoid, &[("zhaiyao", &zhaiyao)]).await;
                println!("[任务处理] 任务={} 日报={} 摘要已生成", renwuid, ribaoid);
                Some(zhaiyao)
            }
            None => {
                println!("[任务处理] 任务={} 摘要生成失败，跳过", renwuid);
                None
            }
        },
        false => {
            println!("[任务处理] 任务={} 日报={} 摘要已存在，跳过", renwuid, ribaoid);
            None
        },
    };

    let daotu_jieguo = match xuyao_siweidaotu {
        true => match ai_shengcheng_siweidaotu(neirong, peizhi).await {
            Some(daotu) => {
                if let Ok(daotu_json) = serde_json::from_str::<Value>(&daotu) {
                    kuozhan_jiegou["siweidaotu"] = daotu_json;
                    kuozhan_yigengxin = true;
                }
                println!("[任务处理] 任务={} 日报={} 思维导图已生成", renwuid, ribaoid);
                true
            }
            None => {
                println!("[任务处理] 任务={} 思维导图生成失败，跳过", renwuid);
                false
            }
        },
        false => {
            println!("[任务处理] 任务={} 日报={} 思维导图已存在，跳过", renwuid, ribaoid);
            false
        },
    };

    let guanxi_jieguo = match xuyao_guanxifenxi {
        true => match ai_shengcheng_guanxifenxi(neirong, peizhi).await {
            Some(guanxi) => {
                if let Ok(guanxi_json) = serde_json::from_str::<Value>(&guanxi) {
                    kuozhan_jiegou["guanxifenxi"] = guanxi_json;
                    kuozhan_yigengxin = true;
                }
                println!("[任务处理] 任务={} 日报={} 关系分析已生成", renwuid, ribaoid);
                true
            }
            None => {
                println!("[任务处理] 任务={} 关系分析生成失败，跳过", renwuid);
                false
            }
        },
        false => {
            println!("[任务处理] 任务={} 日报={} 关系分析已存在，跳过", renwuid, ribaoid);
            false
        },
    };

    if kuozhan_yigengxin {
        let kuozhan_wenben = kuozhan_jiegou.to_string();
        let _ = shujucaozuo_ribao::gengxin(&ribaoid, &[("kuozhan", &kuozhan_wenben)]).await;
    }

    println!("[任务处理] ✓ 任务={} 日报={} 绑定标签数={}", renwuid, ribaoid, bangdingshu);
    json!({
        "chenggong": true,
        "renwuid": renwuid,
        "ribaoid": ribaoid,
        "bangdingshu": bangdingshu,
        "tiqujieguo": tiqujieguo,
        "zhaiyao": zhaiyao_jieguo,
        "siweidaotu": daotu_jieguo,
        "guanxifenxi": guanxi_jieguo,
    })
}

pub async fn zhixing_neibu() -> Result<Value, String> {
    let peizhi = peizhixitongzhuti::duqupeizhi::<Ai>(Ai::wenjianming()).unwrap_or_default();
    let jieguo = shujucaozuo_ribao_biaoqianrenwu::qidong_diaodu(move |renwu| {
        let p = peizhi.clone();
        async move { chuli_dange_renwu(renwu, &p).await }
    })
    .await;

    jieguo
        .get("zhuangtai")
        .and_then(|z| z.as_str())
        .filter(|&z| z == "yunxingzhong")
        .map(|_| jieguo["xiaoxi"].as_str().unwrap_or("未知错误").to_string())
        .map_or(Ok(jieguo.clone()), Err)
}
pub async fn zhixing(_canshu: &str, lingpai: &str) -> String {
    let zaiti = match yonghuyanzheng::yanzhenglingpaijiquanxian(lingpai, "/jiekou/ribao/guanli").await {
        Ok(z) => z,
        Err(yonghuyanzheng::Lingpaicuowu::Yibeifengjin(y)) => return json!({"cuowu": format!("账号已被封禁：{}", y)}).to_string(),
        Err(yonghuyanzheng::Lingpaicuowu::Quanxianbuzu) => return json!({"cuowu": "权限不足"}).to_string(),
        Err(_) => return json!({"cuowu": "令牌无效或已过期"}).to_string(),
    };

    let zumingcheng = shujucaozuo_yonghuzu::chaxun_id(&zaiti.yonghuzuid).await
        .and_then(|zu| zu.get("mingcheng").and_then(|v| v.as_str()).map(String::from))
        .unwrap_or_else(|| "未知".to_string());
    println!(
        "[日报任务处理] 用户={} 账号={} 用户组={}（{}）",
        zaiti.yonghuid, zaiti.zhanghao, zaiti.yonghuzuid, zumingcheng
    );

    match zhixing_neibu().await {
        Ok(shuju) => json!({"chenggong": true, "shuju": shuju}).to_string(),
        Err(xiaoxi) => json!({"cuowu": xiaoxi}).to_string(),
    }
}
