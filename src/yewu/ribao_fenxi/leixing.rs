use serde::Serialize;
use serde_json::Value;

/// 标签统计项（juhe_biaoqian_zhi_anleixing 返回值）
#[derive(Debug, Clone, Serialize)]
pub struct BiaoqianTongjixiang {
    pub zhi: String,
    pub ribao_shu: i64,
}

impl BiaoqianTongjixiang {
    pub fn cong_value(v: &Value) -> Option<Self> {
        Some(Self {
            zhi: v.get("zhi")?.as_str()?.to_string(),
            ribao_shu: v.get("ribao_shu")
                .and_then(|x| x.as_str())
                .and_then(|s| s.parse().ok())
                .unwrap_or(0),
        })
    }
}

/// 交流内容项（juhe_jiaoliuneirong_anshiti 返回值）
#[derive(Debug, Clone, Serialize)]
pub struct JiaoliuNeirongxiang {
    pub jiaoliu_neirong: String,
    pub fabushijian: String,
    pub ribaoid: String,
}

impl JiaoliuNeirongxiang {
    pub fn cong_value(v: &Value) -> Option<Self> {
        Some(Self {
            jiaoliu_neirong: v.get("jiaoliu_neirong")?.as_str()?.to_string(),
            fabushijian: v.get("fabushijian").and_then(|x| x.as_str()).unwrap_or("").to_string(),
            ribaoid: v.get("ribaoid").and_then(|x| x.as_str()).unwrap_or("").to_string(),
        })
    }
}

/// 实体标签聚合项（juhe_shiti_biaoqian 返回值）
#[derive(Debug, Clone, Serialize)]
pub struct ShitiBiaoqianxiang {
    pub leixingmingcheng: String,
    pub zhi: String,
    pub cishu: i64,
}

impl ShitiBiaoqianxiang {
    pub fn cong_value(v: &Value) -> Option<Self> {
        Some(Self {
            leixingmingcheng: v.get("leixingmingcheng")?.as_str()?.to_string(),
            zhi: v.get("zhi")?.as_str()?.to_string(),
            cishu: v.get("cishu")
                .and_then(|x| x.as_str())
                .and_then(|s| s.parse().ok())
                .unwrap_or(0),
        })
    }
}

/// 日报摘要（分析链路中使用的日报数据子集）
#[derive(Debug, Clone, Serialize)]
pub struct RibaoZhaiyao {
    pub id: String,
    pub biaoti: String,
    pub neirong: String,
    pub zhaiyao: String,
    pub fabushijian: String,
}

impl RibaoZhaiyao {
    pub fn cong_value(v: &Value) -> Option<Self> {
        Some(Self {
            id: v.get("id").and_then(|x| x.as_str()).unwrap_or("").to_string(),
            biaoti: v.get("biaoti").and_then(|x| x.as_str()).unwrap_or("").to_string(),
            neirong: v.get("neirong").and_then(|x| x.as_str()).unwrap_or("").to_string(),
            zhaiyao: v.get("zhaiyao").and_then(|x| x.as_str()).unwrap_or("").to_string(),
            fabushijian: v.get("fabushijian").and_then(|x| x.as_str()).unwrap_or("").to_string(),
        })
    }
}

#[cfg(test)]
mod ceshi {
    use super::*;
    use serde_json::json;

    #[test]
    fn ceshi_biaoqian_tongjixiang_zhengchang() {
        let v = json!({"zhi": "项目A", "ribao_shu": "5"});
        let xiang = BiaoqianTongjixiang::cong_value(&v).unwrap();
        assert_eq!(xiang.zhi, "项目A");
        assert_eq!(xiang.ribao_shu, 5);
    }

    #[test]
    fn ceshi_biaoqian_tongjixiang_queshi_zhi() {
        let v = json!({"ribao_shu": "3"});
        assert!(BiaoqianTongjixiang::cong_value(&v).is_none());
    }

    #[test]
    fn ceshi_biaoqian_tongjixiang_shu_morenzhi() {
        let v = json!({"zhi": "项目B"});
        let xiang = BiaoqianTongjixiang::cong_value(&v).unwrap();
        assert_eq!(xiang.ribao_shu, 0);
    }

    #[test]
    fn ceshi_jiaoliu_neirongxiang_zhengchang() {
        let v = json!({"jiaoliu_neirong": "客户沟通", "fabushijian": "1700000000", "ribaoid": "42"});
        let xiang = JiaoliuNeirongxiang::cong_value(&v).unwrap();
        assert_eq!(xiang.jiaoliu_neirong, "客户沟通");
        assert_eq!(xiang.ribaoid, "42");
    }

    #[test]
    fn ceshi_jiaoliu_neirongxiang_queshi_neirong() {
        let v = json!({"fabushijian": "123"});
        assert!(JiaoliuNeirongxiang::cong_value(&v).is_none());
    }

    #[test]
    fn ceshi_shiti_biaoqianxiang_zhengchang() {
        let v = json!({"leixingmingcheng": "我方人员", "zhi": "张三", "cishu": "8"});
        let xiang = ShitiBiaoqianxiang::cong_value(&v).unwrap();
        assert_eq!(xiang.leixingmingcheng, "我方人员");
        assert_eq!(xiang.cishu, 8);
    }

    #[test]
    fn ceshi_shiti_biaoqianxiang_queshi_ziduan() {
        let v = json!({"zhi": "张三"});
        assert!(ShitiBiaoqianxiang::cong_value(&v).is_none());
    }

    #[test]
    fn ceshi_ribao_zhaiyao_wanzheng() {
        let v = json!({"id": "1", "biaoti": "标题", "neirong": "内容", "zhaiyao": "摘要", "fabushijian": "123"});
        let zy = RibaoZhaiyao::cong_value(&v).unwrap();
        assert_eq!(zy.id, "1");
        assert_eq!(zy.biaoti, "标题");
    }

    #[test]
    fn ceshi_ribao_zhaiyao_queshi_ziduan_morenkong() {
        let v = json!({});
        let zy = RibaoZhaiyao::cong_value(&v).unwrap();
        assert_eq!(zy.id, "");
        assert_eq!(zy.biaoti, "");
        assert_eq!(zy.neirong, "");
    }
}
