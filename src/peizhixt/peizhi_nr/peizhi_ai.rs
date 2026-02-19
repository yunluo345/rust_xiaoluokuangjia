#![allow(non_upper_case_globals)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ribaobiaoqian {
    pub mingcheng: String,
    pub miaoshu: String,
    #[serde(default = "moren_bitian")]
    pub bitian: bool,
}

fn moren_bitian() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ai {
    pub zuida_xunhuancishu: u32,
    pub ribao_biaoqian: Vec<Ribaobiaoqian>,
}

impl Default for Ai {
    fn default() -> Self {
        Self {
            zuida_xunhuancishu: 20,
            ribao_biaoqian: vec![
                Ribaobiaoqian {
                    mingcheng: "wofangrenyuan".to_string(),
                    miaoshu: "我方公司参与人员姓名".to_string(),
                    bitian: true,
                },
                Ribaobiaoqian {
                    mingcheng: "duifangrenyuan".to_string(),
                    miaoshu: "对方公司参与人员姓名".to_string(),
                    bitian: true,
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
