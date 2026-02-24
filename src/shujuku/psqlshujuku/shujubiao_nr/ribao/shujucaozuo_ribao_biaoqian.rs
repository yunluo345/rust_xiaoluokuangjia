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
