use crate::gongju::jwtgongju;
use crate::peizhixt::peizhi_nr::peizhi_ai::Ai;
use crate::peizhixt::peizhixitongzhuti;
use crate::shujuku::psqlshujuku::shujubiao_nr::ribao::{
    shujucaozuo_ribao,
    shujucaozuo_ribao_biaoqian,
    shujucaozuo_ribao_biaoqianrenwu,
};
use llm::chat::Tool;
use serde::Deserialize;
use serde_json::{json, Value};
use super::gongju_ribaorenwuchuli;

#[derive(Debug, Clone)]
pub enum Gongjufenzu {
    Guanli,
    Xitong,
}

#[derive(Deserialize)]
struct Qingqiucanshu {
    neirong: String,
    fabushijian: String,
    biaoqianidlie: Option<Vec<String>>,
}

/// 获取工具关键词
pub fn huoqu_guanjianci() -> Vec<String> {
    vec![
        "日报".to_string(),
        "提交".to_string(),
        "提交日报".to_string(),
        "发布日报".to_string(),
        "新增日报".to_string(),
        "写日报".to_string(),
        "report".to_string(),
        "submit".to_string(),
        "daily report".to_string(),
    ]
}

/// 获取工具分组
pub fn huoqu_fenzu() -> Gongjufenzu {
    Gongjufenzu::Xitong
}

/// 工具定义
pub fn dinyi() -> Tool {
    Tool {
        tool_type: "function".to_string(),
        function: llm::chat::FunctionTool {
            name: "ribao_tijiao".to_string(),
            description: "提交日报。自动使用当前登录用户身份提交，无需手动指定用户ID。可选择关联标签。".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "neirong": {
                        "type": "string",
                        "description": "日报完整原文（必须包含用户提交的原始内容和所有补充信息，保持原文格式，不要只提交结构化字段）"
                    },
                    "fabushijian": {
                        "type": "string",
                        "description": "发布时间（时间戳字符串）"
                    },
                    "biaoqianidlie": {
                        "type": "array",
                        "items": {"type": "string"},
                        "description": "可选：要关联的标签ID列表"
                    }
                },
                "required": ["neirong", "fabushijian"]
            }),
        },
    }
}

/// 工具执行
pub async fn zhixing(canshu: &str, lingpai: &str) -> String {
    let zaiti = match jwtgongju::yanzheng(lingpai).await {
        Some(z) => z,
        None => return json!({"cuowu": "令牌无效或已过期"}).to_string(),
    };

    let qingqiu: Qingqiucanshu = match serde_json::from_str(canshu) {
        Ok(q) => q,
        Err(_) => return json!({"cuowu": "参数格式错误"}).to_string(),
    };

    let ribaoid = match shujucaozuo_ribao::xinzeng(&zaiti.yonghuid, &qingqiu.neirong, &qingqiu.fabushijian).await {
        Some(id) => id,
        None => return json!({"cuowu": "日报提交失败"}).to_string(),
    };

    let zuidachangshicishu = peizhixitongzhuti::duqupeizhi::<Ai>(Ai::wenjianming())
        .map(|p| p.ribao_biaoqianrenwu_chongshi_cishu as i64)
        .unwrap_or(3);

    let renwuid = match shujucaozuo_ribao_biaoqianrenwu::faburenwu(&ribaoid, &zaiti.yonghuid, zuidachangshicishu).await {
        Some(id) => id,
        None => return json!({"cuowu": "日报提交成功但任务发布失败", "ribaoid": ribaoid}).to_string(),
    };

    let mut shuju = json!({"ribaoid": ribaoid, "renwuid": renwuid});

    if let Some(biaoqianidlie) = qingqiu.biaoqianidlie.as_ref().filter(|lie| !lie.is_empty()) {
        let biaoqianlie: Vec<&str> = biaoqianidlie.iter().map(|s| s.as_str()).collect();
        match shujucaozuo_ribao_biaoqian::piliang_xinzeng(&ribaoid, &biaoqianlie).await {
            Some(n) => {
                shuju.as_object_mut().map(|obj| obj.insert("guanlianshuliang".to_string(), json!(n)));
            }
            None => return json!({"cuowu": "日报提交成功但标签关联失败", "ribaoid": ribaoid, "renwuid": renwuid}).to_string(),
        }
    }

    match gongju_ribaorenwuchuli::zhixing_neibu(1).await {
        Ok(chulishuju) => {
            shuju.as_object_mut().map(|obj| obj.insert("biaoqianchuli".to_string(), chulishuju));
        }
        Err(cuowu) => {
            shuju.as_object_mut().map(|obj| obj.insert("biaoqianchuli_cuowu".to_string(), json!(cuowu)));
        }
    }

    json!({"chenggong": true, "shuju": shuju}).to_string()
}
