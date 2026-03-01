use llm::chat::Tool;
use serde_json::{json, Value};
use crate::shujuku::psqlshujuku::shujubiao_nr::ribao::{
    shujucaozuo_biaoqian,
    shujucaozuo_biaoqianleixing,
    shujucaozuo_ribao_biaoqian,
};
use crate::shujuku::psqlshujuku::shujubiao_nr::yonghu::yonghuyanzheng;
use super::Gongjufenzu;

pub fn huoqu_guanjianci() -> Vec<String> {
    vec![
        "标签管理".to_string(),
        "标签类型".to_string(),
        "分类管理".to_string(),
        "修改标签".to_string(),
        "获取标签".to_string(),
        "标签分类".to_string(),
    ]
}

pub fn huoqu_fenzu() -> Gongjufenzu {
    Gongjufenzu::Guanli
}

pub fn dinyi() -> Tool {
    Tool {
        tool_type: "function".to_string(),
        function: llm::chat::FunctionTool {
            name: "ribao_biaoqian_guanli".to_string(),
            description: "日报标签与分类管理：支持先识别分类后分页/关键词查列表，再做精确标签匹配；并支持标签类型、标签、日报标签关联的增删改查".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "caozuo": {
                        "type": "string",
                        "enum": [
                            "leixing_chaxun_quanbu",
                            "leixing_xinzeng",
                            "leixing_gengxin",
                            "leixing_shanchu",
                            "biaoqian_chaxun_quanbu",
                            "biaoqian_chaxun_id",
                            "biaoqian_chaxun_leixingid",
                            "biaoqian_chaxun_leixingid_zhi",
                            "biaoqian_xinzeng",
                            "biaoqian_gengxin",
                            "biaoqian_shanchu",
                            "guanlian_xinzeng",
                            "guanlian_shanchu",
                            "guanlian_shanchu_ribaoid",
                            "guanlian_chaxun_ribaoid_daixinxi",
                            "guanlian_piliang_xinzeng",
                            "guanlian_piliang_shanchu_ribaoidlie"
                        ]
                    },
                    "id": { "type": "string" },
                    "mingcheng": { "type": "string" },
                    "leixingid": { "type": "string", "description": "分类ID或分类名称（支持如 kehu_gongsi 这类别名）" },
                    "zhi": { "type": "string", "description": "标签值；为空时将先返回该分类下标签列表" },
                    "guanjianci": { "type": "string", "description": "列表搜索关键词；用于先缩小范围再精确匹配" },
                    "yeshu": { "type": "integer", "description": "页码，从1开始；仅用于列表查询" },
                    "meiyetiaoshu": { "type": "integer", "description": "每页数量；仅用于列表查询" },
                    "ribaoid": { "type": "string" },
                    "biaoqianid": { "type": "string" },
                    "biaoqianidlie": { "type": "array", "items": { "type": "string" } },
                    "ribaoidlie": { "type": "array", "items": { "type": "string" } }
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

fn qu_id_zifuchuan(v: &Value) -> Option<String> {
    let idv = v.get("id")?;
    idv.as_str().map(|s| s.to_string())
        .or_else(|| idv.as_i64().map(|n| n.to_string()))
        .or_else(|| idv.as_u64().map(|n| n.to_string()))
}

fn guifan_fenleimingcheng(mingcheng: &str) -> String {
    mingcheng
        .trim()
        .to_lowercase()
        .chars()
        .filter(|c| *c != '_' && *c != '-' && !c.is_whitespace())
        .collect()
}

fn tuice_fenleimingcheng(leixing_shuru: &str) -> Option<&'static str> {
    let guifan = guifan_fenleimingcheng(leixing_shuru);
    match guifan.as_str() {
        "kehugongsi" | "kehu" => Some("客户公司"),
        "xiangmu" | "xiangmumingcheng" => Some("项目名称"),
        "wofangrenyuan" | "wofang" => Some("我方人员"),
        "duifangrenyuan" | "duifang" => Some("对方人员"),
        "jiaoliuneirong" | "jiaoliu" => Some("交流内容"),
        "gongzuodidian" | "didian" => Some("工作地点"),
        "ribaoshijian" | "shijian" => Some("日报时间"),
        _ => None,
    }
}

async fn jiexi_leixingid_huo_fenlei(leixing_shuru: &str) -> Option<(String, Value)> {
    if let Some(shuju) = shujucaozuo_biaoqianleixing::chaxun_id(leixing_shuru).await {
        if let Some(id) = qu_id_zifuchuan(&shuju) {
            return Some((id, shuju));
        }
    }
    if let Some(shuju) = shujucaozuo_biaoqianleixing::chaxun_mingcheng(leixing_shuru).await {
        if let Some(id) = qu_id_zifuchuan(&shuju) {
            return Some((id, shuju));
        }
    }
    if let Some(tuice_mingcheng) = tuice_fenleimingcheng(leixing_shuru) {
        if let Some(shuju) = shujucaozuo_biaoqianleixing::chaxun_mingcheng(tuice_mingcheng).await {
            if let Some(id) = qu_id_zifuchuan(&shuju) {
                return Some((id, shuju));
            }
        }
    }
    let mubiao = guifan_fenleimingcheng(leixing_shuru);
    let fenleilie = shujucaozuo_biaoqianleixing::chaxun_quanbu().await?;
    for fenlei in fenleilie {
        let mingcheng = fenlei.get("mingcheng").and_then(|x| x.as_str()).unwrap_or_default();
        if guifan_fenleimingcheng(mingcheng) == mubiao {
            if let Some(id) = qu_id_zifuchuan(&fenlei) {
                return Some((id, fenlei));
            }
        }
    }
    None
}

fn tiqu_feikong_zifuchuan(v: &Value, jian: &str) -> Option<String> {
    v.get(jian)
        .and_then(|x| x.as_str())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

fn qu_zhengshu(v: &Value, jian: &str, moren: i64) -> i64 {
    v.get(jian).and_then(|x| x.as_i64()).unwrap_or(moren)
}

fn fenye_qu_canshu(qingqiu: &Value) -> (i64, i64) {
    let yeshu = qu_zhengshu(qingqiu, "yeshu", 1).max(1);
    let meiyetiaoshu = qu_zhengshu(qingqiu, "meiyetiaoshu", 20).clamp(1, 200);
    (yeshu, meiyetiaoshu)
}

fn shaixuan_fenye_biaoqian_liebiao(
    mut liebiao: Vec<Value>,
    guanjianci: Option<&str>,
    yeshu: i64,
    meiyetiaoshu: i64,
) -> (Vec<Value>, usize, i64, i64) {
    if let Some(ci) = guanjianci.map(str::trim).filter(|s| !s.is_empty()) {
        liebiao = liebiao
            .into_iter()
            .filter(|x| {
                x.get("zhi")
                    .and_then(|z| z.as_str())
                    .is_some_and(|z| z.contains(ci))
            })
            .collect();
    }
    let zongshu = liebiao.len();
    let yeshu = yeshu.max(1);
    let meiyetiaoshu = meiyetiaoshu.clamp(1, 200);
    let kaishi = ((yeshu - 1) * meiyetiaoshu) as usize;
    if kaishi >= zongshu {
        return (Vec::new(), zongshu, yeshu, meiyetiaoshu);
    }
    let jieshu = (kaishi + meiyetiaoshu as usize).min(zongshu);
    (liebiao[kaishi..jieshu].to_vec(), zongshu, yeshu, meiyetiaoshu)
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
        "leixing_chaxun_quanbu" => {
            match shujucaozuo_biaoqianleixing::chaxun_quanbu().await {
                Some(liebiao) => chenggong(json!({"liebiao": liebiao})),
                None => shibai("查询失败"),
            }
        }
        "leixing_xinzeng" => {
            let mingcheng = match qu_zifuchuan(&qingqiu, "mingcheng") {
                Some(v) => v,
                None => return shibai("缺少参数: mingcheng"),
            };
            if shujucaozuo_biaoqianleixing::mingchengcunzai(&mingcheng).await {
                return shibai("类型名称已存在");
            }
            match shujucaozuo_biaoqianleixing::xinzeng(&mingcheng).await {
                Some(id) => chenggong(json!({"id": id})),
                None => shibai("新增失败"),
            }
        }
        "leixing_gengxin" => {
            let id = match qu_zifuchuan(&qingqiu, "id") {
                Some(v) => v,
                None => return shibai("缺少参数: id"),
            };
            let mingcheng = match qu_zifuchuan(&qingqiu, "mingcheng") {
                Some(v) => v,
                None => return shibai("缺少参数: mingcheng"),
            };
            match shujucaozuo_biaoqianleixing::gengxin(&id, &mingcheng).await {
                Some(n) if n > 0 => chenggong(json!({"yingxiang": n})),
                Some(_) => shibai("标签类型不存在"),
                None => shibai("更新失败"),
            }
        }
        "leixing_shanchu" => {
            let id = match qu_zifuchuan(&qingqiu, "id") {
                Some(v) => v,
                None => return shibai("缺少参数: id"),
            };
            match shujucaozuo_biaoqianleixing::shanchu(&id).await {
                Some(n) if n > 0 => chenggong(json!({"yingxiang": n})),
                Some(_) => shibai("标签类型不存在"),
                None => shibai("删除失败"),
            }
        }
        "biaoqian_chaxun_quanbu" => {
            match shujucaozuo_biaoqian::chaxun_quanbu().await {
                Some(liebiao) => chenggong(json!({"liebiao": liebiao})),
                None => shibai("查询失败"),
            }
        }
        "biaoqian_chaxun_id" => {
            let id = match qu_zifuchuan(&qingqiu, "id") {
                Some(v) => v,
                None => return shibai("缺少参数: id"),
            };
            match shujucaozuo_biaoqian::chaxun_id(&id).await {
                Some(shuju) => chenggong(shuju),
                None => shibai("标签不存在"),
            }
        }
        "biaoqian_chaxun_leixingid" => {
            let leixing_shuru = match qu_zifuchuan(&qingqiu, "leixingid") {
                Some(v) => v,
                None => return shibai("缺少参数: leixingid"),
            };
            let guanjianci = tiqu_feikong_zifuchuan(&qingqiu, "guanjianci");
            let (yeshu, meiyetiaoshu) = fenye_qu_canshu(&qingqiu);
            let (leixingid, fenlei) = match jiexi_leixingid_huo_fenlei(&leixing_shuru).await {
                Some(v) => v,
                None => return shibai("未找到对应分类，请先调用 leixing_chaxun_quanbu 确认分类后再查标签"),
            };
            match shujucaozuo_biaoqian::chaxun_leixingid(&leixingid).await {
                Some(liebiao_yuanshi) => {
                    let (liebiao, zongshu, yeshu, meiyetiaoshu) = shaixuan_fenye_biaoqian_liebiao(
                        liebiao_yuanshi,
                        guanjianci.as_deref(),
                        yeshu,
                        meiyetiaoshu,
                    );
                    chenggong(json!({
                    "liucheng": "xianfenleihoufenyeliebiaochaxun",
                    "fenlei": fenlei,
                    "liebiao": liebiao,
                    "zongshu": zongshu,
                    "yeshu": yeshu,
                    "meiyetiaoshu": meiyetiaoshu,
                    "guanjianci": guanjianci
                }))
                }
                None => shibai("查询失败"),
            }
        }
        "biaoqian_chaxun_leixingid_zhi" => {
            let leixing_shuru = match qu_zifuchuan(&qingqiu, "leixingid") {
                Some(v) => v,
                None => return shibai("缺少参数: leixingid"),
            };
            let guanjianci = tiqu_feikong_zifuchuan(&qingqiu, "guanjianci");
            let (yeshu, meiyetiaoshu) = fenye_qu_canshu(&qingqiu);
            let (leixingid, fenlei) = match jiexi_leixingid_huo_fenlei(&leixing_shuru).await {
                Some(v) => v,
                None => return shibai("未找到对应分类，请先调用 leixing_chaxun_quanbu 确认分类后再查标签"),
            };
            let zhi = tiqu_feikong_zifuchuan(&qingqiu, "zhi");
            if zhi.is_none() {
                return match shujucaozuo_biaoqian::chaxun_leixingid(&leixingid).await {
                    Some(liebiao_yuanshi) => {
                        let (liebiao, zongshu, yeshu, meiyetiaoshu) = shaixuan_fenye_biaoqian_liebiao(
                            liebiao_yuanshi,
                            guanjianci.as_deref(),
                            yeshu,
                            meiyetiaoshu,
                        );
                        chenggong(json!({
                        "liucheng": "xianfenyehuoguanjiancisousuoliebiao",
                        "fenlei": fenlei,
                        "liebiao": liebiao,
                        "zongshu": zongshu,
                        "yeshu": yeshu,
                        "meiyetiaoshu": meiyetiaoshu,
                        "guanjianci": guanjianci,
                        "tishi": "当前先返回分类列表（分页/关键词结果）；如需精确匹配请补充 zhi"
                    }))
                    }
                    None => shibai("查询失败"),
                };
            }
            let zhi = zhi.unwrap_or_default();
            match shujucaozuo_biaoqian::chaxun_leixingid_zhi(&leixingid, &zhi).await {
                Some(shuju) => chenggong(json!({
                    "liucheng": "xianfenleihoujingquechaxun",
                    "fenlei": fenlei,
                    "jieguo": shuju
                })),
                None => {
                    let liebiao_yuanshi = shujucaozuo_biaoqian::chaxun_leixingid(&leixingid).await.unwrap_or_default();
                    let yongyu_liebiao_de_ci = guanjianci.clone().unwrap_or_else(|| zhi.clone());
                    let (liebiao, zongshu, yeshu, meiyetiaoshu) = shaixuan_fenye_biaoqian_liebiao(
                        liebiao_yuanshi,
                        Some(&yongyu_liebiao_de_ci),
                        yeshu,
                        meiyetiaoshu,
                    );
                    if !liebiao.is_empty() {
                        chenggong(json!({
                            "liucheng": "xianfenyehuoguanjiancisousuoliebiaozaijingque",
                            "fenlei": fenlei,
                            "jingque_mingzhong": false,
                            "zhi": zhi,
                            "liebiao": liebiao,
                            "zongshu": zongshu,
                            "yeshu": yeshu,
                            "meiyetiaoshu": meiyetiaoshu,
                            "guanjianci": yongyu_liebiao_de_ci,
                            "tishi": "未命中精确标签，已先返回关键词列表，请先从列表确认后再精确查询"
                        }))
                    } else {
                        chenggong(json!({
                            "liucheng": "xianfenyehuoguanjiancisousuoliebiaozaijingque",
                            "fenlei": fenlei,
                            "jingque_mingzhong": false,
                            "zhi": zhi,
                            "liebiao": [],
                            "zongshu": 0,
                            "yeshu": yeshu,
                            "meiyetiaoshu": meiyetiaoshu,
                            "guanjianci": yongyu_liebiao_de_ci,
                            "tishi": "未命中标签，请先分页查看列表或换关键词搜索列表"
                        }))
                    }
                }
            }
        }
        "biaoqian_xinzeng" => {
            let leixingid = match qu_zifuchuan(&qingqiu, "leixingid") {
                Some(v) => v,
                None => return shibai("缺少参数: leixingid"),
            };
            let zhi = match qu_zifuchuan(&qingqiu, "zhi") {
                Some(v) => v,
                None => return shibai("缺少参数: zhi"),
            };
            match shujucaozuo_biaoqian::xinzeng(&leixingid, &zhi).await {
                Some(id) => chenggong(json!({"id": id})),
                None => shibai("新增失败"),
            }
        }
        "biaoqian_gengxin" => {
            let id = match qu_zifuchuan(&qingqiu, "id") {
                Some(v) => v,
                None => return shibai("缺少参数: id"),
            };
            let zhi = match qu_zifuchuan(&qingqiu, "zhi") {
                Some(v) => v,
                None => return shibai("缺少参数: zhi"),
            };
            match shujucaozuo_biaoqian::gengxin(&id, &zhi).await {
                Some(n) if n > 0 => chenggong(json!({"yingxiang": n})),
                Some(_) => shibai("标签不存在"),
                None => shibai("更新失败"),
            }
        }
        "biaoqian_shanchu" => {
            let id = match qu_zifuchuan(&qingqiu, "id") {
                Some(v) => v,
                None => return shibai("缺少参数: id"),
            };
            match shujucaozuo_biaoqian::shanchu(&id).await {
                Some(n) if n > 0 => chenggong(json!({"yingxiang": n})),
                Some(_) => shibai("标签不存在"),
                None => shibai("删除失败"),
            }
        }
        "guanlian_xinzeng" => {
            let ribaoid = match qu_zifuchuan(&qingqiu, "ribaoid") {
                Some(v) => v,
                None => return shibai("缺少参数: ribaoid"),
            };
            let biaoqianid = match qu_zifuchuan(&qingqiu, "biaoqianid") {
                Some(v) => v,
                None => return shibai("缺少参数: biaoqianid"),
            };
            match shujucaozuo_ribao_biaoqian::xinzeng(&ribaoid, &biaoqianid).await {
                Some(n) if n > 0 => chenggong(json!({"yingxiang": n})),
                _ => shibai("关联失败"),
            }
        }
        "guanlian_shanchu" => {
            let ribaoid = match qu_zifuchuan(&qingqiu, "ribaoid") {
                Some(v) => v,
                None => return shibai("缺少参数: ribaoid"),
            };
            let biaoqianid = match qu_zifuchuan(&qingqiu, "biaoqianid") {
                Some(v) => v,
                None => return shibai("缺少参数: biaoqianid"),
            };
            match shujucaozuo_ribao_biaoqian::shanchu_guanlian(&ribaoid, &biaoqianid).await {
                Some(n) if n > 0 => chenggong(json!({"yingxiang": n})),
                _ => shibai("关联不存在或删除失败"),
            }
        }
        "guanlian_shanchu_ribaoid" => {
            let ribaoid = match qu_zifuchuan(&qingqiu, "ribaoid") {
                Some(v) => v,
                None => return shibai("缺少参数: ribaoid"),
            };
            match shujucaozuo_ribao_biaoqian::shanchu_ribaoid(&ribaoid).await {
                Some(n) => chenggong(json!({"yingxiang": n})),
                None => shibai("删除失败"),
            }
        }
        "guanlian_chaxun_ribaoid_daixinxi" => {
            let ribaoid = match qu_zifuchuan(&qingqiu, "ribaoid") {
                Some(v) => v,
                None => return shibai("缺少参数: ribaoid"),
            };
            match shujucaozuo_ribao_biaoqian::chaxun_ribaoid_daixinxi(&ribaoid).await {
                Some(liebiao) => chenggong(json!({"liebiao": liebiao})),
                None => shibai("查询失败"),
            }
        }
        "guanlian_piliang_xinzeng" => {
            let ribaoid = match qu_zifuchuan(&qingqiu, "ribaoid") {
                Some(v) => v,
                None => return shibai("缺少参数: ribaoid"),
            };
            let biaoqianidlie = qu_zifuchuanlie(&qingqiu, "biaoqianidlie");
            if biaoqianidlie.is_empty() {
                return shibai("缺少参数: biaoqianidlie");
            }
            let idlie: Vec<&str> = biaoqianidlie.iter().map(String::as_str).collect();
            match shujucaozuo_ribao_biaoqian::piliang_xinzeng(&ribaoid, &idlie).await {
                Some(n) => chenggong(json!({"yingxiang": n})),
                None => shibai("批量关联失败"),
            }
        }
        "guanlian_piliang_shanchu_ribaoidlie" => {
            let ribaoidlie = qu_zifuchuanlie(&qingqiu, "ribaoidlie");
            if ribaoidlie.is_empty() {
                return shibai("缺少参数: ribaoidlie");
            }
            let idlie: Vec<&str> = ribaoidlie.iter().map(String::as_str).collect();
            match shujucaozuo_ribao_biaoqian::piliang_shanchu_ribaoidlie(&idlie).await {
                Some(n) => chenggong(json!({"yingxiang": n})),
                None => shibai("批量删除失败"),
            }
        }
        _ => shibai("未知操作类型"),
    }
}
