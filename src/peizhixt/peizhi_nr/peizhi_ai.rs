#![allow(non_upper_case_globals)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Aipeizhi {
    pub biaoqiantiqu: Biaoqiantiqu,
    pub qudaohuoqu: Qudaohuoqu,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Biaoqiantiqu {
    pub bixuyou: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Qudaohuoqu {
    /// 是否启用重试机制
    pub qiyongchongshi: bool,
    /// 重试次数
    pub chongshicishu: u32,
    /// 重试间隔（毫秒）
    pub chongshijiange: u64,
}

impl Default for Aipeizhi {
    fn default() -> Self {
        Self {
            biaoqiantiqu: Biaoqiantiqu::default(),
            qudaohuoqu: Qudaohuoqu::default(),
        }
    }
}

impl Default for Biaoqiantiqu {
    fn default() -> Self {
        Self {
            bixuyou: vec!["人名".to_string()],
        }
    }
}

impl Default for Qudaohuoqu {
    fn default() -> Self {
        Self {
            qiyongchongshi: true,
            chongshicishu: 3,
            chongshijiange: 1000,
        }
    }
}

impl Aipeizhi {
    pub fn wenjianming() -> &'static str {
        "ai"
    }
}
