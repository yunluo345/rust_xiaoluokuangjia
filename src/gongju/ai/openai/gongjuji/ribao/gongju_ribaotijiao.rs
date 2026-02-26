use crate::gongju::ai::openai::gongjuji::ribao::gongju_ribaorenwuchuli;
use crate::peizhixt::peizhi_nr::peizhi_ai::Ai;
use crate::peizhixt::peizhixitongzhuti;
use crate::shujuku::psqlshujuku::shujubiao_nr::ribao::{
    shujucaozuo_ribao,
    shujucaozuo_ribao_biaoqianrenwu,
};

pub struct Tijiaojieguo {
    pub ribaoid: String,
    pub renwuid: String,
}

pub enum Tijiaocuowu {
    Ribaochuangjianshibai,
    Renwufabushibai { ribaoid: String },
}

pub fn huoqu_moren_chongshicishu() -> i64 {
    peizhixitongzhuti::duqupeizhi::<Ai>(Ai::wenjianming())
        .map(|p| p.ribao_biaoqianrenwu_chongshi_cishu as i64)
        .unwrap_or(3)
}

fn yibu_changshiqidong(ribaoid: String, renwuid: String) {
    actix_web::rt::spawn(async move {
        match gongju_ribaorenwuchuli::zhixing_dange_renwu_neibu(&renwuid).await {
            Ok(jieguo) => {
                let chenggong = jieguo.get("chenggong").and_then(|v| v.as_bool()).unwrap_or(false);
                println!(
                    "[日报提交] 单任务处理{} ribaoid={} renwuid={}",
                    if chenggong { "成功" } else { "失败" }, ribaoid, renwuid
                );
            }
            Err(xiaoxi) => {
                println!(
                    "[日报提交] 单任务处理跳过 ribaoid={} renwuid={} yuanyin={}",
                    ribaoid, renwuid, xiaoxi
                );
            }
        }
    });
}

pub async fn tijiao_ribao_bingzidongqidong(
    yonghuid: &str,
    neirong: &str,
    fabushijian: &str,
    chongshicishu: i64,
    biaoti: Option<&str>,
) -> Result<Tijiaojieguo, Tijiaocuowu> {
    let ribaoid = shujucaozuo_ribao::xinzeng(yonghuid, neirong, fabushijian, biaoti)
        .await
        .ok_or(Tijiaocuowu::Ribaochuangjianshibai)?;

    // 调度器运行中：只负责确保任务在队列里（避免重复启动导致无意义日志）
    if shujucaozuo_ribao_biaoqianrenwu::shifou_yunxingzhong() {
        if let Some(renwu) = shujucaozuo_ribao_biaoqianrenwu::chaxun_ribaoid(&ribaoid).await {
            let renwuid = renwu
                .get("id")
                .and_then(|v| v.as_str())
                .map(String::from)
                .or_else(|| renwu.get("id").and_then(|v| v.as_i64()).map(|n| n.to_string()))
                .unwrap_or_default();
            println!(
                "[日报提交] 调度器运行中，任务已在队列 ribaoid={} renwuid={}",
                ribaoid, renwuid
            );
            return Ok(Tijiaojieguo { ribaoid, renwuid });
        }

        let renwuid = shujucaozuo_ribao_biaoqianrenwu::faburenwu(&ribaoid, yonghuid, chongshicishu)
            .await
            .ok_or_else(|| Tijiaocuowu::Renwufabushibai { ribaoid: ribaoid.clone() })?;
        println!(
            "[日报提交] 调度器运行中，已补入任务队列 ribaoid={} renwuid={}",
            ribaoid, renwuid
        );
        return Ok(Tijiaojieguo { ribaoid, renwuid });
    }

    // 调度器未运行：入队 + 异步尝试启动
    let renwuid = shujucaozuo_ribao_biaoqianrenwu::faburenwu(&ribaoid, yonghuid, chongshicishu)
        .await
        .ok_or_else(|| Tijiaocuowu::Renwufabushibai { ribaoid: ribaoid.clone() })?;

    yibu_changshiqidong(ribaoid.clone(), renwuid.clone());

    Ok(Tijiaojieguo { ribaoid, renwuid })
}
