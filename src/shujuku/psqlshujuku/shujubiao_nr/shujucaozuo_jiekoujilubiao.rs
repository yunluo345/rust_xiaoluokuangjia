use serde_json::Value;
use crate::shujuku::psqlshujuku::psqlcaozuo;

#[allow(non_upper_case_globals)]
const biaoming: &str = "jiekoujilubiao";

/// 查询不允许普通用户访问的接口路径列表
pub async fn chaxun_jinyongputong() -> Option<Vec<String>> {
    let jieguo = psqlcaozuo::chaxun(
        &format!("SELECT lujing FROM {} WHERE yunxuputong = '0'", biaoming),
        &[],
    ).await?;
    Some(jieguo.iter().filter_map(|v| v.get("lujing")?.as_str().map(String::from)).collect())
}

/// 查询所有接口记录
pub async fn chaxun_quanbu() -> Option<Vec<Value>> {
    psqlcaozuo::chaxun(
        &format!("SELECT * FROM {} ORDER BY lujing ASC", biaoming),
        &[],
    ).await
}
