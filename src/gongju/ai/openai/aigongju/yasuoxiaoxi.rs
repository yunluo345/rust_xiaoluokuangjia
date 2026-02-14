use llm::chat::{FunctionTool, Tool};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[allow(non_upper_case_globals)]
const gongju_mingcheng: &str = "yasuoxiaoxi";

#[derive(Debug, Serialize, Deserialize)]
pub struct Yasuojieguo {
    pub chenggong: bool,
    pub xiaoxi: String,
    pub zongjie: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Yasuocanshu {
    zongjie: String,
}

pub fn goujian_gongju() -> Tool {
    Tool {
        tool_type: "function".to_string(),
        function: FunctionTool {
            name: gongju_mingcheng.to_string(),
            description: "当对话历史过长超过token限制时，总结并压缩历史消息。你需要提供一个简洁的总结，包含关键信息、重要结论和上下文。".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "zongjie": {
                        "type": "string",
                        "description": "对历史对话的简洁总结，必须包含：1.用户的主要问题和需求 2.已完成的操作和结果 3.当前状态和待解决问题 4.重要的上下文信息"
                    }
                },
                "required": ["zongjie"],
                "additionalProperties": false
            }),
        },
    }
}

pub fn zhixing(canshu_json: &str) -> String {
    let canshu: Yasuocanshu = match serde_json::from_str(canshu_json) {
        Ok(c) => c,
        Err(_) => return xilie_cuowujieguo("参数解析失败"),
    };

    if canshu.zongjie.trim().is_empty() {
        return xilie_cuowujieguo("总结内容不能为空");
    }

    let jieguo = Yasuojieguo {
        chenggong: true,
        xiaoxi: "历史消息已压缩".to_string(),
        zongjie: Some(canshu.zongjie),
    };
    
    serde_json::to_string(&jieguo).unwrap_or_default()
}

pub fn mingcheng() -> &'static str {
    gongju_mingcheng
}

fn xilie_cuowujieguo(xiaoxi: &str) -> String {
    let jieguo = Yasuojieguo {
        chenggong: false,
        xiaoxi: xiaoxi.to_string(),
        zongjie: None,
    };
    serde_json::to_string(&jieguo).unwrap_or_default()
}
