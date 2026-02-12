#![allow(non_upper_case_globals)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shujuku {
    pub xiangliangku: Xiangliangku,
    pub psql: Psql,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Xiangliangku {
    pub qiyong: bool,
    pub zhiji: String,
    pub grpc_duankou: u16,
    pub miyao: String,
    pub jheqianzhui: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Psql {
    pub qiyong: bool,
    pub zhiji: String,
    pub duankou: u16,
    pub yonghuming: String,
    pub mima: String,
    pub shujukuming: String,
}

impl Default for Shujuku {
    fn default() -> Self {
        Self {
            xiangliangku: Xiangliangku::default(),
            psql: Psql::default(),
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

impl Default for Psql {
    fn default() -> Self {
        Self {
            qiyong: false,
            zhiji: "localhost".to_string(),
            duankou: 5432,
            yonghuming: "postgres".to_string(),
            mima: "".to_string(),
            shujukuming: "rust_luokuangjia".to_string(),
        }
    }
}

impl Shujuku {
    pub fn wenjianming() -> &'static str {
        "shujuku"
    }
}
