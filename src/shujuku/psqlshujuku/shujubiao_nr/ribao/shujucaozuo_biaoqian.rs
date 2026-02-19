use serde_json::Value;
use crate::gongju::jichugongju;
use crate::shujuku::psqlshujuku::psqlcaozuo;

#[allow(non_upper_case_globals)]
const biaoming: &str = "biaoqian";

/// 新增标签，返回自增ID
pub async fn xinzeng(leixingid: &str, zhi: &str) -> Option<String> {
    let shijian = jichugongju::huoqushijianchuo().to_string();
    let jieguo = psqlcaozuo::chaxun(
        &format!("INSERT INTO {} (leixingid, zhi, chuangjianshijian, gengxinshijian) VALUES ($1::BIGINT,$2,$3,$4) RETURNING id::TEXT", biaoming),
        &[leixingid, zhi, &shijian, &shijian],
    ).await?;
    jieguo.first().and_then(|v| v.get("id")?.as_str().map(String::from))
}

/// 根据ID删除标签
pub async fn shanchu(id: &str) -> Option<u64> {
    psqlcaozuo::zhixing(
        &format!("DELETE FROM {} WHERE id = $1::BIGINT", biaoming),
        &[id],
    ).await
}

/// 根据ID更新标签值
pub async fn gengxin(id: &str, zhi: &str) -> Option<u64> {
    let shijian = jichugongju::huoqushijianchuo().to_string();
    psqlcaozuo::zhixing(
        &format!("UPDATE {} SET zhi = $2, gengxinshijian = $3 WHERE id = $1::BIGINT", biaoming),
        &[id, zhi, &shijian],
    ).await
}

/// 根据ID查询单个标签
pub async fn chaxun_id(id: &str) -> Option<Value> {
    let jieguo = psqlcaozuo::chaxun(
        &format!("SELECT * FROM {} WHERE id = $1::BIGINT", biaoming),
        &[id],
    ).await?;
    jieguo.into_iter().next()
}

/// 根据类型ID查询标签列表
pub async fn chaxun_leixingid(leixingid: &str) -> Option<Vec<Value>> {
    psqlcaozuo::chaxun(
        &format!("SELECT * FROM {} WHERE leixingid = $1::BIGINT ORDER BY chuangjianshijian ASC", biaoming),
        &[leixingid],
    ).await
}

/// 根据类型ID和值查询标签
pub async fn chaxun_leixingid_zhi(leixingid: &str, zhi: &str) -> Option<Value> {
    let jieguo = psqlcaozuo::chaxun(
        &format!("SELECT * FROM {} WHERE leixingid = $1::BIGINT AND zhi = $2", biaoming),
        &[leixingid, zhi],
    ).await?;
    jieguo.into_iter().next()
}

/// 查询所有标签
pub async fn chaxun_quanbu() -> Option<Vec<Value>> {
    psqlcaozuo::chaxun(
        &format!("SELECT * FROM {} ORDER BY leixingid ASC, chuangjianshijian ASC", biaoming),
        &[],
    ).await
}

/// 检查标签值是否已存在（同类型下）
pub async fn zhicunzai(leixingid: &str, zhi: &str) -> bool {
    psqlcaozuo::chaxun(
        &format!("SELECT 1 FROM {} WHERE leixingid = $1::BIGINT AND zhi = $2 LIMIT 1", biaoming),
        &[leixingid, zhi],
    ).await
    .is_some_and(|jieguo| !jieguo.is_empty())
}
