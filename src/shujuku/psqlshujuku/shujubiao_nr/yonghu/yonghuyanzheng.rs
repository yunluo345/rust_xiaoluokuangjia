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

fn quziduan<'a>(yonghu: &'a serde_json::Value, ming: &str) -> &'a str {
    yonghu.get(ming).and_then(|v| v.as_str()).unwrap_or("")
}

fn fengjinshengxiao(yonghu: &serde_json::Value) -> bool {
    let jieshu = quziduan(yonghu, "fengjinjieshu");
    jieshu.is_empty() || jieshu.parse::<u64>().map_or(true, |s| jichugongju::huoqushijianchuo() <= s)
}

/// 验证账号密码、检查封禁、签发令牌、更新登录时间
pub async fn denglu(zhanghao: &str, mima: &str) -> Result<Denglujieguo, Denglucuowu> {
    let yonghu = match shujucaozuo_yonghu::chaxun_zhanghao(zhanghao).await {
        Some(y) if quziduan(&y, "mima") == mima => y,
        _ => return Err(Denglucuowu::Zhanghaomimacuowu),
    };
    if quziduan(&yonghu, "fengjin") == "1" && fengjinshengxiao(&yonghu) {
        return Err(Denglucuowu::Yibeifengjin(quziduan(&yonghu, "fengjinyuanyin").to_string()));
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
