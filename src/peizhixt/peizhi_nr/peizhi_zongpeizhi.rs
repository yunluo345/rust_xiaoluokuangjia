#![allow(non_upper_case_globals)]

#![allow(non_upper_case_globals)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Zongpeizhi {
    pub wangzhanmingcheng: String,
    pub houduanyunxingduankou: u16,
}

impl Default for Zongpeizhi {
    fn default() -> Self {
        Self {
            wangzhanmingcheng: "默认网站".to_string(),
            houduanyunxingduankou: 8080,
        }
    }
}

impl Zongpeizhi {
    pub fn wenjianming() -> &'static str {
        "zongpeizhi"
    }
}
