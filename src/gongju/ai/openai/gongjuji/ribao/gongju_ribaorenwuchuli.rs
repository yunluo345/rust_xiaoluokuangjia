use crate::gongju::jwtgongju;
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

#[derive(Debug, Clone)]
pub enum Gongjufenzu {
    Guanli,
    Xitong,
}

#[derive(Deserialize)]
struct Qingqiucanshu {
    shuliang: Option<i64>,
}

pub fn huoqu_guanjianci() -> Vec<String> {
    vec![
        "日报".to_string(),
        "任务".to_string(),
        "标签".to_string(),
        "自动标签".to_string(),
        "任务处理".to_string(),
        "日报任务".to_string(),
        "自动打标签".to_string(),
    ]
}

pub fn huoqu_fenzu() -> Gongjufenzu {
    Gongjufenzu::Xitong
}

pub fn dinyi() -> Tool {
    Tool {
        tool_type: "function".to_string(),
        function: llm::chat::FunctionTool {
            name: "ribao_renwubiaoqian_chuli".to_string(),
            description: "处理日报标签任务。自动领取未处理任务，并依据ai配置的日报标签规则提取与绑定标签。支持从原文键值或由模型整理出的结构化内容中识别标签。".to_string(),
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

    let biaoqianxiang = tichubiaoqianxiang(neirong, peizhi);
    if biaoqianxiang.is_empty() {
        return chuli_shibai(&renwuid, &ribaoid, "未提取到可用标签").await;
    }

    let mut leixinghuancun: HashMap<String, String> = HashMap::new();
    let mut biaoqianhuancun: HashMap<String, String> = HashMap::new();
    let mut bangdingshu: u64 = 0;
    let mut jieguolie: Vec<Value> = Vec::new();

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
