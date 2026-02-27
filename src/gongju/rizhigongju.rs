//! 结构化日志与 trace_id 工具
//!
//! 提供请求级 trace_id 生成与分类日志输出，
//! 将"AI调用失败、JSON解析失败、仓储失败、业务校验失败"分开埋点。

use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

/// 全局自增序列，保证同一毫秒内 trace_id 唯一
static XULIE: AtomicU64 = AtomicU64::new(0);

/// 日志分类
pub enum RizhiFenlei {
    /// AI 调用（请求/响应/超时）
    AiDiaoyong,
    /// JSON 解析（清洗/反序列化/校验）
    JsonJiexi,
    /// 仓储层（数据库读写错误）
    Cangchu,
    /// 业务校验（必填标签/参数缺失/逻辑冲突）
    YewuJiaoyan,
    /// 一般信息
    Xinxi,
}

impl RizhiFenlei {
    fn biaoji(&self) -> &'static str {
        match self {
            Self::AiDiaoyong => "AI",
            Self::JsonJiexi => "JSON",
            Self::Cangchu => "DB",
            Self::YewuJiaoyan => "BIZ",
            Self::Xinxi => "INFO",
        }
    }
}

/// 生成唯一 trace_id（毫秒时间戳 + 自增序列）
pub fn shengcheng_trace_id() -> String {
    let shijian = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0);
    let xuhao = XULIE.fetch_add(1, Ordering::Relaxed);
    format!("t{:x}-{:04x}", shijian, xuhao & 0xFFFF)
}

/// 输出结构化日志
pub fn jilu(fenlei: RizhiFenlei, trace_id: &str, mokuai: &str, xiaoxi: &str) {
    println!(
        "[{}][{}][{}] {}",
        fenlei.biaoji(),
        trace_id,
        mokuai,
        xiaoxi
    );
}

/// 输出结构化错误日志
pub fn jilu_cuowu(fenlei: RizhiFenlei, trace_id: &str, mokuai: &str, cuowu: &str) {
    eprintln!(
        "[{}][{}][{}] 错误: {}",
        fenlei.biaoji(),
        trace_id,
        mokuai,
        cuowu
    );
}

#[cfg(test)]
mod ceshi {
    use super::*;

    #[test]
    fn ceshi_trace_id_weiyi() {
        let id1 = shengcheng_trace_id();
        let id2 = shengcheng_trace_id();
        assert_ne!(id1, id2);
    }

    #[test]
    fn ceshi_trace_id_geishi() {
        let id = shengcheng_trace_id();
        assert!(id.starts_with('t'));
        assert!(id.contains('-'));
    }

    #[test]
    fn ceshi_fenlei_biaoji() {
        assert_eq!(RizhiFenlei::AiDiaoyong.biaoji(), "AI");
        assert_eq!(RizhiFenlei::JsonJiexi.biaoji(), "JSON");
        assert_eq!(RizhiFenlei::Cangchu.biaoji(), "DB");
        assert_eq!(RizhiFenlei::YewuJiaoyan.biaoji(), "BIZ");
        assert_eq!(RizhiFenlei::Xinxi.biaoji(), "INFO");
    }
}
