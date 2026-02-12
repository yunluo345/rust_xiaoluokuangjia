use serde::Deserialize;

#[allow(non_upper_case_globals)]
pub const lujing: &str = "/jiekou/xitong/jiankang";
#[allow(non_upper_case_globals)]
pub const fangshi: &str = "GET";

#[derive(Deserialize)]
pub struct Xiangyingshuju {
    pub zhuangtai: String,
}

#[derive(Deserialize)]
pub struct Xiangying {
    pub zhuangtaima: u16,
    pub xiaoxi: String,
    pub shuju: Option<Xiangyingshuju>,
}
