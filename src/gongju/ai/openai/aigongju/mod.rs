pub mod tiqubiaoqian;
pub mod yasuoxiaoxi;

use llm::chat::Tool;

/// 获取所有预定义工具的 JSON 列表
pub fn huoqu_suoyougongju() -> Vec<serde_json::Value> {
    [tiqubiaoqian::goujian_gongju(), yasuoxiaoxi::goujian_gongju()]
        .into_iter()
        .filter_map(|g| serde_json::to_value(g).ok())
        .collect()
}

/// 获取所有预定义工具的 Tool 列表
pub fn huoqu_gongjulie() -> Vec<Tool> {
    vec![tiqubiaoqian::goujian_gongju(), yasuoxiaoxi::goujian_gongju()]
}

/// 执行工具调用，根据工具名分发到对应执行函数
pub fn zhixing_gongju(diaoyong: &llm::ToolCall) -> llm::ToolCall {
    let jieguo = match diaoyong.function.name.as_str() {
        n if n == tiqubiaoqian::mingcheng() => tiqubiaoqian::zhixing(&diaoyong.function.arguments),
        n if n == yasuoxiaoxi::mingcheng() => yasuoxiaoxi::zhixing(&diaoyong.function.arguments),
        _ => format!("{{\"cuowu\": \"未知工具: {}\"}}", diaoyong.function.name),
    };
    llm::ToolCall {
        id: diaoyong.id.clone(),
        call_type: "function".to_string(),
        function: llm::FunctionCall {
            name: diaoyong.function.name.clone(),
            arguments: jieguo,
        },
    }
}

/// 批量执行工具调用
pub async fn pizhixing(diaoyonglie: Vec<llm::ToolCall>) -> Vec<llm::ToolCall> {
    diaoyonglie.iter().map(zhixing_gongju).collect()
}
