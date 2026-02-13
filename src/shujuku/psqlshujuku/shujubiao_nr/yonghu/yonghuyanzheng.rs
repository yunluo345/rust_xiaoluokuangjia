use serde::Serialize;
use crate::gongju::{jichugongju, jwtgongju};
use super::shujucaozuo_yonghu;

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
