#![allow(non_upper_case_globals)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ribaobiaoqian {
    pub mingcheng: String,
    pub miaoshu: String,
    #[serde(default = "moren_bitian")]
    pub bitian: bool,
    #[serde(default)]
    pub biecheng: Vec<String>,
}

fn moren_bitian() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ai {
    pub zuida_xunhuancishu: u32,
    #[serde(default = "moren_ribao_biaoqianrenwu_chongshi_cishu")]
    pub ribao_biaoqianrenwu_chongshi_cishu: u32,
    #[serde(default = "moren_ribao_biaoqianrenwu_bingfashuliang")]
    pub ribao_biaoqianrenwu_bingfashuliang: u32,
    #[serde(default = "moren_bingxingrenwushu")]
    pub bingxingrenwushu: u32,
    pub ribao_biaoqian: Vec<Ribaobiaoqian>,
}

fn moren_ribao_biaoqianrenwu_chongshi_cishu() -> u32 {
    3
}

fn moren_ribao_biaoqianrenwu_bingfashuliang() -> u32 {
    1
}

fn moren_bingxingrenwushu() -> u32 {
    5
}

impl Default for Ai {
    fn default() -> Self {
        Self {
            zuida_xunhuancishu: 20,
            ribao_biaoqianrenwu_chongshi_cishu: 3,
            ribao_biaoqianrenwu_bingfashuliang: 1,
            bingxingrenwushu: 5,
            ribao_biaoqian: vec![
                Ribaobiaoqian {
                    mingcheng: "我方人员".to_string(),
                    miaoshu: "我方公司参与人员姓名".to_string(),
                    bitian: true,
                    biecheng: vec![],
                },
                Ribaobiaoqian {
                    mingcheng: "对方人员".to_string(),
                    miaoshu: "对方公司参与人员姓名".to_string(),
                    bitian: true,
                    biecheng: vec![],
                },
            ],
        }
    }
}

impl Ai {
    pub fn wenjianming() -> &'static str {
        "ai"
    }
}

