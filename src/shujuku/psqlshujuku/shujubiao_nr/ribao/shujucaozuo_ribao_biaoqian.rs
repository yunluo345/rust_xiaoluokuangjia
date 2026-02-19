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
        &format!("SELECT b.* FROM biaoqian b INNER JOIN {} rb ON b.id = rb.biaoqianid WHERE rb.ribaoid = $1::BIGINT", biaoming),
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
