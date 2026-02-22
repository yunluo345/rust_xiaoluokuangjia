use actix_web::{HttpMessage, HttpRequest, HttpResponse, web};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::jiekouxt::jiekouxtzhuti::{self, Jiekoudinyi, Qingqiufangshi};
use crate::jiekouxt::jiamichuanshu::jiamichuanshuzhongjian;
use crate::shujuku::psqlshujuku::shujubiao_nr::ribao::{
    shujucaozuo_biaoqianleixing, shujucaozuo_biaoqian,
    shujucaozuo_ribao, shujucaozuo_ribao_biaoqian
};
use crate::gongju::ai::openai::gongjuji::ribao::gongju_ribaorenwuchuli;
use crate::shujuku::psqlshujuku::shujubiao_nr::yonghu::{shujucaozuo_yonghuzu, yonghuyanzheng};
use crate::gongju::jwtgongju;

use crate::gongju::zhuangtaima::ribaojiekou_zhuangtai::{Zhuangtai, cuowu};

#[allow(non_upper_case_globals)]
pub const dinyi: Jiekoudinyi = Jiekoudinyi {
    lujing: "/ribao",
    nicheng: "日报管理",
    jieshao: "管理日报、标签类型、标签及其关联的增删改查操作",
    fangshi: Qingqiufangshi::Post,
    jiami: true,
    xudenglu: true,
    xuyonghuzu: false,
    yunxuputong: true,
};

#[derive(Deserialize, Serialize)]
struct Qingqiuti { caozuo: String, #[serde(flatten)] canshu: Value }

#[derive(Deserialize)]
struct Idcanshu { id: String }

#[derive(Deserialize)]
struct Mingchengcanshu { mingcheng: String }

#[derive(Deserialize)]
struct Leixingxinzengcanshu { mingcheng: String }

#[derive(Deserialize)]
struct Leixinggengxincanshu { id: String, mingcheng: String }

#[derive(Deserialize)]
struct Biaoqianxinzengcanshu { leixingid: String, zhi: String }

#[derive(Deserialize)]
struct Biaoqiangengxincanshu { id: String, zhi: String }

#[derive(Deserialize)]
struct Biaoqianchaxuncanshu { leixingid: String }

#[derive(Deserialize)]
struct Biaoqianchaxunzhicanshu { leixingid: String, zhi: String }

#[derive(Deserialize)]
struct Ribaoxinzengcanshu { yonghuid: String, neirong: String, fabushijian: String }

#[derive(Deserialize)]
struct Ribaogengxincanshu { id: String, ziduanlie: Vec<(String, String)> }

#[derive(Deserialize)]
struct Yonghuidcanshu { yonghuid: String }

#[derive(Deserialize)]
struct Fenyecanshu { yeshu: i64, meiyetiaoshu: i64 }

#[derive(Deserialize)]
struct Yonghuidfenyecanshu { yonghuid: String, yeshu: i64, meiyetiaoshu: i64 }

#[derive(Deserialize)]
struct Guanlianxinzengcanshu { ribaoid: String, biaoqianid: String }

#[derive(Deserialize)]
struct Ribaoidcanshu { ribaoid: String }

#[derive(Deserialize)]
struct Biaoqianidcanshu { biaoqianid: String }

#[derive(Deserialize)]
struct Guanlianshanchu { ribaoid: String, biaoqianid: String }

#[derive(Deserialize)]
struct Piliangguanliancanshu { ribaoid: String, biaoqianidlie: Vec<String> }

#[derive(Deserialize)]
struct Leixingmingchengzhicanshu { leixingmingcheng: String, zhi: String }

#[derive(Deserialize)]
struct Xiangguanbiaoqiancanshu { biaoqianid: String, leixingmingcheng: String }

#[derive(Deserialize)]
struct Guanjiancifenyecanshu { guanjianci: String, yeshu: i64, meiyetiaoshu: i64 }

#[derive(Deserialize)]
struct Renwuchulicanshu { shuliang: Option<i64> }

macro_rules! jiexi_canshu {
    ($qingqiu:expr, $canshu_leixing:ty, $miyao:expr) => {
        match serde_json::from_value::<$canshu_leixing>($qingqiu.canshu) {
            Ok(c) => c,
            Err(_) => return jiamishibai(&cuowu::canshugeshibuzhengque, $miyao),
        }
    };
}

macro_rules! chuli_chaxun {
    ($canshu:expr, $miyao:expr, $shujuku_fn:expr, $shibai_cuowu:expr) => {
        match $shujuku_fn.await {
            Some(jieguo) => jiamichenggong("查询成功", jieguo, $miyao),
            None => jiamishibai($shibai_cuowu, $miyao),
        }
    };
}

macro_rules! chuli_chaxun_liebiao {
    ($canshu:expr, $miyao:expr, $shujuku_fn:expr) => {
        match $shujuku_fn.await {
            Some(jieguo) => jiamichenggong("查询成功", serde_json::json!(jieguo), $miyao),
            None => jiamishibai(&cuowu::chaxunshibai, $miyao),
        }
    };
}

macro_rules! chuli_shanchu_gengxin {
    ($canshu:expr, $miyao:expr, $shujuku_fn:expr, $chenggong_msg:expr, $shibai_cuowu:expr) => {
        match $shujuku_fn.await {
            Some(n) if n > 0 => jiamichenggong($chenggong_msg, serde_json::json!({"count": n}), $miyao),
            _ => jiamishibai($shibai_cuowu, $miyao),
        }
    };
}

macro_rules! chuli_xinzeng {
    ($canshu:expr, $miyao:expr, $shujuku_fn:expr) => {
        match $shujuku_fn.await {
            Some(id) => jiamichenggong("创建成功", serde_json::json!({"id": id}), $miyao),
            None => jiamishibai(&cuowu::chuangjianshi, $miyao),
        }
    };
}

fn jiamishibai(zhuangtai: &Zhuangtai, miyao: &[u8]) -> HttpResponse {
    jiamichuanshuzhongjian::jiamixiangying(jiekouxtzhuti::shibai(zhuangtai.ma, zhuangtai.xiaoxi), miyao)
}

fn jiamishibai_dongtai(zhuangtaima: u16, xiaoxi: impl Into<String>, miyao: &[u8]) -> HttpResponse {
    jiamichuanshuzhongjian::jiamixiangying(jiekouxtzhuti::shibai(zhuangtaima, xiaoxi), miyao)
}

fn jiamichenggong(xiaoxi: impl Into<String>, shuju: Value, miyao: &[u8]) -> HttpResponse {
    jiamichuanshuzhongjian::jiamixiangying(jiekouxtzhuti::chenggong(xiaoxi, shuju), miyao)
}

#[allow(non_upper_case_globals)]
const putongkezhicaozuo: &[&str] = &[
    "ribao_xinzeng",
    "ribao_chaxun_yonghuid",
    "ribao_chaxun_yonghuid_fenye",
    "ribao_tongji_yonghuid_zongshu",
];

fn shifouputongkezhixing(caozuo: &str) -> bool {
    putongkezhicaozuo.iter().any(|v| *v == caozuo)
}


async fn shifouguanlicaozuoquanxian(yonghuzuid: &str) -> bool {
    let shifoujiekouyunxu = yonghuyanzheng::jianchajiekouquanxian(yonghuzuid, "/jiekou/xitong/ribao")
        .await
        .is_ok();
    if !shifoujiekouyunxu {
        return false;
    }
    let zu = match shujucaozuo_yonghuzu::chaxun_id(yonghuzuid).await {
        Some(v) => v,
        None => return false,
    };
    let mingcheng = zu.get("mingcheng").and_then(|v| v.as_str()).unwrap_or("");
    mingcheng != "user"
}

fn xieruwodeyonghuid(qingqiu: &mut Qingqiuti, yonghuid: &str) -> bool {
    let duixiang = match qingqiu.canshu.as_object_mut() {
        Some(d) => d,
        None => return false,
    };
    let caozuo = qingqiu.caozuo.as_str();
    if caozuo == "ribao_xinzeng" {
        duixiang.insert("yonghuid".to_string(), serde_json::json!(yonghuid));
        return true;
    }
    if caozuo == "ribao_chaxun_yonghuid" || caozuo == "ribao_chaxun_yonghuid_fenye" || caozuo == "ribao_tongji_yonghuid_zongshu" {
        duixiang.insert("yonghuid".to_string(), serde_json::json!(yonghuid));
        if caozuo == "ribao_chaxun_yonghuid_fenye" {
            if !duixiang.contains_key("yeshu") {
                duixiang.insert("yeshu".to_string(), serde_json::json!(1));
            }
            if !duixiang.contains_key("meiyetiaoshu") {
                duixiang.insert("meiyetiaoshu".to_string(), serde_json::json!(10));
            }
        }
        return true;
    }
    false
}

async fn chulicaozuo(mingwen: &[u8], miyao: &[u8]) -> HttpResponse {
    let qingqiu: Qingqiuti = match serde_json::from_slice::<Qingqiuti>(mingwen) {
        Ok(q) => q,
        Err(_) => return jiamishibai(&cuowu::qingqiugeshibuzhengque, miyao),
    };

    match qingqiu.caozuo.as_str() {
        "leixing_xinzeng" => {
            let canshu = jiexi_canshu!(qingqiu, Leixingxinzengcanshu, miyao);
            match shujucaozuo_biaoqianleixing::mingchengcunzai(&canshu.mingcheng).await {
                true => jiamishibai(&cuowu::mingchengyicunzai, miyao),
                false => chuli_xinzeng!(canshu, miyao, shujucaozuo_biaoqianleixing::xinzeng(&canshu.mingcheng))
            }
        }
        "leixing_shanchu" => {
            let canshu = jiexi_canshu!(qingqiu, Idcanshu, miyao);
            chuli_shanchu_gengxin!(canshu, miyao, shujucaozuo_biaoqianleixing::shanchu(&canshu.id), "删除成功", &cuowu::biaoqianleixingbucunzai)
        }
        "leixing_gengxin" => {
            let canshu = jiexi_canshu!(qingqiu, Leixinggengxincanshu, miyao);
            chuli_shanchu_gengxin!(canshu, miyao, shujucaozuo_biaoqianleixing::gengxin(&canshu.id, &canshu.mingcheng), "更新成功", &cuowu::biaoqianleixingbucunzai)
        }
        "leixing_chaxun_id" => {
            let canshu = jiexi_canshu!(qingqiu, Idcanshu, miyao);
            chuli_chaxun!(canshu, miyao, shujucaozuo_biaoqianleixing::chaxun_id(&canshu.id), &cuowu::biaoqianleixingbucunzai)
        }
        "leixing_chaxun_mingcheng" => {
            let canshu = jiexi_canshu!(qingqiu, Mingchengcanshu, miyao);
            chuli_chaxun!(canshu, miyao, shujucaozuo_biaoqianleixing::chaxun_mingcheng(&canshu.mingcheng), &cuowu::biaoqianleixingbucunzai)
        }
        "leixing_chaxun_quanbu" => {
            chuli_chaxun_liebiao!((), miyao, shujucaozuo_biaoqianleixing::chaxun_quanbu())
        }
        "biaoqian_xinzeng" => {
            let canshu = jiexi_canshu!(qingqiu, Biaoqianxinzengcanshu, miyao);
            chuli_xinzeng!(canshu, miyao, shujucaozuo_biaoqian::xinzeng(&canshu.leixingid, &canshu.zhi))
        }
        "biaoqian_shanchu" => {
            let canshu = jiexi_canshu!(qingqiu, Idcanshu, miyao);
            chuli_shanchu_gengxin!(canshu, miyao, shujucaozuo_biaoqian::shanchu(&canshu.id), "删除成功", &cuowu::biaoqianbucunzai)
        }
        "biaoqian_gengxin" => {
            let canshu = jiexi_canshu!(qingqiu, Biaoqiangengxincanshu, miyao);
            chuli_shanchu_gengxin!(canshu, miyao, shujucaozuo_biaoqian::gengxin(&canshu.id, &canshu.zhi), "更新成功", &cuowu::biaoqianbucunzai)
        }
        "biaoqian_chaxun_id" => {
            let canshu = jiexi_canshu!(qingqiu, Idcanshu, miyao);
            chuli_chaxun!(canshu, miyao, shujucaozuo_biaoqian::chaxun_id(&canshu.id), &cuowu::biaoqianbucunzai)
        }
        "biaoqian_chaxun_leixingid" => {
            let canshu = jiexi_canshu!(qingqiu, Biaoqianchaxuncanshu, miyao);
            chuli_chaxun_liebiao!(canshu, miyao, shujucaozuo_biaoqian::chaxun_leixingid(&canshu.leixingid))
        }
        "biaoqian_chaxun_leixingid_zhi" => {
            let canshu = jiexi_canshu!(qingqiu, Biaoqianchaxunzhicanshu, miyao);
            chuli_chaxun!(canshu, miyao, shujucaozuo_biaoqian::chaxun_leixingid_zhi(&canshu.leixingid, &canshu.zhi), &cuowu::biaoqianbucunzai)
        }
        "biaoqian_chaxun_quanbu" => {
            chuli_chaxun_liebiao!((), miyao, shujucaozuo_biaoqian::chaxun_quanbu())
        }
        "biaoqian_chaxun_leixing" => {
            let canshu = jiexi_canshu!(qingqiu, Biaoqianidcanshu, miyao);
            chuli_chaxun!(canshu, miyao, shujucaozuo_biaoqian::chaxun_leixing(&canshu.biaoqianid), &cuowu::biaoqianleixingbucunzai)
        }
        "ribao_xinzeng" => {
            let canshu = jiexi_canshu!(qingqiu, Ribaoxinzengcanshu, miyao);
            chuli_xinzeng!(canshu, miyao, shujucaozuo_ribao::xinzeng(&canshu.yonghuid, &canshu.neirong, &canshu.fabushijian))
        }
        "ribao_shanchu" => {
            let canshu = jiexi_canshu!(qingqiu, Idcanshu, miyao);
            chuli_shanchu_gengxin!(canshu, miyao, shujucaozuo_ribao::shanchu(&canshu.id), "删除成功", &cuowu::ribaobucunzai)
        }
        "ribao_gengxin" => {
            let canshu = jiexi_canshu!(qingqiu, Ribaogengxincanshu, miyao);
            let ziduanlie: Vec<(&str, &str)> = canshu.ziduanlie.iter()
                .map(|(k, v)| (k.as_str(), v.as_str()))
                .collect();
            chuli_shanchu_gengxin!(canshu, miyao, shujucaozuo_ribao::gengxin(&canshu.id, &ziduanlie), "更新成功", &cuowu::ribaobucunzai)
        }
        "ribao_chaxun_id" => {
            let canshu = jiexi_canshu!(qingqiu, Idcanshu, miyao);
            chuli_chaxun!(canshu, miyao, shujucaozuo_ribao::chaxun_id(&canshu.id), &cuowu::ribaobucunzai)
        }
        "ribao_chaxun_yonghuid" => {
            let canshu = jiexi_canshu!(qingqiu, Yonghuidcanshu, miyao);
            chuli_chaxun_liebiao!(canshu, miyao, shujucaozuo_ribao::chaxun_yonghuid(&canshu.yonghuid))
        }
        "ribao_chaxun_quanbu" => {
            chuli_chaxun_liebiao!((), miyao, shujucaozuo_ribao::chaxun_quanbu())
        }
        "ribao_chaxun_fenye" => {
            let canshu = jiexi_canshu!(qingqiu, Fenyecanshu, miyao);
            match shujucaozuo_ribao::chaxun_fenye(canshu.yeshu, canshu.meiyetiaoshu).await {
                Some(jieguo) => {
                    let zongshu = shujucaozuo_ribao::tongji_zongshu().await.unwrap_or(0);
                    jiamichenggong("查询成功", serde_json::json!({"liebiao": jieguo, "zongshu": zongshu}), miyao)
                }
                None => jiamishibai(&cuowu::chaxunshibai, miyao),
            }
        }
        "ribao_chaxun_yonghuid_fenye" => {
            let canshu = jiexi_canshu!(qingqiu, Yonghuidfenyecanshu, miyao);
            let liebiao = shujucaozuo_ribao::chaxun_yonghuid_fenye(&canshu.yonghuid, canshu.yeshu, canshu.meiyetiaoshu).await.unwrap_or_default();
            let zongshu = shujucaozuo_ribao::tongji_yonghuid_zongshu(&canshu.yonghuid).await.unwrap_or(0);
            jiamichenggong("查询成功", serde_json::json!({"liebiao": liebiao, "zongshu": zongshu}), miyao)
        }
        "ribao_tongji_zongshu" => {
            match shujucaozuo_ribao::tongji_zongshu().await {
                Some(zongshu) => jiamichenggong("统计成功", serde_json::json!({"count": zongshu}), miyao),
                None => jiamishibai(&cuowu::tongjishibai, miyao),
            }
        }
        "ribao_tongji_yonghuid_zongshu" => {
            let canshu = jiexi_canshu!(qingqiu, Yonghuidcanshu, miyao);
            match shujucaozuo_ribao::tongji_yonghuid_zongshu(&canshu.yonghuid).await {
                Some(zongshu) => jiamichenggong("统计成功", serde_json::json!({"count": zongshu}), miyao),
                None => jiamishibai(&cuowu::tongjishibai, miyao),
            }
        }
        "ribao_chaxun_guanjianci_fenye" => {
            let canshu = jiexi_canshu!(qingqiu, Guanjiancifenyecanshu, miyao);
            let liebiao = shujucaozuo_ribao::chaxun_guanjianci_fenye(&canshu.guanjianci, canshu.yeshu, canshu.meiyetiaoshu).await.unwrap_or_default();
            let zongshu = shujucaozuo_ribao::tongji_guanjianci_zongshu(&canshu.guanjianci).await.unwrap_or(0);
            jiamichenggong("查询成功", serde_json::json!({"liebiao": liebiao, "zongshu": zongshu}), miyao)
        }
        "guanlian_xinzeng" => {
            let canshu = jiexi_canshu!(qingqiu, Guanlianxinzengcanshu, miyao);
            chuli_shanchu_gengxin!(canshu, miyao, shujucaozuo_ribao_biaoqian::xinzeng(&canshu.ribaoid, &canshu.biaoqianid), "关联成功", &cuowu::guanlianshibai)
        }
        "guanlian_shanchu_ribaoid" => {
            let canshu = jiexi_canshu!(qingqiu, Ribaoidcanshu, miyao);
            match shujucaozuo_ribao_biaoqian::shanchu_ribaoid(&canshu.ribaoid).await {
                Some(n) => jiamichenggong("删除成功", serde_json::json!({"count": n}), miyao),
                None => jiamishibai(&cuowu::shanchushibai, miyao),
            }
        }
        "guanlian_shanchu" => {
            let canshu = jiexi_canshu!(qingqiu, Guanlianshanchu, miyao);
            chuli_shanchu_gengxin!(canshu, miyao, shujucaozuo_ribao_biaoqian::shanchu_guanlian(&canshu.ribaoid, &canshu.biaoqianid), "删除成功", &cuowu::guanlianbucunzai)
        }
        "guanlian_chaxun_ribaoid" => {
            let canshu = jiexi_canshu!(qingqiu, Ribaoidcanshu, miyao);
            chuli_chaxun_liebiao!(canshu, miyao, shujucaozuo_ribao_biaoqian::chaxun_ribaoid(&canshu.ribaoid))
        }
        "guanlian_chaxun_biaoqianid" => {
            let canshu = jiexi_canshu!(qingqiu, Biaoqianidcanshu, miyao);
            chuli_chaxun_liebiao!(canshu, miyao, shujucaozuo_ribao_biaoqian::chaxun_biaoqianid(&canshu.biaoqianid))
        }
        "guanlian_chaxun_leixingmingcheng_zhi" => {
            let canshu = jiexi_canshu!(qingqiu, Leixingmingchengzhicanshu, miyao);
            chuli_chaxun_liebiao!(canshu, miyao, shujucaozuo_ribao_biaoqian::chaxun_leixingmingcheng_zhi(&canshu.leixingmingcheng, &canshu.zhi))
        }
        "guanlian_chaxun_ribaoid_daixinxi" => {
            let canshu = jiexi_canshu!(qingqiu, Ribaoidcanshu, miyao);
            chuli_chaxun_liebiao!(canshu, miyao, shujucaozuo_ribao_biaoqian::chaxun_ribaoid_daixinxi(&canshu.ribaoid))
        }
        "guanlian_chaxun_xiangguanbiaoqian" => {
            let canshu = jiexi_canshu!(qingqiu, Xiangguanbiaoqiancanshu, miyao);
            chuli_chaxun_liebiao!(canshu, miyao, shujucaozuo_ribao_biaoqian::chaxun_xiangguanbiaoqian(&canshu.biaoqianid, &canshu.leixingmingcheng))
        }
        "guanlian_piliang_xinzeng" => {
            let canshu = jiexi_canshu!(qingqiu, Piliangguanliancanshu, miyao);
            let biaoqianidlie: Vec<&str> = canshu.biaoqianidlie.iter().map(|s| s.as_str()).collect();
            match shujucaozuo_ribao_biaoqian::piliang_xinzeng(&canshu.ribaoid, &biaoqianidlie).await {
                Some(n) => jiamichenggong("批量关联成功", serde_json::json!({"count": n}), miyao),
                None => jiamishibai(&cuowu::piliangguanlianshibai, miyao),
            }
        }
        "renwu_biaoqian_ai_chuli" => {
            let canshu = jiexi_canshu!(qingqiu, Renwuchulicanshu, miyao);
            match gongju_ribaorenwuchuli::zhixing_neibu(canshu.shuliang.unwrap_or(0)).await {
                Ok(shuju) => jiamichenggong("处理成功", shuju, miyao),
                Err(xiaoxi) => jiamishibai_dongtai(500, xiaoxi, miyao),
            }
        }
        _ => jiamishibai(&cuowu::bucaozuoleixing, miyao),
    }
}

/// 日报管理接口处理函数
pub async fn chuli(req: HttpRequest, ti: web::Bytes) -> HttpResponse {
    let miyao = match jiamichuanshuzhongjian::paishengyao(&req).await {
        Some(m) => m,
        None => return jiekouxtzhuti::shibai(401, "加密会话无效"),
    };

    let mingwen = match jiamichuanshuzhongjian::jiemiqingqiuti(&ti, &miyao) {
        Some(m) => m,
        None => return jiekouxtzhuti::shibai(400, "解密失败"),
    };

    let mut qingqiu: Qingqiuti = match serde_json::from_slice::<Qingqiuti>(&mingwen) {
        Ok(q) => q,
        Err(_) => return jiekouxtzhuti::shibai(400, "请求参数格式错误"),
    };

    let zaiti = req.extensions().get::<jwtgongju::Zaiti>().cloned().unwrap();

    let shifouquanxian = shifouguanlicaozuoquanxian(&zaiti.yonghuzuid).await;
    if !shifouquanxian && !shifouputongkezhixing(&qingqiu.caozuo) {
        return jiamishibai(&cuowu::quanxianbuzu, &miyao);
    }

    if !shifouquanxian {
        if !xieruwodeyonghuid(&mut qingqiu, &zaiti.yonghuid) {
            return jiamishibai(&cuowu::putongkezhixianshibai, &miyao);
        }
    } else {
        let caozuo = qingqiu.caozuo.as_str();
        if caozuo == "ribao_xinzeng" || caozuo == "ribao_chaxun_yonghuid" || caozuo == "ribao_chaxun_yonghuid_fenye" || caozuo == "ribao_tongji_yonghuid_zongshu" {
            if let Some(duixiang) = qingqiu.canshu.as_object_mut() {
                if duixiang.get("yonghuid").and_then(|v| v.as_str()).unwrap_or("").is_empty() {
                    duixiang.insert("yonghuid".to_string(), serde_json::json!(zaiti.yonghuid));
                }
            }
        }
    }

    let xinmingwen = match serde_json::to_vec(&qingqiu) {
        Ok(m) => m,
        Err(_) => return jiamishibai(&cuowu::qingqiuchulishibai, &miyao),
    };

    chulicaozuo(&xinmingwen, &miyao).await
}
