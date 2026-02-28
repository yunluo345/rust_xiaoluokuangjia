use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, OnceLock, RwLock};
use tokio::sync::{OwnedSemaphorePermit, Semaphore};

use crate::peizhixt::peizhi_nr::peizhi_ai::{Ai, DiaoduqiPeizhi};
use super::renwuzu::{Renwuzu, DANGQIAN_RENWUZU};

// ── 全局状态 ──

struct NeibuZhuangtai {
    xinhaoling: RwLock<Arc<Semaphore>>,
    dangqian_bingfashu: AtomicU32,
    dengdaishu: AtomicU32,
    quanju_shangxian: AtomicU32,
    paidui_chaoshi_miao: AtomicU32,
}

#[allow(non_upper_case_globals)]
static quanju: OnceLock<NeibuZhuangtai> = OnceLock::new();
fn xinjian_neibu_zhuangtai(quanju_shangxian: u32, paidui_chaoshi_miao: u32) -> NeibuZhuangtai {
    NeibuZhuangtai {
        xinhaoling: RwLock::new(Arc::new(Semaphore::new(quanju_shangxian as usize))),
        dangqian_bingfashu: AtomicU32::new(0),
        dengdaishu: AtomicU32::new(0),
        quanju_shangxian: AtomicU32::new(quanju_shangxian),
        paidui_chaoshi_miao: AtomicU32::new(paidui_chaoshi_miao),
    }
}

fn huoqu_quanju() -> &'static NeibuZhuangtai {
    quanju.get_or_init(|| {
        println!("[调度器] 未显式初始化，使用默认配置");
        let moren = DiaoduqiPeizhi::default();
        xinjian_neibu_zhuangtai(moren.quanju_bingfa_shangxian, moren.paidui_chaoshi_miao as u32)
    })
}

// ── 对外数据结构 ──

/// 调度器状态快照
pub struct DiaoduZhuangtai {
    pub quanju_shangxian: u32,
    pub dangqian_bingfashu: u32,
    pub dengdaishu: u32,
}

impl DiaoduZhuangtai {
    pub fn shengyu_weizhi(&self) -> u32 {
        self.quanju_shangxian.saturating_sub(self.dangqian_bingfashu)
    }
}

/// 调度错误
#[derive(Debug)]
pub struct DiaoduCuowu {
    pub xiaoxi: String,
}

impl std::fmt::Display for DiaoduCuowu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.xiaoxi)
    }
}

/// 许可证守卫（RAII），drop 时自动归还 permit 并递减全局和任务组计数
pub struct XukezhengShouwei {
    _xukezheng: OwnedSemaphorePermit,
    renwuzu: Option<Arc<Renwuzu>>,
}

impl Drop for XukezhengShouwei {
    fn drop(&mut self) {
        huoqu_quanju().dangqian_bingfashu.fetch_sub(1, Ordering::SeqCst);
        if let Some(ref zu) = self.renwuzu {
            zu.huoyue_shu.fetch_sub(1, Ordering::SeqCst);
        }
    }
}

/// 等待计数守卫：防止 future 被取消时 dengdaishu 泄漏
struct DengdaiShouwei;

impl Drop for DengdaiShouwei {
    fn drop(&mut self) {
        huoqu_quanju().dengdaishu.fetch_sub(1, Ordering::SeqCst);
    }
}

// ── 内部辅助 ──

/// 获取信号量 Arc 克隆（RwLock 读锁，极快）
fn huoqu_xinhaoling() -> Arc<Semaphore> {
    huoqu_quanju().xinhaoling.read().unwrap().clone()
}

/// 获取当前任务组引用（通过 task-local，无任务组时返回 None）
fn huoqu_dangqian_renwuzu() -> Option<Arc<Renwuzu>> {
    DANGQIAN_RENWUZU.try_with(|zu| zu.clone()).ok()
}

/// 构建许可证守卫，同时更新全局和任务组计数
fn goujian_shouwei(xukezheng: OwnedSemaphorePermit) -> XukezhengShouwei {
    huoqu_quanju().dangqian_bingfashu.fetch_add(1, Ordering::SeqCst);
    let renwuzu = huoqu_dangqian_renwuzu();
    if let Some(ref zu) = renwuzu {
        zu.huoyue_shu.fetch_add(1, Ordering::SeqCst);
    }
    XukezhengShouwei { _xukezheng: xukezheng, renwuzu }
}

/// 检查任务组取消状态，已取消则返回错误
fn jiancha_renwuzu_quxiao() -> Result<(), DiaoduCuowu> {
    if let Some(zu) = huoqu_dangqian_renwuzu() {
        if zu.shifou_yiquxiao() {
            return Err(DiaoduCuowu { xiaoxi: "任务组已取消".to_string() });
        }
    }
    Ok(())
}

fn zhunbei_huoqu_xukezheng() -> Result<DengdaiShouwei, DiaoduCuowu> {
    jiancha_renwuzu_quxiao()?;
    huoqu_quanju().dengdaishu.fetch_add(1, Ordering::SeqCst);
    Ok(DengdaiShouwei)
}

// ── 核心函数 ──

/// 从配置文件初始化调度器（应用启动时调用一次）
pub fn chushihua_cong_peizhi() {
    let peizhi = Ai::duqu_huo_moren().diaoduqi;

    let shangxian = peizhi.quanju_bingfa_shangxian.max(1);
    let chaoshi = peizhi.paidui_chaoshi_miao as u32;

    let _ = quanju.get_or_init(|| {
        println!("[调度器] 初始化 全局并发上限={} 排队超时={}秒", shangxian, chaoshi);
        xinjian_neibu_zhuangtai(shangxian, chaoshi)
    });
}

/// 无超时获取许可（永久等待直到可用），支持任务组取消检测
pub async fn huoqu_xukezheng() -> Result<XukezhengShouwei, DiaoduCuowu> {
    let dengdai_shouwei = zhunbei_huoqu_xukezheng()?;

    let jieguo = huoqu_xinhaoling().acquire_owned().await;
    drop(dengdai_shouwei);

    match jieguo {
        Ok(xukezheng) => Ok(goujian_shouwei(xukezheng)),
        Err(_) => Err(DiaoduCuowu { xiaoxi: "调度器信号量已关闭".to_string() }),
    }
}

/// 带超时获取许可（使用配置中的默认超时）
pub async fn changshi_huoqu_xukezheng_moren() -> Result<XukezhengShouwei, DiaoduCuowu> {
    let chaoshi_miao = huoqu_quanju().paidui_chaoshi_miao.load(Ordering::Relaxed) as u64;
    changshi_huoqu_xukezheng(chaoshi_miao).await
}

pub fn huoqu_paidui_chaoshi_miao() -> u64 {
    huoqu_quanju().paidui_chaoshi_miao.load(Ordering::Relaxed) as u64
}

/// 带超时获取许可
pub async fn changshi_huoqu_xukezheng(chaoshi_miao: u64) -> Result<XukezhengShouwei, DiaoduCuowu> {
    let zhuangtai = huoqu_quanju();
    let dengdai_shouwei = zhunbei_huoqu_xukezheng()?;

    let jieguo = tokio::time::timeout(
        std::time::Duration::from_secs(chaoshi_miao),
        huoqu_xinhaoling().acquire_owned(),
    )
    .await;

    // 正常完成，显式释放守卫
    drop(dengdai_shouwei);

    match jieguo {
        Ok(Ok(xukezheng)) => Ok(goujian_shouwei(xukezheng)),
        Ok(Err(_)) => Err(DiaoduCuowu {
            xiaoxi: "调度器信号量已关闭".to_string(),
        }),
        Err(_) => {
            let shangxian = zhuangtai.quanju_shangxian.load(Ordering::Relaxed);
            let bingfa = zhuangtai.dangqian_bingfashu.load(Ordering::Relaxed);
            Err(DiaoduCuowu {
                xiaoxi: format!(
                    "AI调度排队超时({}秒) 并发:{}/{}", chaoshi_miao, bingfa, shangxian
                ),
            })
        }
    }
}

/// 查询调度器当前状态
pub fn chaxun_zhuangtai() -> DiaoduZhuangtai {
    let zhuangtai = huoqu_quanju();
    DiaoduZhuangtai {
        quanju_shangxian: zhuangtai.quanju_shangxian.load(Ordering::Relaxed),
        dangqian_bingfashu: zhuangtai.dangqian_bingfashu.load(Ordering::Relaxed),
        dengdaishu: zhuangtai.dengdaishu.load(Ordering::Relaxed),
    }
}

/// 热更新全局并发上限（重建信号量）
///
/// 已持有的 permit 不受影响（旧 Semaphore 通过 Arc 保活）。
/// 新请求将使用新 Semaphore，上限立即生效。
#[allow(dead_code)]
pub fn regengxin_shangxian(xin_shangxian: u32) {
    let shangxian = xin_shangxian.max(1);
    let zhuangtai = huoqu_quanju();
    let mut xinhaoling = zhuangtai.xinhaoling.write().unwrap();
    *xinhaoling = Arc::new(Semaphore::new(shangxian as usize));
    zhuangtai.quanju_shangxian.store(shangxian, Ordering::SeqCst);
    println!("[调度器] 重建信号量 新上限={}", shangxian);
}

/// 热更新排队超时
#[allow(dead_code)]
pub fn regengxin_chaoshi(xin_chaoshi_miao: u64) {
    huoqu_quanju().paidui_chaoshi_miao.store(xin_chaoshi_miao as u32, Ordering::SeqCst);
    println!("[调度器] 更新排队超时={}秒", xin_chaoshi_miao);
}
