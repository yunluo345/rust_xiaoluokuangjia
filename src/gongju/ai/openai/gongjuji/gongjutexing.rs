use llm::chat::Tool;
use std::future::Future;
use std::pin::Pin;
use super::Gongjufenzu;

/// 工具特性：所有工具必须实现此 trait
pub trait Gongju: Send + Sync {
    /// 工具唯一名称
    fn mingcheng(&self) -> &str;
    
    /// 工具定义（JSON Schema）
    fn dinyi(&self) -> Tool;
    
    /// 关键词列表
    fn guanjianci(&self) -> Vec<String>;
    
    /// 工具分组
    fn fenzu(&self) -> Gongjufenzu;
    
    /// 执行工具（异步）
    fn zhixing(&self, canshu: &str, lingpai: &str) -> Pin<Box<dyn Future<Output = String> + Send + 'static>>;
}
