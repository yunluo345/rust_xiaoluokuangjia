use serde::{Deserialize, Serialize};
use serde_json::Value;

pub const lujing: &str = "/jiekou/ribao/guanli";
pub const fangshi: &str = "POST";

pub type Xiangying = super::super::Xiangying<Value>;

#[derive(Serialize)]
pub struct Qingqiuti {
    pub caozuo: String,
    #[serde(flatten)]
    pub canshu: Value,
}

#[derive(Deserialize, Serialize)]
pub struct Liebiaoxiangying {
    pub zhuangtaima: u16,
    pub xiaoxi: String,
    pub shuju: Option<Value>,
}
