use llm::chat::{ChatMessage, Tool};
use crate::gongju::tokengongju;

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

    pub fn zhuijia_zhushou_gongjudiaoyong(&mut self, diaoyong: Vec<llm::ToolCall>) {
        self.xiaoxilie.push(ChatMessage::assistant().tool_use(diaoyong).build());
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

    pub fn qingkong_gongjulie(&mut self) {
        self.gongjulie.clear();
    }

    pub fn huoqu_gongjulie(&self) -> Option<&[Tool]> {
        (!self.gongjulie.is_empty()).then(|| self.gongjulie.as_slice())
    }

    /// 计算当前上下文总 token 数
    pub fn dangqian_token(&self) -> usize {
        tokengongju::jisuan_xiaoxilie(self.xitongtishici.as_deref(), &self.xiaoxilie)
    }

    /// 裁剪旧消息，保证总 token 不超过上限
    /// 保留系统提示词 + 最近的消息，从前面逐条删除
    pub fn caijian_shangxiawen(&mut self, zuida_token: u32) {
        let shangxian = zuida_token as usize;
        if shangxian == 0 {
            return;
        }
        let mut dangqian = self.dangqian_token();
        if dangqian <= shangxian {
            return;
        }
        println!("[上下文裁剪] 当前 {} token，上限 {}，开始裁剪", dangqian, shangxian);
        // 从最旧的消息开始删除，至少保留最后一条用户消息
        while dangqian > shangxian && self.xiaoxilie.len() > 1 {
            let yichu = self.xiaoxilie.remove(0);
            let jianshao = tokengongju::jisuan_xiaoxi(&yichu);
            dangqian = dangqian.saturating_sub(jianshao);
        }
        println!("[上下文裁剪] 裁剪后 {} token，剩余 {} 条消息", dangqian, self.xiaoxilie.len());
    }
}
