use serde_json::{Map, Value};
use sqlx::{Column, Row};
use super::psqlshujukuzhuti::huoquchi;

/// 执行查询语句（SELECT），返回结果集 - 支持 Option 参数（新版本）
pub async fn chaxun(sql: &str, canshu: &[Option<&str>]) -> Option<Vec<Value>> {
    let chi = huoquchi()?;
    let mut chaxun = sqlx::query(sql);
    for c in canshu {
        chaxun = match c {
            Some(v) => chaxun.bind(*v),
            None => chaxun.bind(None::<String>),
        };
    }
    let hanglie = match chaxun.fetch_all(chi).await {
        Ok(h) => h,
        Err(e) => {
            println!("[SQL查询错误] SQL: {} | 参数: {:?} | 错误: {}", sql, canshu, e);
            return None;
        }
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

/// 执行查询语句（SELECT），返回结果集 - 兼容旧版本（不支持 NULL）
pub async fn chaxun_jiuban(sql: &str, canshu: &[&str]) -> Option<Vec<Value>> {
    let canshu_zhuanhuan: Vec<Option<&str>> = canshu.iter().map(|s| Some(*s)).collect();
    chaxun(sql, &canshu_zhuanhuan).await
}

/// 执行写操作（INSERT/UPDATE/DELETE/DDL），返回影响行数 - 支持 Option 参数（新版本）
pub async fn zhixing(sql: &str, canshu: &[Option<&str>]) -> Option<u64> {
    let chi = huoquchi()?;
    let mut chaxun = sqlx::query(sql);
    for c in canshu {
        chaxun = match c {
            Some(v) => chaxun.bind(*v),
            None => chaxun.bind(None::<String>),
        };
    }
    match chaxun.execute(chi).await {
        Ok(r) => Some(r.rows_affected()),
        Err(e) => {
            println!("[SQL执行错误] SQL: {} | 参数: {:?} | 错误: {}", sql, canshu, e);
            None
        }
    }
}

/// 执行写操作（INSERT/UPDATE/DELETE/DDL），返回影响行数 - 兼容旧版本（不支持 NULL）
pub async fn zhixing_jiuban(sql: &str, canshu: &[&str]) -> Option<u64> {
    let canshu_zhuanhuan: Vec<Option<&str>> = canshu.iter().map(|s| Some(*s)).collect();
    zhixing(sql, &canshu_zhuanhuan).await
}
