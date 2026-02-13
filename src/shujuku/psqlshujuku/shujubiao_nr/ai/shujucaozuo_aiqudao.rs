use serde_json::Value;
use rand::prelude::*;
use crate::gongju::jichugongju;
use crate::shujuku::psqlshujuku::psqlcaozuo;

#[allow(non_upper_case_globals)]
const biaoming: &str = "aiqudao";

#[allow(non_upper_case_globals)]
pub const yunxuleixing: &[&str] = &["openai", "xiangliang"];

/// 新增AI渠道，返回自增ID
pub async fn xinzeng(mingcheng: &str, leixing: &str, jiekoudizhi: &str, miyao: &str, moxing: &str, wendu: &str, beizhu: Option<&str>, zuidatoken: Option<&str>) -> Option<String> {
    let shijian = jichugongju::huoqushijianchuo().to_string();
    let beizhu_zhi = beizhu.unwrap_or("");
    let zuidatoken_zhi = zuidatoken.unwrap_or("0");
    let jieguo = psqlcaozuo::chaxun(
        &format!("INSERT INTO {} (mingcheng, leixing, jiekoudizhi, miyao, moxing, wendu, zhuangtai, youxianji, beizhu, zuidatoken, chuangjianshijian, gengxinshijian) VALUES ($1,$2,$3,$4,$5,$6,$7,$8::INTEGER,$9,$10::INTEGER,$11,$12) RETURNING id::TEXT", biaoming),
        &[mingcheng, leixing, jiekoudizhi, miyao, moxing, wendu, "1", "0", beizhu_zhi, zuidatoken_zhi, &shijian, &shijian],
    ).await?;
    jieguo.first().and_then(|v| v.get("id")?.as_str().map(String::from))
}

/// 根据ID删除渠道
pub async fn shanchu(id: &str) -> Option<u64> {
    psqlcaozuo::zhixing(
        &format!("DELETE FROM {} WHERE id = $1::BIGINT", biaoming),
        &[id],
    ).await
}

/// 根据ID更新渠道信息（仅更新传入的非None字段）
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

/// 根据ID查询单个渠道
pub async fn chaxun_id(id: &str) -> Option<Value> {
    let jieguo = psqlcaozuo::chaxun(
        &format!("SELECT * FROM {} WHERE id = $1::BIGINT", biaoming),
        &[id],
    ).await?;
    jieguo.into_iter().next()
}

/// 查询所有渠道（按优先级降序）
pub async fn chaxun_quanbu() -> Option<Vec<Value>> {
    psqlcaozuo::chaxun(
        &format!("SELECT * FROM {} ORDER BY youxianji DESC, chuangjianshijian DESC", biaoming),
        &[],
    ).await
}

/// 查询所有启用的渠道（按优先级降序）
pub async fn chaxun_qiyong() -> Option<Vec<Value>> {
    psqlcaozuo::chaxun(
        &format!("SELECT * FROM {} WHERE zhuangtai = '1' ORDER BY youxianji DESC", biaoming),
        &[],
    ).await
}

/// 根据渠道类型查询
pub async fn chaxun_leixing(leixing: &str) -> Option<Vec<Value>> {
    psqlcaozuo::chaxun(
        &format!("SELECT * FROM {} WHERE leixing = $1 AND zhuangtai = '1' ORDER BY COALESCE(youxianji, 0) DESC", biaoming),
        &[leixing],
    ).await
}

/// 切换渠道启用/禁用状态
pub async fn qiehuanzhuangtai(id: &str) -> Option<u64> {
    let shijian = jichugongju::huoqushijianchuo().to_string();
    psqlcaozuo::zhixing(
        &format!("UPDATE {} SET zhuangtai = CASE WHEN zhuangtai = '1' THEN '0' ELSE '1' END, gengxinshijian = $2 WHERE id = $1::BIGINT", biaoming),
        &[id, &shijian],
    ).await
}

/// 更新渠道优先级
pub async fn gengxinyouxianji(id: &str, youxianji: &str) -> Option<u64> {
    let shijian = jichugongju::huoqushijianchuo().to_string();
    psqlcaozuo::zhixing(
        &format!("UPDATE {} SET youxianji = $2, gengxinshijian = $3 WHERE id = $1::BIGINT", biaoming),
        &[id, youxianji, &shijian],
    ).await
}

/// 检查渠道名称是否已存在
pub async fn mingchengcunzai(mingcheng: &str) -> bool {
    psqlcaozuo::chaxun(
        &format!("SELECT 1 FROM {} WHERE mingcheng = $1 LIMIT 1", biaoming),
        &[mingcheng],
    ).await
    .is_some_and(|jieguo| !jieguo.is_empty())
}

/// 统计渠道总数
pub async fn tongjishuliang() -> Option<Value> {
    let jieguo = psqlcaozuo::chaxun(
        &format!("SELECT COUNT(*) as shuliang FROM {}", biaoming),
        &[],
    ).await?;
    jieguo.into_iter().next()
}

pub fn leixingyunxu(leixing: &str) -> bool {
    yunxuleixing.contains(&leixing)
}

/// 按类型轮训选取渠道：启用状态、优先级最高、同优先级随机
pub async fn lunxun(leixing: &str) -> Option<Value> {
    let liebie = chaxun_leixing(leixing).await?;
    if liebie.is_empty() {
        return None;
    }
    let zuigao = liebie[0].get("youxianji").and_then(quzhengshu).unwrap_or(0);
    let houxuanlie: Vec<&Value> = liebie.iter()
        .take_while(|v| v.get("youxianji").and_then(quzhengshu).unwrap_or(0) == zuigao)
        .collect();
    houxuanlie.choose(&mut rand::rng()).map(|v| (*v).clone())
}

fn quzhengshu(v: &Value) -> Option<i64> {
    v.as_i64().or_else(|| v.as_str()?.parse().ok())
}
