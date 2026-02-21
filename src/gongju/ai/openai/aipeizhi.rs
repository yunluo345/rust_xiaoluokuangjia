use llm::builder::LLMBackend;
use crate::gongju::ai::aitongyonggongju;

pub struct Aipeizhi {
    pub leixing: LLMBackend,
    pub jiekoudizhi: String,
    pub miyao: String,
    pub moxing: String,
    pub wendu: f32,
    pub zuida_token: u32,
    pub chaoshishijian: u64,
    pub chongshicishu: u32,
}

impl Aipeizhi {
    pub fn cong_qudaoshuju(shuju: &serde_json::Value) -> Option<Self> {
        let leixing_str = quziduan(shuju, "leixing");
        let leixing = jiexi_leixing(leixing_str)?;
        let yuanshi_dizhi = quziduan(shuju, "jiekoudizhi");
        let jiekoudizhi = aitongyonggongju::buquan_wangguandizhi(leixing_str, yuanshi_dizhi)
            .unwrap_or_else(|| yuanshi_dizhi.to_string());
        Some(Self {
            leixing,
            jiekoudizhi,
            miyao: quziduan(shuju, "miyao").to_string(),
            moxing: quziduan(shuju, "moxing").to_string(),
            wendu: quziduan(shuju, "wendu").parse().unwrap_or(0.0),
            zuida_token: {
                let zhi: u32 = quziduan(shuju, "zuida_token").parse().unwrap_or(0);
                if zhi == 0 { 22767 } else { zhi }
            },
            chaoshishijian: 240,
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
        "openai" | "openapi" => Some(LLMBackend::OpenAI),
        "claude" | "anthropic" => Some(LLMBackend::Anthropic),
        "deepseek" => Some(LLMBackend::DeepSeek),
        "google" | "gemini" => Some(LLMBackend::Google),
        "ollama" => Some(LLMBackend::Ollama),
        "groq" => Some(LLMBackend::Groq),
        "xai" => Some(LLMBackend::XAI),
        _ => None,
    }
}
