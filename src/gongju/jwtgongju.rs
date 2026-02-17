use std::sync::OnceLock;
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use crate::gongju::jichugongju;
use crate::peizhixt::peizhixitongzhuti;
use crate::peizhixt::peizhi_nr::peizhi_zongpeizhi::Zongpeizhi;
use crate::shujuku::redisshujuku::rediscaozuo;

#[allow(non_upper_case_globals)]
const redis_qianzhui: &str = "jwt:yonghu:";

#[allow(non_upper_case_globals)]
static miyao_huancun: OnceLock<String> = OnceLock::new();

#[derive(Debug, Serialize, Deserialize)]
pub struct Zaiti {
    pub yonghuid: String,
    pub zhanghao: String,
    pub yonghuzuid: String,
    pub qianfashijian: u64,
    pub guoqishijian: u64,
}

fn redis_jian(yonghuid: &str) -> String {
    format!("{}{}", redis_qianzhui, yonghuid)
}

/// 签发 JWT，写入 Redis 覆盖旧令牌
pub async fn qianfa(yonghuid: &str, zhanghao: &str, yonghuzuid: &str) -> Option<String> {
    let miyao = huoqumiyao()?;
    let guoqishijian_miao = huoquguoqishijian()?;
    let dangqian = jichugongju::huoqushijianchuo() / 1000;
    let zaiti = Zaiti {
        yonghuid: yonghuid.to_string(),
        zhanghao: zhanghao.to_string(),
        yonghuzuid: yonghuzuid.to_string(),
        qianfashijian: dangqian,
        guoqishijian: dangqian + guoqishijian_miao,
    };
    let lingpai = encode(&Header::default(), &zaiti, &EncodingKey::from_secret(miyao.as_bytes())).ok()?;
    rediscaozuo::shezhidaiguoqi(&redis_jian(yonghuid), &lingpai, guoqishijian_miao).await;
    Some(lingpai)
}

/// 验证 JWT 签名、有效期，并校验 Redis 中是否为当前有效令牌
pub async fn yanzheng(lingpai: &str) -> Option<Zaiti> {
    let miyao = huoqumiyao()?;
    let mut yanzhengqi = Validation::default();
    yanzhengqi.required_spec_claims.clear();
    yanzhengqi.validate_exp = false;
    let zaiti = decode::<Zaiti>(lingpai, &DecodingKey::from_secret(miyao.as_bytes()), &yanzhengqi).ok()?.claims;
    let dangqian = jichugongju::huoqushijianchuo() / 1000;
    (dangqian < zaiti.guoqishijian).then_some(())?;
    let cunchu: String = rediscaozuo::huoqu(&redis_jian(&zaiti.yonghuid)).await?;
    (cunchu == lingpai).then_some(zaiti)
}

/// 注销令牌，从 Redis 删除
pub async fn zhuxiao(yonghuid: &str) -> bool {
    rediscaozuo::shanchu(&redis_jian(yonghuid)).await
}

fn huoqumiyao() -> Option<&'static str> {
    Some(miyao_huancun.get_or_init(|| {
        peizhixitongzhuti::duqupeizhi::<Zongpeizhi>(Zongpeizhi::wenjianming())
            .map(|p| p.jwtmiyao)
            .filter(|m| !m.is_empty())
            .unwrap_or_default()
    }).as_str()).filter(|s| !s.is_empty())
}

fn huoquguoqishijian() -> Option<u64> {
    peizhixitongzhuti::duqupeizhi::<Zongpeizhi>(Zongpeizhi::wenjianming())
        .map(|p| p.jwtguoqishijian_miao)
        .filter(|&t| t > 0)
}
