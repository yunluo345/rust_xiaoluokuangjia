use serde_json::Value;
use crate::gongju::jichugongju;
use crate::shujuku::psqlshujuku::psqlcaozuo;

#[allow(non_upper_case_globals)]
const biaoming: &str = "yonghu";

/// 新增用户，返回自增ID
pub async fn xinzeng(zhanghao: &str, mima: &str, nicheng: &str, yonghuzuid: &str, beizhu: Option<&str>) -> Option<String> {
    let shijian = jichugongju::huoqushijianchuo().to_string();
    let beizhu_zhi = beizhu.unwrap_or("");
    let jieguo = psqlcaozuo::chaxun(
        &format!("INSERT INTO {} (zhanghao, mima, nicheng, yonghuzuid, beizhu, chuangjianshijian, gengxinshijian) VALUES ($1,$2,$3,$4,$5,$6,$7) RETURNING id::TEXT", biaoming),
        &[zhanghao, mima, nicheng, yonghuzuid, beizhu_zhi, &shijian, &shijian],
    ).await?;
    jieguo.first().and_then(|v| v.get("id")?.as_str().map(String::from))
}

/// 根据ID删除用户
pub async fn shanchu(id: &str) -> Option<u64> {
    psqlcaozuo::zhixing(
        &format!("DELETE FROM {} WHERE id = $1", biaoming),
        &[id],
    ).await
}

/// 根据ID更新用户信息
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

/// 根据ID查询单个用户
pub async fn chaxun_id(id: &str) -> Option<Value> {
    let jieguo = psqlcaozuo::chaxun(
        &format!("SELECT * FROM {} WHERE id = $1", biaoming),
        &[id],
    ).await?;
    jieguo.into_iter().next()
}

/// 根据账号查询用户
pub async fn chaxun_zhanghao(zhanghao: &str) -> Option<Value> {
    let jieguo = psqlcaozuo::chaxun(
        &format!("SELECT * FROM {} WHERE zhanghao = $1", biaoming),
        &[zhanghao],
    ).await?;
    jieguo.into_iter().next()
}

/// 查询所有用户（按创建时间升序）
pub async fn chaxun_quanbu() -> Option<Vec<Value>> {
    psqlcaozuo::chaxun(
        &format!("SELECT * FROM {} ORDER BY chuangjianshijian ASC", biaoming),
        &[],
    ).await
}

/// 根据用户组ID查询用户列表
pub async fn chaxun_yonghuzuid(yonghuzuid: &str) -> Option<Vec<Value>> {
    psqlcaozuo::chaxun(
        &format!("SELECT * FROM {} WHERE yonghuzuid = $1 ORDER BY chuangjianshijian ASC", biaoming),
        &[yonghuzuid],
    ).await
}

/// 封禁用户
pub async fn fengjin(id: &str, yuanyin: &str, jieshu: Option<&str>) -> Option<u64> {
    let shijian = jichugongju::huoqushijianchuo().to_string();
    let jieshu_zhi = jieshu.unwrap_or("");
    psqlcaozuo::zhixing(
        &format!("UPDATE {} SET fengjin = '1', fengjinyuanyin = $2, fengjinjieshu = $3, gengxinshijian = $4 WHERE id = $1", biaoming),
        &[id, yuanyin, jieshu_zhi, &shijian],
    ).await
}

/// 解封用户
pub async fn jiefeng(id: &str) -> Option<u64> {
    let shijian = jichugongju::huoqushijianchuo().to_string();
    psqlcaozuo::zhixing(
        &format!("UPDATE {} SET fengjin = '0', fengjinyuanyin = NULL, fengjinjieshu = NULL, gengxinshijian = $2 WHERE id = $1", biaoming),
        &[id, &shijian],
    ).await
}

/// 更新最后登录时间
pub async fn gengxindenglu(id: &str) -> Option<u64> {
    let shijian = jichugongju::huoqushijianchuo().to_string();
    psqlcaozuo::zhixing(
        &format!("UPDATE {} SET zuihoudenglu = $2, gengxinshijian = $2 WHERE id = $1", biaoming),
        &[id, &shijian],
    ).await
}

/// 检查账号是否已存在
pub async fn zhanghaocunzai(zhanghao: &str) -> bool {
    psqlcaozuo::chaxun(
        &format!("SELECT 1 FROM {} WHERE zhanghao = $1 LIMIT 1", biaoming),
        &[zhanghao],
    ).await
    .is_some_and(|jieguo| !jieguo.is_empty())
}

/// 分页查询用户
pub async fn chaxun_fenye(pianyi: &str, shuliang: &str) -> Option<Vec<Value>> {
    psqlcaozuo::chaxun(
        &format!("SELECT * FROM {} ORDER BY chuangjianshijian ASC LIMIT $1 OFFSET $2", biaoming),
        &[shuliang, pianyi],
    ).await
}

/// 查询用户总数
pub async fn chaxun_zongshu() -> Option<Value> {
    let jieguo = psqlcaozuo::chaxun(
        &format!("SELECT COUNT(*) as shuliang FROM {}", biaoming),
        &[],
    ).await?;
    jieguo.into_iter().next()
}
