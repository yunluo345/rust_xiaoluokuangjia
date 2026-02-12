#![allow(non_upper_case_globals)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shujuku {
    pub xiangliangku: Xiangliangku,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Xiangliangku {
    pub qiyong: bool,
    pub zhiji: String,
    pub grpc_duankou: u16,
    pub miyao: String,
    pub jheqianzhui: String,
}

impl Default for Shujuku {
    fn default() -> Self {
        Self {
            xiangliangku: Xiangliangku::default(),
        }
    }
}

impl Default for Xiangliangku {
    fn default() -> Self {
        Self {
            qiyong: true,
            zhiji: "localhost".to_string(),
            grpc_duankou: 6334,
            miyao: "".to_string(),
            jheqianzhui: "rust_luokuangjia".to_string(),
        }
    }
}

impl Shujuku {
    pub fn wenjianming() -> &'static str {
        "shujuku"
    }
}
