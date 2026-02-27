//! 任务处理流程步骤化拆分
//!
//! 将原 `chuli_dange_renwu` 巨型函数拆为可独立测试的步骤：
//! 1. yanzheng_renwu    → 校验任务参数、获取日报、验证内容
//! 2. tiqu_biaoqian     → AI 标签提取（含字符串匹配降级）
//! 3. yanzheng_bitian   → 必填标签校验
//! 4. bangding_biaoqian → 创建标签类型/标签实体并绑定到日报
//! 5. ai_fengfu         → 并发生成标题、摘要、思维导图、关系分析
//! 6. wanjie_renwu      → 标记任务成功

use crate::peizhixt::peizhi_nr::peizhi_ai::Ai;
use crate::shujuku::psqlshujuku::shujubiao_nr::ribao::{
    shujucaozuo_biaoqian,
    shujucaozuo_biaoqianleixing,
    shujucaozuo_ribao,
    shujucaozuo_ribao_biaoqian,
    shujucaozuo_ribao_biaoqianrenwu,
    shujucaozuo_ribao_guanxi,
};
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};

use super::gongyong::{
    huoquzifuchuan, yanzheng_bitian_biaoqian as gongyong_yanzheng_bitian,
    chuli_wenben_aijieguo, chuli_kuozhan_aijieguo,
    jiexi_kuozhan, jisuan_sha256,
};
use super::biaoqiantiqu;
use super::aishengcheng;
use super::guanxifenxi;

// ==================== 步骤错误 ====================

/// 步骤执行错误
pub struct BuzhouCuowu {
    pub xiaoxi: String,
}

impl BuzhouCuowu {
    pub fn new(xiaoxi: impl Into<String>) -> Self {
        BuzhouCuowu { xiaoxi: xiaoxi.into() }
    }
}

// ==================== 步骤1 上下文 ====================

/// 任务校验后的上下文（在后续步骤间传递）
pub struct RenwuShanxiawen {
    pub renwuid: String,
    pub ribaoid: String,
    pub ribao: Value,
    pub neirong: String,
    /// 已有标签的文本摘要（供 AI 去重用）
    pub yiyou_biaoqian_wenben: Option<String>,
    /// 已有标签类型名称集合
    pub yiyou_biaoqianmingcheng: HashSet<String>,
}

/// 步骤1：校验任务参数并加载日报
pub async fn yanzheng_renwu(renwu: &Value) -> Result<RenwuShanxiawen, BuzhouCuowu> {
    let renwuid = huoquzifuchuan(renwu, "id")
        .ok_or_else(|| BuzhouCuowu::new("任务缺少ID"))?;
    let ribaoid = huoquzifuchuan(renwu, "ribaoid")
        .ok_or_else(|| BuzhouCuowu::new("任务缺少日报ID"))?;

    println!("[任务处理] 开始处理 任务={} 日报={}", renwuid, ribaoid);

    let ribao = shujucaozuo_ribao::chaxun_id(&ribaoid)
        .await
        .ok_or_else(|| BuzhouCuowu::new("日报不存在"))?;

    let neirong = ribao.get("neirong")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|v| !v.is_empty())
        .ok_or_else(|| BuzhouCuowu::new("日报内容为空"))?
        .to_string();

    // 加载已有标签
    let yiyou_shuju = shujucaozuo_ribao_biaoqian::chaxun_ribaoid_daixinxi(&ribaoid)
        .await
        .filter(|lie| !lie.is_empty());

    let yiyou_biaoqian_wenben = yiyou_shuju.as_ref().map(|lie| {
        lie.iter()
            .filter_map(|b| {
                let lx = b["leixingmingcheng"].as_str()?;
                let zhi = b["zhi"].as_str()?;
                Some(format!("- {}：{}", lx, zhi))
            })
            .collect::<Vec<_>>()
            .join("\n")
    });

    if yiyou_biaoqian_wenben.is_some() {
        println!("[任务处理] 任务={} 已有 {} 个标签",
            renwuid, yiyou_biaoqian_wenben.as_ref().unwrap().lines().count());
    }

    let yiyou_biaoqianmingcheng: HashSet<String> = yiyou_shuju.as_ref()
        .map(|lie| lie.iter()
            .filter_map(|b| b["leixingmingcheng"].as_str().map(String::from))
            .collect())
        .unwrap_or_default();

    Ok(RenwuShanxiawen {
        renwuid,
        ribaoid,
        ribao,
        neirong,
        yiyou_biaoqian_wenben,
        yiyou_biaoqianmingcheng,
    })
}

// ==================== 步骤2：标签提取 ====================

/// 步骤2：AI 标签提取（失败时降级为字符串匹配）
pub async fn tiqu_biaoqian(
    neirong: &str,
    peizhi: &Ai,
    yiyou: Option<&str>,
    renwuid: &str,
) -> Result<Vec<(String, String)>, BuzhouCuowu> {
    if let Some(xiang) = biaoqiantiqu::ai_tiqu_biaoqian(neirong, peizhi, yiyou).await {
        println!("[任务处理] 任务={} AI提取到 {} 个标签", renwuid, xiang.len());
        return Ok(xiang);
    }

    println!("[任务处理] 任务={} AI提取失败，尝试字符串匹配", renwuid);
    let xiang = biaoqiantiqu::tichubiaoqianxiang(neirong, peizhi);
    if xiang.is_empty() {
        return Err(BuzhouCuowu::new("AI和字符串匹配均未提取到标签"));
    }
    println!("[任务处理] 任务={} 字符串匹配到 {} 个标签", renwuid, xiang.len());
    Ok(xiang)
}

// ==================== 步骤3：必填标签校验 ====================

/// 步骤3：校验必填标签是否齐全（合并已有标签）
pub fn yanzheng_bitian(
    biaoqianxiang: &[(String, String)],
    yiyou_mingcheng: &HashSet<String>,
    peizhi: &Ai,
) -> Result<(), BuzhouCuowu> {
    let mut yanzheng_xiang: Vec<(String, String)> = biaoqianxiang.to_vec();
    for ming in yiyou_mingcheng {
        if !yanzheng_xiang.iter().any(|(m, _)| m == ming) {
            yanzheng_xiang.push((ming.clone(), String::new()));
        }
    }

    if let Some(queshi) = gongyong_yanzheng_bitian(&yanzheng_xiang, peizhi) {
        return Err(BuzhouCuowu::new(format!("缺少必填标签: {}", queshi.join("、"))));
    }
    Ok(())
}

// ==================== 步骤4：标签绑定 ====================

/// 标签绑定结果
pub struct BiaoqianBangdingJieguo {
    pub bangdingshu: u64,
    pub jieguolie: Vec<Value>,
    pub tiqujieguo: HashMap<String, String>,
}

/// 步骤4：创建标签类型/标签实体并绑定到日报
pub async fn bangding_biaoqian(
    ribaoid: &str,
    biaoqianxiang: Vec<(String, String)>,
    renwuid: &str,
) -> Result<BiaoqianBangdingJieguo, BuzhouCuowu> {
    let mut leixinghuancun: HashMap<String, String> = HashMap::new();
    let mut biaoqianhuancun: HashMap<String, String> = HashMap::new();
    let mut bangdingshu: u64 = 0;
    let mut jieguolie: Vec<Value> = Vec::new();

    let tiqujieguo: HashMap<String, String> = biaoqianxiang.iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();

    for (leixingmingcheng, zhi) in biaoqianxiang {
        let leixingid = huoquhuochuangjian_leixingid(&leixingmingcheng, &mut leixinghuancun)
            .await
            .ok_or_else(|| BuzhouCuowu::new("标签类型创建失败"))?;

        let biaoqianid = huoquhuochuangjian_biaoqianid(&leixingid, &zhi, &mut biaoqianhuancun)
            .await
            .ok_or_else(|| BuzhouCuowu::new("标签创建失败"))?;

        let xinbangding = bangdingdange(ribaoid, &biaoqianid)
            .await
            .ok_or_else(|| BuzhouCuowu::new("标签绑定失败"))?;

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

    println!("[任务处理] 任务={} 绑定标签 新增={}", renwuid, bangdingshu);
    Ok(BiaoqianBangdingJieguo { bangdingshu, jieguolie, tiqujieguo })
}

// ==================== 步骤5：AI 内容丰富 ====================

/// AI 丰富结果
pub struct AiFengfuJieguo {
    pub biaoti: Option<String>,
    pub zhaiyao: Option<String>,
    pub siweidaotu: bool,
    pub guanxifenxi: bool,
}

/// 步骤5：并发生成标题、摘要、思维导图、关系分析，写入日报
pub async fn ai_fengfu(
    shanxiawen: &RenwuShanxiawen,
    peizhi: &Ai,
) -> Result<AiFengfuJieguo, BuzhouCuowu> {
    let neirong = shanxiawen.neirong.as_str();
    let ribao = &shanxiawen.ribao;
    let renwuid = &shanxiawen.renwuid;
    let ribaoid = &shanxiawen.ribaoid;

    // 判断哪些需要生成
    let xuyao_biaoti = ribao.get("biaoti").and_then(|v| v.as_str()).map_or(true, |s| s.trim().is_empty());
    let xuyao_zhaiyao = ribao.get("zhaiyao").and_then(|v| v.as_str()).map_or(true, |s| s.trim().is_empty());
    let kuozhan_yuanshi = ribao.get("kuozhan").and_then(|v| v.as_str());
    let mut kuozhan_jiegou = jiexi_kuozhan(kuozhan_yuanshi);
    let neirong_hash = jisuan_sha256(neirong);
    let xuyao_siweidaotu = kuozhan_jiegou.get("siweidaotu").is_none();
    let guanxifenxi_hash_yiyou = kuozhan_jiegou.get("guanxifenxi_neirong_hash").and_then(|v| v.as_str()).map(String::from);
    let xuyao_guanxifenxi = kuozhan_jiegou.get("guanxifenxi").is_none()
        || guanxifenxi_hash_yiyou.as_deref() != Some(neirong_hash.as_str());
    let mut kuozhan_yigengxin = false;

    // 并发执行 AI 生成
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
    let (biaoti_yuanshi, zhaiyao_yuanshi, daotu_yuanshi, guanxi_yuanshi) =
        futures::join!(biaoti_fut, zhaiyao_fut, daotu_fut, guanxi_fut);

    // 写入文本型结果
    let biaoti_jieguo = chuli_wenben_aijieguo(xuyao_biaoti, biaoti_yuanshi, "biaoti", "标题", renwuid, ribaoid).await;
    let zhaiyao_jieguo = chuli_wenben_aijieguo(xuyao_zhaiyao, zhaiyao_yuanshi, "zhaiyao", "摘要", renwuid, ribaoid).await;

    // 写入扩展 JSON 型结果
    let daotu_jieguo = chuli_kuozhan_aijieguo(
        xuyao_siweidaotu, daotu_yuanshi, "siweidaotu", "思维导图",
        renwuid, ribaoid, &mut kuozhan_jiegou, &mut kuozhan_yigengxin, None,
    );
    let guanxi_jieguo = chuli_kuozhan_aijieguo(
        xuyao_guanxifenxi, guanxi_yuanshi, "guanxifenxi", "关系分析",
        renwuid, ribaoid, &mut kuozhan_jiegou, &mut kuozhan_yigengxin,
        Some(("guanxifenxi_neirong_hash", &neirong_hash)),
    );

    // 持久化扩展字段
    if kuozhan_yigengxin {
        let kuozhan_wenben = kuozhan_jiegou.to_string();
        let _ = shujucaozuo_ribao::gengxin(ribaoid, &[("kuozhan", &kuozhan_wenben)]).await;
    }

    // 增量写入关系边到 ribao_guanxi_bian 表
    if guanxi_jieguo {
        if let Some(guanxilie) = kuozhan_jiegou.get("guanxifenxi")
            .and_then(|gx| gx.get("guanxi"))
            .and_then(|g| g.as_array())
        {
            let shu = shujucaozuo_ribao_guanxi::gengxin_ribao_guanxi(ribaoid, guanxilie)
                .await
                .unwrap_or(0);
            println!("[任务处理] 任务={} 日报={} 写入关系边 {}", renwuid, ribaoid, shu);
        }
    }

    // 检查是否有必须成功但失败的步骤
    let ai_shibai = (xuyao_biaoti && biaoti_jieguo.is_none())
        || (xuyao_zhaiyao && zhaiyao_jieguo.is_none())
        || (xuyao_siweidaotu && !daotu_jieguo)
        || (xuyao_guanxifenxi && !guanxi_jieguo);
    if ai_shibai {
        return Err(BuzhouCuowu::new("AI生成步骤部分失败"));
    }

    Ok(AiFengfuJieguo {
        biaoti: biaoti_jieguo,
        zhaiyao: zhaiyao_jieguo,
        siweidaotu: daotu_jieguo,
        guanxifenxi: guanxi_jieguo,
    })
}

// ==================== 步骤6：任务完结 ====================

/// 步骤6：标记任务成功
pub async fn wanjie_renwu(
    renwuid: &str,
    biaoqianjieguo: &str,
) -> Result<(), BuzhouCuowu> {
    shujucaozuo_ribao_biaoqianrenwu::biaojichenggong(renwuid, biaoqianjieguo)
        .await
        .ok_or_else(|| BuzhouCuowu::new("任务完成状态更新失败"))?;
    Ok(())
}

// ==================== 内部辅助（从 renwuchuli.rs 迁移） ====================

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

async fn bangdingdange(ribaoid: &str, biaoqianid: &str) -> Option<bool> {
    if shujucaozuo_ribao_biaoqian::guanliancunzai(ribaoid, biaoqianid).await {
        return Some(false);
    }
    shujucaozuo_ribao_biaoqian::xinzeng(ribaoid, biaoqianid)
        .await
        .map(|n| n > 0)
}
