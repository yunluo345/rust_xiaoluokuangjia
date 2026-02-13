use serde::Serialize;
use crate::gongju::{jichugongju, jwtgongju};
use super::{shujucaozuo_yonghu, shujucaozuo_yonghuzu};

#[derive(Serialize)]
pub struct Denglujieguo {
    pub lingpai: String,
    pub yonghuid: String,
    pub nicheng: String,
    pub yonghuzuid: String,
}

pub enum Denglucuowu {
    Zhanghaomimacuowu,
    Yibeifengjin(String),
    Lingpaishibai,
}

pub enum Lingpaicuowu {
    Wuxiao,
    Yibeifengjin(String),
}

pub enum Quanxiancuowu {
    Weidenglu,
    Lingpaiwuxiao,
    Yibeifengjin(String),
    Jiekoubeijinyong,
}

fn quziduan<'a>(yonghu: &'a serde_json::Value, ming: &str) -> &'a str {
    yonghu.get(ming).and_then(|v| v.as_str()).unwrap_or("")
}

fn jianchafengjin(yonghu: &serde_json::Value) -> Option<String> {
    if quziduan(yonghu, "fengjin") != "1" {
        return None;
    }
    let jieshu = quziduan(yonghu, "fengjinjieshu");
    let yiguoqi = !jieshu.is_empty() && jieshu.parse::<u64>().map_or(false, |s| jichugongju::huoqushijianchuo() > s);
    (!yiguoqi).then(|| quziduan(yonghu, "fengjinyuanyin").to_string())
}

/// 验证账号密码、检查封禁、签发令牌、更新登录时间
pub async fn denglu(zhanghao: &str, mima: &str) -> Result<Denglujieguo, Denglucuowu> {
    let yonghu = match shujucaozuo_yonghu::chaxun_zhanghao(zhanghao).await {
        Some(y) if quziduan(&y, "mima") == mima => y,
        _ => return Err(Denglucuowu::Zhanghaomimacuowu),
    };
    if let Some(yuanyin) = jianchafengjin(&yonghu) {
        return Err(Denglucuowu::Yibeifengjin(yuanyin));
    }
    let yonghuid = quziduan(&yonghu, "id");
    let yonghuzuid = quziduan(&yonghu, "yonghuzuid");
    let lingpai = jwtgongju::qianfa(yonghuid, zhanghao, yonghuzuid).await
        .ok_or(Denglucuowu::Lingpaishibai)?;
    let _ = shujucaozuo_yonghu::gengxindenglu(yonghuid).await;
    Ok(Denglujieguo {
        lingpai,
        yonghuid: yonghuid.to_string(),
        nicheng: quziduan(&yonghu, "nicheng").to_string(),
        yonghuzuid: yonghuzuid.to_string(),
    })
}

/// 验证令牌有效性，同时检查用户封禁状态
pub async fn yanzhenglingpai(lingpai: &str) -> Result<jwtgongju::Zaiti, Lingpaicuowu> {
    let zaiti = jwtgongju::yanzheng(lingpai).await.ok_or(Lingpaicuowu::Wuxiao)?;
    let yonghu = shujucaozuo_yonghu::chaxun_id(&zaiti.yonghuid).await.ok_or(Lingpaicuowu::Wuxiao)?;
    if let Some(yuanyin) = jianchafengjin(&yonghu) {
        return Err(Lingpaicuowu::Yibeifengjin(yuanyin));
    }
    Ok(zaiti)
}

fn jiexi_jinjiekou(yonghuzu: &serde_json::Value) -> Vec<String> {
    quziduan(yonghuzu, "jinjiekou")
        .parse::<serde_json::Value>()
        .ok()
        .and_then(|v| v.as_array().cloned())
        .map(|arr| arr.into_iter().filter_map(|v| v.as_str().map(String::from)).collect())
        .unwrap_or_default()
}

/// 统一接口权限验证：登录检查 + 用户组黑名单检查
pub async fn yanzhengquanxian(lingpai: Option<&str>, lujing: &str, xudenglu: bool, xuyonghuzu: bool) -> Result<Option<jwtgongju::Zaiti>, Quanxiancuowu> {
    if !xudenglu {
        return Ok(None);
    }
    let lingpai = lingpai.ok_or(Quanxiancuowu::Weidenglu)?;
    let zaiti = match yanzhenglingpai(lingpai).await {
        Ok(z) => z,
        Err(Lingpaicuowu::Wuxiao) => return Err(Quanxiancuowu::Lingpaiwuxiao),
        Err(Lingpaicuowu::Yibeifengjin(y)) => return Err(Quanxiancuowu::Yibeifengjin(y)),
    };
    if !xuyonghuzu {
        return Ok(Some(zaiti));
    }
    let yonghuzu = shujucaozuo_yonghuzu::chaxun_id(&zaiti.yonghuzuid).await
        .ok_or(Quanxiancuowu::Lingpaiwuxiao)?;
    let jinjiekou = jiexi_jinjiekou(&yonghuzu);
    if jinjiekou.iter().any(|j| j == lujing) {
        return Err(Quanxiancuowu::Jiekoubeijinyong);
    }
    Ok(Some(zaiti))
}
