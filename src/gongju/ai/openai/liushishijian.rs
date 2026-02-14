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

    #[serde(rename = "yasuo_wancheng")]
    Yasuowancheng { zongjie: String },

    /// 工具发现：AI正在搜索可用工具
    #[serde(rename = "gongju_faxian")]
    Gongjufaxian { yitu: String, jieguo: Vec<FaxianGongju> },

    /// 思考过程：AI的推理步骤推送给前端
    #[serde(rename = "sikao_guocheng")]
    Sikaoguocheng { neirong: String },

    /// 意图分析：AI分析用户意图后生成的关键词
    #[serde(rename = "yitu_fenxi")]
    Yitufenxi { yitu: String, guanjianci: Vec<String> },

    #[serde(rename = "wancheng")]
    Wancheng { yuanyin: String },

    #[serde(rename = "cuowu")]
    Cuowu { xinxi: String },
}

/// 工具发现结果中的单个工具信息
#[derive(Clone, Serialize)]
pub struct FaxianGongju {
    pub mingcheng: String,
    pub miaoshu: String,
    pub defen: f64,
    pub yuanyin: String,
}

impl Liushishijian {
    pub fn zhuansse(&self) -> String {
        match serde_json::to_string(self) {
            Ok(json) => format!("data: {}\n\n", json),
            Err(_) => "data: {\"leixing\":\"cuowu\",\"xinxi\":\"序列化失败\"}\n\n".to_string(),
        }
    }
}
