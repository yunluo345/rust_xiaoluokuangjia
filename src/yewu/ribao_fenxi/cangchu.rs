//! 分析链路仓储层
//!
//! 在 `shujucaozuo_ribao_biaoqian` 原始 Value 查询之上提供强类型封装，
//! 业务层不再接触 Value 字段解析，错误语义在此层统一处理。

use crate::shujuku::psqlshujuku::shujubiao_nr::ribao::shujucaozuo_ribao_biaoqian;
use super::leixing::{JiaoliuNeirongxiang, ShitiBiaoqianxiang, RibaoZhaiyao};

/// 仓储错误
pub enum CangchuCuowu {
    /// 查询失败（数据库错误）
    ChaxunShibai,
    /// 结果为空
    JieguoWeikong,
}

impl CangchuCuowu {
    pub fn xiaoxi(&self) -> &str {
        match self {
            CangchuCuowu::ChaxunShibai => "数据库查询失败",
            CangchuCuowu::JieguoWeikong => "未找到相关数据",
        }
    }
}

/// 查询实体的交流内容（强类型返回）
pub async fn chaxun_jiaoliu_neirong(
    shiti_leixing: &str,
    shiti_mingcheng: &str,
) -> Result<Vec<JiaoliuNeirongxiang>, CangchuCuowu> {
    let yuanshi = shujucaozuo_ribao_biaoqian::juhe_jiaoliuneirong_anshiti(shiti_leixing, shiti_mingcheng)
        .await
        .ok_or(CangchuCuowu::ChaxunShibai)?;

    let jieguo: Vec<JiaoliuNeirongxiang> = yuanshi.iter()
        .filter_map(JiaoliuNeirongxiang::cong_value)
        .collect();

    if jieguo.is_empty() {
        return Err(CangchuCuowu::JieguoWeikong);
    }
    Ok(jieguo)
}

/// 查询实体关联的所有标签聚合（强类型返回）
pub async fn chaxun_shiti_biaoqian(
    shiti_leixing: &str,
    shiti_mingcheng: &str,
) -> Vec<ShitiBiaoqianxiang> {
    shujucaozuo_ribao_biaoqian::juhe_shiti_biaoqian(shiti_leixing, shiti_mingcheng)
        .await
        .unwrap_or_default()
        .iter()
        .filter_map(ShitiBiaoqianxiang::cong_value)
        .collect()
}

/// 查询实体关联的日报（强类型返回）
pub async fn chaxun_shiti_ribao(
    shiti_leixing: &str,
    shiti_mingcheng: &str,
) -> Result<Vec<RibaoZhaiyao>, CangchuCuowu> {
    let yuanshi = shujucaozuo_ribao_biaoqian::chaxun_leixingmingcheng_zhi(shiti_leixing, shiti_mingcheng)
        .await
        .ok_or(CangchuCuowu::ChaxunShibai)?;

    let jieguo: Vec<RibaoZhaiyao> = yuanshi.iter()
        .filter_map(RibaoZhaiyao::cong_value)
        .collect();

    if jieguo.is_empty() {
        return Err(CangchuCuowu::JieguoWeikong);
    }
    Ok(jieguo)
}
