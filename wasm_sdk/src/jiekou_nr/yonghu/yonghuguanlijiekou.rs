use serde::Serialize;

pub const lujing: &str = "/jiekou/yonghu/yonghuguanli";
pub const fangshi: &str = "POST";

pub type Xiangying = super::super::Xiangying<serde_json::Value>;

#[derive(Serialize)]
pub struct Qingqiuti {
    pub caozuo: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dangqianyeshu: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meiyeshuliang: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guanjianci: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zhanghao: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nicheng: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mima: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beizhu: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yuanyin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jieshu: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yonghuzuid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mingcheng: Option<String>,
}
