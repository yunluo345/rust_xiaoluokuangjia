use llm::chat::Tool;
use serde_json::{json, Value};
use crate::shujuku::psqlshujuku::shujubiao_nr::ribao::{shujucaozuo_ribao_biaoqian, shujucaozuo_ribao_guanxi};
use crate::shujuku::psqlshujuku::shujubiao_nr::yonghu::yonghuyanzheng;
use super::Gongjufenzu;

pub fn huoqu_guanjianci() -> Vec<String> {
    vec![
        "图谱".to_string(),
        "关系图谱".to_string(),
        "关系网".to_string(),
        "人物关系".to_string(),
        "节点".to_string(),
        "边".to_string(),
        "标签查询".to_string(),
        "实体查询".to_string(),
        "关联日报".to_string(),
        "王经理".to_string(),
    ]
}

pub fn huoqu_fenzu() -> Gongjufenzu {
    Gongjufenzu::Guanli
}

pub fn dinyi() -> Tool {
    Tool {
        tool_type: "function".to_string(),
        function: llm::chat::FunctionTool {
            name: "ribao_tupu_guanli".to_string(),
            description: "日报图谱与关系查询工具：支持全图、节点子图、关键词/实体搜索（如王经理）、关联日报分页、关系边查询".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "caozuo": {
                        "type": "string",
                        "description": "操作类型",
                        "enum": [
                            "tupu_quanbu",
                            "tupu_biaoqianid",
                            "tupu_leixingmingcheng",
                            "tupu_sousuo",
                            "biaoqian_shiti_chaxun",
                            "tupu_ribao_fenye",
                            "tupu_bian_ribao_fenye",
                            "tupu_ribao_duobiaoqian_fenye",
                            "tupu_guanxi_shiti_ribao_fenye",
                            "tupu_guanxi_bian_ribao_fenye",
                            "guanxi_chaxun_ribaoid"
                        ]
                    },
                    "biaoqianid": { "type": "string" },
                    "yuan_biaoqianid": { "type": "string" },
                    "mubiao_biaoqianid": { "type": "string" },
                    "biaoqianidlie": { "type": "array", "items": { "type": "string" } },
                    "mingcheng": { "type": "string" },
                    "guanjianci": { "type": "string" },
                    "leixingmingcheng": { "type": "string" },
                    "shitimingcheng": { "type": "string" },
                    "ren1": { "type": "string" },
                    "ren2": { "type": "string" },
                    "ribaoid": { "type": "string" },
                    "yeshu": { "type": "integer" },
                    "meiyetiaoshu": { "type": "integer" },
                    "limit": { "type": "integer" }
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

fn qu_zhengshu(v: &Value, jian: &str, moren: i64) -> i64 {
    v.get(jian).and_then(|x| x.as_i64()).unwrap_or(moren)
}

fn qu_zifuchuanlie(v: &Value, jian: &str) -> Vec<String> {
    v.get(jian).and_then(|x| x.as_array()).map(|arr| {
        arr.iter()
            .filter_map(|x| x.as_str().map(|s| s.trim().to_string()))
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
    }).unwrap_or_default()
}

fn tiqu_id(v: &Value, jian: &str) -> Option<String> {
    let idv = v.get(jian)?;
    idv.as_str().map(|s| s.to_string())
        .or_else(|| idv.as_i64().map(|n| n.to_string()))
        .or_else(|| idv.as_u64().map(|n| n.to_string()))
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
        "tupu_quanbu" => {
            match shujucaozuo_ribao_biaoqian::chaxun_tupu_quanbu().await {
                Some(shuju) => chenggong(shuju),
                None => shibai("查询失败"),
            }
        }
        "tupu_biaoqianid" => {
            let biaoqianid = match qu_zifuchuan(&qingqiu, "biaoqianid") {
                Some(v) => v,
                None => return shibai("缺少参数: biaoqianid"),
            };
            match shujucaozuo_ribao_biaoqian::chaxun_tupu_biaoqianid(&biaoqianid).await {
                Some(shuju) => chenggong(shuju),
                None => shibai("查询失败"),
            }
        }
        "tupu_leixingmingcheng" => {
            let mingcheng = match qu_zifuchuan(&qingqiu, "mingcheng") {
                Some(v) => v,
                None => return shibai("缺少参数: mingcheng"),
            };
            match shujucaozuo_ribao_biaoqian::chaxun_tupu_leixingmingcheng(&mingcheng).await {
                Some(shuju) => chenggong(shuju),
                None => shibai("查询失败"),
            }
        }
        "tupu_sousuo" => {
            let guanjianci = match qu_zifuchuan(&qingqiu, "guanjianci") {
                Some(v) => v,
                None => return shibai("缺少参数: guanjianci"),
            };
            let leixingmingcheng = qu_zifuchuan(&qingqiu, "leixingmingcheng");
            let limit = qu_zhengshu(&qingqiu, "limit", 20);
            match shujucaozuo_ribao_biaoqian::tupu_sousuo(&guanjianci, leixingmingcheng.as_deref(), limit).await {
                Some(liebiao) => chenggong(json!({"liebiao": liebiao, "zongshu": liebiao.len()})),
                None => shibai("查询失败"),
            }
        }
        "biaoqian_shiti_chaxun" => {
            let guanjianci = match qu_zifuchuan(&qingqiu, "guanjianci") {
                Some(v) => v,
                None => return shibai("缺少参数: guanjianci"),
            };
            let leixingmingcheng = qu_zifuchuan(&qingqiu, "leixingmingcheng");
            let limit = qu_zhengshu(&qingqiu, "limit", 10);
            let liebiao = match shujucaozuo_ribao_biaoqian::tupu_sousuo(&guanjianci, leixingmingcheng.as_deref(), limit).await {
                Some(v) => v,
                None => return shibai("查询失败"),
            };
            let shouge_tupu = match liebiao.first().and_then(|x| tiqu_id(x, "biaoqianid")) {
                Some(id) => shujucaozuo_ribao_biaoqian::chaxun_tupu_biaoqianid(&id).await,
                None => None,
            };
            chenggong(json!({
                "chaxun_ci": guanjianci,
                "mingzhongshu": liebiao.len(),
                "liebiao": liebiao,
                "shouge_tupu": shouge_tupu
            }))
        }
        "tupu_ribao_fenye" => {
            let biaoqianid = match qu_zifuchuan(&qingqiu, "biaoqianid") {
                Some(v) => v,
                None => return shibai("缺少参数: biaoqianid"),
            };
            let yeshu = qu_zhengshu(&qingqiu, "yeshu", 1);
            let meiyetiaoshu = qu_zhengshu(&qingqiu, "meiyetiaoshu", 10);
            let liebiao = shujucaozuo_ribao_biaoqian::tupu_ribao_fenye(&biaoqianid, yeshu, meiyetiaoshu).await.unwrap_or_default();
            let zongshu = shujucaozuo_ribao_biaoqian::tongji_tupu_ribao_zongshu(&biaoqianid).await.unwrap_or(0);
            chenggong(json!({"liebiao": liebiao, "zongshu": zongshu}))
        }
        "tupu_bian_ribao_fenye" => {
            let yuan_biaoqianid = match qu_zifuchuan(&qingqiu, "yuan_biaoqianid") {
                Some(v) => v,
                None => return shibai("缺少参数: yuan_biaoqianid"),
            };
            let mubiao_biaoqianid = match qu_zifuchuan(&qingqiu, "mubiao_biaoqianid") {
                Some(v) => v,
                None => return shibai("缺少参数: mubiao_biaoqianid"),
            };
            let yeshu = qu_zhengshu(&qingqiu, "yeshu", 1);
            let meiyetiaoshu = qu_zhengshu(&qingqiu, "meiyetiaoshu", 10);
            let liebiao = shujucaozuo_ribao_biaoqian::tupu_bian_ribao_fenye(&yuan_biaoqianid, &mubiao_biaoqianid, yeshu, meiyetiaoshu).await.unwrap_or_default();
            let zongshu = shujucaozuo_ribao_biaoqian::tongji_tupu_bian_ribao_zongshu(&yuan_biaoqianid, &mubiao_biaoqianid).await.unwrap_or(0);
            chenggong(json!({"liebiao": liebiao, "zongshu": zongshu}))
        }
        "tupu_ribao_duobiaoqian_fenye" => {
            let biaoqianidlie = qu_zifuchuanlie(&qingqiu, "biaoqianidlie");
            if biaoqianidlie.is_empty() {
                return shibai("缺少参数: biaoqianidlie");
            }
            let yeshu = qu_zhengshu(&qingqiu, "yeshu", 1);
            let meiyetiaoshu = qu_zhengshu(&qingqiu, "meiyetiaoshu", 10);
            let idlie: Vec<&str> = biaoqianidlie.iter().map(String::as_str).collect();
            let liebiao = shujucaozuo_ribao_biaoqian::tupu_ribao_duobiaoqian_fenye(&idlie, yeshu, meiyetiaoshu).await.unwrap_or_default();
            let zongshu = shujucaozuo_ribao_biaoqian::tongji_tupu_duobiaoqian_zongshu(&idlie).await.unwrap_or(0);
            chenggong(json!({"liebiao": liebiao, "zongshu": zongshu}))
        }
        "tupu_guanxi_shiti_ribao_fenye" => {
            let shitimingcheng = match qu_zifuchuan(&qingqiu, "shitimingcheng") {
                Some(v) => v,
                None => return shibai("缺少参数: shitimingcheng"),
            };
            let yeshu = qu_zhengshu(&qingqiu, "yeshu", 1);
            let meiyetiaoshu = qu_zhengshu(&qingqiu, "meiyetiaoshu", 10);
            let liebiao = shujucaozuo_ribao_guanxi::chaxun_ribao_an_shitimingcheng(&shitimingcheng, yeshu, meiyetiaoshu).await.unwrap_or_default();
            let zongshu = shujucaozuo_ribao_guanxi::tongji_ribao_an_shitimingcheng(&shitimingcheng).await.unwrap_or(0);
            chenggong(json!({"liebiao": liebiao, "zongshu": zongshu}))
        }
        "tupu_guanxi_bian_ribao_fenye" => {
            let ren1 = match qu_zifuchuan(&qingqiu, "ren1") {
                Some(v) => v,
                None => return shibai("缺少参数: ren1"),
            };
            let ren2 = match qu_zifuchuan(&qingqiu, "ren2") {
                Some(v) => v,
                None => return shibai("缺少参数: ren2"),
            };
            let yeshu = qu_zhengshu(&qingqiu, "yeshu", 1);
            let meiyetiaoshu = qu_zhengshu(&qingqiu, "meiyetiaoshu", 10);
            let liebiao = shujucaozuo_ribao_guanxi::chaxun_ribao_an_guanxidui(&ren1, &ren2, yeshu, meiyetiaoshu).await.unwrap_or_default();
            let zongshu = shujucaozuo_ribao_guanxi::tongji_ribao_an_guanxidui(&ren1, &ren2).await.unwrap_or(0);
            chenggong(json!({"liebiao": liebiao, "zongshu": zongshu}))
        }
        "guanxi_chaxun_ribaoid" => {
            let ribaoid = match qu_zifuchuan(&qingqiu, "ribaoid") {
                Some(v) => v,
                None => return shibai("缺少参数: ribaoid"),
            };
            match shujucaozuo_ribao_guanxi::chaxun_ribaoid(&ribaoid).await {
                Some(liebiao) => chenggong(json!({"liebiao": liebiao})),
                None => shibai("查询失败"),
            }
        }
        _ => shibai("未知操作类型"),
    }
}
