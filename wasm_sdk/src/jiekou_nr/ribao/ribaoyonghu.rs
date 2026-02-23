use serde::Serialize;
use serde_json::Value;

pub const lujing: &str = "/jiekou/ribao/yonghu";
pub const fangshi: &str = "POST";

pub type Xiangying = super::super::Xiangying<Value>;

#[derive(Serialize)]
pub struct Qingqiuti {
    pub caozuo: String,
    #[serde(flatten)]
    pub canshu: Value,
}
