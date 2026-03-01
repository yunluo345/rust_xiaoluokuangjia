use llm::chat::Tool;
use serde_json::{json, Value};
use crate::peizhixt::peizhi_nr::peizhi_ai::Ai;
use crate::peizhixt::peizhixitongzhuti;
use crate::yewu::ribao_fenxi::fenxi_yongli;
use crate::shujuku::psqlshujuku::shujubiao_nr::yonghu::yonghuyanzheng;
use super::Gongjufenzu;

pub fn huoqu_guanjianci() -> Vec<String> {
    vec![
        "深度分析".to_string(),
        "AI分析".to_string(),
        "关联分析".to_string(),
        "交流分析".to_string(),
        "维度分析".to_string(),
        "项目关联".to_string(),
    ]
}

pub fn huoqu_fenzu() -> Gongjufenzu {
    Gongjufenzu::Guanli
}

pub fn dinyi() -> Tool {
    Tool {
        tool_type: "function".to_string(),
        function: llm::chat::FunctionTool {
            name: "ribao_shendu_fenxi".to_string(),
            description: "日报AI深度分析工具：交流内容分析、维度深度分析、实体关联分析、综合关联分析".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "caozuo": {
                        "type": "string",
                        "enum": [
                            "fenxi_huoqu_shiti_leixing",
                            "jiaoliu_neirong_fenxi",
                            "shendu_fenxi",
                            "shiti_guanlian_fenxi",
                            "zonghe_guanlian_fenxi"
                        ]
                    },
                    "shiti_leixing": { "type": "string" },
                    "shiti_mingcheng": { "type": "string" },
                    "weidu": { "type": "string" },
                    "leixingmingcheng": { "type": "string" },
                    "zhi_liebiao": { "type": "array", "items": { "type": "string" } },
                    "shiti_liebiao": {
                        "type": "array",
                        "items": {
                            "type": "object",
                            "properties": {
                                "leixing": { "type": "string" },
                                "zhi": { "type": "string" }
                            },
                            "required": ["leixing", "zhi"]
                        }
                    },
                    "yonghu_tishi": { "type": "string" }
                },
                "required": ["caozuo"]
            }),
        },
    }
}

fn chenggong(shuju: Value) -> String {
    json!({"chenggong": true, "shuju": shuju}).to_string()
}

fn shibai(xinxi: &str) -> String {
    json!({"cuowu": xinxi}).to_string()
}

fn qu_zifuchuan(v: &Value, jian: &str) -> Option<String> {
    v.get(jian).and_then(|x| x.as_str()).map(|s| s.trim().to_string()).filter(|s| !s.is_empty())
}

fn qu_zifuchuanlie(v: &Value, jian: &str) -> Vec<String> {
    v.get(jian).and_then(|x| x.as_array()).map(|arr| {
        arr.iter()
            .filter_map(|x| x.as_str().map(|s| s.trim().to_string()))
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
    }).unwrap_or_default()
}

fn qu_shiti_liebiao(v: &Value) -> Vec<(String, String)> {
    v.get("shiti_liebiao").and_then(|x| x.as_array()).map(|arr| {
        arr.iter().filter_map(|item| {
            let leixing = item.get("leixing")?.as_str()?.trim().to_string();
            let zhi = item.get("zhi")?.as_str()?.trim().to_string();
            if leixing.is_empty() || zhi.is_empty() {
                return None;
            }
            Some((leixing, zhi))
        }).collect::<Vec<_>>()
    }).unwrap_or_default()
}

pub async fn zhixing(canshu: &str, lingpai: &str) -> String {
    let _zaiti = match yonghuyanzheng::yanzhenglingpaijiquanxian(lingpai, "/jiekou/ribao/guanli").await {
        Ok(z) => z,
        Err(yonghuyanzheng::Lingpaicuowu::Yibeifengjin(y)) => return shibai(&format!("账号已被封禁：{}", y)),
        Err(yonghuyanzheng::Lingpaicuowu::Quanxianbuzu) => return shibai("权限不足"),
        Err(_) => return shibai("令牌无效或已过期"),
    };

    let qingqiu: Value = match serde_json::from_str(canshu) {
        Ok(v) => v,
        Err(_) => return shibai("参数格式错误"),
    };
    let caozuo = match qu_zifuchuan(&qingqiu, "caozuo") {
        Some(c) => c,
        None => return shibai("缺少参数: caozuo"),
    };

    match caozuo.as_str() {
        "fenxi_huoqu_shiti_leixing" => {
            let peizhi = peizhixitongzhuti::duqupeizhi::<Ai>(Ai::wenjianming()).unwrap_or_default();
            chenggong(json!(peizhi.fenxi_shiti_leixing))
        }
        "jiaoliu_neirong_fenxi" => {
            let shiti_leixing = match qu_zifuchuan(&qingqiu, "shiti_leixing") {
                Some(v) => v,
                None => return shibai("缺少参数: shiti_leixing"),
            };
            let shiti_mingcheng = match qu_zifuchuan(&qingqiu, "shiti_mingcheng") {
                Some(v) => v,
                None => return shibai("缺少参数: shiti_mingcheng"),
            };
            match fenxi_yongli::jiaoliu_neirong_fenxi(&shiti_leixing, &shiti_mingcheng).await {
                Ok(shuju) => chenggong(shuju),
                Err(e) => shibai(e.xiaoxi()),
            }
        }
        "shendu_fenxi" => {
            let shiti_leixing = match qu_zifuchuan(&qingqiu, "shiti_leixing") {
                Some(v) => v,
                None => return shibai("缺少参数: shiti_leixing"),
            };
            let shiti_mingcheng = match qu_zifuchuan(&qingqiu, "shiti_mingcheng") {
                Some(v) => v,
                None => return shibai("缺少参数: shiti_mingcheng"),
            };
            let weidu = match qu_zifuchuan(&qingqiu, "weidu") {
                Some(v) => v,
                None => return shibai("缺少参数: weidu"),
            };
            match fenxi_yongli::shendu_fenxi(&shiti_leixing, &shiti_mingcheng, &weidu).await {
                Ok(shuju) => chenggong(shuju),
                Err(e) => shibai(e.xiaoxi()),
            }
        }
        "shiti_guanlian_fenxi" => {
            let leixingmingcheng = match qu_zifuchuan(&qingqiu, "leixingmingcheng") {
                Some(v) => v,
                None => return shibai("缺少参数: leixingmingcheng"),
            };
            let zhi_liebiao = qu_zifuchuanlie(&qingqiu, "zhi_liebiao");
            if zhi_liebiao.len() < 2 {
                return shibai("zhi_liebiao 至少需要两个值");
            }
            let yonghu_tishi = qu_zifuchuan(&qingqiu, "yonghu_tishi").unwrap_or_default();
            match fenxi_yongli::shiti_guanlian_fenxi(&leixingmingcheng, &zhi_liebiao, &yonghu_tishi).await {
                Ok(shuju) => chenggong(shuju),
                Err(e) => shibai(e.xiaoxi()),
            }
        }
        "zonghe_guanlian_fenxi" => {
            let shiti_liebiao = qu_shiti_liebiao(&qingqiu);
            if shiti_liebiao.len() < 2 {
                return shibai("shiti_liebiao 至少需要两个实体");
            }
            let yonghu_tishi = qu_zifuchuan(&qingqiu, "yonghu_tishi").unwrap_or_default();
            let canshu_lie: Vec<(&str, &str)> = shiti_liebiao.iter().map(|(l, z)| (l.as_str(), z.as_str())).collect();
            match fenxi_yongli::zonghe_guanlian_fenxi(&canshu_lie, &yonghu_tishi).await {
                Ok(shuju) => chenggong(shuju),
                Err(e) => shibai(e.xiaoxi()),
            }
        }
        _ => shibai("未知操作类型"),
    }
}
