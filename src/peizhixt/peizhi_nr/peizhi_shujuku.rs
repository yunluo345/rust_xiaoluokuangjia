#![allow(non_upper_case_globals)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shujuku {
    pub xiangliangku: Xiangliangku,
    pub psql: Psql,
    pub redis: Redispeizhi,
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
            redis: Redispeizhi::default(),
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Redispeizhi {
    pub bixuchushihua: bool,
    pub fuwuqiid: String,
    pub fuwuqimingcheng: String,
    pub zhujidizhi: String,
    pub duankou: u16,
    pub zhanghao: String,
    pub mima: String,
}

impl Default for Redispeizhi {
    fn default() -> Self {
        Self {
            bixuchushihua: true,
            fuwuqiid: "zhuredis".to_string(),
            fuwuqimingcheng: "主要redis".to_string(),
            zhujidizhi: "127.0.0.1".to_string(),
            duankou: 6379,
            zhanghao: "default".to_string(),
            mima: "111222".to_string(),
        }
    }
}

impl Shujuku {
    pub fn wenjianming() -> &'static str {
        "shujuku"
    }
}
