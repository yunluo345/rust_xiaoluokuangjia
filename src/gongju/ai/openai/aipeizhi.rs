use llm::builder::LLMBackend;

pub struct Aipeizhi {
    pub leixing: LLMBackend,
    pub jiekoudizhi: String,
    pub miyao: String,
    pub moxing: String,
    pub wendu: f32,
    pub chaoshishijian: u64,
    pub chongshicishu: u32,
}

impl Aipeizhi {
    pub fn cong_qudaoshuju(shuju: &serde_json::Value) -> Option<Self> {
        let leixing = jiexi_leixing(quziduan(shuju, "leixing"))?;
        Some(Self {
            leixing,
            jiekoudizhi: quziduan(shuju, "jiekoudizhi").to_string(),
            miyao: quziduan(shuju, "miyao").to_string(),
            moxing: quziduan(shuju, "moxing").to_string(),
            wendu: quziduan(shuju, "wendu").parse().unwrap_or(0.0),
            chaoshishijian: 30,
            chongshicishu: 0,
        })
    }

    pub fn shezhi_chaoshi(mut self, miao: u64) -> Self {
        self.chaoshishijian = miao;
        self
    }

    pub fn shezhi_chongshi(mut self, cishu: u32) -> Self {
        self.chongshicishu = cishu;
        self
    }
}

fn quziduan<'a>(shuju: &'a serde_json::Value, ming: &str) -> &'a str {
    shuju.get(ming).and_then(|v| v.as_str()).unwrap_or("")
}

fn jiexi_leixing(leixing: &str) -> Option<LLMBackend> {
    match leixing {
        "openai" => Some(LLMBackend::OpenAI),
        "claude" | "anthropic" => Some(LLMBackend::Anthropic),
        "deepseek" => Some(LLMBackend::DeepSeek),
        "google" | "gemini" => Some(LLMBackend::Google),
        "ollama" => Some(LLMBackend::Ollama),
        "groq" => Some(LLMBackend::Groq),
        "xai" => Some(LLMBackend::XAI),
        _ => None,
    }
}
