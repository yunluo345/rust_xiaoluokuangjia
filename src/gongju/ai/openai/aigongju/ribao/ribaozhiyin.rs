use llm::chat::{FunctionTool, Tool};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[allow(non_upper_case_globals)]
const gongju_mingcheng: &str = "ribaozhiyin";

#[allow(non_upper_case_globals)]
const liucheng_buzhou: &[&str] = &[
    "第一步：调用 xieribao 工具（moshi=shencha），将用户提供的日报内容传入进行基本格式审查（如内容长度等）。如果格式审查不通过，向用户反馈问题。格式审查通过后进入下一步。",
    "第二步：再次调用 ribaozhiyin 工具（即本工具），传入 buzhou 为 2，获取后续指示。这一步确保你不会遗忘上下文和后续流程。",
    "第三步：调用 tiqubiaoqian 工具，对日报文本进行标签提取。你需要通过语义理解从日报中提取所有必需类别的标签。注意：用户的表述可能与类别名称不完全一致（如\"沟通内容\"对应\"对话内容\"，\"明日计划\"对应\"后续计划\"，\"备注\"对应文末的补充说明），标签的 leibie 必须使用标准类别名称。此阶段会严格校验所有必需类别是否都有对应标签，缺少则报错。",
    "第四步：再次调用 ribaozhiyin 工具，传入 buzhou 为 4，表示流程已全部完成。向用户确认日报处理流程已结束。",
];

#[derive(Debug, Serialize, Deserialize)]
pub struct Zhiyinjieguo {
    pub chenggong: bool,
    pub xiaoxi: String,
    pub dangqianbuzhou: u8,
    pub xiayibu: Option<String>,
    pub liuchengwancheng: bool,
}

/// 构建日报指南工具定义
pub fn goujian_gongju() -> Tool {
    Tool {
        tool_type: "function".to_string(),
        function: FunctionTool {
            name: gongju_mingcheng.to_string(),
            description: "日报处理流程指南。当用户发来日报相关内容时，必须首先调用此工具获取处理流程。此工具会指导你按正确的步骤顺序处理日报：审查→索引→标签提取→完成。每完成一步后需再次调用本工具获取下一步指示。".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "buzhou": {
                        "type": "integer",
                        "description": "当前执行到的步骤编号。首次调用传 1，后续按指示传入对应步骤号。可选值：1（开始/审查日报）、2（获取后续指示）、3（标签提取）、4（完成流程）"
                    }
                },
                "required": ["buzhou"]
            }),
        },
    }
}

/// 执行日报指南工具
pub fn zhixing(canshu_json: &str) -> String {
    let buzhou = jiexi_buzhou(canshu_json).unwrap_or(1);
    let jieguo = match buzhou {
        1 => Zhiyinjieguo {
            chenggong: true,
            xiaoxi: liucheng_buzhou[0].to_string(),
            dangqianbuzhou: 1,
            xiayibu: Some("完成日报审查后，调用 ribaozhiyin 工具并传入 buzhou=2 获取下一步指示".to_string()),
            liuchengwancheng: false,
        },
        2 => Zhiyinjieguo {
            chenggong: true,
            xiaoxi: liucheng_buzhou[2].to_string(),
            dangqianbuzhou: 2,
            xiayibu: Some("现在请调用 tiqubiaoqian 工具对日报内容进行标签提取，完成后调用 ribaozhiyin 工具并传入 buzhou=4".to_string()),
            liuchengwancheng: false,
        },
        3 => Zhiyinjieguo {
            chenggong: true,
            xiaoxi: liucheng_buzhou[2].to_string(),
            dangqianbuzhou: 3,
            xiayibu: Some("标签提取完成后，调用 ribaozhiyin 工具并传入 buzhou=4 完成流程".to_string()),
            liuchengwancheng: false,
        },
        4 => Zhiyinjieguo {
            chenggong: true,
            xiaoxi: "日报处理流程已全部完成。日报已通过审查、标签已提取。请向用户确认处理结果。".to_string(),
            dangqianbuzhou: 4,
            xiayibu: None,
            liuchengwancheng: true,
        },
        _ => Zhiyinjieguo {
            chenggong: false,
            xiaoxi: format!("无效的步骤编号：{}，有效范围为 1-4", buzhou),
            dangqianbuzhou: buzhou,
            xiayibu: Some("请传入有效的步骤编号（1-4）".to_string()),
            liuchengwancheng: false,
        },
    };
    serde_json::to_string(&jieguo).unwrap_or_default()
}

/// 获取工具名称
pub fn mingcheng() -> &'static str {
    gongju_mingcheng
}

/// 解析步骤参数
fn jiexi_buzhou(canshu_json: &str) -> Option<u8> {
    let canshu: serde_json::Value = serde_json::from_str(canshu_json).ok()?;
    canshu.get("buzhou")?.as_u64().map(|v| v as u8)
}
