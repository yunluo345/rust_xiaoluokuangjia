use serde_json::{json, Value};
use std::future::Future;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::OnceLock;
use futures::stream::{self, StreamExt};
use crate::gongju::jichugongju;
use crate::peizhixt::peizhi_nr::peizhi_ai::Ai;
use crate::peizhixt::peizhixitongzhuti;
use crate::shujuku::psqlshujuku::psqlcaozuo;

#[allow(non_upper_case_globals)]
const biaoming: &str = "ribao_biaoqianrenwu";

#[allow(non_upper_case_globals)]
static yunxingzhong: OnceLock<AtomicBool> = OnceLock::new();

fn huoqu_yunxingbiaozhi() -> &'static AtomicBool {
    yunxingzhong.get_or_init(|| AtomicBool::new(false))
}

/// 查询任务调度器是否正在运行
pub fn shifou_yunxingzhong() -> bool {
    huoqu_yunxingbiaozhi().load(Ordering::Relaxed)
}

/// 停止任务调度器
pub fn tingzhi() -> bool {
    huoqu_yunxingbiaozhi().swap(false, Ordering::SeqCst)
}

/// 启动任务调度器，按配置并发数处理队列任务，超出并发的任务排队等待，仅可通过 tingzhi 停止
pub async fn qidong_diaodu<F, Fut>(chulihanshu: F) -> Value
where
    F: Fn(Value) -> Fut + Send + Sync + Clone + 'static,
    Fut: Future<Output = Value> + Send,
{
    if huoqu_yunxingbiaozhi()
        .compare_exchange(false, true, Ordering::SeqCst, Ordering::Relaxed)
        .is_err()
    {
        println!("[任务调度] 调度器已在运行中，跳过本次启动");
        return json!({"zhuangtai": "yunxingzhong", "xiaoxi": "任务调度器已在运行中"});
    }

    let peizhi = peizhixitongzhuti::duqupeizhi::<Ai>(Ai::wenjianming()).unwrap_or_default();
    let bingfa = peizhi.ribao_biaoqianrenwu_bingfashuliang.max(1) as usize;
    println!("[任务调度] 启动调度器 并发数={}", bingfa);
    let mut zongjieguolie: Vec<Value> = Vec::new();

    if let Some(tongji) = psqlcaozuo::chaxun(
        &format!("SELECT id::TEXT, zhuangtai, changshicishu::TEXT, zuidachangshicishu::TEXT FROM {} ORDER BY id", biaoming), &[]
    ).await {
        for t in &tongji {
            println!("[任务调度] 任务id={} zhuangtai={} 尝试={}/{}", t["id"], t["zhuangtai"], t["changshicishu"], t["zuidachangshicishu"]);
        }
    }

    while huoqu_yunxingbiaozhi().load(Ordering::Relaxed) {
        let renwulie = match lingqu_zuijin_piliang_suiji(bingfa as i64).await {
            Some(lie) if !lie.is_empty() => {
                println!("[任务调度] 领取到 {} 个任务", lie.len());
                lie
            }
            _ => {
                println!("[任务调度] 无更多待处理任务，停止调度");
                break;
            }
        };

        let jieguolie: Vec<Value> = stream::iter(renwulie.into_iter().map(|renwu| {
            let f = chulihanshu.clone();
            async move { f(renwu).await }
        }))
        .buffer_unordered(bingfa)
        .collect()
        .await;

        let pici_chenggong = jieguolie.iter().filter(|v| v.get("chenggong").and_then(|z| z.as_bool()).unwrap_or(false)).count();
        println!("[任务调度] 本批完成: 总数={} 成功={} 失败={}", jieguolie.len(), pici_chenggong, jieguolie.len() - pici_chenggong);
        zongjieguolie.extend(jieguolie);
    }

    huoqu_yunxingbiaozhi().store(false, Ordering::SeqCst);

    let chenggongshu = zongjieguolie
        .iter()
        .filter(|v| v.get("chenggong").and_then(|z| z.as_bool()).unwrap_or(false))
        .count();
    let zongshu = zongjieguolie.len();
    println!("[任务调度] 调度结束 总处理={} 成功={} 失败={}", zongshu, chenggongshu, zongshu.saturating_sub(chenggongshu));

    json!({
        "zongshu": zongshu,
        "chenggongshu": chenggongshu,
        "shibaishu": zongshu.saturating_sub(chenggongshu),
        "jieguolie": zongjieguolie,
    })
}

/// 新增标签任务，若已存在则更新并重置为等待状态
pub async fn xinzeng_huogengxin(ribaoid: &str, yonghuid: &str, zuidachangshicishu: i64) -> Option<String> {
    let shijian = jichugongju::huoqushijianchuo().to_string();
    let jieguo = psqlcaozuo::chaxun(
        &format!("INSERT INTO {} (ribaoid, yonghuid, zhuangtai, changshicishu, zuidachangshicishu, biaoqianjieguo, chuangjianshijian, gengxinshijian) VALUES ($1::BIGINT,$2::BIGINT,'false',0,$3::INT,NULL,$4,$4) ON CONFLICT (ribaoid) DO UPDATE SET yonghuid = EXCLUDED.yonghuid, zhuangtai = 'false', changshicishu = 0, zuidachangshicishu = EXCLUDED.zuidachangshicishu, biaoqianjieguo = NULL, gengxinshijian = $4 RETURNING id::TEXT", biaoming),
        &[ribaoid, yonghuid, &zuidachangshicishu.to_string(), &shijian],
    ).await?;
    jieguo.first().and_then(|v| v.get("id")?.as_str().map(String::from))
}

/// 发布任务（语义化别名）
pub async fn faburenwu(ribaoid: &str, yonghuid: &str, zuidachangshicishu: i64) -> Option<String> {
    xinzeng_huogengxin(ribaoid, yonghuid, zuidachangshicishu).await
}

/// 根据ID删除任务
pub async fn shanchu(id: &str) -> Option<u64> {
    psqlcaozuo::zhixing(
        &format!("DELETE FROM {} WHERE id = $1::BIGINT", biaoming),
        &[id],
    ).await
}

/// 重新入队（将已有任务重新发布为等待状态）
pub async fn chongxin_ruidui(id: &str) -> Option<u64> {
    let shijian = jichugongju::huoqushijianchuo().to_string();
    psqlcaozuo::zhixing(
        &format!("UPDATE {} SET zhuangtai = 'false', changshicishu = 0, biaoqianjieguo = NULL, gengxinshijian = $2 WHERE id = $1::BIGINT", biaoming),
        &[id, &shijian],
    ).await
}

/// 领取一个可执行任务，成功返回任务详情
pub async fn lingqu_yige() -> Option<Value> {
    let shijian = jichugongju::huoqushijianchuo().to_string();
    let jieguo = psqlcaozuo::chaxun(
        &format!("WITH dailingqu AS (SELECT id FROM {} WHERE zhuangtai = 'false' AND changshicishu < zuidachangshicishu ORDER BY chuangjianshijian DESC, random() LIMIT 1 FOR UPDATE SKIP LOCKED) UPDATE {} r SET zhuangtai = 'processing', changshicishu = changshicishu + 1, gengxinshijian = $1 FROM dailingqu d WHERE r.id = d.id RETURNING r.*", biaoming, biaoming),
        &[&shijian],
    ).await?;
    jieguo.into_iter().next()
}

/// 按最近创建时间批量领取未处理任务，同时间随机
pub async fn lingqu_zuijin_piliang_suiji(shuliang: i64) -> Option<Vec<Value>> {
    let shijian = jichugongju::huoqushijianchuo().to_string();
    psqlcaozuo::chaxun(
        &format!("WITH dailingqu AS (SELECT id FROM {} WHERE zhuangtai = 'false' AND changshicishu < zuidachangshicishu ORDER BY chuangjianshijian DESC, random() LIMIT GREATEST($2::BIGINT, 0) FOR UPDATE SKIP LOCKED) UPDATE {} r SET zhuangtai = 'processing', changshicishu = changshicishu + 1, gengxinshijian = $1 FROM dailingqu d WHERE r.id = d.id RETURNING r.*", biaoming, biaoming),
        &[&shijian, &shuliang.to_string()],
    ).await
}

/// 标记任务成功并写入标签结果
pub async fn biaojichenggong(id: &str, biaoqianjieguo: &str) -> Option<u64> {
    let shijian = jichugongju::huoqushijianchuo().to_string();
    psqlcaozuo::zhixing(
        &format!("UPDATE {} SET zhuangtai = 'true', biaoqianjieguo = $2, gengxinshijian = $3 WHERE id = $1::BIGINT", biaoming),
        &[id, biaoqianjieguo, &shijian],
    ).await
}

/// 标记任务失败
pub async fn biaojishibai(id: &str) -> Option<u64> {
    let shijian = jichugongju::huoqushijianchuo().to_string();
    psqlcaozuo::zhixing(
        &format!("UPDATE {} SET zhuangtai = 'shibai', gengxinshijian = $2 WHERE id = $1::BIGINT", biaoming),
        &[id, &shijian],
    ).await
}

/// 根据任务ID查询详情
pub async fn chaxun_id(id: &str) -> Option<Value> {
    let jieguo = psqlcaozuo::chaxun(
        &format!("SELECT * FROM {} WHERE id = $1::BIGINT", biaoming),
        &[id],
    ).await?;
    jieguo.into_iter().next()
}

/// 根据日报ID查询任务
pub async fn chaxun_ribaoid(ribaoid: &str) -> Option<Value> {
    let jieguo = psqlcaozuo::chaxun(
        &format!("SELECT * FROM {} WHERE ribaoid = $1::BIGINT", biaoming),
        &[ribaoid],
    ).await?;
    jieguo.into_iter().next()
}

/// 重新入队（按日报ID）
pub async fn chongxin_ruidui_ribaoid(ribaoid: &str) -> Option<u64> {
    let shijian = jichugongju::huoqushijianchuo().to_string();
    psqlcaozuo::zhixing(
        &format!("UPDATE {} SET zhuangtai = 'false', changshicishu = 0, biaoqianjieguo = NULL, gengxinshijian = $2 WHERE ribaoid = $1::BIGINT", biaoming),
        &[ribaoid, &shijian],
    ).await
}

/// 按状态分页查询任务（None=全部），支持页码和每页条数
pub async fn chaxun_fenye(zhuangtai: Option<&str>, yeshu: i64, meiyetiaoshu: i64) -> Option<Vec<Value>> {
    let (tiaoshu, pianyi) = jichugongju::jisuanfenye(yeshu, meiyetiaoshu);
    match zhuangtai {
        Some(z) => psqlcaozuo::chaxun(
            &format!("SELECT * FROM {} WHERE zhuangtai = $1 ORDER BY chuangjianshijian DESC LIMIT $2::BIGINT OFFSET $3::BIGINT", biaoming),
            &[z, &tiaoshu, &pianyi],
        ).await,
        None => psqlcaozuo::chaxun(
            &format!("SELECT * FROM {} ORDER BY chuangjianshijian DESC LIMIT $1::BIGINT OFFSET $2::BIGINT", biaoming),
            &[&tiaoshu, &pianyi],
        ).await,
    }
}

/// 统计任务总数（按可选状态），用于分页
pub async fn tongji_fenye_zongshu(zhuangtai: Option<&str>) -> Option<i64> {
    let jieguo = match zhuangtai {
        Some(z) => psqlcaozuo::chaxun(
            &format!("SELECT COUNT(*)::TEXT as count FROM {} WHERE zhuangtai = $1", biaoming),
            &[z],
        ).await?,
        None => psqlcaozuo::chaxun(
            &format!("SELECT COUNT(*)::TEXT as count FROM {}", biaoming),
            &[],
        ).await?,
    };
    jieguo.first()?.get("count")?.as_str()?.parse().ok()
}

/// 查询待处理且可重试的任务列表
pub async fn chaxun_dengdai(shuliang: i64) -> Option<Vec<Value>> {
    psqlcaozuo::chaxun(
        &format!("SELECT * FROM {} WHERE zhuangtai = 'false' AND changshicishu < zuidachangshicishu ORDER BY chuangjianshijian DESC LIMIT $1::BIGINT", biaoming),
        &[&shuliang.to_string()],
    ).await
}

/// 查询指定用户的任务列表
pub async fn chaxun_yonghuid(yonghuid: &str, shuliang: i64) -> Option<Vec<Value>> {
    psqlcaozuo::chaxun(
        &format!("SELECT * FROM {} WHERE yonghuid = $1::BIGINT ORDER BY chuangjianshijian DESC LIMIT $2::BIGINT", biaoming),
        &[yonghuid, &shuliang.to_string()],
    ).await
}

/// 统计指定状态任务数量
pub async fn tongji_zhuangtai(zhuangtai: &str) -> Option<i64> {
    let jieguo = psqlcaozuo::chaxun(
        &format!("SELECT COUNT(*)::TEXT as count FROM {} WHERE zhuangtai = $1", biaoming),
        &[zhuangtai],
    ).await?;
    jieguo.first()?.get("count")?.as_str()?.parse().ok()
}

/// 统计可重试待处理任务数量
pub async fn tongji_kechuli_dengdai() -> Option<i64> {
    let jieguo = psqlcaozuo::chaxun(
        &format!("SELECT COUNT(*)::TEXT as count FROM {} WHERE zhuangtai = 'false' AND changshicishu < zuidachangshicishu", biaoming),
        &[],
    ).await?;
    jieguo.first()?.get("count")?.as_str()?.parse().ok()
}
