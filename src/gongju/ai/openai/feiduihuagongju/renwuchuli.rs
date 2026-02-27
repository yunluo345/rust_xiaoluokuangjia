use crate::shujuku::psqlshujuku::shujubiao_nr::yonghu::{yonghuyanzheng, shujucaozuo_yonghuzu};
use crate::peizhixt::peizhi_nr::peizhi_ai::Ai;
use crate::peizhixt::peizhixitongzhuti;
use crate::shujuku::psqlshujuku::shujubiao_nr::ribao::{
    shujucaozuo_biaoqian,
    shujucaozuo_biaoqianleixing,
    shujucaozuo_ribao,
    shujucaozuo_ribao_biaoqian,
    shujucaozuo_ribao_biaoqianrenwu,
};
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};

use super::gongyong::{
    huoquzifuchuan, yanzheng_bitian_biaoqian,
    chuli_wenben_aijieguo, chuli_kuozhan_aijieguo,
    jiexi_kuozhan, jisuan_sha256,
};
use super::biaoqiantiqu;
use super::aishengcheng;
use super::guanxifenxi;

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

    let biaoqianxiang = match biaoqiantiqu::ai_tiqu_biaoqian(neirong, peizhi, yiyou.as_deref()).await {
        Some(xiang) => {
            println!("[任务处理] 任务={} AI提取到 {} 个标签", renwuid, xiang.len());
            xiang
        }
        None => {
            println!("[任务处理] 任务={} AI提取失败，尝试字符串匹配", renwuid);
            let xiang = biaoqiantiqu::tichubiaoqianxiang(neirong, peizhi);
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

    let xuyao_biaoti = ribao.get("biaoti").and_then(|v| v.as_str()).map_or(true, |s| s.trim().is_empty());
    let xuyao_zhaiyao = ribao.get("zhaiyao").and_then(|v| v.as_str()).map_or(true, |s| s.trim().is_empty());
    let kuozhan_yuanshi = ribao.get("kuozhan").and_then(|v| v.as_str());
    let mut kuozhan_jiegou = jiexi_kuozhan(kuozhan_yuanshi);
    let neirong_hash = jisuan_sha256(neirong);
    let xuyao_siweidaotu = kuozhan_jiegou.get("siweidaotu").is_none();
    let guanxifenxi_hash_yiyou = kuozhan_jiegou.get("guanxifenxi_neirong_hash").and_then(|v| v.as_str());
    let xuyao_guanxifenxi = kuozhan_jiegou.get("guanxifenxi").is_none()
        || guanxifenxi_hash_yiyou != Some(neirong_hash.as_str());
    let mut kuozhan_yigengxin = false;

    let biaoti_fut = async {
        if xuyao_biaoti { aishengcheng::ai_shengcheng_biaoti(neirong).await } else { None }
    };
    let zhaiyao_fut = async {
        if xuyao_zhaiyao { aishengcheng::ai_shengcheng_zhaiyao(neirong).await } else { None }
    };
    let daotu_fut = async {
        if xuyao_siweidaotu { aishengcheng::ai_shengcheng_siweidaotu(neirong, peizhi).await } else { None }
    };
    let guanxi_fut = async {
        if xuyao_guanxifenxi { guanxifenxi::ai_shengcheng_guanxifenxi(neirong, peizhi).await } else { None }
    };
    let (biaoti_yuanshi, zhaiyao_yuanshi, daotu_yuanshi, guanxi_yuanshi) = futures::join!(biaoti_fut, zhaiyao_fut, daotu_fut, guanxi_fut);

    let biaoti_jieguo = chuli_wenben_aijieguo(xuyao_biaoti, biaoti_yuanshi, "biaoti", "标题", &renwuid, &ribaoid).await;
    let zhaiyao_jieguo = chuli_wenben_aijieguo(xuyao_zhaiyao, zhaiyao_yuanshi, "zhaiyao", "摘要", &renwuid, &ribaoid).await;

    let daotu_jieguo = chuli_kuozhan_aijieguo(
        xuyao_siweidaotu, daotu_yuanshi, "siweidaotu", "思维导图",
        &renwuid, &ribaoid, &mut kuozhan_jiegou, &mut kuozhan_yigengxin, None,
    );
    let guanxi_jieguo = chuli_kuozhan_aijieguo(
        xuyao_guanxifenxi, guanxi_yuanshi, "guanxifenxi", "关系分析",
        &renwuid, &ribaoid, &mut kuozhan_jiegou, &mut kuozhan_yigengxin, Some(("guanxifenxi_neirong_hash", &neirong_hash)),
    );

    if kuozhan_yigengxin {
        let kuozhan_wenben = kuozhan_jiegou.to_string();
        let _ = shujucaozuo_ribao::gengxin(&ribaoid, &[("kuozhan", &kuozhan_wenben)]).await;
    }

    let ai_shibai = (xuyao_biaoti && biaoti_jieguo.is_none())
        || (xuyao_zhaiyao && zhaiyao_jieguo.is_none())
        || (xuyao_siweidaotu && !daotu_jieguo)
        || (xuyao_guanxifenxi && !guanxi_jieguo);
    if ai_shibai {
        return chuli_shibai(&renwuid, &ribaoid, "AI生成步骤部分失败").await;
    }

    // 全部 AI 处理完成后才标记任务成功
    if shujucaozuo_ribao_biaoqianrenwu::biaojichenggong(&renwuid, &biaoqianjieguo)
        .await
        .is_none()
    {
        return chuli_shibai(&renwuid, &ribaoid, "任务完成状态更新失败").await;
    }

    println!("[任务处理] ✓ 任务={} 日报={} 绑定标签数={}", renwuid, ribaoid, bangdingshu);
    json!({
        "chenggong": true,
        "renwuid": renwuid,
        "ribaoid": ribaoid,
        "bangdingshu": bangdingshu,
        "tiqujieguo": tiqujieguo,
        "biaoti": biaoti_jieguo,
        "zhaiyao": zhaiyao_jieguo,
        "siweidaotu": daotu_jieguo,
        "guanxifenxi": guanxi_jieguo,
    })
}

/// 按任务ID单独处理一条任务（不经过调度器，直接执行）
pub async fn zhixing_dange_renwu_neibu(renwuid: &str) -> Result<Value, String> {
    let renwu = shujucaozuo_ribao_biaoqianrenwu::chaxun_id(renwuid)
        .await
        .ok_or_else(|| format!("任务不存在: {}", renwuid))?;

    let zhuangtai = renwu.get("zhuangtai").and_then(|v| v.as_str()).unwrap_or("");
    if zhuangtai == "processing" {
        return Err(format!("任务正在处理中: {}", renwuid));
    }

    // 已完成或已失败的任务需要先清除旧产物并重置状态再重新处理
    if zhuangtai == "true" || zhuangtai == "shibai" {
        shujucaozuo_ribao_biaoqianrenwu::chongxin_ruidui(renwuid)
            .await
            .ok_or_else(|| format!("重置任务失败: {}", renwuid))?;
    }

    // 原子领取任务（置为 processing 并增加尝试次数）
    let renwu = shujucaozuo_ribao_biaoqianrenwu::lingqu_zhiding(renwuid)
        .await
        .ok_or_else(|| format!("领取任务失败（可能已被其他进程处理）: {}", renwuid))?;

    let peizhi = peizhixitongzhuti::duqupeizhi::<Ai>(Ai::wenjianming()).unwrap_or_default();
    let jieguo = chuli_dange_renwu(renwu, &peizhi).await;
    Ok(jieguo)
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
