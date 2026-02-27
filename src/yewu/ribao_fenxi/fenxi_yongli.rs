use serde_json::Value;
use crate::gongju::ai::openai::feiduihuagongju::kuaribaofenxi;
use super::cangchu;
use super::leixing::RibaoZhaiyao;

/// 分析领域错误
pub enum FenxiCuowu {
    /// 输入参数校验不通过
    ShuruWuxiao(String),
    /// 未找到相关数据
    ShujiWeizhaodao(String),
    /// AI 分析调用失败
    AiFenxiShibai(String),
}

impl FenxiCuowu {
    pub fn zhuangtaima(&self) -> u16 {
        match self {
            FenxiCuowu::ShuruWuxiao(_) => 400,
            FenxiCuowu::ShujiWeizhaodao(_) => 404,
            FenxiCuowu::AiFenxiShibai(_) => 500,
        }
    }

    pub fn xiaoxi(&self) -> &str {
        match self {
            FenxiCuowu::ShuruWuxiao(s) => s,
            FenxiCuowu::ShujiWeizhaodao(s) => s,
            FenxiCuowu::AiFenxiShibai(s) => s,
        }
    }
}

impl From<cangchu::CangchuCuowu> for FenxiCuowu {
    fn from(e: cangchu::CangchuCuowu) -> Self {
        match e {
            cangchu::CangchuCuowu::ChaxunShibai => FenxiCuowu::AiFenxiShibai("数据库查询失败".to_string()),
            cangchu::CangchuCuowu::JieguoWeikong => FenxiCuowu::ShujiWeizhaodao("未找到相关数据".to_string()),
        }
    }
}

// ==================== 交流内容 AI 分析 ====================

/// 交流内容AI分析：查询交流记录 → 拼装AI输入 → 调用AI → 返回结构化结果
pub async fn jiaoliu_neirong_fenxi(shiti_leixing: &str, shiti_mingcheng: &str) -> Result<Value, FenxiCuowu> {
    let jiaoliu_lie = cangchu::chaxun_jiaoliu_neirong(shiti_leixing, shiti_mingcheng)
        .await
        .map_err(|_| FenxiCuowu::ShujiWeizhaodao("未找到相关交流内容".to_string()))?;

    let ai_shuru: Vec<Value> = jiaoliu_lie.iter().map(|x| {
        serde_json::json!({
            "riqi": x.fabushijian,
            "neirong": x.jiaoliu_neirong,
        })
    }).collect();

    let fenxi_jieguo = kuaribaofenxi::ai_jiaoliu_fenxi(&ai_shuru)
        .await
        .ok_or_else(|| FenxiCuowu::AiFenxiShibai("AI 分析失败".to_string()))?;

    let fenxi_json: Value = serde_json::from_str(&fenxi_jieguo)
        .unwrap_or(serde_json::json!(null));

    Ok(serde_json::json!({"ai_fenxi": fenxi_json}))
}

// ==================== AI 深度分析 ====================

/// AI深度分析：查询关联日报 → 拼接日报原文 → 按维度调用AI深度分析
pub async fn shendu_fenxi(shiti_leixing: &str, shiti_mingcheng: &str, weidu: &str) -> Result<Value, FenxiCuowu> {
    let ribaolie = cangchu::chaxun_shiti_ribao(shiti_leixing, shiti_mingcheng)
        .await
        .map_err(|_| FenxiCuowu::ShujiWeizhaodao("未找到相关日报".to_string()))?;

    let pingjie = pingjie_ribao_neirong(shiti_mingcheng, &ribaolie);

    let fenxi_jieguo = kuaribaofenxi::ai_ribao_shendu_fenxi(&pingjie, weidu)
        .await
        .ok_or_else(|| FenxiCuowu::AiFenxiShibai("AI 深度分析失败".to_string()))?;

    let fenxi_json: Value = serde_json::from_str(&fenxi_jieguo)
        .unwrap_or(serde_json::json!(null));

    Ok(serde_json::json!({
        "weidu": weidu,
        "ai_fenxi": fenxi_json,
    }))
}

/// 拼接多篇日报原文为 AI 输入文本（使用强类型 RibaoZhaiyao）
fn pingjie_ribao_neirong(shiti_mingcheng: &str, ribaolie: &[RibaoZhaiyao]) -> String {
    let mut neirong_duanlie: Vec<String> = Vec::new();
    for rb in ribaolie {
        let mut duan = format!("---\n[{}] {}\n", rb.fabushijian, rb.biaoti);
        if !rb.zhaiyao.is_empty() {
            duan.push_str(&format!("摘要：{}\n", rb.zhaiyao));
        }
        if !rb.neirong.is_empty() {
            duan.push_str(&rb.neirong);
        }
        duan.push('\n');
        neirong_duanlie.push(duan);
    }
    format!(
        "以下是关于「{}」的日报原文（共{}篇）：\n\n{}",
        shiti_mingcheng,
        ribaolie.len(),
        neirong_duanlie.join("\n")
    )
}

// ==================== 综合关联分析（跨类型） ====================

/// 跨类型关联分析：接受多个 (leixing, zhi) 对，查询标签+日报原文后统一发送AI
pub async fn zonghe_guanlian_fenxi(shiti_liebiao: &[(&str, &str)], yonghu_tishi: &str) -> Result<Value, FenxiCuowu> {
    if shiti_liebiao.len() < 2 {
        return Err(FenxiCuowu::ShuruWuxiao("至少勾选两个实体进行关联分析".to_string()));
    }

    let mut shiti_shuju: Vec<Value> = Vec::new();
    let mut ribao_duanlie: Vec<String> = Vec::new();
    for (leixing, zhi) in shiti_liebiao {
        let biaoqianlie = cangchu::chaxun_shiti_biaoqian(leixing, zhi).await;
        shiti_shuju.push(serde_json::json!({
            "xiangmu_mingcheng": format!("[{}] {}", leixing, zhi),
            "biaoqianlie": biaoqianlie,
        }));
        // 加载日报原文以提供深度分析上下文
        if let Ok(ribaolie) = cangchu::chaxun_shiti_ribao(leixing, zhi).await {
            ribao_duanlie.push(pingjie_ribao_neirong(&format!("[{}] {}", leixing, zhi), &ribaolie));
        }
    }

    let shuru = serde_json::json!({"xiangmulie": shiti_shuju});
    let ribao_neirong = ribao_duanlie.join("\n\n");

    let fenxi_jieguo = kuaribaofenxi::ai_guanlian_shendu_fenxi(&shuru, &ribao_neirong, yonghu_tishi)
        .await
        .ok_or_else(|| FenxiCuowu::AiFenxiShibai("AI 关联分析失败".to_string()))?;

    let fenxi_json: Value = serde_json::from_str(&fenxi_jieguo)
        .unwrap_or(serde_json::json!(null));

    Ok(serde_json::json!({
        "xiangmu_shuju": shiti_shuju,
        "ai_fenxi": fenxi_json,
    }))
}

// ==================== 实体关联分析（统一入口） ====================

/// 实体关联分析（统一处理 fenxi_shiti_guanlian 和 fenxi_xiangmu_guanlian）
/// 输入实体类型名称和值列表 → 聚合标签+日报原文 → 调用AI深度关联分析
pub async fn shiti_guanlian_fenxi(leixingmingcheng: &str, zhi_liebiao: &[String], yonghu_tishi: &str) -> Result<Value, FenxiCuowu> {
    if zhi_liebiao.len() < 2 {
        return Err(FenxiCuowu::ShuruWuxiao("至少选择两个进行关联分析".to_string()));
    }

    let mut shiti_shuju: Vec<Value> = Vec::new();
    let mut ribao_duanlie: Vec<String> = Vec::new();
    for zhi in zhi_liebiao {
        let biaoqianlie = cangchu::chaxun_shiti_biaoqian(leixingmingcheng, zhi).await;
        shiti_shuju.push(serde_json::json!({
            "xiangmu_mingcheng": zhi,
            "biaoqianlie": biaoqianlie,
        }));
        if let Ok(ribaolie) = cangchu::chaxun_shiti_ribao(leixingmingcheng, zhi).await {
            ribao_duanlie.push(pingjie_ribao_neirong(zhi, &ribaolie));
        }
    }

    let shuru = serde_json::json!({"xiangmulie": shiti_shuju});
    let ribao_neirong = ribao_duanlie.join("\n\n");

    let fenxi_jieguo = kuaribaofenxi::ai_guanlian_shendu_fenxi(&shuru, &ribao_neirong, yonghu_tishi)
        .await
        .ok_or_else(|| FenxiCuowu::AiFenxiShibai("AI 关联分析失败".to_string()))?;

    let fenxi_json: Value = serde_json::from_str(&fenxi_jieguo)
        .unwrap_or(serde_json::json!(null));

    Ok(serde_json::json!({
        "xiangmu_shuju": shiti_shuju,
        "ai_fenxi": fenxi_json,
    }))
}
