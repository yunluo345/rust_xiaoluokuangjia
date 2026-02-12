use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::sync::OnceLock;
use std::time::Duration;
use crate::gongju::neicungongju;

#[allow(non_upper_case_globals)]
const psql_zhuangtai_jian: &str = "psql_lianjie_zhuangtai";
#[allow(non_upper_case_globals)]
static quanju_chi: OnceLock<PgPool> = OnceLock::new();

/// PostgreSQL 连接配置
pub struct Psqlpeizhi {
    pub zhiji: String,
    pub duankou: u16,
    pub yonghuming: String,
    pub mima: String,
    pub shujukuming: String,
    pub zuida_lianjie: u32,
    pub zuixiao_lianjie: u32,
    pub huoqu_chaoshi_miao: u64,
    pub kongxian_chaoshi_miao: u64,
    pub zuida_shengming_miao: u64,
}

fn goujianurl(peizhi: &Psqlpeizhi, shujukuming: &str) -> String {
    format!(
        "postgres://{}:{}@{}:{}/{}",
        peizhi.yonghuming, peizhi.mima, peizhi.zhiji, peizhi.duankou, shujukuming
    )
}

fn gengxinzhuangtai(zhuangtai: bool) {
    neicungongju::xieru(psql_zhuangtai_jian, if zhuangtai { "1" } else { "0" });
}

async fn goujianchi(url: &str, peizhi: &Psqlpeizhi) -> Option<PgPool> {
    PgPoolOptions::new()
        .max_connections(peizhi.zuida_lianjie)
        .min_connections(peizhi.zuixiao_lianjie)
        .acquire_timeout(Duration::from_secs(peizhi.huoqu_chaoshi_miao))
        .idle_timeout(Duration::from_secs(peizhi.kongxian_chaoshi_miao))
        .max_lifetime(Duration::from_secs(peizhi.zuida_shengming_miao))
        .connect(url)
        .await
        .map_err(|e| eprintln!("连接池构建失败: {}", e))
        .ok()
}

async fn shujukucunzai(chi: &PgPool, mingcheng: &str) -> Option<bool> {
    sqlx::query("SELECT 1 FROM pg_database WHERE datname = $1")
        .bind(mingcheng)
        .fetch_optional(chi)
        .await
        .ok()
        .map(|hang| hang.is_some())
}

async fn chuangjianshujuku(chi: &PgPool, mingcheng: &str) -> bool {
    sqlx::query(&format!("CREATE DATABASE \"{}\"", mingcheng))
        .execute(chi)
        .await
        .is_ok()
}

async fn goujianlinshichi(url: &str) -> Option<PgPool> {
    PgPool::connect(url).await.ok()
}

async fn shuaxinmuban(chi: &PgPool) {
    let _ = sqlx::query("ALTER DATABASE template1 REFRESH COLLATION VERSION")
        .execute(chi)
        .await;
}

async fn querenshujuku(peizhi: &Psqlpeizhi) -> bool {
    let guanlichi = match goujianlinshichi(&goujianurl(peizhi, "postgres")).await {
        Some(c) => c,
        None => return false,
    };

    shuaxinmuban(&guanlichi).await;

    let jieguo = shujukucunzai(&guanlichi, &peizhi.shujukuming)
        .await
        .filter(|&c| c)
        .is_some()
        || chuangjianshujuku(&guanlichi, &peizhi.shujukuming).await;

    guanlichi.close().await;
    jieguo
}

/// 获取全局 PostgreSQL 连接池（零开销，无锁）
pub fn huoquchi() -> Option<&'static PgPool> {
    quanju_chi.get()
}

/// 判断 PostgreSQL 是否已连接（零开销，无锁）
pub fn shifouqiyong() -> bool {
    quanju_chi.get().is_some()
}

/// 连接并初始化 PostgreSQL（自动创建数据库），连接状态写入内存缓存
pub async fn lianjie(peizhi: &Psqlpeizhi) -> bool {
    let chenggong = async {
        querenshujuku(peizhi).await.then_some(())?;
        let chi = goujianchi(&goujianurl(peizhi, &peizhi.shujukuming), peizhi).await?;
        let _ = quanju_chi.set(chi);
        Some(())
    }
    .await
    .is_some();

    gengxinzhuangtai(chenggong);
    chenggong
}
