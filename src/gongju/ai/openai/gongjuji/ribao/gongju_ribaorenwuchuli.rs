use crate::gongju::jwtgongju;
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
use crate::shujuku::psqlshujuku::shujubiao_nr::yonghu::shujucaozuo_yonghuzu;
use futures::stream::{self, StreamExt};
use llm::chat::Tool;
use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};
use super::super::Gongjufenzu;

#[derive(Deserialize)]
struct Qingqiucanshu {
    shuliang: Option<i64>,
}

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

async fn ai_tiqu_biaoqian(neirong: &str, peizhi: &Ai) -> Option<Vec<(String, String)>> {
    let biaoqian_tishi = peizhi.ribao_biaoqian.iter()
        .map(|bq| {
            let biecheng_str = bq.biecheng.join("、");
            let bitian_str = if bq.bitian { "【必填】" } else { "" };
            format!("{}{}（{}，别名：{}）", bitian_str, bq.mingcheng, bq.miaoshu, biecheng_str)
        })
        .collect::<Vec<_>>()
        .join("；");
    
    let xitongtishici = format!(
        "你是日报标签提取助手。从日报内容中提取以下标签信息：{}\n\
        请仔细阅读日报内容，提取所有相关标签。\n\
        返回JSON格式：{{\"标签名1\": \"值1\", \"标签名2\": \"值2\"}}\n\
        注意：\n\
        1. 标签名必须使用配置中的标准名称（不要使用别名）\n\
        2. 如果日报中没有某个标签的信息，不要返回该标签\n\
        3. 只返回JSON，不要返回其他内容",
        biaoqian_tishi
    );
    
    let aipeizhi = match crate::jiekouxt::jiekou_nr::ai::huoqu_peizhi().await {
        Some(p) => p.shezhi_chaoshi(60).shezhi_chongshi(1),
        None => {
            println!("[AI标签提取] 获取AI配置失败");
            return None;
        }
    };
    
    let mut guanli = aixiaoxiguanli::Xiaoxiguanli::xingjian()
        .shezhi_xitongtishici(&xitongtishici);
    guanli.zhuijia_yonghuxiaoxi(format!("请从以下日报中提取标签：\n\n{}", neirong));
    
    let huifu = match openaizhuti::putongqingqiu(&aipeizhi, &guanli).await {
        Some(h) => h,
        None => {
            println!("[AI标签提取] AI调用失败");
            return None;
        }
    };
    
    println!("[AI标签提取] AI返回: {}", huifu);
    
    let tiquxiang = tichubiaoqianxiang(&huifu, peizhi);
    if tiquxiang.is_empty() {
        println!("[AI标签提取] 解析AI返回失败");
        return None;
    }
    
    Some(tiquxiang)
}

fn tichubiaoqianxiang(neirong: &str, peizhi: &Ai) -> Vec<(String, String)> {
    let mut biezhuan: HashMap<String, String> = HashMap::new();
    for biaoqian in &peizhi.ribao_biaoqian {
        biezhuan.insert(biaoqian.mingcheng.trim().to_string(), biaoqian.mingcheng.clone());
        biezhuan.insert(biaoqian.miaoshu.trim().to_string(), biaoqian.mingcheng.clone());
        for biecheng in &biaoqian.biecheng {
            let jian = biecheng.trim();
            if !jian.is_empty() {
                biezhuan.insert(jian.to_string(), biaoqian.mingcheng.clone());
            }
        }
    }

    let mut jieguo: Vec<(String, String)> = Vec::new();
    let mut quchong: HashSet<String> = HashSet::new();

    if let Ok(Value::Object(duixiang)) = serde_json::from_str::<Value>(neirong) {
        for (leixing, zhi) in duixiang {
            let leixing_trim = leixing.trim();
            let biaozhun = match biezhuan.get(leixing_trim) {
                Some(v) => v,
                None => continue,
            };
            if let Some(wenzi) = zhi
                .as_str()
                .map(|s| s.trim().to_string())
                .or_else(|| zhi.as_i64().map(|n| n.to_string()))
                .or_else(|| zhi.as_u64().map(|n| n.to_string()))
            {
                let jian = format!("{}|{}", biaozhun, wenzi);
                if !wenzi.is_empty() && !quchong.contains(&jian) {
                    quchong.insert(jian);
                    jieguo.push((biaozhun.clone(), wenzi));
                }
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
                Some(v) => v,
                None => continue,
            };
            let jian = format!("{}|{}", biaozhun, zhi);
            if !zhi.is_empty() && !quchong.contains(&jian) {
                quchong.insert(jian);
                jieguo.push((biaozhun.clone(), zhi.to_string()));
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
        let jian = format!("{}|{}", biaozhun, zhi);
        if quchong.contains(&jian) {
            continue;
        }
        quchong.insert(jian);
        jieguo.push((biaozhun.clone(), zhi.to_string()));
    }

    jieguo
}

async fn shifouquanxianyonghu(yonghuzuid: &str) -> bool {
    shujucaozuo_yonghuzu::chaxun_id(yonghuzuid)
        .await
        .map(|zu| {
            let mingcheng = zu.get("mingcheng").and_then(|v| v.as_str()).unwrap_or("");
            let beizhu = zu.get("beizhu").and_then(|v| v.as_str()).unwrap_or("");
            mingcheng == "root" || beizhu.contains("root授权")
        })
        .unwrap_or(false)
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

    let ribao = match shujucaozuo_ribao::chaxun_id(&ribaoid).await {
        Some(v) => v,
        None => return chuli_shibai(&renwuid, &ribaoid, "日报不存在").await,
    };

    let neirong = match ribao.get("neirong").and_then(|v| v.as_str()).map(str::trim) {
        Some(v) if !v.is_empty() => v,
        _ => return chuli_shibai(&renwuid, &ribaoid, "日报内容为空").await,
    };

    println!("[标签提取] 开始处理日报 {}", ribaoid);
    
    let biaoqianxiang = match ai_tiqu_biaoqian(neirong, peizhi).await {
        Some(xiang) => {
            println!("[标签提取] AI提取成功，提取到 {} 个标签", xiang.len());
            xiang
        }
        None => {
            println!("[标签提取] AI提取失败，回退到字符串匹配");
            let xiang = tichubiaoqianxiang(neirong, peizhi);
            if xiang.is_empty() {
                return chuli_shibai(&renwuid, &ribaoid, "AI和字符串匹配均未提取到标签").await;
            }
            xiang
        }
    };

    if let Some(queshi) = yanzheng_bitian_biaoqian(&biaoqianxiang, peizhi) {
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

    json!({
        "chenggong": true,
        "renwuid": renwuid,
        "ribaoid": ribaoid,
        "bangdingshu": bangdingshu,
        "tiqujieguo": tiqujieguo,
    })
}

pub async fn zhixing_neibu(shuliang: i64) -> Result<Value, String> {
    let peizhi = peizhixitongzhuti::duqupeizhi::<Ai>(Ai::wenjianming()).unwrap_or_default();
    let bingfashuliang = peizhi.ribao_biaoqianrenwu_bingfashuliang.max(1) as usize;
    let shulian = if shuliang > 0 { shuliang } else { bingfashuliang as i64 };

    let renwulie = shujucaozuo_ribao_biaoqianrenwu::lingqu_zuijin_piliang_suiji(shulian)
        .await
        .ok_or_else(|| "任务领取失败".to_string())?;

    if renwulie.is_empty() {
        return Ok(json!({
            "zongshu": 0,
            "chenggongshu": 0,
            "shibaishu": 0,
            "jieguolie": []
        }));
    }

    let jieguolie: Vec<Value> = stream::iter(renwulie.into_iter().map(|renwu| {
        let peizhi = peizhi.clone();
        async move { chuli_dange_renwu(renwu, &peizhi).await }
    }))
    .buffer_unordered(bingfashuliang)
    .collect()
    .await;

    let chenggongshu = jieguolie
        .iter()
        .filter(|v| v.get("chenggong").and_then(|z| z.as_bool()).unwrap_or(false))
        .count();
    let zongshu = jieguolie.len();
    let shibaishu = zongshu.saturating_sub(chenggongshu);

    Ok(json!({
        "zongshu": zongshu,
        "chenggongshu": chenggongshu,
        "shibaishu": shibaishu,
        "jieguolie": jieguolie,
    }))
}
pub async fn zhixing(canshu: &str, lingpai: &str) -> String {
    let zaiti = match jwtgongju::yanzheng(lingpai).await {
        Some(z) => z,
        None => return json!({"cuowu": "令牌无效或已过期"}).to_string(),
    };

    if !shifouquanxianyonghu(&zaiti.yonghuzuid).await {
        return json!({"cuowu": "权限不足"}).to_string();
    }

    let qingqiu: Qingqiucanshu = if canshu.trim().is_empty() {
        Qingqiucanshu { shuliang: Some(1) }
    } else {
        match serde_json::from_str(canshu) {
            Ok(q) => q,
            Err(_) => return json!({"cuowu": "参数格式错误"}).to_string(),
        }
    };

    match zhixing_neibu(qingqiu.shuliang.unwrap_or(1)).await {
        Ok(shuju) => json!({"chenggong": true, "shuju": shuju}).to_string(),
        Err(xiaoxi) => json!({"cuowu": xiaoxi}).to_string(),
    }
}
