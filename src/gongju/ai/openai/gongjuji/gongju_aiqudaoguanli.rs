use crate::shujuku::psqlshujuku::shujubiao_nr::ai::shujucaozuo_aiqudao;
use crate::shujuku::psqlshujuku::shujubiao_nr::yonghu::yonghuyanzheng;
use llm::chat::Tool;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use super::Gongjufenzu;

/// 操作类型枚举
#[derive(Debug)]
enum Caozuoleixing {
    ChaxunQuanbu,
    ChaxunQiyong,
    ChaxunId(String),
    Xinzeng {
        mingcheng: String,
        leixing: String,
        jiekoudizhi: String,
        miyao: String,
        moxing: String,
        wendu: String,
        zuida_token: String,
        beizhu: Option<String>,
    },
    Gengxin {
        id: String,
        ziduanlie: Vec<Vec<String>>,
    },
    Shanchu(String),
    Qiehuanzhuangtai(String),
    Gengxinyouxianji {
        id: String,
        youxianji: String,
    },
}

/// 获取工具关键词
pub fn huoqu_guanjianci() -> Vec<String> {
    vec![
        "AI渠道".to_string(),
        "渠道配置".to_string(),
        "渠道管理".to_string(),
        "AI渠道管理".to_string(),
    ]
}

/// 获取工具分组
pub fn huoqu_fenzu() -> Gongjufenzu {
    Gongjufenzu::Guanli
}

/// 工具定义
pub fn dinyi() -> Tool {
    Tool {
        tool_type: "function".to_string(),
        function: llm::chat::FunctionTool {
            name: "aiqudao_guanli".to_string(),
            description: "管理AI渠道配置：查询全部渠道、查询启用渠道、按ID查询、新增渠道、更新渠道、删除渠道、切换启用状态、更新优先级".to_string(),
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
                        "description": "渠道ID（数字字符串），用于chaxun_id、gengxin、shanchu、qiehuanzhuangtai、gengxinyouxianji操作"
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
                    "zuida_token": {
                        "type": "string",
                        "description": "模型最大Token数，xinzeng操作时可选，默认0表示不限制"
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
    zuida_token: Option<String>,
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

/// 解析请求参数为操作类型
fn jiexi_caozuo(qingqiu: Qingqiucanshu) -> Result<Caozuoleixing, String> {
    match qingqiu.caozuo.as_str() {
        "chaxun_quanbu" => Ok(Caozuoleixing::ChaxunQuanbu),
        "chaxun_qiyong" => Ok(Caozuoleixing::ChaxunQiyong),
        "chaxun_id" => {
            let id = tiqucansu(qingqiu.id, "id")?;
            Ok(Caozuoleixing::ChaxunId(id))
        }
        "xinzeng" => {
            let mingcheng = tiqucansu(qingqiu.mingcheng, "mingcheng")?;
            let leixing = tiqucansu(qingqiu.leixing, "leixing")?;
            let jiekoudizhi = tiqucansu(qingqiu.jiekoudizhi, "jiekoudizhi")?;
            let miyao = tiqucansu(qingqiu.miyao, "miyao")?;
            let moxing = tiqucansu(qingqiu.moxing, "moxing")?;
            let wendu = qingqiu.wendu.unwrap_or_else(|| "0".to_string());
            let zuida_token = qingqiu.zuida_token.unwrap_or_else(|| "0".to_string());
            
            if !yanzheng_leixing(&leixing) {
                return Err(chaxun_shibai("类型只能是 openapi、xiangliang 或 yuyin"));
            }
            
            Ok(Caozuoleixing::Xinzeng {
                mingcheng,
                leixing,
                jiekoudizhi,
                miyao,
                moxing,
                wendu,
                zuida_token,
                beizhu: qingqiu.beizhu,
            })
        }
        "gengxin" => {
            let id = tiqucansu(qingqiu.id, "id")?;
            let ziduanlie = qingqiu.ziduanlie.ok_or_else(|| chaxun_shibai("缺少参数: ziduanlie"))?;
            
            if ziduanlie.is_empty() {
                return Err(chaxun_shibai("更新字段不能为空"));
            }
            
            for ziduan in &ziduanlie {
                if ziduan.len() != 2 {
                    return Err(chaxun_shibai("字段格式错误，应为 [字段名, 值]"));
                }
                if ziduan[0] == "leixing" && !yanzheng_leixing(&ziduan[1]) {
                    return Err(chaxun_shibai("类型只能是 openapi、xiangliang 或 yuyin"));
                }
            }
            
            Ok(Caozuoleixing::Gengxin { id, ziduanlie })
        }
        "shanchu" => {
            let id = tiqucansu(qingqiu.id, "id")?;
            Ok(Caozuoleixing::Shanchu(id))
        }
        "qiehuanzhuangtai" => {
            let id = tiqucansu(qingqiu.id, "id")?;
            Ok(Caozuoleixing::Qiehuanzhuangtai(id))
        }
        "gengxinyouxianji" => {
            let id = tiqucansu(qingqiu.id, "id")?;
            let youxianji = tiqucansu(qingqiu.youxianji, "youxianji")?;
            Ok(Caozuoleixing::Gengxinyouxianji { id, youxianji })
        }
        _ => Err(chaxun_shibai("未知操作类型")),
    }
}

/// 执行具体操作
async fn zhixing_caozuo(caozuo: Caozuoleixing) -> String {
    match caozuo {
        Caozuoleixing::ChaxunQuanbu => {
            match shujucaozuo_aiqudao::chaxun_quanbu().await {
                Some(jieguo) => chaxun_chenggong(jieguo),
                None => chaxun_shibai("查询失败"),
            }
        }
        Caozuoleixing::ChaxunQiyong => {
            match shujucaozuo_aiqudao::chaxun_qiyong().await {
                Some(jieguo) => chaxun_chenggong(jieguo),
                None => chaxun_shibai("查询失败"),
            }
        }
        Caozuoleixing::ChaxunId(id) => {
            match shujucaozuo_aiqudao::chaxun_id(&id).await {
                Some(jieguo) => chaxun_chenggong(jieguo),
                None => chaxun_shibai("渠道不存在"),
            }
        }
        Caozuoleixing::Xinzeng {
            mingcheng,
            leixing,
            jiekoudizhi,
            miyao,
            moxing,
            wendu,
            zuida_token,
            beizhu,
        } => {
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
                &zuida_token,
                beizhu.as_deref(),
            ).await {
                Some(id) => chaxun_chenggong(json!({"id": id})),
                None => chaxun_shibai("新增失败"),
            }
        }
        Caozuoleixing::Gengxin { id, ziduanlie } => {
            let ziduanlie_ref: Vec<(&str, &str)> = ziduanlie.iter()
                .map(|z| (z[0].as_str(), z[1].as_str()))
                .collect();
            chuli_yingxiang(
                shujucaozuo_aiqudao::gengxin(&id, &ziduanlie_ref).await,
                "更新失败"
            )
        }
        Caozuoleixing::Shanchu(id) => {
            chuli_yingxiang(
                shujucaozuo_aiqudao::shanchu(&id).await,
                "删除失败"
            )
        }
        Caozuoleixing::Qiehuanzhuangtai(id) => {
            chuli_yingxiang(
                shujucaozuo_aiqudao::qiehuanzhuangtai(&id).await,
                "操作失败"
            )
        }
        Caozuoleixing::Gengxinyouxianji { id, youxianji } => {
            chuli_yingxiang(
                shujucaozuo_aiqudao::gengxinyouxianji(&id, &youxianji).await,
                "更新优先级失败"
            )
        }
    }
}
/// 工具执行
pub async fn zhixing(canshu: &str, lingpai: &str) -> String {
    let _zaiti = match yonghuyanzheng::yanzhenglingpaijiquanxian(lingpai, "/jiekou/xitong/aiqudao").await {
        Ok(z) => z,
        Err(yonghuyanzheng::Lingpaicuowu::Yibeifengjin(y)) => return json!({"cuowu": format!("账号已被封禁：{}", y)}).to_string(),
        Err(yonghuyanzheng::Lingpaicuowu::Quanxianbuzu) => return json!({"cuowu": "权限不足"}).to_string(),
        Err(_) => return json!({"cuowu": "令牌无效或已过期"}).to_string(),
    };

    // 解析参数
    let qingqiu: Qingqiucanshu = match serde_json::from_str(canshu) {
        Ok(q) => q,
        Err(_) => return json!({"cuowu": "参数格式错误"}).to_string(),
    };

    // 解析操作类型
    let caozuo = match jiexi_caozuo(qingqiu) {
        Ok(c) => c,
        Err(e) => return e,
    };

    // 执行操作
    zhixing_caozuo(caozuo).await
}