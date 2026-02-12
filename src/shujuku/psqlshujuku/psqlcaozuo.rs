use serde_json::{Map, Value};
use sqlx::{Column, Row};
use super::psqlshujukuzhuti::huoquchi;

/// 执行查询语句（SELECT），返回结果集
pub async fn chaxun(sql: &str, canshu: &[&str]) -> Option<Vec<Value>> {
    let chi = huoquchi()?;
    let mut chaxun = sqlx::query(sql);
    for c in canshu {
        chaxun = chaxun.bind(*c);
    }
    let hanglie = chaxun.fetch_all(chi).await.ok()?;
    let jieguo = hanglie
        .iter()
        .map(|hang| {
            let mut duixiang = Map::new();
            for (i, lie) in hang.columns().iter().enumerate() {
                let zhi: Option<String> = hang.try_get(i).ok();
                duixiang.insert(
                    lie.name().to_string(),
                    zhi.map(Value::String).unwrap_or(Value::Null),
                );
            }
            Value::Object(duixiang)
        })
        .collect();
    Some(jieguo)
}

/// 执行写操作（INSERT/UPDATE/DELETE/DDL），返回影响行数
pub async fn zhixing(sql: &str, canshu: &[&str]) -> Option<u64> {
    let chi = huoquchi()?;
    let mut chaxun = sqlx::query(sql);
    for c in canshu {
        chaxun = chaxun.bind(*c);
    }
    chaxun.execute(chi).await.ok().map(|r| r.rows_affected())
}
