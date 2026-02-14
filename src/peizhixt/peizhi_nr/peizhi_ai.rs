#![allow(non_upper_case_globals)]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Aipeizhi {
    pub ribaoshengcheng: Ribaoshengcheng,
    pub qudaohuoqu: Qudaohuoqu,
}

/// 日报生成配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ribaoshengcheng {
    /// 信息类别到模板占位符的映射（如："时间" -> "riqi"），key 即为必须信息
    pub xinxi_yingshe: HashMap<String, String>,
    /// 日报输出格式模板
    pub shuchu_moban: String,
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
            ribaoshengcheng: Ribaoshengcheng::default(),
            qudaohuoqu: Qudaohuoqu::default(),
        }
    }
}

impl Default for Ribaoshengcheng {
    fn default() -> Self {
        Self {
            xinxi_yingshe: HashMap::new(),
            shuchu_moban: String::new(),
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
