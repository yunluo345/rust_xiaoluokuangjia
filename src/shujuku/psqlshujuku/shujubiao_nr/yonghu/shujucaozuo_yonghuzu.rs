use serde_json::Value;
use crate::gongju::jichugongju;
use crate::shujuku::psqlshujuku::psqlcaozuo;

#[allow(non_upper_case_globals)]
const biaoming: &str = "yonghuzu";

/// 新增用户组，返回自增ID
pub async fn xinzeng(mingcheng: &str, beizhu: Option<&str>) -> Option<String> {
    let shijian = jichugongju::huoqushijianchuo().to_string();
    let beizhu_zhi = beizhu.unwrap_or("");
    let jieguo = psqlcaozuo::chaxun(
        &format!("INSERT INTO {} (mingcheng, beizhu, chuangjianshijian, gengxinshijian) VALUES ($1,$2,$3,$4) RETURNING id::TEXT", biaoming),
        &[mingcheng, beizhu_zhi, &shijian, &shijian],
    ).await?;
    jieguo.first().and_then(|v| v.get("id")?.as_str().map(String::from))
}

/// 根据ID删除用户组
pub async fn shanchu(id: &str) -> Option<u64> {
    psqlcaozuo::zhixing(
        &format!("DELETE FROM {} WHERE id = $1", biaoming),
        &[id],
    ).await
}

/// 根据ID更新用户组信息
pub async fn gengxin(id: &str, ziduanlie: &[(&str, &str)]) -> Option<u64> {
    if ziduanlie.is_empty() {
        return None;
    }
    let shijian = jichugongju::huoqushijianchuo().to_string();
    let mut shezhi: Vec<String> = ziduanlie.iter().enumerate()
        .map(|(i, (ming, _))| format!("{} = ${}", ming, i + 2))
        .collect();
    shezhi.push(format!("gengxinshijian = ${}", ziduanlie.len() + 2));
    let sql = format!("UPDATE {} SET {} WHERE id = $1", biaoming, shezhi.join(", "));
    let mut canshu: Vec<&str> = vec![id];
    canshu.extend(ziduanlie.iter().map(|(_, zhi)| *zhi));
    canshu.push(&shijian);
    psqlcaozuo::zhixing(&sql, &canshu).await
}

/// 根据ID查询单个用户组
pub async fn chaxun_id(id: &str) -> Option<Value> {
    let jieguo = psqlcaozuo::chaxun(
        &format!("SELECT * FROM {} WHERE id = $1", biaoming),
        &[id],
    ).await?;
    jieguo.into_iter().next()
}

/// 查询所有用户组
pub async fn chaxun_quanbu() -> Option<Vec<Value>> {
    psqlcaozuo::chaxun(
        &format!("SELECT * FROM {} ORDER BY chuangjianshijian ASC", biaoming),
        &[],
    ).await
}

/// 设置默认用户组（先清除旧的，再设置新的）
pub async fn shezhimorenzhu(id: &str) -> Option<u64> {
    let shijian = jichugongju::huoqushijianchuo().to_string();
    psqlcaozuo::zhixing(
        &format!("UPDATE {} SET morenzhu = '0', gengxinshijian = $1", biaoming),
        &[&shijian],
    ).await?;
    psqlcaozuo::zhixing(
        &format!("UPDATE {} SET morenzhu = '1', gengxinshijian = $2 WHERE id = $1", biaoming),
        &[id, &shijian],
    ).await
}

/// 查询默认用户组
pub async fn chaxunmorenzhu() -> Option<Value> {
    let jieguo = psqlcaozuo::chaxun(
        &format!("SELECT * FROM {} WHERE morenzhu = '1' LIMIT 1", biaoming),
        &[],
    ).await?;
    jieguo.into_iter().next()
}

/// 更新禁用接口列表
pub async fn gengxinjinjiekou(id: &str, jinjiekou: &str) -> Option<u64> {
    let shijian = jichugongju::huoqushijianchuo().to_string();
    psqlcaozuo::zhixing(
        &format!("UPDATE {} SET jinjiekou = $2, gengxinshijian = $3 WHERE id = $1", biaoming),
        &[id, jinjiekou, &shijian],
    ).await
}

/// 检查组名称是否已存在
pub async fn mingchengcunzai(mingcheng: &str) -> bool {
    psqlcaozuo::chaxun(
        &format!("SELECT 1 FROM {} WHERE mingcheng = $1 LIMIT 1", biaoming),
        &[mingcheng],
    ).await
    .is_some_and(|jieguo| !jieguo.is_empty())
}

/// 查询该用户组下的用户数量
pub async fn yonghushuliang(id: &str) -> Option<Value> {
    let jieguo = psqlcaozuo::chaxun(
        "SELECT COUNT(*) as shuliang FROM yonghu WHERE yonghuzuid = $1",
        &[id],
    ).await?;
    jieguo.into_iter().next()
}
