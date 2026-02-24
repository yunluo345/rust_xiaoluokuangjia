use rand::prelude::*;
use serde_json::Value;
use crate::gongju::jichugongju;
use crate::gongju::ai::aitongyonggongju;
use crate::shujuku::psqlshujuku::psqlcaozuo;

#[allow(non_upper_case_globals)]
const biaoming: &str = "aiqudao";

/// 新增AI渠道，返回自增ID
pub async fn xinzeng(mingcheng: &str, leixing: &str, jiekoudizhi: &str, miyao: &str, moxing: &str, wendu: &str, zuida_token: &str, beizhu: Option<&str>) -> Option<String> {
    let shijian = jichugongju::huoqushijianchuo().to_string();
    let beizhu_zhi = beizhu.unwrap_or("");
    let jieguo = psqlcaozuo::chaxun(
        &format!("INSERT INTO {} (mingcheng, leixing, jiekoudizhi, miyao, moxing, wendu, zuida_token, zhuangtai, youxianji, beizhu, chuangjianshijian, gengxinshijian) VALUES ($1,$2,$3,$4,$5,$6,CAST($7 AS INTEGER),$8,CAST($9 AS INTEGER),$10,$11,$12) RETURNING id::TEXT", biaoming),
        &[mingcheng, leixing, jiekoudizhi, miyao, moxing, wendu, zuida_token, "1", "0", beizhu_zhi, &shijian, &shijian],
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

/// 批量删除渠道
pub async fn piliang_shanchu(idlie: &[&str]) -> Option<u64> {
    jichugongju::piliang_shanchu(biaoming, idlie).await
}

/// 根据ID更新渠道信息（仅更新传入的非None字段）
pub async fn gengxin(id: &str, ziduanlie: &[(&str, &str)]) -> Option<u64> {
    if ziduanlie.is_empty() {
        return None;
    }
    let shijian = jichugongju::huoqushijianchuo().to_string();
    let zhengshuziduan = ["zuida_token", "youxianji"];
    let mut shezhi: Vec<String> = ziduanlie.iter().enumerate()
        .map(|(i, (ming, _))| {
            let weizhi = format!("${}", i + 2);
            if zhengshuziduan.contains(ming) {
                format!("{} = CAST({} AS INTEGER)", ming, weizhi)
            } else {
                format!("{} = {}", ming, weizhi)
            }
        })
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

/// 查询所有渠道（按优先级升序）
pub async fn chaxun_quanbu() -> Option<Vec<Value>> {
    psqlcaozuo::chaxun(
        &format!("SELECT * FROM {} ORDER BY youxianji ASC, chuangjianshijian DESC", biaoming),
        &[],
    ).await
}

/// 查询所有启用的渠道（按优先级升序）
pub async fn chaxun_qiyong() -> Option<Vec<Value>> {
    psqlcaozuo::chaxun(
        &format!("SELECT * FROM {} WHERE zhuangtai = '1' ORDER BY youxianji ASC", biaoming),
        &[],
    ).await
}

/// 根据渠道类型查询
pub async fn chaxun_leixing(leixing: &str) -> Option<Vec<Value>> {
    psqlcaozuo::chaxun(
        &format!("SELECT * FROM {} WHERE leixing = $1 AND zhuangtai = '1' ORDER BY youxianji ASC", biaoming),
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
        &format!("UPDATE {} SET youxianji = CAST($2 AS INTEGER), gengxinshijian = $3 WHERE id = $1::BIGINT", biaoming),
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

/// 根据类型按优先级随机获取一个启用的渠道
pub async fn suiji_huoqu_qudao(leixing: &str) -> Option<Value> {
    let liebiao = chaxun_leixing(leixing).await?;
    let zuigao = quzhengshuzhi(liebiao.first()?, "youxianji")?;
    let houxuan: Vec<&Value> = liebiao.iter()
        .filter(|v| quzhengshuzhi(v, "youxianji") == Some(zuigao))
        .collect();
    let mut qudao = houxuan.choose(&mut rand::rng()).map(|v| (*v).clone())?;
    if let Some(yuandizhi) = qudao.get("jiekoudizhi").and_then(|v| v.as_str()) {
        if let Some(buquandizhi) = aitongyonggongju::buquan_wangguandizhi(leixing, yuandizhi) {
            qudao.as_object_mut()?.insert("jiekoudizhi".to_string(), Value::String(buquandizhi));
        }
    }
    Some(qudao)
}

/// 从Value中提取整数值，兼容字符串和数字类型
fn quzhengshuzhi(zhi: &Value, jianming: &str) -> Option<i64> {
    zhi.get(jianming).and_then(|v| v.as_i64().or_else(|| v.as_str()?.parse().ok()))
}

/// 统计渠道总数
pub async fn tongjishuliang() -> Option<Value> {
    let jieguo = psqlcaozuo::chaxun(
        &format!("SELECT COUNT(*) as shuliang FROM {}", biaoming),
        &[],
    ).await?;
    jieguo.into_iter().next()
}
