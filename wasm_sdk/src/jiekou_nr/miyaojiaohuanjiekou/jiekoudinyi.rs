use serde::{Deserialize, Serialize};

#[allow(non_upper_case_globals)]
pub const lujing: &str = "/jiekou/jiami/gongyao";
#[allow(non_upper_case_globals)]
pub const fangshi: &str = "POST";

#[derive(Serialize)]
pub struct Qingqiuti {
    pub zhiwen: String,
}

#[derive(Deserialize)]
pub struct Xiangyingshuju {
    pub huihuaid: String,
    pub gongyao: String,
}

#[derive(Deserialize)]
pub struct Xiangying {
    pub zhuangtaima: u16,
    pub xiaoxi: String,
    pub shuju: Option<Xiangyingshuju>,
}
