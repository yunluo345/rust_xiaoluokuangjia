use serde::{Deserialize, Serialize};

pub const lujing: &str = "/jiekou/ai/duihualiushi";
pub const fangshi: &str = "POST";

#[derive(Serialize, Deserialize)]
pub struct Xiaoxi {
    pub juese: String,
    pub neirong: String,
}

#[derive(Serialize)]
pub struct Qingqiuti {
    pub xiaoxilie: Vec<Xiaoxi>,
}
