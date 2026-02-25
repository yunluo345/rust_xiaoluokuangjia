use serde_json::Value;
use crate::gongju::jichugongju;
use crate::shujuku::psqlshujuku::psqlcaozuo;

#[allow(non_upper_case_globals)]
const biaoming: &str = "ribao_biaoqian";

/// 新增日报标签关联
pub async fn xinzeng(ribaoid: &str, biaoqianid: &str) -> Option<u64> {
    let shijian = jichugongju::huoqushijianchuo().to_string();
    psqlcaozuo::zhixing(
        &format!("INSERT INTO {} (ribaoid, biaoqianid, chuangjianshijian) VALUES ($1::BIGINT,$2::BIGINT,$3)", biaoming),
        &[ribaoid, biaoqianid, &shijian],
    ).await
}

/// 删除日报的所有标签关联
pub async fn shanchu_ribaoid(ribaoid: &str) -> Option<u64> {
    psqlcaozuo::zhixing(
        &format!("DELETE FROM {} WHERE ribaoid = $1::BIGINT", biaoming),
        &[ribaoid],
    ).await
}

/// 删除特定的日报标签关联
pub async fn shanchu_guanlian(ribaoid: &str, biaoqianid: &str) -> Option<u64> {
    psqlcaozuo::zhixing(
        &format!("DELETE FROM {} WHERE ribaoid = $1::BIGINT AND biaoqianid = $2::BIGINT", biaoming),
        &[ribaoid, biaoqianid],
    ).await
}

/// 查询日报的所有标签
pub async fn chaxun_ribaoid(ribaoid: &str) -> Option<Vec<Value>> {
    psqlcaozuo::chaxun(
        &format!("SELECT rb.ribaoid, rb.biaoqianid, b.zhi, b.leixingid FROM {} rb INNER JOIN biaoqian b ON rb.biaoqianid = b.id WHERE rb.ribaoid = $1::BIGINT", biaoming),
        &[ribaoid],
    ).await
}

/// 查询标签关联的所有日报
pub async fn chaxun_biaoqianid(biaoqianid: &str) -> Option<Vec<Value>> {
    psqlcaozuo::chaxun(
        &format!("SELECT r.* FROM ribao r INNER JOIN {} rb ON r.id = rb.ribaoid WHERE rb.biaoqianid = $1::BIGINT ORDER BY r.fabushijian DESC", biaoming),
        &[biaoqianid],
    ).await
}

/// 批量删除日报标签关联（按日报ID列表）
pub async fn piliang_shanchu_ribaoidlie(ribaoidlie: &[&str]) -> Option<u64> {
    jichugongju::piliang_shanchu_ziduan(biaoming, "ribaoid", ribaoidlie).await
}

/// 批量新增日报标签关联
pub async fn piliang_xinzeng(ribaoid: &str, biaoqianidlie: &[&str]) -> Option<u64> {
    if biaoqianidlie.is_empty() {
        return None;
    }
    let mut zongshu = 0u64;
    for biaoqianid in biaoqianidlie {
        if let Some(shu) = xinzeng(ribaoid, biaoqianid).await {
            zongshu += shu;
        }
    }
    Some(zongshu)
}

/// 检查关联是否存在
pub async fn guanliancunzai(ribaoid: &str, biaoqianid: &str) -> bool {
    psqlcaozuo::chaxun(
        &format!("SELECT 1 FROM {} WHERE ribaoid = $1::BIGINT AND biaoqianid = $2::BIGINT LIMIT 1", biaoming),
        &[ribaoid, biaoqianid],
    ).await
    .is_some_and(|jieguo| !jieguo.is_empty())
}

/// 按标签类型名称和值查询关联的日报
pub async fn chaxun_leixingmingcheng_zhi(leixingmingcheng: &str, zhi: &str) -> Option<Vec<Value>> {
    psqlcaozuo::chaxun(
        "SELECT r.* FROM ribao r INNER JOIN ribao_biaoqian rb ON r.id = rb.ribaoid INNER JOIN biaoqian b ON rb.biaoqianid = b.id INNER JOIN biaoqianleixing l ON b.leixingid = l.id WHERE l.mingcheng = $1 AND b.zhi = $2 ORDER BY r.fabushijian DESC",
        &[leixingmingcheng, zhi],
    ).await
}

/// 查询日报的所有标签（包含类型信息）
pub async fn chaxun_ribaoid_daixinxi(ribaoid: &str) -> Option<Vec<Value>> {
    psqlcaozuo::chaxun(
        "SELECT b.id AS biaoqianid, b.zhi, b.leixingid, l.mingcheng AS leixingmingcheng FROM ribao_biaoqian rb INNER JOIN biaoqian b ON rb.biaoqianid = b.id INNER JOIN biaoqianleixing l ON b.leixingid = l.id WHERE rb.ribaoid = $1::BIGINT",
        &[ribaoid],
    ).await
}

/// 按标签ID查询相关日报的其他标签（按类型筛选）
pub async fn chaxun_xiangguanbiaoqian(biaoqianid: &str, leixingmingcheng: &str) -> Option<Vec<Value>> {
    psqlcaozuo::chaxun(
        "SELECT DISTINCT b.id, b.zhi, b.leixingid, l.mingcheng AS leixingmingcheng FROM ribao_biaoqian rb1 INNER JOIN ribao_biaoqian rb2 ON rb1.ribaoid = rb2.ribaoid INNER JOIN biaoqian b ON rb2.biaoqianid = b.id INNER JOIN biaoqianleixing l ON b.leixingid = l.id WHERE rb1.biaoqianid = $1::BIGINT AND l.mingcheng = $2",
        &[biaoqianid, leixingmingcheng],
    ).await
}

/// 查询全量图谱数据：所有标签节点及共现边
pub async fn chaxun_tupu_quanbu() -> Option<Value> {
    let jiedian = psqlcaozuo::chaxun(
        "SELECT b.id, b.zhi, b.leixingid, l.mingcheng AS leixingmingcheng FROM biaoqian b INNER JOIN biaoqianleixing l ON b.leixingid = l.id ORDER BY l.mingcheng, b.zhi",
        &[],
    ).await?;
    let bian = psqlcaozuo::chaxun(
        "SELECT b1.id::TEXT AS yuan, b2.id::TEXT AS mubiao, COUNT(DISTINCT rb1.ribaoid)::TEXT AS quanzhong \
         FROM ribao_biaoqian rb1 \
         JOIN ribao_biaoqian rb2 ON rb1.ribaoid = rb2.ribaoid AND rb1.biaoqianid < rb2.biaoqianid \
         JOIN biaoqian b1 ON rb1.biaoqianid = b1.id \
         JOIN biaoqian b2 ON rb2.biaoqianid = b2.id \
         GROUP BY b1.id, b2.id",
        &[],
    ).await.unwrap_or_default();
    Some(serde_json::json!({"jiedian": jiedian, "bian": bian}))
}

/// 以某标签为中心查询子图（1层关联）
pub async fn chaxun_tupu_biaoqianid(biaoqianid: &str) -> Option<Value> {
    let guanlianid = psqlcaozuo::chaxun(
        "SELECT DISTINCT b2.id, b2.zhi, b2.leixingid, l.mingcheng AS leixingmingcheng \
         FROM ribao_biaoqian rb1 \
         JOIN ribao_biaoqian rb2 ON rb1.ribaoid = rb2.ribaoid AND rb1.biaoqianid != rb2.biaoqianid \
         JOIN biaoqian b2 ON rb2.biaoqianid = b2.id \
         JOIN biaoqianleixing l ON b2.leixingid = l.id \
         WHERE rb1.biaoqianid = $1::BIGINT",
        &[biaoqianid],
    ).await?;
    let zhongxin = psqlcaozuo::chaxun(
        "SELECT b.id, b.zhi, b.leixingid, l.mingcheng AS leixingmingcheng FROM biaoqian b INNER JOIN biaoqianleixing l ON b.leixingid = l.id WHERE b.id = $1::BIGINT",
        &[biaoqianid],
    ).await.unwrap_or_default();
    let mut jiedian = zhongxin;
    jiedian.extend(guanlianid);
    let idlie: Vec<String> = jiedian.iter()
        .filter_map(|j| j.get("id").and_then(|v| v.as_i64().map(|n| n.to_string()).or_else(|| v.as_str().map(String::from))))
        .collect();
    let tiaojian = idlie.iter().enumerate()
        .map(|(i, _)| format!("${}", i + 1))
        .collect::<Vec<_>>()
        .join(",");
    let sql = format!(
        "SELECT b1.id::TEXT AS yuan, b2.id::TEXT AS mubiao, COUNT(DISTINCT rb1.ribaoid)::TEXT AS quanzhong \
         FROM ribao_biaoqian rb1 \
         JOIN ribao_biaoqian rb2 ON rb1.ribaoid = rb2.ribaoid AND rb1.biaoqianid < rb2.biaoqianid \
         JOIN biaoqian b1 ON rb1.biaoqianid = b1.id \
         JOIN biaoqian b2 ON rb2.biaoqianid = b2.id \
         WHERE b1.id::TEXT IN ({t}) AND b2.id::TEXT IN ({t}) \
         GROUP BY b1.id, b2.id",
        t = tiaojian
    );
    let canshu: Vec<&str> = idlie.iter().map(String::as_str).collect();
    let bian = psqlcaozuo::chaxun(&sql, &canshu).await.unwrap_or_default();
    Some(serde_json::json!({"jiedian": jiedian, "bian": bian}))
}

/// 按标签类型筛选图谱
pub async fn chaxun_tupu_leixingmingcheng(leixingmingcheng: &str) -> Option<Value> {
    let jiedian = psqlcaozuo::chaxun(
        "SELECT b.id, b.zhi, b.leixingid, l.mingcheng AS leixingmingcheng \
         FROM biaoqian b INNER JOIN biaoqianleixing l ON b.leixingid = l.id \
         WHERE l.mingcheng = $1 ORDER BY b.zhi",
        &[leixingmingcheng],
    ).await?;
    let bian = psqlcaozuo::chaxun(
        "SELECT b1.id::TEXT AS yuan, b2.id::TEXT AS mubiao, COUNT(DISTINCT rb1.ribaoid)::TEXT AS quanzhong \
         FROM ribao_biaoqian rb1 \
         JOIN ribao_biaoqian rb2 ON rb1.ribaoid = rb2.ribaoid AND rb1.biaoqianid < rb2.biaoqianid \
         JOIN biaoqian b1 ON rb1.biaoqianid = b1.id \
         JOIN biaoqian b2 ON rb2.biaoqianid = b2.id \
         JOIN biaoqianleixing l1 ON b1.leixingid = l1.id \
         JOIN biaoqianleixing l2 ON b2.leixingid = l2.id \
         WHERE l1.mingcheng = $1 OR l2.mingcheng = $1 \
         GROUP BY b1.id, b2.id",
        &[leixingmingcheng],
    ).await.unwrap_or_default();
    Some(serde_json::json!({"jiedian": jiedian, "bian": bian}))
}
