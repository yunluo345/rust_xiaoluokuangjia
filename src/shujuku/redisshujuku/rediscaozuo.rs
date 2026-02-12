use deadpool_redis::redis::{cmd, FromRedisValue, ToRedisArgs};
use super::redisshujukuzhuti::huoquchi;

async fn huoqulianjie() -> Option<deadpool_redis::Connection> {
    huoquchi()?.get().await.ok()
}

/// 设置键值
pub async fn shezhi(jian: &str, zhi: impl ToRedisArgs) -> bool {
    let Some(mut lianjie) = huoqulianjie().await else { return false };
    cmd("SET").arg(jian).arg(zhi)
        .query_async::<()>(&mut *lianjie).await.is_ok()
}

/// 设置键值并指定过期秒数
pub async fn shezhidaiguoqi(jian: &str, zhi: impl ToRedisArgs, miao: u64) -> bool {
    let Some(mut lianjie) = huoqulianjie().await else { return false };
    cmd("SET").arg(jian).arg(zhi).arg("EX").arg(miao)
        .query_async::<()>(&mut *lianjie).await.is_ok()
}

/// 获取键值
pub async fn huoqu<T: FromRedisValue>(jian: &str) -> Option<T> {
    let mut lianjie = huoqulianjie().await?;
    cmd("GET").arg(jian)
        .query_async(&mut *lianjie).await.ok()
}

/// 删除键
pub async fn shanchu(jian: &str) -> bool {
    let Some(mut lianjie) = huoqulianjie().await else { return false };
    cmd("DEL").arg(jian)
        .query_async::<i64>(&mut *lianjie).await.unwrap_or(0) > 0
}

/// 判断键是否存在
pub async fn cunzai(jian: &str) -> bool {
    let Some(mut lianjie) = huoqulianjie().await else { return false };
    cmd("EXISTS").arg(jian)
        .query_async::<i64>(&mut *lianjie).await.unwrap_or(0) > 0
}

/// 设置过期时间（秒）
pub async fn shezhiguoqi(jian: &str, miao: u64) -> bool {
    let Some(mut lianjie) = huoqulianjie().await else { return false };
    cmd("EXPIRE").arg(jian).arg(miao)
        .query_async::<i64>(&mut *lianjie).await.unwrap_or(0) > 0
}

/// 自增并返回新值
pub async fn zizeng(jian: &str) -> Option<i64> {
    let mut lianjie = huoqulianjie().await?;
    cmd("INCR").arg(jian)
        .query_async(&mut *lianjie).await.ok()
}

/// 自增并设置过期（用于频率限制），返回自增后的值
pub async fn zizengdaiguoqi(jian: &str, miao: u64) -> Option<i64> {
    let mut lianjie = huoqulianjie().await?;
    let zhi: i64 = cmd("INCR").arg(jian)
        .query_async(&mut *lianjie).await.ok()?;
    if zhi == 1 {
        cmd("EXPIRE").arg(jian).arg(miao)
            .query_async::<i64>(&mut *lianjie).await.ok();
    }
    Some(zhi)
}

/// 设置 Hash 字段
pub async fn hshezhi(jian: &str, ziduan: &str, zhi: impl ToRedisArgs) -> bool {
    let Some(mut lianjie) = huoqulianjie().await else { return false };
    cmd("HSET").arg(jian).arg(ziduan).arg(zhi)
        .query_async::<()>(&mut *lianjie).await.is_ok()
}

/// 批量设置 Hash 字段
pub async fn hpiliangshezhi(jian: &str, ziduanlie: &[(&str, &str)]) -> bool {
    let Some(mut lianjie) = huoqulianjie().await else { return false };
    let mut minglin = cmd("HSET");
    minglin.arg(jian);
    for (ziduan, zhi) in ziduanlie {
        minglin.arg(*ziduan).arg(*zhi);
    }
    minglin.query_async::<()>(&mut *lianjie).await.is_ok()
}

/// 获取 Hash 字段值
pub async fn hhuoqu<T: FromRedisValue>(jian: &str, ziduan: &str) -> Option<T> {
    let mut lianjie = huoqulianjie().await?;
    cmd("HGET").arg(jian).arg(ziduan)
        .query_async(&mut *lianjie).await.ok()
}

/// 获取 Hash 所有字段和值
pub async fn hhuoququanbu(jian: &str) -> Option<Vec<(String, String)>> {
    let mut lianjie = huoqulianjie().await?;
    let jieguo: Vec<String> = cmd("HGETALL").arg(jian)
        .query_async(&mut *lianjie).await.ok()?;
    if jieguo.is_empty() {
        return None;
    }
    Some(
        jieguo.chunks_exact(2)
            .map(|dui| (dui[0].clone(), dui[1].clone()))
            .collect()
    )
}

/// 删除 Hash 字段
pub async fn hshanchu(jian: &str, ziduan: &str) -> bool {
    let Some(mut lianjie) = huoqulianjie().await else { return false };
    cmd("HDEL").arg(jian).arg(ziduan)
        .query_async::<i64>(&mut *lianjie).await.unwrap_or(0) > 0
}

/// 获取剩余过期时间（秒），-1 表示永不过期，-2 表示键不存在
pub async fn shengyuttl(jian: &str) -> Option<i64> {
    let mut lianjie = huoqulianjie().await?;
    cmd("TTL").arg(jian)
        .query_async(&mut *lianjie).await.ok()
}
