use serde::{Deserialize, Serialize};

pub const lujing: &str = "/jiekou/xitong/jiamiceshi";
pub const fangshi: &str = "POST";

pub type Xiangying = super::super::Xiangying<Xiangyingshuju>;

#[derive(Serialize)]
pub struct Qingqiuti {
    pub neirong: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct Xiangyingshuju {
    pub huifu: String,
    pub yuanshishuju: Option<String>,
}
