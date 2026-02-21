use serde::Serialize;

pub const lujing: &str = "/jiekou/yonghu/yonghuguanli";
pub const fangshi: &str = "POST";

pub type Xiangying = super::super::Xiangying<serde_json::Value>;

#[derive(Serialize)]
pub struct Qingqiuti {
    pub caozuo: String,
    pub dangqianyeshu: Option<i32>,
    pub meiyeshuliang: Option<i32>,
    pub guanjianci: Option<String>,
    pub id: Option<String>,
}
