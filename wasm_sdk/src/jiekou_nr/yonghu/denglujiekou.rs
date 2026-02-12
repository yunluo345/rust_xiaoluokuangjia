use serde::{Deserialize, Serialize};

pub const lujing: &str = "/jiekou/yonghu/denglu";
pub const fangshi: &str = "POST";

pub type Xiangying = super::super::Xiangying<Xiangyingshuju>;

#[derive(Serialize)]
pub struct Qingqiuti {
    pub zhanghao: String,
    pub mima: String,
}

#[derive(Deserialize, Serialize)]
pub struct Xiangyingshuju {
    pub lingpai: String,
    pub yonghuid: String,
    pub nicheng: String,
    pub yonghuzuid: String,
}
