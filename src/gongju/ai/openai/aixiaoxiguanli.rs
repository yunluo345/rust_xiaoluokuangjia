use llm::chat::{ChatMessage, Tool};

pub struct Xiaoxiguanli {
    xitongtishici: Option<String>,
    xiaoxilie: Vec<ChatMessage>,
    gongjulie: Vec<Tool>,
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
}
