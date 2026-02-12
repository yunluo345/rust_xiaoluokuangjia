use deadpool_redis::{Config, Pool, Runtime, redis::cmd};
use std::sync::OnceLock;
use crate::gongju::neicungongju;

#[allow(non_upper_case_globals)]
const redis_zhuangtai_jian: &str = "redis_lianjie_zhuangtai";
#[allow(non_upper_case_globals)]
static quanju_chi: OnceLock<Pool> = OnceLock::new();

/// Redis 连接配置
pub struct Redislianjiepeizhi {
    pub zhujidizhi: String,
    pub duankou: u16,
    pub zhanghao: String,
    pub mima: String,
}

fn gengxinzhuangtai(zhuangtai: bool) {
    neicungongju::xieru(redis_zhuangtai_jian, if zhuangtai { "1" } else { "0" });
}

fn goujianurl(peizhi: &Redislianjiepeizhi) -> String {
    match (peizhi.zhanghao.is_empty(), peizhi.mima.is_empty()) {
        (true, true) => format!("redis://{}:{}", peizhi.zhujidizhi, peizhi.duankou),
        (false, true) => format!("redis://{}@{}:{}", peizhi.zhanghao, peizhi.zhujidizhi, peizhi.duankou),
        (_, _) => format!("redis://{}:{}@{}:{}", peizhi.zhanghao, peizhi.mima, peizhi.zhujidizhi, peizhi.duankou),
    }
}

fn goujianchi(url: &str) -> Option<Pool> {
    Config::from_url(url).create_pool(Some(Runtime::Tokio1)).ok()
}

async fn ceshilianjie(chi: &Pool) -> bool {
    async {
        let mut lianjie = chi.get().await.ok()?;
        cmd("PING").query_async::<String>(&mut *lianjie).await.ok()
    }
    .await
    .is_some()
}

/// 获取全局 Redis 连接池
pub fn huoquchi() -> Option<&'static Pool> {
    quanju_chi.get()
}

/// 判断 Redis 是否已连接
pub fn shifouqiyong() -> bool {
    quanju_chi.get().is_some()
}

/// 连接并初始化 Redis，连接状态写入内存缓存
pub async fn lianjie(peizhi: &Redislianjiepeizhi) -> bool {
    let chenggong = async {
        let chi = goujianchi(&goujianurl(peizhi))?;
        ceshilianjie(&chi).await.then_some(())?;
        let _ = quanju_chi.set(chi);
        Some(())
    }
    .await
    .is_some();

    gengxinzhuangtai(chenggong);
    chenggong
}
