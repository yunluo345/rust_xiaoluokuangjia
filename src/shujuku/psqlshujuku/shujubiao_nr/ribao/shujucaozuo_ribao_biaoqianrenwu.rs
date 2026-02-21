use serde_json::Value;
use crate::gongju::jichugongju;
use crate::shujuku::psqlshujuku::psqlcaozuo;

#[allow(non_upper_case_globals)]
const biaoming: &str = "ribao_biaoqianrenwu";

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
        &format!("WITH dailingqu AS (SELECT id FROM {} WHERE zhuangtai = 'false' AND changshicishu < zuidachangshicishu ORDER BY chuangjianshijian DESC, random() LIMIT 1 FOR UPDATE SKIP LOCKED) UPDATE {} r SET zhuangtai = 'true', changshicishu = changshicishu + 1, gengxinshijian = $1 FROM dailingqu d WHERE r.id = d.id RETURNING r.*", biaoming, biaoming),
        &[&shijian],
    ).await?;
    jieguo.into_iter().next()
}

/// 按最近创建时间批量领取未处理任务，同时间随机
pub async fn lingqu_zuijin_piliang_suiji(shuliang: i64) -> Option<Vec<Value>> {
    let shijian = jichugongju::huoqushijianchuo().to_string();
    psqlcaozuo::chaxun(
        &format!("WITH dailingqu AS (SELECT id FROM {} WHERE zhuangtai = 'false' AND changshicishu < zuidachangshicishu ORDER BY chuangjianshijian DESC, random() LIMIT GREATEST($2::BIGINT, 0) FOR UPDATE SKIP LOCKED) UPDATE {} r SET zhuangtai = 'true', changshicishu = changshicishu + 1, gengxinshijian = $1 FROM dailingqu d WHERE r.id = d.id RETURNING r.*", biaoming, biaoming),
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
        &format!("UPDATE {} SET zhuangtai = 'true', gengxinshijian = $2 WHERE id = $1::BIGINT", biaoming),
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

/// 查询待处理任务列表
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

/// 统计指定状态任务数量（状态仅支持 true/false）
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
