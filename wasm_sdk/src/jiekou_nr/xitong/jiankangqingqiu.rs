use serde::{Deserialize, Serialize};

pub const lujing: &str = "/jiekou/xitong/jiankang";
pub const fangshi: &str = "GET";

pub type Xiangying = super::super::Xiangying<Xiangyingshuju>;

#[derive(Deserialize, Serialize)]
pub struct Xiangyingshuju {
    pub zhuangtai: String,
}
