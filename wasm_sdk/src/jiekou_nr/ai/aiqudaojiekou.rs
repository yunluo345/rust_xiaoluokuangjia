use serde::Serialize;

pub const lujing: &str = "/jiekou/ai/aiqudao";
pub const fangshi: &str = "POST";

#[allow(non_upper_case_globals)]
pub const yunxuleixing: &[&str] = &["openai", "xiangliang"];

pub type Xiangying = super::super::Xiangying<serde_json::Value>;

#[derive(Serialize)]
pub struct Qingqiuti {
    pub caozuo: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mingcheng: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leixing: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jiekoudizhi: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub miyao: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moxing: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wendu: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beizhu: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zuidatoken: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub youxianji: Option<String>,
}

impl Qingqiuti {
    pub fn caozuo(ming: &str) -> Self {
        Self {
            caozuo: ming.to_string(),
            id: None,
            mingcheng: None,
            leixing: None,
            jiekoudizhi: None,
            miyao: None,
            moxing: None,
            wendu: None,
            beizhu: None,
            zuidatoken: None,
            youxianji: None,
        }
    }
}
