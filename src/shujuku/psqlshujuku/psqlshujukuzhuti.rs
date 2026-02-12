use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, Row};
use std::collections::HashSet;
use std::sync::OnceLock;
use std::time::Duration;
use crate::gongju::neicungongju;
use crate::gongju::jichugongju;

#[allow(non_upper_case_globals)]
const psql_zhuangtai_jian: &str = "psql_lianjie_zhuangtai";
#[allow(non_upper_case_globals)]
const huoqu_chaoshi_miao: u64 = 5;
#[allow(non_upper_case_globals)]
const kongxian_chaoshi_miao: u64 = 300;
#[allow(non_upper_case_globals)]
const zuida_shengming_miao: u64 = 1800;
#[allow(non_upper_case_globals)]
static quanju_chi: OnceLock<PgPool> = OnceLock::new();

/// 字段定义
pub struct Ziduandinyi {
    pub mingcheng: &'static str,
    pub nicheng: &'static str,
    pub jieshao: &'static str,
    pub leixing: &'static str,
    pub morenzhi: Option<&'static str>,
}

/// 表定义 trait，所有表文件必须实现
pub trait Shujubiaodinyi {
    fn biaoming() -> &'static str;
    fn biaonicheng() -> &'static str;
    fn biaojieshao() -> &'static str;
    fn ziduanlie() -> &'static [Ziduandinyi];
}

/// PostgreSQL 连接配置
pub struct Psqlpeizhi {
    pub zhiji: String,
    pub duankou: u16,
    pub yonghuming: String,
    pub mima: String,
    pub shujukuming: String,
}

/// 表注册信息，用于初始化时建表、同步字段、记录元信息
pub struct Biaozhucexinxi {
    pub biaoming: &'static str,
    pub biaonicheng: &'static str,
    pub biaojieshao: &'static str,
    pub ziduanlie: &'static [Ziduandinyi],
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

fn huoquhexinshu() -> u32 {
    std::thread::available_parallelism()
        .map(|n| n.get() as u32)
        .unwrap_or(4)
}

fn jisuanchicansu() -> (u32, u32, Duration, Duration, Duration) {
    let hexin = huoquhexinshu();
    let zuida = hexin * 2 + 1;
    let zuixiao = (zuida / 5).max(1);
    (
        zuida,
        zuixiao,
        Duration::from_secs(huoqu_chaoshi_miao),
        Duration::from_secs(kongxian_chaoshi_miao),
        Duration::from_secs(zuida_shengming_miao),
    )
}

async fn goujianchi(url: &str) -> Option<PgPool> {
    let (zuida, zuixiao, huoqu, kongxian, shengming) = jisuanchicansu();
    PgPoolOptions::new()
        .max_connections(zuida)
        .min_connections(zuixiao)
        .acquire_timeout(huoqu)
        .idle_timeout(kongxian)
        .max_lifetime(shengming)
        .connect(url)
        .await
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

fn shengchengjianbiaosql(biaoming: &str, ziduanlie: &[Ziduandinyi]) -> String {
    let ziduan = ziduanlie
        .iter()
        .map(|z| format!("\"{}\" {}", z.mingcheng, z.leixing))
        .collect::<Vec<_>>()
        .join(", ");
    format!("CREATE TABLE IF NOT EXISTS \"{}\" ({})", biaoming, ziduan)
}

async fn chuangjianbiao(chi: &PgPool, biaoming: &str, ziduanlie: &[Ziduandinyi]) -> bool {
    sqlx::query(&shengchengjianbiaosql(biaoming, ziduanlie))
        .execute(chi)
        .await
        .is_ok()
}

#[allow(non_upper_case_globals)]
const yueshu_guanjianzi: &[&str] = &["PRIMARY KEY", "NOT NULL", "UNIQUE", "DEFAULT", "CHECK", "REFERENCES"];
#[allow(non_upper_case_globals)]
const chaxun_ziduan_sql: &str = "SELECT column_name FROM information_schema.columns WHERE table_name = $1 AND table_schema = 'public'";

fn tiquchunleixing(leixing: &str) -> &str {
    let daxie = leixing.to_uppercase();
    yueshu_guanjianzi
        .iter()
        .filter_map(|&gj| daxie.find(gj))
        .min()
        .map(|w| leixing[..w].trim())
        .unwrap_or(leixing)
}

fn shengchengtianjia_sql(biaoming: &str, ziduan: &Ziduandinyi) -> String {
    let chunleixing = tiquchunleixing(ziduan.leixing);
    match ziduan.morenzhi {
        Some(moren) => format!(
            "ALTER TABLE \"{}\" ADD COLUMN \"{}\" {} DEFAULT '{}'",
            biaoming, ziduan.mingcheng, chunleixing, moren
        ),
        None => format!(
            "ALTER TABLE \"{}\" ADD COLUMN \"{}\" {}",
            biaoming, ziduan.mingcheng, chunleixing
        ),
    }
}

fn shengchengshanchu_sql(biaoming: &str, ziduanming: &str) -> String {
    format!("ALTER TABLE \"{}\" DROP COLUMN \"{}\"", biaoming, ziduanming)
}

async fn huoqushijiziduan(chi: &PgPool, biaoming: &str) -> Option<HashSet<String>> {
    sqlx::query(chaxun_ziduan_sql)
        .bind(biaoming)
        .fetch_all(chi)
        .await
        .ok()
        .map(|hanglie| hanglie.iter().map(|h| h.get::<String, _>("column_name")).collect())
}

async fn zhixing_sql_lie(chi: &PgPool, sqlie: Vec<String>) -> bool {
    for sql in sqlie {
        if sqlx::query(&sql).execute(chi).await.is_err() {
            return false;
        }
    }
    true
}

async fn tongbuziduan(chi: &PgPool, biaoming: &str, ziduanlie: &[Ziduandinyi]) -> bool {
    let shiji = match huoqushijiziduan(chi, biaoming).await {
        Some(j) => j,
        None => return false,
    };

    let dingyi: HashSet<&str> = ziduanlie.iter().map(|z| z.mingcheng).collect();

    let sqlie: Vec<String> = ziduanlie
        .iter()
        .filter(|z| !shiji.contains(z.mingcheng))
        .map(|z| shengchengtianjia_sql(biaoming, z))
        .chain(
            shiji
                .iter()
                .filter(|m| !dingyi.contains(m.as_str()))
                .map(|m| shengchengshanchu_sql(biaoming, m)),
        )
        .collect();

    zhixing_sql_lie(chi, sqlie).await
}

/// 获取全局 PostgreSQL 连接池（零开销，无锁）
pub fn huoquchi() -> Option<&'static PgPool> {
    quanju_chi.get()
}

/// 判断 PostgreSQL 是否已连接（零开销，无锁）
pub fn shifouqiyong() -> bool {
    quanju_chi.get().is_some()
}

#[allow(non_upper_case_globals)]
const jilubiao_upsert_sql: &str = "\
INSERT INTO \"shujubiaojilubiao\" (\"biaoming\", \"biaonicheng\", \"biaojieshao\", \"ziduanxinxi\", \"chuangjianshijian\", \"gengxinshijian\") \
VALUES ($1, $2, $3, $4, $5, $5) \
ON CONFLICT (\"biaoming\") DO UPDATE SET \
\"biaonicheng\" = $2, \"biaojieshao\" = $3, \"ziduanxinxi\" = $4, \"gengxinshijian\" = $5";

fn xuliehua_ziduanlie(ziduanlie: &[Ziduandinyi]) -> String {
    let shuzhu: Vec<serde_json::Value> = ziduanlie
        .iter()
        .map(|z| serde_json::json!({
            "mingcheng": z.mingcheng,
            "nicheng": z.nicheng,
            "jieshao": z.jieshao,
            "leixing": z.leixing
        }))
        .collect();
    serde_json::to_string(&shuzhu).unwrap_or_default()
}

async fn tongbujilubiao(chi: &PgPool, biaolie: &[Biaozhucexinxi]) -> bool {
    let shijianchuo = jichugongju::huoqushijianchuo().to_string();
    for biao in biaolie {
        let ziduanjson = xuliehua_ziduanlie(biao.ziduanlie);
        if sqlx::query(jilubiao_upsert_sql)
            .bind(biao.biaoming)
            .bind(biao.biaonicheng)
            .bind(biao.biaojieshao)
            .bind(&ziduanjson)
            .bind(&shijianchuo)
            .execute(chi)
            .await
            .is_err()
        {
            return false;
        }
    }
    true
}

/// 连接并初始化 PostgreSQL（自动创建数据库、建表、同步字段、记录元信息），连接状态写入内存缓存
pub async fn lianjie(
    peizhi: &Psqlpeizhi,
    biaolie: &[Biaozhucexinxi],
) -> bool {
    let chenggong = async {
        querenshujuku(peizhi).await.then_some(())?;
        let chi = goujianchi(&goujianurl(peizhi, &peizhi.shujukuming)).await?;
        for biao in biaolie {
            if !chuangjianbiao(&chi, biao.biaoming, biao.ziduanlie).await {
                return None;
            }
            if !tongbuziduan(&chi, biao.biaoming, biao.ziduanlie).await {
                return None;
            }
        }
        if !tongbujilubiao(&chi, biaolie).await {
            return None;
        }
        let _ = quanju_chi.set(chi);
        Some(())
    }
    .await
    .is_some();

    gengxinzhuangtai(chenggong);
    chenggong
}
