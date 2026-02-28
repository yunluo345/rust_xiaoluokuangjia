use std::collections::{HashMap, HashSet};
use serde_json::Value;
use crate::gongju::jichugongju;
use crate::shujuku::psqlshujuku::psqlcaozuo;
use super::shujucaozuo_ribao_guanxi;

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
    let mut jiedian = chaxun_tupu_jiedian("", &[], "l.mingcheng, b.zhi").await?;
    let bian = chaxun_tupu_bian("", "", &[]).await;
    let juhe = shujucaozuo_ribao_guanxi::chaxun_juhe_quanbu().await;
    let (guanxi_bian, ewai_jiedian) = chaxun_tupu_guanxi_bian(&jiedian, juhe, false, true).await;
    jiedian.extend(ewai_jiedian);
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
    let juhe = shujucaozuo_ribao_guanxi::chaxun_juhe_an_biaoqianid(biaoqianid).await;
    let (guanxi_bian, ewai_jiedian) = chaxun_tupu_guanxi_bian(&jiedian, juhe, false, true).await;
    jiedian.extend(ewai_jiedian);
    Some(serde_json::json!({"jiedian": jiedian, "bian": bian, "guanxi_bian": guanxi_bian}))
}

/// 按标签类型筛选图谱
pub async fn chaxun_tupu_leixingmingcheng(leixingmingcheng: &str) -> Option<Value> {
    let mut jiedian = chaxun_tupu_jiedian("l.mingcheng = $1", &[leixingmingcheng], "b.zhi").await?;
    // 类型视图下，共现边严格限制为当前节点集合内部，避免连到集合外实体
    let bian = chaxun_tupu_bian_anzifanwei(&jiedian).await;
    let juhe = shujucaozuo_ribao_guanxi::chaxun_juhe_an_leixingmingcheng(leixingmingcheng).await;
    // 类型视图下，关系边仅保留两端都在当前节点集合内的关系，且不补虚拟节点
    let (guanxi_bian, ewai_jiedian) = chaxun_tupu_guanxi_bian(&jiedian, juhe, true, false).await;
    jiedian.extend(ewai_jiedian);
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

/// 泛称→真名替换：将"我方""对方"等泛称映射为对应标签类型的真实节点名称
/// 非泛称直接返回原名称；泛称返回对应类型的所有真实节点名称；无法映射返回空
fn fancheng_tihuan_mingcheng(mingcheng: &str, leixing_dao_mingchenglie: &HashMap<String, Vec<String>>) -> Vec<String> {
    let duiying_leixing: &[&str] = match mingcheng {
        "我" | "我们" | "我方" | "本人" | "自己" | "我司" | "本公司" | "本部门" | "乙方" => &["我方人员"],
        "他" | "她" | "对方" | "对方公司" => &["对方人员", "客户名字", "客户公司"],
        "客户" | "客户方" | "甲方" => &["客户名字", "客户公司"],
        "领导" | "老板" | "上级" | "同事" | "下属" | "负责人" | "经理" => &["我方人员", "对方人员"],
        "你" | "老师" | "朋友" => return Vec::new(),
        _ => return vec![mingcheng.to_string()],
    };
    let mut jieguo = Vec::new();
    for leixing in duiying_leixing {
        if let Some(mingchenglie) = leixing_dao_mingchenglie.get(*leixing) {
            for mc in mingchenglie {
                if !jieguo.contains(mc) {
                    jieguo.push(mc.clone());
                }
            }
        }
    }
    jieguo
}

/// 将关系聚合数据转换为图谱边 + 虚拟节点
/// juhe_jieguo: 由调用方按作用域预查询的聚合关系数据
/// strict_scope: true=两端都必须在当前节点集；false=至少一端命中即可
/// allow_virtual_nodes: true=允许补充虚拟节点；false=禁止补虚拟节点
/// 返回 (关系边列表, 额外虚拟节点列表)
async fn chaxun_tupu_guanxi_bian(
    jiedianlie: &[Value],
    juhe_jieguo: Vec<Value>,
    strict_scope: bool,
    allow_virtual_nodes: bool,
) -> (Vec<Value>, Vec<Value>) {
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

    let zhenshi_mingcheng: HashSet<String> = mingcheng_dao_id.keys().cloned().collect();

    // 按类型分组真实节点名称（用于泛称→真名替换）
    let mut leixing_dao_mingchenglie: HashMap<String, Vec<String>> = HashMap::new();
    for j in jiedianlie {
        if let (Some(zhi), Some(leixing)) = (
            j.get("zhi").and_then(|v| v.as_str()),
            j.get("leixingmingcheng").and_then(|v| v.as_str()),
        ) {
            let lie = leixing_dao_mingchenglie.entry(leixing.to_string()).or_default();
            let zhi_str = zhi.to_string();
            if !lie.contains(&zhi_str) {
                lie.push(zhi_str);
            }
        }
    }

    // 虚拟节点计数器（使用负数ID避免与真实标签ID冲突）
    let mut xuni_jishu: i64 = -1;
    let mut ewai_jiedian: Vec<Value> = Vec::new();

    // 辅助：获取或创建虚拟节点ID
    let huoqu_id = |mingcheng: &str, juese: Option<&str>, mingcheng_dao_id: &mut HashMap<String, String>, ewai: &mut Vec<Value>, jishu: &mut i64| -> Option<String> {
        if let Some(id) = mingcheng_dao_id.get(mingcheng) {
            return Some(id.clone());
        }
        if !allow_virtual_nodes {
            return None;
        }
        let xuni_id = jishu.to_string();
        *jishu -= 1;
        let leixing = juese.unwrap_or("关系实体");
        ewai.push(serde_json::json!({
            "id": xuni_id,
            "zhi": mingcheng,
            "leixingid": null,
            "leixingmingcheng": leixing,
        }));
        mingcheng_dao_id.insert(mingcheng.to_string(), xuni_id.clone());
        println!("[图谱关系边] 补充虚拟节点 \"{}\"({}), ID={}", mingcheng, leixing, xuni_id);
        Some(xuni_id)
    };

    // 转换为图谱格式（泛称→真名替换 + 实体名称→节点ID映射）
    let mut jieguo: Vec<Value> = Vec::new();
    for gx in &juhe_jieguo {
        let ren1_yuan = match gx.get("ren1").and_then(|v| v.as_str()) { Some(s) => s, None => continue };
        let ren2_yuan = match gx.get("ren2").and_then(|v| v.as_str()) { Some(s) => s, None => continue };
        let guanxi_str = gx.get("guanxi").and_then(|v| v.as_str()).unwrap_or("");
        if guanxi_str.is_empty() || guanxi_str.contains("无关") || guanxi_str == "无" {
            continue;
        }

        // 泛称替换：将"我方""对方"等替换为对应类型的真实节点名称
        let ren1_lie = fancheng_tihuan_mingcheng(ren1_yuan, &leixing_dao_mingchenglie);
        let ren2_lie = fancheng_tihuan_mingcheng(ren2_yuan, &leixing_dao_mingchenglie);
        if ren1_lie.is_empty() || ren2_lie.is_empty() {
            continue;
        }

        let juese_r1 = gx.get("juese_ren1").and_then(|v| v.as_str()).filter(|s| !s.is_empty());
        let juese_r2 = gx.get("juese_ren2").and_then(|v| v.as_str()).filter(|s| !s.is_empty());
        let miaoshu = gx.get("miaoshu").and_then(|v| v.as_str()).unwrap_or("");
        let cishu = gx.get("cishu").and_then(|v| v.as_str()).unwrap_or("1");
        let xindu_str = gx.get("xindu").and_then(|v| v.as_str()).unwrap_or("0");
        let xindu: f64 = xindu_str.parse().unwrap_or(0.0);
        let zhengju = gx.get("zhengjupianduan").and_then(|v| v.as_str()).unwrap_or("");

        for ren1 in &ren1_lie {
            for ren2 in &ren2_lie {
                if ren1 == ren2 { continue; }
                if strict_scope {
                    if !zhenshi_mingcheng.contains(ren1.as_str()) || !zhenshi_mingcheng.contains(ren2.as_str()) {
                        continue;
                    }
                } else if !zhenshi_mingcheng.contains(ren1.as_str()) && !zhenshi_mingcheng.contains(ren2.as_str()) {
                    continue;
                }

                let yuan_id = match huoqu_id(ren1, juese_r1, &mut mingcheng_dao_id, &mut ewai_jiedian, &mut xuni_jishu) {
                    Some(v) => v,
                    None => continue,
                };
                let mubiao_id = match huoqu_id(ren2, juese_r2, &mut mingcheng_dao_id, &mut ewai_jiedian, &mut xuni_jishu) {
                    Some(v) => v,
                    None => continue,
                };
                if yuan_id == mubiao_id { continue; }

                let (k_yuan, k_mubiao) = if yuan_id < mubiao_id {
                    (yuan_id, mubiao_id)
                } else {
                    (mubiao_id, yuan_id)
                };

                let mut bian = serde_json::json!({
                    "yuan": k_yuan,
                    "mubiao": k_mubiao,
                    "guanxi": guanxi_str,
                    "miaoshu": miaoshu,
                    "cishu": cishu,
                    "xindu": xindu,
                    "zhengjupianduan": zhengju,
                });
                if juese_r1.is_some() || juese_r2.is_some() {
                    bian["juese"] = serde_json::json!({
                        "ren1": juese_r1.unwrap_or_default(),
                        "ren2": juese_r2.unwrap_or_default(),
                    });
                }
                jieguo.push(bian);
            }
        }
    }
    if !ewai_jiedian.is_empty() {
        println!("[图谱关系边] 共补充 {} 个虚拟节点", ewai_jiedian.len());
    }
    (jieguo, ewai_jiedian)
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
