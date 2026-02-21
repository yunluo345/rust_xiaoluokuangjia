use crate::gongju::jichugongju;
use llm::chat::Tool;
use serde_json::json;
use super::Gongjufenzu;

pub fn huoqu_guanjianci() -> Vec<String> {
    vec![
        "现在几点".to_string(),
        "当前时间".to_string(),
        "服务器时间".to_string(),
        "查询时间".to_string(),
    ]
}

pub fn huoqu_fenzu() -> Gongjufenzu {
    Gongjufenzu::Xitong
}

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

pub async fn zhixing(_canshu: &str, _lingpai: &str) -> String {
    let shijianchuo = jichugongju::huoqushijianchuo();
    format!("当前服务器时间戳: {}", shijianchuo)
}
