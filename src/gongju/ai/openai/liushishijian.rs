use serde::Serialize;

#[derive(Clone, Serialize)]
#[serde(tag = "leixing")]
pub enum Liushishijian {
    #[serde(rename = "wenben_kuai")]
    Wenbenkuai { neirong: String },

    #[serde(rename = "gongju_kaishi")]
    Gongjukaishi { suoyin: usize, gongjuid: String, gongjuming: String },

    #[serde(rename = "gongju_canshu")]
    Gongjucanshu { suoyin: usize, bufen_json: String },

    #[serde(rename = "gongju_wancheng")]
    Gongjuwancheng { suoyin: usize, gongjuid: String, gongjuming: String, canshu: String },

    #[serde(rename = "gongju_jieguo")]
    Gongjujieguo { gongjuid: String, gongjuming: String, jieguo: String },

    #[serde(rename = "wancheng")]
    Wancheng { yuanyin: String },

    #[serde(rename = "cuowu")]
    Cuowu { xinxi: String },
}

impl Liushishijian {
    pub fn zhuansse(&self) -> String {
        match serde_json::to_string(self) {
            Ok(json) => format!("data: {}\n\n", json),
            Err(_) => "data: {\"leixing\":\"cuowu\",\"xinxi\":\"序列化失败\"}\n\n".to_string(),
        }
    }
}
