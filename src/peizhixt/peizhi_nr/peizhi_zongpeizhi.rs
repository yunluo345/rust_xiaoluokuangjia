#![allow(non_upper_case_globals)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Zongpeizhi {
    pub wangzhanmingcheng: String,
    pub houduanyunxingduankou: u16,
    pub jwtmiyao: String,
    pub jwtguoqishijian_miao: u64,
}

impl Default for Zongpeizhi {
    fn default() -> Self {
        Self {
            wangzhanmingcheng: "默认网站".to_string(),
            houduanyunxingduankou: 8080,
            jwtmiyao: "qingxiugaicimiyao_xiaoLuo2026".to_string(),
            jwtguoqishijian_miao: 86400,
        }
    }
}

impl Zongpeizhi {
    pub fn wenjianming() -> &'static str {
        "zongpeizhi"
    }
}
