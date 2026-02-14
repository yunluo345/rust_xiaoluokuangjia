use llm::chat::{FunctionTool, Tool};
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::peizhixt::peizhixitongzhuti;
use crate::peizhixt::peizhi_nr::peizhi_ai::Aipeizhi as Aipeizhiwenjian;

#[allow(non_upper_case_globals)]
const gongju_mingcheng: &str = "ribaozhiyin";

#[allow(non_upper_case_globals)]
const buzhou_yi: &str = "第一步：调用 xieribao 工具（moshi=shencha），将用户提供的日报内容通过 ribaoneirong 参数传入进行基本格式审查（如内容长度等）。如果格式审查不通过，向用户反馈问题。格式审查通过后进入下一步。";

fn shengcheng_buzhou_san() -> String {
    let leibie_miaoshu = peizhixitongzhuti::duqupeizhi::<Aipeizhiwenjian>(Aipeizhiwenjian::wenjianming())
        .map(|p| p.ribaoshengcheng.xinxi_yingshe.keys().cloned().collect::<Vec<_>>().join("、"))
        .unwrap_or_else(|| "自己名称、客户公司、对方派遣人员、地名、时间、工作内容、对话内容、后续计划、备注".to_string());
    format!(
        "第三步：调用 tiqubiaoqian 工具，必须通过 yuanwen 参数传入日报原文，并通过 biaoqianlie 参数传入从原文中提取的标签。\
        如果用户在对话中补充了原文中缺失的信息（如{}等），必须通过 buchongxinxi 参数传入用户补充的原始内容。\
        注意：用户的表述可能与类别名称不完全一致（如\"沟通内容\"对应\"对话内容\"，\"明日计划\"对应\"后续计划\"，\"备注\"对应文末的补充说明），\
        标签的 leibie 必须使用标准类别名称。此阶段会严格校验所有必需类别是否都有对应标签，缺少则报错。",
        leibie_miaoshu
    )
}

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
            description: "日报处理流程指南。流程共四步：1（审查日报）→2（获取后续指示）→3（标签提取）→4（完成）。\
            你必须根据当前对话上下文自行判断处于哪个步骤，传入对应的 buzhou 值。\
            判断依据：尚未审查→1；审查通过但未获取提取指示→2；已获取指示但未提取标签→3；标签提取完成→4。\
            如果用户补充了缺失信息（如之前标签提取失败后用户补充了内容），应回到步骤 3 重新提取。".to_string(),
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
    let buzhou_san = shengcheng_buzhou_san();
    let jieguo = match buzhou {
        1 => Zhiyinjieguo {
            chenggong: true,
            xiaoxi: buzhou_yi.to_string(),
            dangqianbuzhou: 1,
            xiayibu: Some("完成日报审查后，调用 ribaozhiyin 工具并传入 buzhou=2 获取下一步指示".to_string()),
            liuchengwancheng: false,
        },
        2 => Zhiyinjieguo {
            chenggong: true,
            xiaoxi: buzhou_san.clone(),
            dangqianbuzhou: 2,
            xiayibu: Some("现在请调用 tiqubiaoqian 工具，通过 yuanwen 参数传入日报原文，通过 biaoqianlie 参数传入从原文中提取的标签。如果用户在对话中补充了信息，通过 buchongxinxi 参数传入。完成后调用 ribaozhiyin 工具并传入 buzhou=4".to_string()),
            liuchengwancheng: false,
        },
        3 => Zhiyinjieguo {
            chenggong: true,
            xiaoxi: buzhou_san,
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
