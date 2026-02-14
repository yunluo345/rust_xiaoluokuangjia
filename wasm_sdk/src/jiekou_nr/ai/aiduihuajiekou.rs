use serde::{Deserialize, Serialize};

pub const lujing: &str = "/jiekou/ai/duihua";
pub const fangshi: &str = "POST";

#[derive(Serialize)]
pub struct Qingqiuti {
    pub leixing: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xitongtishici: Option<String>,
    pub xiaoxilie: Vec<Xiaoxixiang>,
}

#[derive(Serialize, Deserialize)]
pub struct Xiaoxixiang {
    pub jiaose: String,
    pub neirong: String,
}
