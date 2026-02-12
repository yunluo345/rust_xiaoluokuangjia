use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use crate::gongju::jichugongju;
use crate::peizhixt::peizhixitongzhuti;
use crate::peizhixt::peizhi_nr::peizhi_zongpeizhi::Zongpeizhi;

#[allow(non_upper_case_globals)]
const guoqishijian_miao: u64 = 86400;

#[derive(Debug, Serialize, Deserialize)]
pub struct Zaiti {
    pub yonghuid: String,
    pub zhanghao: String,
    pub yonghuzuid: String,
    pub qianfashijian: u64,
    pub guoqishijian: u64,
}

/// 签发 JWT
pub fn qianfa(yonghuid: &str, zhanghao: &str, yonghuzuid: &str) -> Option<String> {
    let miyao = huoqumiyao()?;
    let dangqian = jichugongju::huoqushijianchuo() / 1000;
    let zaiti = Zaiti {
        yonghuid: yonghuid.to_string(),
        zhanghao: zhanghao.to_string(),
        yonghuzuid: yonghuzuid.to_string(),
        qianfashijian: dangqian,
        guoqishijian: dangqian + guoqishijian_miao,
    };
    encode(&Header::default(), &zaiti, &EncodingKey::from_secret(miyao.as_bytes())).ok()
}

/// 验证并解析 JWT
pub fn yanzheng(lingpai: &str) -> Option<Zaiti> {
    let miyao = huoqumiyao()?;
    let mut yanzhengqi = Validation::default();
    yanzhengqi.required_spec_claims.clear();
    yanzhengqi.validate_exp = false;
    let jieguo = decode::<Zaiti>(lingpai, &DecodingKey::from_secret(miyao.as_bytes()), &yanzhengqi).ok()?;
    let zaiti = jieguo.claims;
    let dangqian = jichugongju::huoqushijianchuo() / 1000;
    (dangqian < zaiti.guoqishijian).then_some(zaiti)
}

fn huoqumiyao() -> Option<String> {
    let peizhi = peizhixitongzhuti::duqupeizhi::<Zongpeizhi>(Zongpeizhi::wenjianming())?;
    let miyao = peizhi.jwtmiyao;
    (!miyao.is_empty()).then_some(miyao)
}
