use actix_web::{HttpMessage, HttpRequest, HttpResponse, web};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::jiekouxt::jiekouxtzhuti::{self, Jiekoudinyi, Qingqiufangshi};
use crate::jiekouxt::jiamichuanshu::jiamichuanshuzhongjian;
use crate::shujuku::psqlshujuku::shujubiao_nr::ribao::shujucaozuo_ribao;
use crate::gongju::jwtgongju;

use crate::gongju::zhuangtaima::ribaojiekou_zhuangtai::{Zhuangtai, cuowu};

#[allow(non_upper_case_globals)]
pub const dinyi: Jiekoudinyi = Jiekoudinyi {
    lujing: "/yonghu",
    nicheng: "日报用户操作",
    jieshao: "普通用户日报操作：新增、查询自己或全部日报、分页查询、统计",
    fangshi: Qingqiufangshi::Post,
    jiami: true,
    xudenglu: true,
    xuyonghuzu: false,
    yunxuputong: true,
};

#[derive(Deserialize, Serialize)]
struct Qingqiuti { caozuo: String, #[serde(flatten)] canshu: Value }

#[derive(Deserialize)]
struct Xinzengcanshu { neirong: String, fabushijian: String }

#[derive(Deserialize)]
struct Fenyecanshu { yeshu: Option<i64>, meiyetiaoshu: Option<i64> }

fn jiamishibai(zhuangtai: &Zhuangtai, miyao: &[u8]) -> HttpResponse {
    jiamichuanshuzhongjian::jiamixiangying(jiekouxtzhuti::shibai(zhuangtai.ma, zhuangtai.xiaoxi), miyao)
}

fn jiamichenggong(xiaoxi: impl Into<String>, shuju: Value, miyao: &[u8]) -> HttpResponse {
    jiamichuanshuzhongjian::jiamixiangying(jiekouxtzhuti::chenggong(xiaoxi, shuju), miyao)
}

async fn chulicaozuo(caozuo: &str, canshu: Value, yonghuid: &str, miyao: &[u8]) -> HttpResponse {
    match caozuo {
        "xinzeng" => {
            let c: Xinzengcanshu = match serde_json::from_value(canshu) {
                Ok(c) => c,
                Err(_) => return jiamishibai(&cuowu::canshugeshibuzhengque, miyao),
            };
            match shujucaozuo_ribao::xinzeng(yonghuid, &c.neirong, &c.fabushijian).await {
                Some(id) => jiamichenggong("创建成功", serde_json::json!({"id": id}), miyao),
                None => jiamishibai(&cuowu::chuangjianshi, miyao),
            }
        }
        "chaxun" => {
            match shujucaozuo_ribao::chaxun_yonghuid(yonghuid).await {
                Some(jieguo) => jiamichenggong("查询成功", serde_json::json!(jieguo), miyao),
                None => jiamishibai(&cuowu::chaxunshibai, miyao),
            }
        }
        "chaxun_fenye" => {
            let c: Fenyecanshu = match serde_json::from_value(canshu) {
                Ok(c) => c,
                Err(_) => return jiamishibai(&cuowu::canshugeshibuzhengque, miyao),
            };
            let yeshu = c.yeshu.unwrap_or(1);
            let meiyetiaoshu = c.meiyetiaoshu.unwrap_or(10);
            let liebiao = shujucaozuo_ribao::chaxun_yonghuid_fenye(yonghuid, yeshu, meiyetiaoshu).await.unwrap_or_default();
            let zongshu = shujucaozuo_ribao::tongji_yonghuid_zongshu(yonghuid).await.unwrap_or(0);
            jiamichenggong("查询成功", serde_json::json!({"liebiao": liebiao, "zongshu": zongshu}), miyao)
        }
        "chaxun_quanbu_fenye" => {
            let c: Fenyecanshu = match serde_json::from_value(canshu) {
                Ok(c) => c,
                Err(_) => return jiamishibai(&cuowu::canshugeshibuzhengque, miyao),
            };
            let yeshu = c.yeshu.unwrap_or(1);
            let meiyetiaoshu = c.meiyetiaoshu.unwrap_or(10);
            let liebiao = shujucaozuo_ribao::chaxun_fenye(yeshu, meiyetiaoshu).await.unwrap_or_default();
            let zongshu = shujucaozuo_ribao::tongji_zongshu().await.unwrap_or(0);
            jiamichenggong("查询成功", serde_json::json!({"liebiao": liebiao, "zongshu": zongshu}), miyao)
        }
        "tongji_zongshu" => {
            match shujucaozuo_ribao::tongji_yonghuid_zongshu(yonghuid).await {
                Some(zongshu) => jiamichenggong("统计成功", serde_json::json!({"count": zongshu}), miyao),
                None => jiamishibai(&cuowu::tongjishibai, miyao),
            }
        }
        "tongji_quanbu_zongshu" => {
            match shujucaozuo_ribao::tongji_zongshu().await {
                Some(zongshu) => jiamichenggong("统计成功", serde_json::json!({"count": zongshu}), miyao),
                None => jiamishibai(&cuowu::tongjishibai, miyao),
            }
        }
        _ => jiamishibai(&cuowu::bucaozuoleixing, miyao),
    }
}

/// 普通用户日报接口处理函数
pub async fn chuli(req: HttpRequest, ti: web::Bytes) -> HttpResponse {
    let miyao = match jiamichuanshuzhongjian::paishengyao(&req).await {
        Some(m) => m,
        None => return jiekouxtzhuti::shibai(401, "加密会话无效"),
    };

    let mingwen = match jiamichuanshuzhongjian::jiemiqingqiuti(&ti, &miyao) {
        Some(m) => m,
        None => return jiekouxtzhuti::shibai(400, "解密失败"),
    };

    let qingqiu: Qingqiuti = match serde_json::from_slice::<Qingqiuti>(&mingwen) {
        Ok(q) => q,
        Err(_) => return jiamishibai(&cuowu::qingqiugeshibuzhengque, &miyao),
    };

    let zaiti = req.extensions().get::<jwtgongju::Zaiti>().cloned().unwrap();

    chulicaozuo(&qingqiu.caozuo, qingqiu.canshu, &zaiti.yonghuid, &miyao).await
}
