use serde::{Deserialize, Serialize};

pub const lujing: &str = "/jiekou/ai/duihua";
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

#[derive(Serialize, Deserialize)]
pub struct Xiangying {
    pub zhuangtaima: u16,
    pub xiaoxi: String,
    pub shuju: Option<Huifushuju>,
}

#[derive(Serialize, Deserialize)]
pub struct Huifushuju {
    pub huifu: String,
}
