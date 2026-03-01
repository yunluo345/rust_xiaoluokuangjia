use llm::chat::Tool;
use serde_json::{json, Value};
use crate::shujuku::psqlshujuku::shujubiao_nr::ribao::{shujucaozuo_ribao, shujucaozuo_ribao_biaoqianrenwu};
use crate::shujuku::psqlshujuku::shujubiao_nr::yonghu::yonghuyanzheng;
use super::ribao::gongju_ribaorenwuchuli;
use super::Gongjufenzu;

pub fn huoqu_guanjianci() -> Vec<String> {
    vec![
        "日报管理".to_string(),
        "任务管理".to_string(),
        "删除日报".to_string(),
        "删除任务".to_string(),
        "重新入队".to_string(),
        "日报操作".to_string(),
    ]
}

pub fn huoqu_fenzu() -> Gongjufenzu {
    Gongjufenzu::Guanli
}

pub fn dinyi() -> Tool {
    Tool {
        tool_type: "function".to_string(),
        function: llm::chat::FunctionTool {
            name: "ribao_renwu_guanli".to_string(),
            description: "日报与任务管理工具：支持日报删除/更新/查询，以及标签任务重入队、删除、分页查询、调度控制".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "caozuo": {
                        "type": "string",
                        "enum": [
                            "ribao_shanchu",
                            "ribao_piliang_shanchu",
                            "ribao_gengxin",
                            "ribao_chaxun_id",
                            "ribao_chaxun_fenye",
                            "ribao_chaxun_yonghuid_fenye",
                            "ribao_tongji_zongshu",
                            "renwu_chaxun_id",
                            "renwu_chaxun_fenye",
                            "renwu_chongxin_ruidui",
                            "renwu_chongxin_ruidui_ribaoid",
                            "renwu_shanchu",
                            "renwu_piliang_shanchu",
                            "renwu_dange_chuli",
                            "renwu_biaoqian_ai_chuli",
                            "renwu_biaoqian_ai_tingzhi",
                            "renwu_biaoqian_ai_zhuangtai"
                        ]
                    },
                    "id": { "type": "string" },
                    "idlie": { "type": "array", "items": { "type": "string" } },
                    "ribaoid": { "type": "string" },
                    "yonghuid": { "type": "string" },
                    "yeshu": { "type": "integer" },
                    "meiyetiaoshu": { "type": "integer" },
                    "zhuangtai": { "type": "string" },
                    "ziduanlie": {
                        "type": "array",
                        "items": {
                            "type": "array",
                            "items": { "type": "string" },
                            "minItems": 2,
                            "maxItems": 2
                        }
                    }
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

fn qu_zhengshu(v: &Value, jian: &str, moren: i64) -> i64 {
    v.get(jian).and_then(|x| x.as_i64()).unwrap_or(moren)
}

fn qu_ziduanlie(v: &Value) -> Vec<(String, String)> {
    v.get("ziduanlie").and_then(|x| x.as_array()).map(|arr| {
        arr.iter().filter_map(|item| {
            let duan = item.as_array()?;
            if duan.len() != 2 {
                return None;
            }
            let k = duan[0].as_str()?.trim().to_string();
            let z = duan[1].as_str()?.to_string();
            if k.is_empty() {
                return None;
            }
            Some((k, z))
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
        "ribao_shanchu" => {
            let id = match qu_zifuchuan(&qingqiu, "id") {
                Some(v) => v,
                None => return shibai("缺少参数: id"),
            };
            match shujucaozuo_ribao::shanchu(&id).await {
                Some(n) if n > 0 => chenggong(json!({"yingxiang": n})),
                Some(_) => shibai("日报不存在"),
                None => shibai("删除失败"),
            }
        }
        "ribao_piliang_shanchu" => {
            let idlie = qu_zifuchuanlie(&qingqiu, "idlie");
            if idlie.is_empty() {
                return shibai("缺少参数: idlie");
            }
            let canshu: Vec<&str> = idlie.iter().map(String::as_str).collect();
            match shujucaozuo_ribao::piliang_shanchu(&canshu).await {
                Some(n) => chenggong(json!({"yingxiang": n})),
                None => shibai("批量删除失败"),
            }
        }
        "ribao_gengxin" => {
            let id = match qu_zifuchuan(&qingqiu, "id") {
                Some(v) => v,
                None => return shibai("缺少参数: id"),
            };
            let ziduanlie = qu_ziduanlie(&qingqiu);
            if ziduanlie.is_empty() {
                return shibai("缺少参数: ziduanlie");
            }
            let canshu: Vec<(&str, &str)> = ziduanlie.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect();
            match shujucaozuo_ribao::gengxin(&id, &canshu).await {
                Some(n) if n > 0 => chenggong(json!({"yingxiang": n})),
                Some(_) => shibai("日报不存在"),
                None => shibai("更新失败"),
            }
        }
        "ribao_chaxun_id" => {
            let id = match qu_zifuchuan(&qingqiu, "id") {
                Some(v) => v,
                None => return shibai("缺少参数: id"),
            };
            match shujucaozuo_ribao::chaxun_id(&id).await {
                Some(shuju) => chenggong(shuju),
                None => shibai("日报不存在"),
            }
        }
        "ribao_chaxun_fenye" => {
            let yeshu = qu_zhengshu(&qingqiu, "yeshu", 1);
            let meiyetiaoshu = qu_zhengshu(&qingqiu, "meiyetiaoshu", 10);
            let liebiao = shujucaozuo_ribao::chaxun_fenye(yeshu, meiyetiaoshu).await.unwrap_or_default();
            let zongshu = shujucaozuo_ribao::tongji_zongshu().await.unwrap_or(0);
            chenggong(json!({"liebiao": liebiao, "zongshu": zongshu}))
        }
        "ribao_chaxun_yonghuid_fenye" => {
            let yonghuid = match qu_zifuchuan(&qingqiu, "yonghuid") {
                Some(v) => v,
                None => return shibai("缺少参数: yonghuid"),
            };
            let yeshu = qu_zhengshu(&qingqiu, "yeshu", 1);
            let meiyetiaoshu = qu_zhengshu(&qingqiu, "meiyetiaoshu", 10);
            let liebiao = shujucaozuo_ribao::chaxun_yonghuid_fenye(&yonghuid, yeshu, meiyetiaoshu).await.unwrap_or_default();
            let zongshu = shujucaozuo_ribao::tongji_yonghuid_zongshu(&yonghuid).await.unwrap_or(0);
            chenggong(json!({"liebiao": liebiao, "zongshu": zongshu}))
        }
        "ribao_tongji_zongshu" => {
            let zongshu = shujucaozuo_ribao::tongji_zongshu().await.unwrap_or(0);
            chenggong(json!({"zongshu": zongshu}))
        }
        "renwu_chaxun_id" => {
            let id = match qu_zifuchuan(&qingqiu, "id") {
                Some(v) => v,
                None => return shibai("缺少参数: id"),
            };
            match shujucaozuo_ribao_biaoqianrenwu::chaxun_id(&id).await {
                Some(shuju) => chenggong(shuju),
                None => shibai("任务不存在"),
            }
        }
        "renwu_chaxun_fenye" => {
            let zhuangtai = qu_zifuchuan(&qingqiu, "zhuangtai");
            let yeshu = qu_zhengshu(&qingqiu, "yeshu", 1);
            let meiyetiaoshu = qu_zhengshu(&qingqiu, "meiyetiaoshu", 10);
            let liebiao = shujucaozuo_ribao_biaoqianrenwu::chaxun_fenye(zhuangtai.as_deref(), yeshu, meiyetiaoshu).await.unwrap_or_default();
            let zongshu = shujucaozuo_ribao_biaoqianrenwu::tongji_fenye_zongshu(zhuangtai.as_deref()).await.unwrap_or(0);
            chenggong(json!({"liebiao": liebiao, "zongshu": zongshu}))
        }
        "renwu_chongxin_ruidui" => {
            let id = match qu_zifuchuan(&qingqiu, "id") {
                Some(v) => v,
                None => return shibai("缺少参数: id"),
            };
            match shujucaozuo_ribao_biaoqianrenwu::chongxin_ruidui(&id).await {
                Some(n) if n > 0 => chenggong(json!({"yingxiang": n})),
                _ => shibai("重入队失败"),
            }
        }
        "renwu_chongxin_ruidui_ribaoid" => {
            let ribaoid = match qu_zifuchuan(&qingqiu, "ribaoid") {
                Some(v) => v,
                None => return shibai("缺少参数: ribaoid"),
            };
            match shujucaozuo_ribao_biaoqianrenwu::chongxin_ruidui_ribaoid(&ribaoid).await {
                Some(n) if n > 0 => chenggong(json!({"yingxiang": n})),
                _ => shibai("重入队失败"),
            }
        }
        "renwu_shanchu" => {
            let id = match qu_zifuchuan(&qingqiu, "id") {
                Some(v) => v,
                None => return shibai("缺少参数: id"),
            };
            match shujucaozuo_ribao_biaoqianrenwu::shanchu(&id).await {
                Some(n) if n > 0 => chenggong(json!({"yingxiang": n})),
                _ => shibai("删除失败"),
            }
        }
        "renwu_piliang_shanchu" => {
            let idlie = qu_zifuchuanlie(&qingqiu, "idlie");
            if idlie.is_empty() {
                return shibai("缺少参数: idlie");
            }
            let canshu: Vec<&str> = idlie.iter().map(String::as_str).collect();
            match shujucaozuo_ribao_biaoqianrenwu::piliang_shanchu(&canshu).await {
                Some(n) => chenggong(json!({"yingxiang": n})),
                None => shibai("批量删除失败"),
            }
        }
        "renwu_dange_chuli" => {
            let id = match qu_zifuchuan(&qingqiu, "id") {
                Some(v) => v,
                None => return shibai("缺少参数: id"),
            };
            match gongju_ribaorenwuchuli::zhixing_dange_renwu_neibu(&id).await {
                Ok(shuju) => chenggong(shuju),
                Err(e) => shibai(&e),
            }
        }
        "renwu_biaoqian_ai_chuli" => {
            match gongju_ribaorenwuchuli::zhixing_neibu().await {
                Ok(shuju) => chenggong(shuju),
                Err(e) => shibai(&e),
            }
        }
        "renwu_biaoqian_ai_tingzhi" => {
            let yuanxian = shujucaozuo_ribao_biaoqianrenwu::tingzhi();
            chenggong(json!({"yuanxianyunxing": yuanxian}))
        }
        "renwu_biaoqian_ai_zhuangtai" => {
            let yunxingzhong = shujucaozuo_ribao_biaoqianrenwu::shifou_yunxingzhong();
            chenggong(json!({"yunxingzhong": yunxingzhong}))
        }
        _ => shibai("未知操作类型"),
    }
}
