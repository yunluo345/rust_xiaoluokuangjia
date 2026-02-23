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
    let hanglie = match chaxun.fetch_all(chi).await {
        Ok(h) => h,
        Err(e) => { println!("[SQL错误] {}", e); return None; }
    };
    let jieguo = hanglie
        .iter()
        .map(|hang| {
            let mut duixiang = Map::new();
            for (i, lie) in hang.columns().iter().enumerate() {
                let zhi = hang.try_get::<Option<String>, _>(i)
                    .or_else(|_| hang.try_get::<Option<i64>, _>(i).map(|v| v.map(|n| n.to_string())))
                    .or_else(|_| hang.try_get::<Option<i32>, _>(i).map(|v| v.map(|n| n.to_string())))
                    .or_else(|_| hang.try_get::<Option<f64>, _>(i).map(|v| v.map(|n| n.to_string())))
                    .or_else(|_| hang.try_get::<Option<bool>, _>(i).map(|v| v.map(|b| b.to_string())))
                    .unwrap_or(None);
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
