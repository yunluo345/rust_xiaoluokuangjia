use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Arc;

tokio::task_local! {
    /// 当前任务的任务组上下文（通过 task-local 隐式传递到调度器）
    pub static DANGQIAN_RENWUZU: Arc<Renwuzu>;
}

/// 任务组：将一批相关AI调用归为一组，支持组级状态查询和取消
///
/// 通过 `zai_renwuzu_zhong()` 设置当前任务组后，
/// 该作用域内的所有AI调用会自动关联到此组：
/// - 调度器获取/释放 permit 时更新 huoyue_shu
/// - 组取消后新AI调用会被拒绝
pub struct Renwuzu {
    /// 组ID
    pub id: String,
    /// 组名（如"标签任务"、"跨日报分析"）
    pub mingcheng: String,
    /// 取消标志：设为 true 后该组新AI调用将被拒绝
    pub yiquxiao: Arc<AtomicBool>,
    /// 前端断开后是否继续执行
    pub houtai_zhixing: bool,
    /// 当前组内活跃AI调用数（由调度器自动维护）
    pub huoyue_shu: AtomicU32,
    /// 当前组内排队等待数
    pub dengdai_shu: AtomicU32,
}

impl Renwuzu {
    pub fn xingjian(id: impl Into<String>, mingcheng: impl Into<String>, houtai_zhixing: bool) -> Arc<Self> {
        Arc::new(Self {
            id: id.into(),
            mingcheng: mingcheng.into(),
            yiquxiao: Arc::new(AtomicBool::new(false)),
            houtai_zhixing,
            huoyue_shu: AtomicU32::new(0),
            dengdai_shu: AtomicU32::new(0),
        })
    }

    /// 取消该任务组（该组后续新AI调用将被拒绝）
    pub fn quxiao(&self) {
        self.yiquxiao.store(true, Ordering::SeqCst);
    }

    pub fn shifou_yiquxiao(&self) -> bool {
        self.yiquxiao.load(Ordering::Relaxed)
    }
}

/// 在任务组上下文中执行异步操作
///
/// 在此作用域内的所有AI调用（经过调度器）会自动关联到该任务组：
/// - 调度器获取 permit 时递增 huoyue_shu，释放时递减
/// - 组取消后新AI调用返回错误
pub async fn zai_renwuzu_zhong<F, T>(zu: Arc<Renwuzu>, future: F) -> T
where
    F: std::future::Future<Output = T>,
{
    DANGQIAN_RENWUZU.scope(zu, future).await
}

/// 检查当前任务组是否已取消（无任务组时返回 false）
pub fn dangqian_yiquxiao() -> bool {
    DANGQIAN_RENWUZU.try_with(|zu| zu.shifou_yiquxiao()).unwrap_or(false)
}
