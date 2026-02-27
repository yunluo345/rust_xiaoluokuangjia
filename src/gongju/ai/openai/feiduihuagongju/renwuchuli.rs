use crate::shujuku::psqlshujuku::shujubiao_nr::yonghu::{yonghuyanzheng, shujucaozuo_yonghuzu};
use crate::peizhixt::peizhi_nr::peizhi_ai::Ai;
use crate::peizhixt::peizhixitongzhuti;
use crate::shujuku::psqlshujuku::shujubiao_nr::ribao::shujucaozuo_ribao_biaoqianrenwu;
use serde_json::{json, Value};

use super::renwubuzhou::{self, BuzhouCuowu};

/// 失败时统一标记并返回错误 JSON
async fn chuli_shibai(renwuid: &str, ribaoid: &str, xiaoxi: &str) -> Value {
    println!("[任务处理] ✗ 任务{}失败 日报={} 原因={}", renwuid, ribaoid, xiaoxi);
    let _ = shujucaozuo_ribao_biaoqianrenwu::biaojishibai(renwuid).await;
    json!({
        "chenggong": false,
        "renwuid": renwuid,
        "ribaoid": ribaoid,
        "xiaoxi": xiaoxi,
    })
}

/// 任务处理核心流程（步骤组合）
async fn chuli_dange_renwu(renwu: Value, peizhi: &Ai) -> Value {
    // 步骤1：校验任务与加载日报
    let shanxiawen = match renwubuzhou::yanzheng_renwu(&renwu).await {
        Ok(s) => s,
        Err(BuzhouCuowu { xiaoxi }) => {
            // 无法解析 renwuid 时的特殊处理
            let renwuid = renwu.get("id").and_then(|v| v.as_str()).unwrap_or("");
            let ribaoid = renwu.get("ribaoid").and_then(|v| v.as_str()).unwrap_or("");
            return chuli_shibai(renwuid, ribaoid, &xiaoxi).await;
        }
    };
    let renwuid = &shanxiawen.renwuid;
    let ribaoid = &shanxiawen.ribaoid;

    // 步骤2：标签提取
    let biaoqianxiang = match renwubuzhou::tiqu_biaoqian(
        &shanxiawen.neirong, peizhi,
        shanxiawen.yiyou_biaoqian_wenben.as_deref(), renwuid,
    ).await {
        Ok(x) => x,
        Err(e) => return chuli_shibai(renwuid, ribaoid, &e.xiaoxi).await,
    };

    // 步骤3：必填标签校验
    if let Err(e) = renwubuzhou::yanzheng_bitian(&biaoqianxiang, &shanxiawen.yiyou_biaoqianmingcheng, peizhi) {
        return chuli_shibai(renwuid, ribaoid, &e.xiaoxi).await;
    }

    // 步骤4：标签绑定
    let bangding = match renwubuzhou::bangding_biaoqian(ribaoid, biaoqianxiang, renwuid).await {
        Ok(b) => b,
        Err(e) => return chuli_shibai(renwuid, ribaoid, &e.xiaoxi).await,
    };
    let biaoqianjieguo = json!({
        "bangdingshu": bangding.bangdingshu,
        "biaoqianlie": bangding.jieguolie,
    }).to_string();

    // 步骤5：AI 内容丰富（标题/摘要/思维导图/关系分析）
    let fengfu = match renwubuzhou::ai_fengfu(&shanxiawen, peizhi).await {
        Ok(f) => f,
        Err(e) => return chuli_shibai(renwuid, ribaoid, &e.xiaoxi).await,
    };

    // 步骤6：标记任务成功
    if let Err(e) = renwubuzhou::wanjie_renwu(renwuid, &biaoqianjieguo).await {
        return chuli_shibai(renwuid, ribaoid, &e.xiaoxi).await;
    }

    println!("[任务处理] ✓ 任务={} 日报={} 绑定标签数={}", renwuid, ribaoid, bangding.bangdingshu);
    json!({
        "chenggong": true,
        "renwuid": renwuid,
        "ribaoid": ribaoid,
        "bangdingshu": bangding.bangdingshu,
        "tiqujieguo": bangding.tiqujieguo,
        "biaoti": fengfu.biaoti,
        "zhaiyao": fengfu.zhaiyao,
        "siweidaotu": fengfu.siweidaotu,
        "guanxifenxi": fengfu.guanxifenxi,
    })
}

/// 按任务ID单独处理一条任务（不经过调度器，直接执行）
pub async fn zhixing_dange_renwu_neibu(renwuid: &str) -> Result<Value, String> {
    let renwu = shujucaozuo_ribao_biaoqianrenwu::chaxun_id(renwuid)
        .await
        .ok_or_else(|| format!("任务不存在: {}", renwuid))?;

    let zhuangtai = renwu.get("zhuangtai").and_then(|v| v.as_str()).unwrap_or("");
    if zhuangtai == "processing" {
        return Err(format!("任务正在处理中: {}", renwuid));
    }

    // 已完成或已失败的任务需要先清除旧产物并重置状态再重新处理
    if zhuangtai == "true" || zhuangtai == "shibai" {
        shujucaozuo_ribao_biaoqianrenwu::chongxin_ruidui(renwuid)
            .await
            .ok_or_else(|| format!("重置任务失败: {}", renwuid))?;
    }

    // 原子领取任务（置为 processing 并增加尝试次数）
    let renwu = shujucaozuo_ribao_biaoqianrenwu::lingqu_zhiding(renwuid)
        .await
        .ok_or_else(|| format!("领取任务失败（可能已被其他进程处理）: {}", renwuid))?;

    let peizhi = peizhixitongzhuti::duqupeizhi::<Ai>(Ai::wenjianming()).unwrap_or_default();
    let jieguo = chuli_dange_renwu(renwu, &peizhi).await;
    Ok(jieguo)
}

pub async fn zhixing_neibu() -> Result<Value, String> {
    let peizhi = peizhixitongzhuti::duqupeizhi::<Ai>(Ai::wenjianming()).unwrap_or_default();
    let jieguo = shujucaozuo_ribao_biaoqianrenwu::qidong_diaodu(move |renwu| {
        let p = peizhi.clone();
        async move { chuli_dange_renwu(renwu, &p).await }
    })
    .await;

    jieguo
        .get("zhuangtai")
        .and_then(|z| z.as_str())
        .filter(|&z| z == "yunxingzhong")
        .map(|_| jieguo["xiaoxi"].as_str().unwrap_or("未知错误").to_string())
        .map_or(Ok(jieguo.clone()), Err)
}

pub async fn zhixing(_canshu: &str, lingpai: &str) -> String {
    let zaiti = match yonghuyanzheng::yanzhenglingpaijiquanxian(lingpai, "/jiekou/ribao/guanli").await {
        Ok(z) => z,
        Err(yonghuyanzheng::Lingpaicuowu::Yibeifengjin(y)) => return json!({"cuowu": format!("账号已被封禁：{}", y)}).to_string(),
        Err(yonghuyanzheng::Lingpaicuowu::Quanxianbuzu) => return json!({"cuowu": "权限不足"}).to_string(),
        Err(_) => return json!({"cuowu": "令牌无效或已过期"}).to_string(),
    };

    let zumingcheng = shujucaozuo_yonghuzu::chaxun_id(&zaiti.yonghuzuid).await
        .and_then(|zu| zu.get("mingcheng").and_then(|v| v.as_str()).map(String::from))
        .unwrap_or_else(|| "未知".to_string());
    println!(
        "[日报任务处理] 用户={} 账号={} 用户组={}（{}）",
        zaiti.yonghuid, zaiti.zhanghao, zaiti.yonghuzuid, zumingcheng
    );

    match zhixing_neibu().await {
        Ok(shuju) => json!({"chenggong": true, "shuju": shuju}).to_string(),
        Err(xiaoxi) => json!({"cuowu": xiaoxi}).to_string(),
    }
}
