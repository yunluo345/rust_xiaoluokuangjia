use crate::gongju::jichugongju;
use llm::chat::Tool;
use serde_json::json;

/// 工具分组枚举
#[derive(Debug, Clone)]
pub enum Gongjufenzu {
    Guanli,  // 管理组
    Xitong,  // 系统组
}

/// 获取工具关键词
pub fn huoqu_guanjianci() -> Vec<String> {
    vec![
        "现在几点".to_string(),
        "当前时间".to_string(),
        "服务器时间".to_string(),
        "查询时间".to_string(),
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
