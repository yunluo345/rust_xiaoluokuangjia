use crate::gongju::jwtgongju;
use crate::shujuku::psqlshujuku::shujubiao_nr::ai::shujucaozuo_aiqudao;
use llm::chat::Tool;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/// 工具定义
pub fn dinyi() -> Tool {
    Tool {
        tool_type: "function".to_string(),
        function: llm::chat::FunctionTool {
            name: "aiqudao_guanli".to_string(),
            description: "管理AI渠道的增删改查操作。操作流程：1)查询操作可直接调用 2)删除/更新操作需先用chaxun_quanbu获取渠道列表找到目标ID，再用该ID执行操作。支持的操作：查询全部(chaxun_quanbu)、查询启用(chaxun_qiyong)、按ID查询(chaxun_id)、新增(xinzeng)、更新(gengxin)、删除(shanchu)、切换状态(qiehuanzhuangtai)、更新优先级(gengxinyouxianji)".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "caozuo": {
                        "type": "string",
                        "description": "操作类型：chaxun_quanbu=查询全部渠道，chaxun_qiyong=查询启用的渠道，chaxun_id=按ID查询，xinzeng=新增渠道，gengxin=更新渠道，shanchu=删除渠道(需要id参数)，qiehuanzhuangtai=切换启用/禁用状态(需要id参数)，gengxinyouxianji=更新优先级(需要id和youxianji参数)",
                        "enum": ["chaxun_quanbu", "chaxun_qiyong", "chaxun_id", "xinzeng", "gengxin", "shanchu", "qiehuanzhuangtai", "gengxinyouxianji"]
                    },
                    "id": {
                        "type": "string",
                        "description": "渠道ID（数字字符串），用于chaxun_id、gengxin、shanchu、qiehuanzhuangtai、gengxinyouxianji操作。删除或更新前需先通过chaxun_quanbu查询获取目标渠道的ID"
                    },
                    "mingcheng": {
                        "type": "string",
                        "description": "渠道名称，xinzeng操作时必填"
                    },
                    "leixing": {
                        "type": "string",
                        "description": "渠道类型，xinzeng操作时必填，只能是 openapi、xiangliang 或 yuyin",
                        "enum": ["openapi", "xiangliang", "yuyin"]
                    },
                    "jiekoudizhi": {
                        "type": "string",
                        "description": "接口地址，xinzeng操作时必填"
                    },
                    "miyao": {
                        "type": "string",
                        "description": "API密钥，xinzeng操作时必填"
                    },
                    "moxing": {
                        "type": "string",
                        "description": "默认模型名，xinzeng操作时必填"
                    },
                    "wendu": {
                        "type": "string",
                        "description": "温度参数(0-2)，xinzeng操作时可选，默认0"
                    },
                    "beizhu": {
                        "type": "string",
                        "description": "备注信息，xinzeng操作时可选"
                    },
                    "ziduanlie": {
                        "type": "array",
                        "description": "更新字段列表，gengxin操作时必填，格式为 [[字段名, 值], ...]，例如：[[\"mingcheng\", \"新名称\"], [\"moxing\", \"gpt-4\"]]",
                        "items": {
                            "type": "array",
                            "items": {"type": "string"},
                            "minItems": 2,
                            "maxItems": 2
                        }
                    },
                    "youxianji": {
                        "type": "string",
                        "description": "优先级数值(整数)，gengxinyouxianji操作时必填，数值越小优先级越高"
                    }
                },
                "required": ["caozuo"]
            }),
        },
    }
}

#[derive(Deserialize)]
struct Qingqiucanshu {
    caozuo: String,
    id: Option<String>,
    mingcheng: Option<String>,
    leixing: Option<String>,
    jiekoudizhi: Option<String>,
    miyao: Option<String>,
    moxing: Option<String>,
    wendu: Option<String>,
    beizhu: Option<String>,
    ziduanlie: Option<Vec<Vec<String>>>,
    youxianji: Option<String>,
}

/// 验证类型是否合法（只允许 openapi、xiangliang、yuyin）
fn yanzheng_leixing(leixing: &str) -> bool {
    matches!(leixing, "openapi" | "xiangliang" | "yuyin")
}

/// 提取必需参数，缺失时返回错误
fn tiqucansu(zhi: Option<String>, mingcheng: &str) -> Result<String, String> {
    zhi.ok_or_else(|| json!({"cuowu": format!("缺少参数: {}", mingcheng)}).to_string())
}

/// 构建查询成功响应
fn chaxun_chenggong<T: Serialize>(shuju: T) -> String {
    json!({"chenggong": true, "shuju": shuju}).to_string()
}

/// 构建查询失败响应
fn chaxun_shibai(xinxi: &str) -> String {
    json!({"cuowu": xinxi}).to_string()
}

/// 处理影响行数的数据库操作结果
fn chuli_yingxiang(jieguo: Option<u64>, shibai_xinxi: &str) -> String {
    match jieguo {
        Some(n) if n > 0 => chaxun_chenggong(json!({"yingxiang": n})),
        Some(_) => chaxun_shibai("渠道不存在"),
        None => chaxun_shibai(shibai_xinxi),
    }
}

/// 工具执行
pub async fn zhixing(canshu: &str, lingpai: &str) -> String {
    // 验证令牌
    let _zaiti = match jwtgongju::yanzheng(lingpai).await {
        Some(z) => z,
        None => return json!({"cuowu": "令牌无效或已过期"}).to_string(),
    };

    // 解析参数
    let qingqiu: Qingqiucanshu = match serde_json::from_str(canshu) {
        Ok(q) => q,
        Err(_) => return json!({"cuowu": "参数格式错误"}).to_string(),
    };

    // 根据操作类型分发处理
    match qingqiu.caozuo.as_str() {
        "chaxun_quanbu" => {
            match shujucaozuo_aiqudao::chaxun_quanbu().await {
                Some(jieguo) => chaxun_chenggong(jieguo),
                None => chaxun_shibai("查询失败"),
            }
        }
        "chaxun_qiyong" => {
            match shujucaozuo_aiqudao::chaxun_qiyong().await {
                Some(jieguo) => chaxun_chenggong(jieguo),
                None => chaxun_shibai("查询失败"),
            }
        }
        "chaxun_id" => {
            let id = match tiqucansu(qingqiu.id, "id") {
                Ok(i) => i,
                Err(e) => return e,
            };
            match shujucaozuo_aiqudao::chaxun_id(&id).await {
                Some(jieguo) => chaxun_chenggong(jieguo),
                None => chaxun_shibai("渠道不存在"),
            }
        }
        "xinzeng" => {
            let mingcheng = match tiqucansu(qingqiu.mingcheng, "mingcheng") {
                Ok(m) => m,
                Err(e) => return e,
            };
            let leixing = match tiqucansu(qingqiu.leixing, "leixing") {
                Ok(l) => l,
                Err(e) => return e,
            };
            let jiekoudizhi = match tiqucansu(qingqiu.jiekoudizhi, "jiekoudizhi") {
                Ok(j) => j,
                Err(e) => return e,
            };
            let miyao = match tiqucansu(qingqiu.miyao, "miyao") {
                Ok(m) => m,
                Err(e) => return e,
            };
            let moxing = match tiqucansu(qingqiu.moxing, "moxing") {
                Ok(m) => m,
                Err(e) => return e,
            };
            let wendu = match tiqucansu(qingqiu.wendu, "wendu") {
                Ok(w) => w,
                Err(e) => return e,
            };

            if !yanzheng_leixing(&leixing) {
                return chaxun_shibai("类型只能是 openapi、xiangliang 或 yuyin");
            }

            if shujucaozuo_aiqudao::mingchengcunzai(&mingcheng).await {
                return chaxun_shibai("渠道名称已存在");
            }

            match shujucaozuo_aiqudao::xinzeng(
                &mingcheng,
                &leixing,
                &jiekoudizhi,
                &miyao,
                &moxing,
                &wendu,
                qingqiu.beizhu.as_deref(),
            ).await {
                Some(id) => chaxun_chenggong(json!({"id": id})),
                None => chaxun_shibai("新增失败"),
            }
        }
        "gengxin" => {
            let id = match tiqucansu(qingqiu.id, "id") {
                Ok(i) => i,
                Err(e) => return e,
            };
            let ziduanlie = match qingqiu.ziduanlie {
                Some(z) => z,
                None => return chaxun_shibai("缺少参数: ziduanlie"),
            };

            if ziduanlie.is_empty() {
                return chaxun_shibai("更新字段不能为空");
            }

            for ziduan in &ziduanlie {
                if ziduan.len() != 2 {
                    return chaxun_shibai("字段格式错误，应为 [字段名, 值]");
                }
                if ziduan[0] == "leixing" && !yanzheng_leixing(&ziduan[1]) {
                    return chaxun_shibai("类型只能是 openapi、xiangliang 或 yuyin");
                }
            }

            let ziduanlie_ref: Vec<(&str, &str)> = ziduanlie.iter()
                .map(|z| (z[0].as_str(), z[1].as_str()))
                .collect();

            chuli_yingxiang(
                shujucaozuo_aiqudao::gengxin(&id, &ziduanlie_ref).await,
                "更新失败"
            )
        }
        "shanchu" => {
            let id = match tiqucansu(qingqiu.id, "id") {
                Ok(i) => i,
                Err(e) => return e,
            };
            chuli_yingxiang(
                shujucaozuo_aiqudao::shanchu(&id).await,
                "删除失败"
            )
        }
        "qiehuanzhuangtai" => {
            let id = match tiqucansu(qingqiu.id, "id") {
                Ok(i) => i,
                Err(e) => return e,
            };
            chuli_yingxiang(
                shujucaozuo_aiqudao::qiehuanzhuangtai(&id).await,
                "操作失败"
            )
        }
        "gengxinyouxianji" => {
            let id = match tiqucansu(qingqiu.id, "id") {
                Ok(i) => i,
                Err(e) => return e,
            };
            let youxianji = match tiqucansu(qingqiu.youxianji, "youxianji") {
                Ok(y) => y,
                Err(e) => return e,
            };
            chuli_yingxiang(
                shujucaozuo_aiqudao::gengxinyouxianji(&id, &youxianji).await,
                "更新失败"
            )
        }
        _ => chaxun_shibai("不支持的操作类型"),
    }
}