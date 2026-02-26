use serde_json::Value;
use crate::gongju::jichugongju;
use crate::shujuku::psqlshujuku::psqlcaozuo;

#[allow(non_upper_case_globals)]
const biaoming: &str = "ribao";

/// 新增日报，返回自增ID
pub async fn xinzeng(yonghuid: &str, neirong: &str, fabushijian: &str) -> Option<String> {
    let shijian = jichugongju::huoqushijianchuo().to_string();
    let jieguo = psqlcaozuo::chaxun(
        &format!("INSERT INTO {} (yonghuid, neirong, fabushijian, zhaiyao, kuozhan, chuangjianshijian, gengxinshijian) VALUES ($1::BIGINT,$2,$3,NULL,NULL,$4,$5) RETURNING id::TEXT", biaoming),
        &[yonghuid, neirong, fabushijian, &shijian, &shijian],
    ).await?;
    jieguo.first().and_then(|v| v.get("id")?.as_str().map(String::from))
}

/// 根据ID删除日报
pub async fn shanchu(id: &str) -> Option<u64> {
    psqlcaozuo::zhixing(
        &format!("DELETE FROM {} WHERE id = $1::BIGINT", biaoming),
        &[id],
    ).await
}

/// 批量删除日报
pub async fn piliang_shanchu(idlie: &[&str]) -> Option<u64> {
    jichugongju::piliang_shanchu(biaoming, idlie).await
}

/// 根据ID更新日报信息
pub async fn gengxin(id: &str, ziduanlie: &[(&str, &str)]) -> Option<u64> {
    if ziduanlie.is_empty() {
        return None;
    }
    let shijian = jichugongju::huoqushijianchuo().to_string();
    let mut shezhi: Vec<String> = ziduanlie.iter().enumerate()
        .map(|(i, (ming, _))| format!("{} = ${}", ming, i + 2))
        .collect();
    shezhi.push(format!("gengxinshijian = ${}", ziduanlie.len() + 2));
    let sql = format!("UPDATE {} SET {} WHERE id = $1::BIGINT", biaoming, shezhi.join(", "));
    let mut canshu: Vec<&str> = vec![id];
    canshu.extend(ziduanlie.iter().map(|(_, zhi)| *zhi));
    canshu.push(&shijian);
    psqlcaozuo::zhixing(&sql, &canshu).await
}

/// 清空日报AI衍生结果（摘要、扩展字段）
pub async fn qingkong_aishengcheng(id: &str) -> Option<u64> {
    let shijian = jichugongju::huoqushijianchuo().to_string();
    psqlcaozuo::zhixing(
        &format!("UPDATE {} SET zhaiyao = NULL, kuozhan = NULL, gengxinshijian = $2 WHERE id = $1::BIGINT", biaoming),
        &[id, &shijian],
    ).await
}

/// 根据ID查询单个日报
pub async fn chaxun_id(id: &str) -> Option<Value> {
    let jieguo = psqlcaozuo::chaxun(
        &format!("SELECT * FROM {} WHERE id = $1::BIGINT", biaoming),
        &[id],
    ).await?;
    jieguo.into_iter().next()
}

/// 根据用户ID查询日报列表
pub async fn chaxun_yonghuid(yonghuid: &str) -> Option<Vec<Value>> {
    psqlcaozuo::chaxun(
        &format!("SELECT * FROM {} WHERE yonghuid = $1::BIGINT ORDER BY fabushijian DESC", biaoming),
        &[yonghuid],
    ).await
}

/// 查询所有日报
pub async fn chaxun_quanbu() -> Option<Vec<Value>> {
    psqlcaozuo::chaxun(
        &format!("SELECT * FROM {} ORDER BY fabushijian DESC", biaoming),
        &[],
    ).await
}

/// 分页查询日报
pub async fn chaxun_fenye(yeshu: i64, meiyetiaoshu: i64) -> Option<Vec<Value>> {
    let (tiaoshu, pianyi) = jichugongju::jisuanfenye(yeshu, meiyetiaoshu);
    psqlcaozuo::chaxun(
        &format!("SELECT r.*, y.nicheng as fabuzhemingcheng, y.zhanghao as fabuzhezhanghao FROM {} r LEFT JOIN yonghu y ON r.yonghuid = y.id ORDER BY r.fabushijian DESC LIMIT $1::BIGINT OFFSET $2::BIGINT", biaoming),
        &[&tiaoshu, &pianyi],
    ).await
}

/// 根据用户ID分页查询日报
pub async fn chaxun_yonghuid_fenye(yonghuid: &str, yeshu: i64, meiyetiaoshu: i64) -> Option<Vec<Value>> {
    let (tiaoshu, pianyi) = jichugongju::jisuanfenye(yeshu, meiyetiaoshu);
    psqlcaozuo::chaxun(
        &format!("SELECT r.*, y.nicheng as fabuzhemingcheng, y.zhanghao as fabuzhezhanghao FROM {} r LEFT JOIN yonghu y ON r.yonghuid = y.id WHERE r.yonghuid = $1::BIGINT ORDER BY r.fabushijian DESC LIMIT $2::BIGINT OFFSET $3::BIGINT", biaoming),
        &[yonghuid, &tiaoshu, &pianyi],
    ).await
}

/// 统计日报总数
pub async fn tongji_zongshu() -> Option<i64> {
    let jieguo = psqlcaozuo::chaxun(
        &format!("SELECT COUNT(*)::TEXT as count FROM {}", biaoming),
        &[],
    ).await?;
    jieguo.first()?.get("count")?.as_str()?.parse().ok()
}

/// 统计用户日报总数
pub async fn tongji_yonghuid_zongshu(yonghuid: &str) -> Option<i64> {
    let jieguo = psqlcaozuo::chaxun(
        &format!("SELECT COUNT(*)::TEXT as count FROM {} WHERE yonghuid = $1::BIGINT", biaoming),
        &[yonghuid],
    ).await?;
    jieguo.first()?.get("count")?.as_str()?.parse().ok()
}

/// 根据关键词分页查询日报
pub async fn chaxun_guanjianci_fenye(guanjianci: &str, yeshu: i64, meiyetiaoshu: i64) -> Option<Vec<Value>> {
    let (tiaoshu, pianyi) = jichugongju::jisuanfenye(yeshu, meiyetiaoshu);
    let mohu = format!("%{}%", guanjianci);
    psqlcaozuo::chaxun(
        &format!("SELECT * FROM {} WHERE neirong LIKE $1 ORDER BY fabushijian DESC LIMIT $2::BIGINT OFFSET $3::BIGINT", biaoming),
        &[&mohu, &tiaoshu, &pianyi],
    ).await
}

/// 统计关键词日报总数
pub async fn tongji_guanjianci_zongshu(guanjianci: &str) -> Option<i64> {
    let mohu = format!("%{}%", guanjianci);
    let jieguo = psqlcaozuo::chaxun(
        &format!("SELECT COUNT(*)::TEXT as count FROM {} WHERE neirong LIKE $1", biaoming),
        &[&mohu],
    ).await?;
    jieguo.first()?.get("count")?.as_str()?.parse().ok()
}
