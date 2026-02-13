use llm::chat::{ChatMessage, Tool};
use tiktoken_rs::o200k_base;
use std::sync::OnceLock;
use tiktoken_rs::CoreBPE;

pub struct Xiaoxiguanli {
    xitongtishici: Option<String>,
    xiaoxilie: Vec<ChatMessage>,
    gongjulie: Vec<Tool>,
}

#[allow(non_upper_case_globals)]
static fenciqi: OnceLock<CoreBPE> = OnceLock::new();

fn jisuan_tokenshu(wenben: &str) -> usize {
    fenciqi
        .get_or_init(|| o200k_base().unwrap_or_else(|_| tiktoken_rs::cl100k_base().unwrap()))
        .encode_with_special_tokens(wenben)
        .len()
}

impl Xiaoxiguanli {
    pub fn xingjian() -> Self {
        Self {
            xitongtishici: None,
            xiaoxilie: Vec::new(),
            gongjulie: Vec::new(),
        }
    }

    pub fn shezhi_xitongtishici(mut self, tishici: impl Into<String>) -> Self {
        self.xitongtishici = Some(tishici.into());
        self
    }

    pub fn tianjia_gongju(mut self, gongju: Tool) -> Self {
        self.gongjulie.push(gongju);
        self
    }

    pub fn zhuijia_yonghuxiaoxi(&mut self, neirong: impl Into<String>) {
        self.xiaoxilie.push(ChatMessage::user().content(neirong).build());
    }

    pub fn zhuijia_zhushouneirong(&mut self, neirong: impl Into<String>) {
        self.xiaoxilie.push(ChatMessage::assistant().content(neirong).build());
    }

    pub fn zhuijia_zhushougongjudiaoyong(&mut self, gongjudiaoyong: Vec<llm::ToolCall>) {
        self.xiaoxilie.push(ChatMessage::assistant().tool_use(gongjudiaoyong).build());
    }

    pub fn zhuijia_gongjujieguo(&mut self, gongjudiaoyong: Vec<llm::ToolCall>) {
        self.xiaoxilie.push(ChatMessage::user().tool_result(gongjudiaoyong).build());
    }

    pub fn huoqu_xitongtishici(&self) -> Option<&str> {
        self.xitongtishici.as_deref()
    }

    pub fn huoqu_xiaoxilie(&self) -> &[ChatMessage] {
        &self.xiaoxilie
    }

    pub fn huoqu_gongjulie(&self) -> Option<&[Tool]> {
        (!self.gongjulie.is_empty()).then(|| self.gongjulie.as_slice())
    }

    pub fn yasuo(&mut self, zuidatoken: usize) {
        if zuidatoken == 0 || self.xiaoxilie.is_empty() {
            return;
        }
        let tishici_tokenshu = self.xitongtishici.as_deref().map_or(0, jisuan_tokenshu);
        let keyong = zuidatoken.saturating_sub(tishici_tokenshu);
        let meitian_tokenshu: Vec<usize> = self.xiaoxilie.iter()
            .map(|x| jisuan_tokenshu(&x.content))
            .collect();
        let zong: usize = meitian_tokenshu.iter().sum();
        if zong <= keyong {
            return;
        }
        let mut yaoshan = 0usize;
        let mut yishantoken = 0usize;
        let baoliu_zuishao = 1;
        while yaoshan < self.xiaoxilie.len() - baoliu_zuishao && zong - yishantoken > keyong {
            yishantoken += meitian_tokenshu[yaoshan];
            yaoshan += 1;
        }
        if yaoshan > 0 {
            self.xiaoxilie.drain(..yaoshan);
        }
    }
}
