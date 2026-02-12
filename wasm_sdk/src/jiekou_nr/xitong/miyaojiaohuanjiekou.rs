use serde::{Deserialize, Serialize};

pub const lujing: &str = "/jiekou/jiami/gongyao";
pub const fangshi: &str = "POST";

pub type Xiangying = super::super::Xiangying<Xiangyingshuju>;

#[derive(Serialize)]
pub struct Qingqiuti {
    pub zhiwen: String,
}

#[derive(Deserialize)]
pub struct Xiangyingshuju {
    pub huihuaid: String,
    pub gongyao: String,
}
