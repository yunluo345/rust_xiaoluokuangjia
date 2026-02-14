#![allow(non_upper_case_globals)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Aipeizhi {
    pub biaoqiantiqu: Biaoqiantiqu,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Biaoqiantiqu {
    pub bixuyou: Vec<String>,
}

impl Default for Aipeizhi {
    fn default() -> Self {
        Self {
            biaoqiantiqu: Biaoqiantiqu::default(),
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

impl Aipeizhi {
    pub fn wenjianming() -> &'static str {
        "ai"
    }
}
