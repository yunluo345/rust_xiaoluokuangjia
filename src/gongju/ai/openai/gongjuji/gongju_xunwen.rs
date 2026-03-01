use llm::chat::Tool;
use serde::Deserialize;
use serde_json::json;
use super::Gongjufenzu;

#[derive(Deserialize)]
struct Qingqiucanshu {
    wenti: Option<String>,
    xuanxiang: Option<Vec<String>>,
}

pub fn huoqu_guanjianci() -> Vec<String> {
    vec![
        "确认".to_string(),
        "询问".to_string(),
        "是否".to_string(),
        "提问".to_string(),
        "确定".to_string(),
    ]
}

pub fn huoqu_fenzu() -> Gongjufenzu {
    Gongjufenzu::Xitong
}

pub fn dinyi() -> Tool {
    Tool {
        tool_type: "function".to_string(),
        function: llm::chat::FunctionTool {
            name: "xunwen".to_string(),
            description: "向用户提出确认性问题并等待回复，调用后会中断当前对话轮次。用于在执行重要操作前征求用户确认（如提交日报前确认）。".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "wenti": {"type": "string", "description": "要向用户提出的问题"},
                    "xuanxiang": {
                        "type": "array",
                        "items": {"type": "string"},
                        "description": "可选：建议选项列表，前端可展示为按钮，用户也可自由输入不受限"
                    }
                },
                "required": ["wenti"]
            }),
        },
    }
}

pub async fn zhixing(canshu: &str, _lingpai: &str) -> String {
    let qingqiu: Qingqiucanshu = match serde_json::from_str(canshu) {
        Ok(q) => q,
        Err(_) => return json!({"cuowu": "参数格式错误"}).to_string(),
    };
    let wenti = match qingqiu.wenti.as_deref().map(str::trim).filter(|s| !s.is_empty()) {
        Some(w) => w.to_string(),
        None => return json!({"cuowu": "问题内容不能为空"}).to_string(),
    };
    let xuanxiang = qingqiu.xuanxiang.unwrap_or_default();
    json!({
        "leixing": "xunwen",
        "wenti": wenti,
        "xuanxiang": xuanxiang,
    }).to_string()
}
