use crate::gongju::jichugongju;
use llm::chat::Tool;
use serde_json::json;

/// 工具定义
pub fn dinyi() -> Tool {
    Tool {
        tool_type: "function".to_string(),
        function: llm::chat::FunctionTool {
            name: "shijian_chaxun".to_string(),
            description: "查询当前服务器时间".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {},
                "required": []
            }),
        },
    }
}

/// 工具执行
pub async fn zhixing(_canshu: &str, _lingpai: &str) -> String {
    let shijianchuo = jichugongju::huoqushijianchuo();
    format!("当前服务器时间戳: {}", shijianchuo)
}
