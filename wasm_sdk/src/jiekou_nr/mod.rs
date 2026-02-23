pub mod xitong;
pub mod yonghu;
pub mod ai;
pub mod ribao;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Xiangying<T> {
    pub zhuangtaima: u16,
    pub xiaoxi: String,
    pub shuju: Option<T>,
}
