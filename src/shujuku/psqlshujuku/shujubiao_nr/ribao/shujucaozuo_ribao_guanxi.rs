use serde_json::Value;
use crate::gongju::jichugongju;
use crate::shujuku::psqlshujuku::psqlcaozuo;

#[allow(non_upper_case_globals)]
const biaoming: &str = "ribao_guanxi_bian";

/// 新增一条关系边
pub async fn xinzeng(
    ribaoid: &str, ren1: &str, ren2: &str, guanxi: &str,
    miaoshu: &str, xindu: &str, zhengjupianduan: &str,
    juese_ren1: &str, juese_ren2: &str, qinggan_qingxiang: &str,
) -> Option<u64> {
    let shijian = jichugongju::huoqushijianchuo().to_string();
    psqlcaozuo::zhixing(
        &format!(
            "INSERT INTO {} (ribaoid, ren1, ren2, guanxi, miaoshu, xindu, zhengjupianduan, juese_ren1, juese_ren2, qinggan_qingxiang, chuangjianshijian) \
             VALUES ($1::BIGINT, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)",
            biaoming
        ),
        &[ribaoid, ren1, ren2, guanxi, miaoshu, xindu, zhengjupianduan, juese_ren1, juese_ren2, qinggan_qingxiang, &shijian],
    ).await
}

/// 删除某日报的所有关系边（用于重新分析前清理）
pub async fn shanchu_ribaoid(ribaoid: &str) -> Option<u64> {
    psqlcaozuo::zhixing(
        &format!("DELETE FROM {} WHERE ribaoid = $1::BIGINT", biaoming),
        &[ribaoid],
    ).await
}

/// 批量写入日报关系边（先删旧再插新）
pub async fn gengxin_ribao_guanxi(ribaoid: &str, guanxilie: &[Value]) -> Option<u64> {
    // 先删除该日报旧的关系边
    let _ = shanchu_ribaoid(ribaoid).await;

    let mut zongshu: u64 = 0;
    for gx in guanxilie {
        let ren1 = gx.get("ren1").and_then(|v| v.as_str()).unwrap_or("");
        let ren2 = gx.get("ren2").and_then(|v| v.as_str()).unwrap_or("");
        let guanxi = gx.get("guanxi").and_then(|v| v.as_str()).unwrap_or("");
        if ren1.is_empty() || ren2.is_empty() || guanxi.is_empty() {
            continue;
        }
        let miaoshu = gx.get("miaoshu").and_then(|v| v.as_str()).unwrap_or("");
        let xindu = gx.get("xindu").and_then(|v| v.as_f64()).unwrap_or(0.0).to_string();
        let zhengjupianduan = gx.get("zhengjupianduan").and_then(|v| v.as_str()).unwrap_or("");
        let juese_ren1 = gx.get("juese").and_then(|j| j.get("ren1")).and_then(|v| v.as_str()).unwrap_or("");
        let juese_ren2 = gx.get("juese").and_then(|j| j.get("ren2")).and_then(|v| v.as_str()).unwrap_or("");
        let qinggan = gx.get("qinggan_qingxiang").and_then(|v| v.as_str()).unwrap_or("");

        if let Some(shu) = xinzeng(ribaoid, ren1, ren2, guanxi, miaoshu, &xindu, zhengjupianduan, juese_ren1, juese_ren2, qinggan).await {
            zongshu += shu;
        }
    }
    Some(zongshu)
}

/// 查询图谱关系边聚合结果（按实体名称匹配节点列表）
/// 替代原 chaxun_tupu_guanxi_bian 的全量扫描逻辑
pub async fn chaxun_juhe_guanxi(shitimingchenglie: &[&str]) -> Vec<Value> {
    if shitimingchenglie.is_empty() {
        return Vec::new();
    }
    // 构建 IN 子句的占位符
    let zhanwei = shitimingchenglie.iter().enumerate()
        .map(|(i, _)| format!("${}", i + 1))
        .collect::<Vec<_>>()
        .join(",");

    let sql = format!(
        "SELECT ren1, ren2, guanxi, \
         STRING_AGG(DISTINCT miaoshu, '；') FILTER (WHERE miaoshu != '') AS miaoshu, \
         COUNT(*)::TEXT AS cishu, \
         MAX(xindu::DOUBLE PRECISION)::TEXT AS xindu, \
         STRING_AGG(DISTINCT zhengjupianduan, '；') FILTER (WHERE zhengjupianduan != '') AS zhengjupianduan, \
         MAX(juese_ren1) FILTER (WHERE juese_ren1 != '') AS juese_ren1, \
         MAX(juese_ren2) FILTER (WHERE juese_ren2 != '') AS juese_ren2, \
         MAX(qinggan_qingxiang) FILTER (WHERE qinggan_qingxiang != '') AS qinggan_qingxiang \
         FROM {} WHERE ren1 IN ({z}) OR ren2 IN ({z}) \
         GROUP BY ren1, ren2, guanxi",
        biaoming, z = zhanwei
    );

    psqlcaozuo::chaxun(&sql, shitimingchenglie).await.unwrap_or_default()
}

/// 聚合查询全部关系边
pub async fn chaxun_juhe_quanbu() -> Vec<Value> {
    let sql = format!(
        "SELECT ren1, ren2, guanxi, \
         STRING_AGG(DISTINCT miaoshu, '；') FILTER (WHERE miaoshu != '') AS miaoshu, \
         COUNT(*)::TEXT AS cishu, \
         MAX(xindu::DOUBLE PRECISION)::TEXT AS xindu, \
         STRING_AGG(DISTINCT zhengjupianduan, '；') FILTER (WHERE zhengjupianduan != '') AS zhengjupianduan, \
         MAX(juese_ren1) FILTER (WHERE juese_ren1 != '') AS juese_ren1, \
         MAX(juese_ren2) FILTER (WHERE juese_ren2 != '') AS juese_ren2, \
         MAX(qinggan_qingxiang) FILTER (WHERE qinggan_qingxiang != '') AS qinggan_qingxiang \
         FROM {} \
         GROUP BY ren1, ren2, guanxi",
        biaoming
    );
    psqlcaozuo::chaxun(&sql, &[]).await.unwrap_or_default()
}

/// 聚合查询某标签关联日报中的关系边
pub async fn chaxun_juhe_an_biaoqianid(biaoqianid: &str) -> Vec<Value> {
    let sql = format!(
        "SELECT g.ren1, g.ren2, g.guanxi, \
         STRING_AGG(DISTINCT g.miaoshu, '；') FILTER (WHERE g.miaoshu != '') AS miaoshu, \
         COUNT(*)::TEXT AS cishu, \
         MAX(g.xindu::DOUBLE PRECISION)::TEXT AS xindu, \
         STRING_AGG(DISTINCT g.zhengjupianduan, '；') FILTER (WHERE g.zhengjupianduan != '') AS zhengjupianduan, \
         MAX(g.juese_ren1) FILTER (WHERE g.juese_ren1 != '') AS juese_ren1, \
         MAX(g.juese_ren2) FILTER (WHERE g.juese_ren2 != '') AS juese_ren2, \
         MAX(g.qinggan_qingxiang) FILTER (WHERE g.qinggan_qingxiang != '') AS qinggan_qingxiang \
         FROM {} g \
         WHERE g.ribaoid IN (SELECT rb.ribaoid FROM ribao_biaoqian rb WHERE rb.biaoqianid = $1::BIGINT) \
         GROUP BY g.ren1, g.ren2, g.guanxi",
        biaoming
    );
    psqlcaozuo::chaxun(&sql, &[biaoqianid]).await.unwrap_or_default()
}

/// 聚合查询某标签类型关联日报中的关系边
pub async fn chaxun_juhe_an_leixingmingcheng(leixingmingcheng: &str) -> Vec<Value> {
    let sql = format!(
        "SELECT g.ren1, g.ren2, g.guanxi, \
         STRING_AGG(DISTINCT g.miaoshu, '；') FILTER (WHERE g.miaoshu != '') AS miaoshu, \
         COUNT(*)::TEXT AS cishu, \
         MAX(g.xindu::DOUBLE PRECISION)::TEXT AS xindu, \
         STRING_AGG(DISTINCT g.zhengjupianduan, '；') FILTER (WHERE g.zhengjupianduan != '') AS zhengjupianduan, \
         MAX(g.juese_ren1) FILTER (WHERE g.juese_ren1 != '') AS juese_ren1, \
         MAX(g.juese_ren2) FILTER (WHERE g.juese_ren2 != '') AS juese_ren2, \
         MAX(g.qinggan_qingxiang) FILTER (WHERE g.qinggan_qingxiang != '') AS qinggan_qingxiang \
         FROM {} g \
         WHERE g.ribaoid IN ( \
           SELECT rb.ribaoid FROM ribao_biaoqian rb \
           JOIN biaoqian b ON rb.biaoqianid = b.id \
           JOIN biaoqianleixing l ON b.leixingid = l.id \
           WHERE l.mingcheng = $1 \
         ) \
         GROUP BY g.ren1, g.ren2, g.guanxi",
        biaoming
    );
    psqlcaozuo::chaxun(&sql, &[leixingmingcheng]).await.unwrap_or_default()
}

/// 按实体名称分页查询关联日报（虚拟节点→日报）
pub async fn chaxun_ribao_an_shitimingcheng(mingcheng: &str, yeshu: i64, meiyetiaoshu: i64) -> Option<Vec<Value>> {
    let (tiaoshu, pianyi) = jichugongju::jisuanfenye(yeshu, meiyetiaoshu);
    psqlcaozuo::chaxun(
        &format!(
            "SELECT DISTINCT r.*, y.nicheng AS fabuzhemingcheng, y.zhanghao AS fabuzhezhanghao \
             FROM ribao r \
             INNER JOIN {} g ON r.id = g.ribaoid \
             LEFT JOIN yonghu y ON r.yonghuid = y.id \
             WHERE g.ren1 = $1 OR g.ren2 = $1 \
             ORDER BY r.fabushijian DESC \
             LIMIT $2::BIGINT OFFSET $3::BIGINT",
            biaoming
        ),
        &[mingcheng, &tiaoshu, &pianyi],
    ).await
}

/// 统计实体名称关联的日报总数
pub async fn tongji_ribao_an_shitimingcheng(mingcheng: &str) -> Option<i64> {
    let jieguo = psqlcaozuo::chaxun(
        &format!(
            "SELECT COUNT(DISTINCT g.ribaoid)::TEXT AS count \
             FROM {} g \
             WHERE g.ren1 = $1 OR g.ren2 = $1",
            biaoming
        ),
        &[mingcheng],
    ).await?;
    jieguo.first()?.get("count")?.as_str()?.parse().ok()
}

/// 回填历史数据：从 ribao.kuozhan 中解析已有关系写入 ribao_guanxi_bian
pub async fn huitian_lishi() -> u64 {
    let ribaolie = match psqlcaozuo::chaxun(
        "SELECT id, kuozhan FROM ribao WHERE kuozhan IS NOT NULL AND kuozhan != '' AND kuozhan LIKE '%\"guanxifenxi\"%'",
        &[],
    ).await {
        Some(lie) => lie,
        None => {
            println!("[关系边回填] 无历史数据需要回填");
            return 0;
        }
    };

    let mut zongbianshu: u64 = 0;
    let mut chenggongshu: u64 = 0;
    for r in &ribaolie {
        let ribaoid = match r.get("id").and_then(|v| v.as_str()) {
            Some(id) => id,
            None => continue,
        };
        let kuozhan_str = match r.get("kuozhan").and_then(|v| v.as_str()).filter(|s| !s.is_empty()) {
            Some(s) => s,
            None => continue,
        };
        let kuozhan: Value = match serde_json::from_str(kuozhan_str) {
            Ok(v) => v,
            Err(_) => continue,
        };
        let guanxilie = match kuozhan.get("guanxifenxi")
            .and_then(|gx| gx.get("guanxi"))
            .and_then(|g| g.as_array()) {
            Some(arr) => arr.clone(),
            None => continue,
        };
        if let Some(shu) = gengxin_ribao_guanxi(ribaoid, &guanxilie).await {
            zongbianshu += shu;
            chenggongshu += 1;
        }
    }
    println!(
        "[关系边回填] 完成 日报数={} 成功={} 写入边数={}",
        ribaolie.len(), chenggongshu, zongbianshu
    );
    zongbianshu
}
