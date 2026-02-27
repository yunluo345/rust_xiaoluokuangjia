use std::collections::HashMap;
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

// ========== 跨日报分析聚合查询 ==========

/// 按标签类型聚合：查询某类型下所有标签值 + 关联日报数
pub async fn juhe_biaoqian_zhi_anleixing(leixingmingcheng: &str) -> Option<Vec<Value>> {
    psqlcaozuo::chaxun(
        "SELECT b.zhi, COUNT(DISTINCT rb.ribaoid)::TEXT AS ribao_shu \
         FROM biaoqian b \
         INNER JOIN biaoqianleixing l ON b.leixingid = l.id \
         INNER JOIN ribao_biaoqian rb ON b.id = rb.biaoqianid \
         WHERE l.mingcheng = $1 \
         GROUP BY b.zhi \
         ORDER BY COUNT(DISTINCT rb.ribaoid) DESC",
        &[leixingmingcheng],
    ).await
}

/// 按项目/客户名称聚合交流内容（查找共同关联日报的「交流内容」标签，按时间排序）
pub async fn juhe_jiaoliuneirong_anshiti(shiti_leixing: &str, shiti_mingcheng: &str) -> Option<Vec<Value>> {
    psqlcaozuo::chaxun(
        "SELECT b_jl.zhi AS jiaoliu_neirong, r.fabushijian, r.id AS ribaoid \
         FROM ribao r \
         INNER JOIN ribao_biaoqian rb1 ON r.id = rb1.ribaoid \
         INNER JOIN biaoqian b1 ON rb1.biaoqianid = b1.id \
         INNER JOIN biaoqianleixing l1 ON b1.leixingid = l1.id \
         INNER JOIN ribao_biaoqian rb2 ON r.id = rb2.ribaoid \
         INNER JOIN biaoqian b_jl ON rb2.biaoqianid = b_jl.id \
         INNER JOIN biaoqianleixing l_jl ON b_jl.leixingid = l_jl.id \
         WHERE l1.mingcheng = $1 AND b1.zhi = $2 AND l_jl.mingcheng = '交流内容' \
         ORDER BY r.fabushijian ASC",
        &[shiti_leixing, shiti_mingcheng],
    ).await
}

/// 查询某个项目/客户关联的所有标签（分类聚合）
pub async fn juhe_shiti_biaoqian(shiti_leixing: &str, shiti_mingcheng: &str) -> Option<Vec<Value>> {
    psqlcaozuo::chaxun(
        "SELECT l2.mingcheng AS leixingmingcheng, b2.zhi, COUNT(DISTINCT rb2.ribaoid)::TEXT AS cishu \
         FROM ribao_biaoqian rb1 \
         INNER JOIN biaoqian b1 ON rb1.biaoqianid = b1.id \
         INNER JOIN biaoqianleixing l1 ON b1.leixingid = l1.id \
         INNER JOIN ribao_biaoqian rb2 ON rb1.ribaoid = rb2.ribaoid \
         INNER JOIN biaoqian b2 ON rb2.biaoqianid = b2.id \
         INNER JOIN biaoqianleixing l2 ON b2.leixingid = l2.id \
         WHERE l1.mingcheng = $1 AND b1.zhi = $2 AND l2.mingcheng != $1 \
         GROUP BY l2.mingcheng, b2.zhi \
         ORDER BY l2.mingcheng, COUNT(DISTINCT rb2.ribaoid) DESC",
        &[shiti_leixing, shiti_mingcheng],
    ).await
}

// ========== 图谱核心辅助函数 ==========

/// 查询图谱节点（标签 + 类型名称）
/// tiaojian: WHERE 片段（不含 WHERE），为空则无过滤
/// paixu: ORDER BY 片段（不含 ORDER BY），为空则不排序
async fn chaxun_tupu_jiedian(tiaojian: &str, canshu: &[&str], paixu: &str) -> Option<Vec<Value>> {
    let where_zi = if tiaojian.is_empty() { String::new() } else { format!(" WHERE {}", tiaojian) };
    let order_zi = if paixu.is_empty() { String::new() } else { format!(" ORDER BY {}", paixu) };
    psqlcaozuo::chaxun(
        &format!(
            "SELECT b.id, b.zhi, b.leixingid, l.mingcheng AS leixingmingcheng \
             FROM biaoqian b INNER JOIN biaoqianleixing l ON b.leixingid = l.id{}{}",
            where_zi, order_zi
        ),
        canshu,
    ).await
}

/// 查询图谱边（共现关系）
/// ewai_lianjie: 额外 JOIN 子句（含前导空格）
/// tiaojian: WHERE 片段（不含 WHERE），为空则无过滤
async fn chaxun_tupu_bian(ewai_lianjie: &str, tiaojian: &str, canshu: &[&str]) -> Vec<Value> {
    let where_zi = if tiaojian.is_empty() { String::new() } else { format!(" WHERE {}", tiaojian) };
    let sql = format!(
        "SELECT b1.id::TEXT AS yuan, b2.id::TEXT AS mubiao, COUNT(DISTINCT rb1.ribaoid)::TEXT AS quanzhong \
         FROM ribao_biaoqian rb1 \
         JOIN ribao_biaoqian rb2 ON rb1.ribaoid = rb2.ribaoid AND rb1.biaoqianid < rb2.biaoqianid \
         JOIN biaoqian b1 ON rb1.biaoqianid = b1.id \
         JOIN biaoqian b2 ON rb2.biaoqianid = b2.id{}{} \
         GROUP BY b1.id, b2.id",
        ewai_lianjie, where_zi
    );
    psqlcaozuo::chaxun(&sql, canshu).await.unwrap_or_default()
}

/// 从节点列表中提取 ID，查询这些节点之间的共现边
async fn chaxun_tupu_bian_anzifanwei(jiedianlie: &[Value]) -> Vec<Value> {
    let idlie: Vec<String> = jiedianlie.iter()
        .filter_map(|j| j.get("id").and_then(|v| v.as_i64().map(|n| n.to_string()).or_else(|| v.as_str().map(String::from))))
        .collect();
    if idlie.is_empty() {
        return Vec::new();
    }
    let zhanwei = idlie.iter().enumerate()
        .map(|(i, _)| format!("${}", i + 1))
        .collect::<Vec<_>>()
        .join(",");
    let tiaojian = format!("b1.id::TEXT IN ({z}) AND b2.id::TEXT IN ({z})", z = zhanwei);
    let canshu: Vec<&str> = idlie.iter().map(String::as_str).collect();
    chaxun_tupu_bian("", &tiaojian, &canshu).await
}

// ========== 图谱公开查询接口 ==========

/// 查询全量图谱数据：所有标签节点及共现边
pub async fn chaxun_tupu_quanbu() -> Option<Value> {
    let jiedian = chaxun_tupu_jiedian("", &[], "l.mingcheng, b.zhi").await?;
    let bian = chaxun_tupu_bian("", "", &[]).await;
    let guanxi_bian = chaxun_tupu_guanxi_bian(&jiedian).await;
    Some(serde_json::json!({"jiedian": jiedian, "bian": bian, "guanxi_bian": guanxi_bian}))
}

/// 以某标签为中心查询子图（1层关联）
pub async fn chaxun_tupu_biaoqianid(biaoqianid: &str) -> Option<Value> {
    let guanlian = psqlcaozuo::chaxun(
        "SELECT DISTINCT b2.id, b2.zhi, b2.leixingid, l.mingcheng AS leixingmingcheng \
         FROM ribao_biaoqian rb1 \
         JOIN ribao_biaoqian rb2 ON rb1.ribaoid = rb2.ribaoid AND rb1.biaoqianid != rb2.biaoqianid \
         JOIN biaoqian b2 ON rb2.biaoqianid = b2.id \
         JOIN biaoqianleixing l ON b2.leixingid = l.id \
         WHERE rb1.biaoqianid = $1::BIGINT",
        &[biaoqianid],
    ).await?;
    let zhongxin = chaxun_tupu_jiedian("b.id = $1::BIGINT", &[biaoqianid], "").await.unwrap_or_default();
    let mut jiedian = zhongxin;
    jiedian.extend(guanlian);
    let bian = chaxun_tupu_bian_anzifanwei(&jiedian).await;
    let guanxi_bian = chaxun_tupu_guanxi_bian(&jiedian).await;
    Some(serde_json::json!({"jiedian": jiedian, "bian": bian, "guanxi_bian": guanxi_bian}))
}

/// 按标签类型筛选图谱
pub async fn chaxun_tupu_leixingmingcheng(leixingmingcheng: &str) -> Option<Value> {
    let jiedian = chaxun_tupu_jiedian("l.mingcheng = $1", &[leixingmingcheng], "b.zhi").await?;
    let bian = chaxun_tupu_bian(
        " JOIN biaoqianleixing l1 ON b1.leixingid = l1.id \
         JOIN biaoqianleixing l2 ON b2.leixingid = l2.id",
        "l1.mingcheng = $1 OR l2.mingcheng = $1",
        &[leixingmingcheng],
    ).await;
    let guanxi_bian = chaxun_tupu_guanxi_bian(&jiedian).await;
    Some(serde_json::json!({"jiedian": jiedian, "bian": bian, "guanxi_bian": guanxi_bian}))
}

// ========== 图谱增强接口 ==========

/// 搜索标签节点（按关键词模糊匹配，返回统计信息）
pub async fn tupu_sousuo(guanjianci: &str, leixingmingcheng: Option<&str>, limit: i64) -> Option<Vec<Value>> {
    let mohu = format!("%{}%", guanjianci);
    let limit_str = limit.to_string();
    match leixingmingcheng {
        Some(lx) => psqlcaozuo::chaxun(
            "SELECT b.id AS biaoqianid, b.zhi, l.mingcheng AS leixingmingcheng, \
             COUNT(DISTINCT rb.ribaoid)::TEXT AS ribao_zongshu, \
             MAX(r.fabushijian) AS zuijin_fabushijian \
             FROM biaoqian b \
             INNER JOIN biaoqianleixing l ON b.leixingid = l.id \
             LEFT JOIN ribao_biaoqian rb ON b.id = rb.biaoqianid \
             LEFT JOIN ribao r ON rb.ribaoid = r.id \
             WHERE b.zhi LIKE $1 AND l.mingcheng = $2 \
             GROUP BY b.id, b.zhi, l.mingcheng \
             ORDER BY COUNT(DISTINCT rb.ribaoid) DESC \
             LIMIT $3::BIGINT",
            &[&mohu, lx, &limit_str],
        ).await,
        None => psqlcaozuo::chaxun(
            "SELECT b.id AS biaoqianid, b.zhi, l.mingcheng AS leixingmingcheng, \
             COUNT(DISTINCT rb.ribaoid)::TEXT AS ribao_zongshu, \
             MAX(r.fabushijian) AS zuijin_fabushijian \
             FROM biaoqian b \
             INNER JOIN biaoqianleixing l ON b.leixingid = l.id \
             LEFT JOIN ribao_biaoqian rb ON b.id = rb.biaoqianid \
             LEFT JOIN ribao r ON rb.ribaoid = r.id \
             WHERE b.zhi LIKE $1 \
             GROUP BY b.id, b.zhi, l.mingcheng \
             ORDER BY COUNT(DISTINCT rb.ribaoid) DESC \
             LIMIT $2::BIGINT",
            &[&mohu, &limit_str],
        ).await,
    }
}

/// 按标签ID分页查询关联的日报（图谱节点→日报）
pub async fn tupu_ribao_fenye(biaoqianid: &str, yeshu: i64, meiyetiaoshu: i64) -> Option<Vec<Value>> {
    let (tiaoshu, pianyi) = jichugongju::jisuanfenye(yeshu, meiyetiaoshu);
    psqlcaozuo::chaxun(
        "SELECT r.*, y.nicheng AS fabuzhemingcheng, y.zhanghao AS fabuzhezhanghao \
         FROM ribao r \
         INNER JOIN ribao_biaoqian rb ON r.id = rb.ribaoid \
         LEFT JOIN yonghu y ON r.yonghuid = y.id \
         WHERE rb.biaoqianid = $1::BIGINT \
         ORDER BY r.fabushijian DESC \
         LIMIT $2::BIGINT OFFSET $3::BIGINT",
        &[biaoqianid, &tiaoshu, &pianyi],
    ).await
}

/// 统计标签关联的日报总数
pub async fn tongji_tupu_ribao_zongshu(biaoqianid: &str) -> Option<i64> {
    let jieguo = psqlcaozuo::chaxun(
        "SELECT COUNT(DISTINCT rb.ribaoid)::TEXT AS count \
         FROM ribao_biaoqian rb \
         WHERE rb.biaoqianid = $1::BIGINT",
        &[biaoqianid],
    ).await?;
    jieguo.first()?.get("count")?.as_str()?.parse().ok()
}

/// 按两个标签共现分页查询日报（图谱边→日报）
pub async fn tupu_bian_ribao_fenye(yuan_biaoqianid: &str, mubiao_biaoqianid: &str, yeshu: i64, meiyetiaoshu: i64) -> Option<Vec<Value>> {
    let (tiaoshu, pianyi) = jichugongju::jisuanfenye(yeshu, meiyetiaoshu);
    psqlcaozuo::chaxun(
        "SELECT r.*, y.nicheng AS fabuzhemingcheng, y.zhanghao AS fabuzhezhanghao \
         FROM ribao r \
         INNER JOIN ribao_biaoqian rb1 ON r.id = rb1.ribaoid \
         INNER JOIN ribao_biaoqian rb2 ON r.id = rb2.ribaoid \
         LEFT JOIN yonghu y ON r.yonghuid = y.id \
         WHERE rb1.biaoqianid = $1::BIGINT AND rb2.biaoqianid = $2::BIGINT \
         ORDER BY r.fabushijian DESC \
         LIMIT $3::BIGINT OFFSET $4::BIGINT",
        &[yuan_biaoqianid, mubiao_biaoqianid, &tiaoshu, &pianyi],
    ).await
}

/// 统计两个标签共现的日报总数
pub async fn tongji_tupu_bian_ribao_zongshu(yuan_biaoqianid: &str, mubiao_biaoqianid: &str) -> Option<i64> {
    let jieguo = psqlcaozuo::chaxun(
        "SELECT COUNT(DISTINCT r.id)::TEXT AS count \
         FROM ribao r \
         INNER JOIN ribao_biaoqian rb1 ON r.id = rb1.ribaoid \
         INNER JOIN ribao_biaoqian rb2 ON r.id = rb2.ribaoid \
         WHERE rb1.biaoqianid = $1::BIGINT AND rb2.biaoqianid = $2::BIGINT",
        &[yuan_biaoqianid, mubiao_biaoqianid],
    ).await?;
    jieguo.first()?.get("count")?.as_str()?.parse().ok()
}

/// 多标签交集分页查询日报
pub async fn tupu_ribao_duobiaoqian_fenye(biaoqianidlie: &[&str], yeshu: i64, meiyetiaoshu: i64) -> Option<Vec<Value>> {
    if biaoqianidlie.is_empty() {
        return None;
    }
    let (tiaoshu, pianyi) = jichugongju::jisuanfenye(yeshu, meiyetiaoshu);
    let shuliang = biaoqianidlie.len();
    let zhanwei = biaoqianidlie.iter().enumerate()
        .map(|(i, _)| format!("${}", i + 1))
        .collect::<Vec<_>>()
        .join(",");
    let sql = format!(
        "SELECT r.*, y.nicheng AS fabuzhemingcheng, y.zhanghao AS fabuzhezhanghao \
         FROM ribao r \
         LEFT JOIN yonghu y ON r.yonghuid = y.id \
         WHERE r.id IN (\
           SELECT rb.ribaoid FROM ribao_biaoqian rb \
           WHERE rb.biaoqianid::TEXT IN ({}) \
           GROUP BY rb.ribaoid \
           HAVING COUNT(DISTINCT rb.biaoqianid) = {} \
         ) \
         ORDER BY r.fabushijian DESC \
         LIMIT ${}::BIGINT OFFSET ${}::BIGINT",
        zhanwei, shuliang, shuliang + 1, shuliang + 2
    );
    let mut canshu: Vec<&str> = biaoqianidlie.to_vec();
    canshu.push(&tiaoshu);
    canshu.push(&pianyi);
    psqlcaozuo::chaxun(&sql, &canshu).await
}

/// 类型优先级：同名实体命中多类型时，按此优先级选取主节点
fn leixing_youxianji(leixing: &str) -> u8 {
    match leixing {
        "我方人员" => 0,
        "对方人员" => 1,
        "客户名字" => 2,
        "客户公司" => 3,
        "地点" => 4,
        _ => 5,
    }
}

/// 从节点列表中提取关系边（基于 ribao.kuozhan 中的 AI 关系分析）
async fn chaxun_tupu_guanxi_bian(jiedianlie: &[Value]) -> Vec<Value> {
    // 构建 name→(id, leixing, youxianji) 映射，同名多节点按优先级选主节点
    let mut mingcheng_dao_hourenlie: HashMap<String, Vec<(String, String, u8)>> = HashMap::new();
    for j in jiedianlie {
        if let (Some(id), Some(zhi), Some(leixing)) = (
            j.get("id").and_then(|v| v.as_i64().map(|n| n.to_string()).or_else(|| v.as_str().map(String::from))),
            j.get("zhi").and_then(|v| v.as_str()),
            j.get("leixingmingcheng").and_then(|v| v.as_str()),
        ) {
            let yxj = leixing_youxianji(leixing);
            mingcheng_dao_hourenlie.entry(zhi.to_string()).or_default().push((id, leixing.to_string(), yxj));
        }
    }

    let mut mingcheng_dao_id: HashMap<String, String> = HashMap::new();
    let mut hebing_mingcheng: Vec<String> = Vec::new();
    for (mingcheng, mut hourenlie) in mingcheng_dao_hourenlie {
        hourenlie.sort_by(|a, b| a.2.cmp(&b.2).then_with(|| a.0.cmp(&b.0)));
        hourenlie.dedup_by(|a, b| a.0 == b.0);
        let (zhu_id, zhu_leixing, _) = &hourenlie[0];
        mingcheng_dao_id.insert(mingcheng.clone(), zhu_id.clone());
        if hourenlie.len() > 1 {
            let houbu: Vec<String> = hourenlie[1..].iter()
                .map(|(id, lx, _)| format!("{}({})", lx, id))
                .collect();
            println!(
                "[图谱关系边] 同名\"{}\" 命中多节点，主节点={}({}) 候选={}",
                mingcheng, zhu_leixing, zhu_id, houbu.join(",")
            );
            hebing_mingcheng.push(mingcheng);
        }
    }
    if !hebing_mingcheng.is_empty() {
        hebing_mingcheng.sort();
        println!("[图谱关系边] 已按优先级合并 {} 个同名实体: {}", hebing_mingcheng.len(), hebing_mingcheng.join("、"));
    }
    if mingcheng_dao_id.is_empty() {
        return Vec::new();
    }
    // 查询所有含 kuozhan 的日报
    let ribaolie = match psqlcaozuo::chaxun(
        "SELECT id, kuozhan FROM ribao WHERE kuozhan IS NOT NULL AND kuozhan != '' AND kuozhan LIKE '%\"guanxifenxi\"%'",
        &[],
    ).await {
        Some(lie) => lie,
        None => return Vec::new(),
    };
    // 解析每条日报的关系，匹配节点
    struct BianJuhe {
        cishu: i64,
        miaoshulie: Vec<String>,
        zuigao_xindu: f64,
        zhengjulie: Vec<String>,
        juese_ren1: Option<String>,
        juese_ren2: Option<String>,
    }
    let mut juhe: HashMap<(String, String, String), BianJuhe> = HashMap::new();
    for r in &ribaolie {
        let kuozhan_str = match r.get("kuozhan").and_then(|v| v.as_str()) {
            Some(s) if !s.is_empty() => s,
            _ => continue,
        };
        let kuozhan: Value = match serde_json::from_str(kuozhan_str) {
            Ok(v) => v,
            Err(_) => continue,
        };
        let guanxilie = match kuozhan.get("guanxifenxi")
            .and_then(|gx| gx.get("guanxi"))
            .and_then(|g| g.as_array()) {
            Some(arr) => arr,
            None => continue,
        };
        for gx in guanxilie {
            let ren1 = match gx.get("ren1").and_then(|v| v.as_str()) { Some(s) => s, None => continue };
            let ren2 = match gx.get("ren2").and_then(|v| v.as_str()) { Some(s) => s, None => continue };
            let guanxi = gx.get("guanxi").and_then(|v| v.as_str()).unwrap_or("").to_string();
            if guanxi.is_empty() || guanxi.contains("无关") || guanxi == "无" {
                continue;
            }
            let miaoshu = gx.get("miaoshu").and_then(|v| v.as_str()).unwrap_or("").to_string();
            let xindu = gx.get("xindu").and_then(|v| v.as_f64()).unwrap_or(0.0);
            let zhengju = gx.get("zhengjupianduan").and_then(|v| v.as_str()).unwrap_or("").to_string();
            let juese_r1 = gx.get("juese").and_then(|j| j.get("ren1")).and_then(|v| v.as_str())
                .filter(|s| !s.is_empty()).map(String::from);
            let juese_r2 = gx.get("juese").and_then(|j| j.get("ren2")).and_then(|v| v.as_str())
                .filter(|s| !s.is_empty()).map(String::from);

            let yuan_id = match mingcheng_dao_id.get(ren1) { Some(id) => id.clone(), None => continue };
            let mubiao_id = match mingcheng_dao_id.get(ren2) { Some(id) => id.clone(), None => continue };
            if yuan_id == mubiao_id { continue; }
            // 保证 key 方向一致（小ID在前）
            let (k_yuan, k_mubiao, k_jr1, k_jr2) = if yuan_id < mubiao_id {
                (yuan_id, mubiao_id, juese_r1, juese_r2)
            } else {
                (mubiao_id, yuan_id, juese_r2, juese_r1)
            };
            let entry = juhe.entry((k_yuan, k_mubiao, guanxi)).or_insert_with(|| BianJuhe {
                cishu: 0, miaoshulie: Vec::new(), zuigao_xindu: 0.0,
                zhengjulie: Vec::new(), juese_ren1: None, juese_ren2: None,
            });
            entry.cishu += 1;
            if !miaoshu.is_empty() && !entry.miaoshulie.contains(&miaoshu) {
                entry.miaoshulie.push(miaoshu);
            }
            if xindu > entry.zuigao_xindu {
                entry.zuigao_xindu = xindu;
            }
            if !zhengju.is_empty() && !entry.zhengjulie.contains(&zhengju) {
                entry.zhengjulie.push(zhengju);
            }
            if entry.juese_ren1.is_none() { entry.juese_ren1 = k_jr1; }
            if entry.juese_ren2.is_none() { entry.juese_ren2 = k_jr2; }
        }
    }
    // 转为 JSON 数组
    juhe.into_iter().map(|((yuan, mubiao, guanxi), bj)| {
        let mut bian = serde_json::json!({
            "yuan": yuan,
            "mubiao": mubiao,
            "guanxi": guanxi,
            "miaoshu": bj.miaoshulie.join("；"),
            "cishu": bj.cishu.to_string(),
            "xindu": bj.zuigao_xindu,
            "zhengjupianduan": bj.zhengjulie.join("；"),
        });
        if bj.juese_ren1.is_some() || bj.juese_ren2.is_some() {
            bian["juese"] = serde_json::json!({
                "ren1": bj.juese_ren1.unwrap_or_default(),
                "ren2": bj.juese_ren2.unwrap_or_default(),
            });
        }
        bian
    }).collect()
}

/// 统计多标签交集的日报总数
pub async fn tongji_tupu_duobiaoqian_zongshu(biaoqianidlie: &[&str]) -> Option<i64> {
    if biaoqianidlie.is_empty() {
        return None;
    }
    let shuliang = biaoqianidlie.len();
    let zhanwei = biaoqianidlie.iter().enumerate()
        .map(|(i, _)| format!("${}", i + 1))
        .collect::<Vec<_>>()
        .join(",");
    let sql = format!(
        "SELECT COUNT(*)::TEXT AS count FROM (\
           SELECT rb.ribaoid FROM ribao_biaoqian rb \
           WHERE rb.biaoqianid::TEXT IN ({}) \
           GROUP BY rb.ribaoid \
           HAVING COUNT(DISTINCT rb.biaoqianid) = {} \
         ) t",
        zhanwei, shuliang
    );
    let jieguo = psqlcaozuo::chaxun(&sql, biaoqianidlie).await?;
    jieguo.first()?.get("count")?.as_str()?.parse().ok()
}
