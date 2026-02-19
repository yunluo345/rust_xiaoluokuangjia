use serde_json::Value;
use crate::gongju::jichugongju;
use crate::shujuku::psqlshujuku::psqlcaozuo;

#[allow(non_upper_case_globals)]
const biaoming: &str = "biaoqianleixing";

/// 新增标签类型，返回自增ID
pub async fn xinzeng(mingcheng: &str) -> Option<String> {
    let shijian = jichugongju::huoqushijianchuo().to_string();
    let jieguo = psqlcaozuo::chaxun(
        &format!("INSERT INTO {} (mingcheng, chuangjianshijian, gengxinshijian) VALUES ($1,$2,$3) RETURNING id::TEXT", biaoming),
        &[mingcheng, &shijian, &shijian],
    ).await?;
    jieguo.first().and_then(|v| v.get("id")?.as_str().map(String::from))
}

/// 根据ID删除标签类型
pub async fn shanchu(id: &str) -> Option<u64> {
    psqlcaozuo::zhixing(
        &format!("DELETE FROM {} WHERE id = $1::BIGINT", biaoming),
        &[id],
    ).await
}

/// 根据ID更新标签类型
pub async fn gengxin(id: &str, mingcheng: &str) -> Option<u64> {
    let shijian = jichugongju::huoqushijianchuo().to_string();
    psqlcaozuo::zhixing(
        &format!("UPDATE {} SET mingcheng = $2, gengxinshijian = $3 WHERE id = $1::BIGINT", biaoming),
        &[id, mingcheng, &shijian],
    ).await
}

/// 根据ID查询单个标签类型
pub async fn chaxun_id(id: &str) -> Option<Value> {
    let jieguo = psqlcaozuo::chaxun(
        &format!("SELECT * FROM {} WHERE id = $1::BIGINT", biaoming),
        &[id],
    ).await?;
    jieguo.into_iter().next()
}

/// 根据名称查询标签类型
pub async fn chaxun_mingcheng(mingcheng: &str) -> Option<Value> {
    let jieguo = psqlcaozuo::chaxun(
        &format!("SELECT * FROM {} WHERE mingcheng = $1", biaoming),
        &[mingcheng],
    ).await?;
    jieguo.into_iter().next()
}

/// 查询所有标签类型
pub async fn chaxun_quanbu() -> Option<Vec<Value>> {
    psqlcaozuo::chaxun(
        &format!("SELECT * FROM {} ORDER BY chuangjianshijian ASC", biaoming),
        &[],
    ).await
}

/// 检查类型名称是否已存在
pub async fn mingchengcunzai(mingcheng: &str) -> bool {
    psqlcaozuo::chaxun(
        &format!("SELECT 1 FROM {} WHERE mingcheng = $1 LIMIT 1", biaoming),
        &[mingcheng],
    ).await
    .is_some_and(|jieguo| !jieguo.is_empty())
}
