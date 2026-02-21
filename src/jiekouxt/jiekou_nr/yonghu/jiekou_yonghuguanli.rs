use actix_web::{HttpRequest, HttpResponse, web};
use serde::{Deserialize, Serialize};
use crate::jiekouxt::jiekouxtzhuti::{self, Jiekoudinyi, Qingqiufangshi};
use crate::jiekouxt::jiamichuanshu::jiamichuanshuzhongjian;
use crate::shujuku::psqlshujuku::shujubiao_nr::yonghu::shujucaozuo_yonghu;

#[allow(non_upper_case_globals)]
pub const dinyi: Jiekoudinyi = Jiekoudinyi {
    lujing: "/yonghuguanli",
    nicheng: "用户管理",
    jieshao: "管理员接口，支持分页列表、模糊搜索、查看详情",
    fangshi: Qingqiufangshi::Post,
    jiami: true,
    xudenglu: true,
    xuyonghuzu: true,
    yunxuputong: false,
};

#[derive(Deserialize)]
struct Qingqiuti {
    caozuo: String,
    dangqianyeshu: Option<i64>,
    meiyeshuliang: Option<i64>,
    guanjianci: Option<String>,
    id: Option<String>,
}

#[derive(Serialize)]
struct Xiangyingti {
    liebiao: Vec<serde_json::Value>,
    zongshu: i64,
}

enum Caozuoleixing {
    Fenye { dangqianyeshu: i64, meiyeshuliang: i64 },
    Sousuo { guanjianci: String, dangqianyeshu: i64, meiyeshuliang: i64 },
    Xiangqing(String),
}

fn jiamishibai(zhuangtaima: u16, xiaoxi: impl Into<String>, miyao: &[u8]) -> HttpResponse {
    jiamichuanshuzhongjian::jiamixiangying(jiekouxtzhuti::shibai(zhuangtaima, xiaoxi), miyao)
}

fn tiqucansu(zhi: Option<String>, mingcheng: &str, miyao: &[u8]) -> Result<String, HttpResponse> {
    zhi.ok_or_else(|| jiamishibai(400, format!("缺少参数: {}", mingcheng), miyao))
}

fn jiexi_caozuo(qingqiu: Qingqiuti, miyao: &[u8]) -> Result<Caozuoleixing, HttpResponse> {
    match qingqiu.caozuo.as_str() {
        "fenye" => {
            let dangqianyeshu = qingqiu.dangqianyeshu.ok_or_else(|| jiamishibai(400, "缺少参数: dangqianyeshu", miyao))?;
            let meiyeshuliang = qingqiu.meiyeshuliang.ok_or_else(|| jiamishibai(400, "缺少参数: meiyeshuliang", miyao))?;
            match (dangqianyeshu > 0, meiyeshuliang > 0) {
                (true, true) => Ok(Caozuoleixing::Fenye { dangqianyeshu, meiyeshuliang }),
                _ => Err(jiamishibai(400, "页数和数量必须大于0", miyao)),
            }
        }
        "sousuo" => {
            let guanjianci = tiqucansu(qingqiu.guanjianci, "guanjianci", miyao)?;
            let dangqianyeshu = qingqiu.dangqianyeshu.ok_or_else(|| jiamishibai(400, "缺少参数: dangqianyeshu", miyao))?;
            let meiyeshuliang = qingqiu.meiyeshuliang.ok_or_else(|| jiamishibai(400, "缺少参数: meiyeshuliang", miyao))?;
            match (dangqianyeshu > 0, meiyeshuliang > 0) {
                (true, true) => Ok(Caozuoleixing::Sousuo { guanjianci, dangqianyeshu, meiyeshuliang }),
                _ => Err(jiamishibai(400, "页数和数量必须大于0", miyao)),
            }
        }
        "xiangqing" => {
            let id = tiqucansu(qingqiu.id, "id", miyao)?;
            Ok(Caozuoleixing::Xiangqing(id))
        }
        _ => Err(jiamishibai(400, "无效的操作类型", miyao)),
    }
}

async fn zhixing_caozuo(caozuo: Caozuoleixing, miyao: &[u8]) -> HttpResponse {
    match caozuo {
        Caozuoleixing::Fenye { dangqianyeshu, meiyeshuliang } => {
            let pianyi = ((dangqianyeshu - 1) * meiyeshuliang).to_string();
            let shuliang = meiyeshuliang.to_string();
            let liebiao = match shujucaozuo_yonghu::chaxun_fenye(&pianyi, &shuliang).await {
                Some(l) => l,
                None => return jiamishibai(500, "查询用户列表失败", miyao),
            };
            let zongshu_jieguo = match shujucaozuo_yonghu::chaxun_zongshu().await {
                Some(j) => j,
                None => return jiamishibai(500, "查询用户总数失败", miyao),
            };
            let zongshu = zongshu_jieguo.get("shuliang").and_then(|v| v.as_i64()).unwrap_or(0);
            let xiangying = Xiangyingti { liebiao, zongshu };
            jiamichuanshuzhongjian::jiamixiangying(jiekouxtzhuti::chenggong("查询成功", xiangying), miyao)
        }
        Caozuoleixing::Sousuo { guanjianci, dangqianyeshu, meiyeshuliang } => {
            let pianyi = ((dangqianyeshu - 1) * meiyeshuliang).to_string();
            let shuliang = meiyeshuliang.to_string();
            let liebiao = match shujucaozuo_yonghu::sousuo_mohu(&guanjianci, &pianyi, &shuliang).await {
                Some(l) => l,
                None => return jiamishibai(500, "搜索用户失败", miyao),
            };
            let zongshu_jieguo = match shujucaozuo_yonghu::sousuo_zongshu(&guanjianci).await {
                Some(j) => j,
                None => return jiamishibai(500, "查询搜索总数失败", miyao),
            };
            let zongshu = zongshu_jieguo.get("shuliang").and_then(|v| v.as_i64()).unwrap_or(0);
            let xiangying = Xiangyingti { liebiao, zongshu };
            jiamichuanshuzhongjian::jiamixiangying(jiekouxtzhuti::chenggong("搜索成功", xiangying), miyao)
        }
        Caozuoleixing::Xiangqing(id) => {
            match shujucaozuo_yonghu::chaxun_id(&id).await {
                Some(yonghu) => jiamichuanshuzhongjian::jiamixiangying(jiekouxtzhuti::chenggong("查询成功", yonghu), miyao),
                None => jiamishibai(404, "用户不存在", miyao),
            }
        }
    }
}

pub async fn chuli(req: HttpRequest, ti: web::Bytes) -> HttpResponse {
    let miyao = match jiamichuanshuzhongjian::paishengyao(&req).await {
        Some(m) => m,
        None => return jiekouxtzhuti::shibai(401, "加密会话无效"),
    };
    let mingwen = match jiamichuanshuzhongjian::jiemiqingqiuti(&ti, &miyao) {
        Some(m) => m,
        None => return jiekouxtzhuti::shibai(400, "解密请求体失败"),
    };
    let qingqiu = match serde_json::from_slice::<Qingqiuti>(&mingwen) {
        Ok(q) => q,
        Err(_) => return jiamishibai(400, "请求参数格式错误", &miyao),
    };
    let caozuo = match jiexi_caozuo(qingqiu, &miyao) {
        Ok(c) => c,
        Err(e) => return e,
    };
    zhixing_caozuo(caozuo, &miyao).await
}
