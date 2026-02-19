use actix_web::{HttpRequest, HttpResponse, web};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::jiekouxt::jiekouxtzhuti::{self, Jiekoudinyi, Qingqiufangshi};
use crate::jiekouxt::jiamichuanshu::jiamichuanshuzhongjian;
use crate::shujuku::psqlshujuku::shujubiao_nr::ribao::{
    shujucaozuo_biaoqianleixing, shujucaozuo_biaoqian,
    shujucaozuo_ribao, shujucaozuo_ribao_biaoqian
};
use crate::shujuku::psqlshujuku::shujubiao_nr::yonghu::{shujucaozuo_yonghuzu, yonghuyanzheng};
use crate::shujuku::psqlshujuku::shujubiao_nr::yonghu::yonghuyanzheng::Lingpaicuowu;
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
struct Qingqiuti {
    caozuo: String,
    #[serde(flatten)]
    canshu: Value,
}

#[derive(Deserialize)]
struct Idcanshu {
    id: String,
}

#[derive(Deserialize)]
struct Mingchengcanshu {
    mingcheng: String,
}

#[derive(Deserialize)]
struct Leixingxinzengcanshu {
    mingcheng: String,
}

#[derive(Deserialize)]
struct Leixinggengxincanshu {
    id: String,
    mingcheng: String,
}

#[derive(Deserialize)]
struct Biaoqianxinzengcanshu {
    leixingid: String,
    zhi: String,
}

#[derive(Deserialize)]
struct Biaoqiangengxincanshu {
    id: String,
    zhi: String,
}

#[derive(Deserialize)]
struct Biaoqianchaxuncanshu {
    leixingid: String,
}

#[derive(Deserialize)]
struct Biaoqianchaxunzhicanshu {
    leixingid: String,
    zhi: String,
}

#[derive(Deserialize)]
struct Ribaoxinzengcanshu {
    yonghuid: String,
    neirong: String,
    fabushijian: String,
}

#[derive(Deserialize)]
struct Ribaogengxincanshu {
    id: String,
    ziduanlie: Vec<(String, String)>,
}

#[derive(Deserialize)]
struct Yonghuidcanshu {
    yonghuid: String,
}

#[derive(Deserialize)]
struct Fenyecanshu {
    yeshu: i64,
    meiyetiaoshu: i64,
}

#[derive(Deserialize)]
struct Yonghuidfenyecanshu {
    yonghuid: String,
    yeshu: i64,
    meiyetiaoshu: i64,
}

#[derive(Deserialize)]
struct Guanlianxinzengcanshu {
    ribaoid: String,
    biaoqianid: String,
}

#[derive(Deserialize)]
struct Ribaoidcanshu {
    ribaoid: String,
}

#[derive(Deserialize)]
struct Biaoqianidcanshu {
    biaoqianid: String,
}

#[derive(Deserialize)]
struct Guanlianshanchu {
    ribaoid: String,
    biaoqianid: String,
}

#[derive(Deserialize)]
struct Piliangguanliancanshu {
    ribaoid: String,
    biaoqianidlie: Vec<String>,
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

fn huoqutoken(req: &HttpRequest) -> Option<String> {
    req.headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .map(String::from)
}

async fn shifouquanxianyonghu(yonghuzuid: &str) -> bool {
    let jieguo = shujucaozuo_yonghuzu::chaxun_id(yonghuzuid).await;
    if jieguo.is_none() {
        return false;
    }
    let zu = jieguo.unwrap();
    let mingcheng = zu.get("mingcheng").and_then(|v| v.as_str()).unwrap_or("");
    let beizhu = zu.get("beizhu").and_then(|v| v.as_str()).unwrap_or("");
    let shifouguanliyuan = mingcheng == "root" || beizhu.contains("root授权");
    shifouguanliyuan
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
            let canshu: Leixingxinzengcanshu = match serde_json::from_value(qingqiu.canshu) {
                Ok(c) => c,
                Err(_) => return jiamishibai(&cuowu::canshugeshibuzhengque, miyao),
            };
            match shujucaozuo_biaoqianleixing::xinzeng(&canshu.mingcheng).await {
                Some(id) => jiamichenggong("创建成功", serde_json::json!({"id": id}), miyao),
                None => jiamishibai(&cuowu::chuangjianshi, miyao),
            }
        }
        "leixing_shanchu" => {
            let canshu: Idcanshu = match serde_json::from_value(qingqiu.canshu) {
                Ok(c) => c,
                Err(_) => return jiamishibai(&cuowu::canshugeshibuzhengque, miyao),
            };
            match shujucaozuo_biaoqianleixing::shanchu(&canshu.id).await {
                Some(n) if n > 0 => jiamichenggong("删除成功", serde_json::json!({"count": n}), miyao),
                _ => jiamishibai(&cuowu::biaoqianleixingbucunzai, miyao),
            }
        }
        "leixing_gengxin" => {
            let canshu: Leixinggengxincanshu = match serde_json::from_value(qingqiu.canshu) {
                Ok(c) => c,
                Err(_) => return jiamishibai(&cuowu::canshugeshibuzhengque, miyao),
            };
            match shujucaozuo_biaoqianleixing::gengxin(&canshu.id, &canshu.mingcheng).await {
                Some(n) if n > 0 => jiamichenggong("更新成功", serde_json::json!({"count": n}), miyao),
                _ => jiamishibai(&cuowu::biaoqianleixingbucunzai, miyao),
            }
        }
        "leixing_chaxun_id" => {
            let canshu: Idcanshu = match serde_json::from_value(qingqiu.canshu) {
                Ok(c) => c,
                Err(_) => return jiamishibai(&cuowu::canshugeshibuzhengque, miyao),
            };
            match shujucaozuo_biaoqianleixing::chaxun_id(&canshu.id).await {
                Some(jieguo) => jiamichenggong("查询成功", jieguo, miyao),
                None => jiamishibai(&cuowu::biaoqianleixingbucunzai, miyao),
            }
        }
        "leixing_chaxun_mingcheng" => {
            let canshu: Mingchengcanshu = match serde_json::from_value(qingqiu.canshu) {
                Ok(c) => c,
                Err(_) => return jiamishibai(&cuowu::canshugeshibuzhengque, miyao),
            };
            match shujucaozuo_biaoqianleixing::chaxun_mingcheng(&canshu.mingcheng).await {
                Some(jieguo) => jiamichenggong("查询成功", jieguo, miyao),
                None => jiamishibai(&cuowu::biaoqianleixingbucunzai, miyao),
            }
        }
        "leixing_chaxun_quanbu" => {
            match shujucaozuo_biaoqianleixing::chaxun_quanbu().await {
                Some(jieguo) => jiamichenggong("查询成功", serde_json::json!(jieguo), miyao),
                None => jiamishibai(&cuowu::chaxunshibai, miyao),
            }
        }
        "biaoqian_xinzeng" => {
            let canshu: Biaoqianxinzengcanshu = match serde_json::from_value(qingqiu.canshu) {
                Ok(c) => c,
                Err(_) => return jiamishibai(&cuowu::canshugeshibuzhengque, miyao),
            };
            match shujucaozuo_biaoqian::xinzeng(&canshu.leixingid, &canshu.zhi).await {
                Some(id) => jiamichenggong("创建成功", serde_json::json!({"id": id}), miyao),
                None => jiamishibai(&cuowu::chuangjianshi, miyao),
            }
        }
        "biaoqian_shanchu" => {
            let canshu: Idcanshu = match serde_json::from_value(qingqiu.canshu) {
                Ok(c) => c,
                Err(_) => return jiamishibai(&cuowu::canshugeshibuzhengque, miyao),
            };
            match shujucaozuo_biaoqian::shanchu(&canshu.id).await {
                Some(n) if n > 0 => jiamichenggong("删除成功", serde_json::json!({"count": n}), miyao),
                _ => jiamishibai(&cuowu::biaoqianbucunzai, miyao),
            }
        }
        "biaoqian_gengxin" => {
            let canshu: Biaoqiangengxincanshu = match serde_json::from_value(qingqiu.canshu) {
                Ok(c) => c,
                Err(_) => return jiamishibai(&cuowu::canshugeshibuzhengque, miyao),
            };
            match shujucaozuo_biaoqian::gengxin(&canshu.id, &canshu.zhi).await {
                Some(n) if n > 0 => jiamichenggong("更新成功", serde_json::json!({"count": n}), miyao),
                _ => jiamishibai(&cuowu::biaoqianbucunzai, miyao),
            }
        }
        "biaoqian_chaxun_id" => {
            let canshu: Idcanshu = match serde_json::from_value(qingqiu.canshu) {
                Ok(c) => c,
                Err(_) => return jiamishibai(&cuowu::canshugeshibuzhengque, miyao),
            };
            match shujucaozuo_biaoqian::chaxun_id(&canshu.id).await {
                Some(jieguo) => jiamichenggong("查询成功", jieguo, miyao),
                None => jiamishibai(&cuowu::biaoqianbucunzai, miyao),
            }
        }
        "biaoqian_chaxun_leixingid" => {
            let canshu: Biaoqianchaxuncanshu = match serde_json::from_value(qingqiu.canshu) {
                Ok(c) => c,
                Err(_) => return jiamishibai(&cuowu::canshugeshibuzhengque, miyao),
            };
            match shujucaozuo_biaoqian::chaxun_leixingid(&canshu.leixingid).await {
                Some(jieguo) => jiamichenggong("查询成功", serde_json::json!(jieguo), miyao),
                None => jiamishibai(&cuowu::chaxunshibai, miyao),
            }
        }
        "biaoqian_chaxun_leixingid_zhi" => {
            let canshu: Biaoqianchaxunzhicanshu = match serde_json::from_value(qingqiu.canshu) {
                Ok(c) => c,
                Err(_) => return jiamishibai(&cuowu::canshugeshibuzhengque, miyao),
            };
            match shujucaozuo_biaoqian::chaxun_leixingid_zhi(&canshu.leixingid, &canshu.zhi).await {
                Some(jieguo) => jiamichenggong("查询成功", jieguo, miyao),
                None => jiamishibai(&cuowu::biaoqianbucunzai, miyao),
            }
        }
        "biaoqian_chaxun_quanbu" => {
            match shujucaozuo_biaoqian::chaxun_quanbu().await {
                Some(jieguo) => jiamichenggong("查询成功", serde_json::json!(jieguo), miyao),
                None => jiamishibai(&cuowu::chaxunshibai, miyao),
            }
        }
        "ribao_xinzeng" => {
            let canshu: Ribaoxinzengcanshu = match serde_json::from_value(qingqiu.canshu) {
                Ok(c) => c,
                Err(_) => return jiamishibai(&cuowu::canshugeshibuzhengque, miyao),
            };
            shujucaozuo_ribao::xinzeng(&canshu.yonghuid, &canshu.neirong, &canshu.fabushijian)
                .await
                .map(|id| jiamichenggong("创建成功", serde_json::json!({"id": id}), miyao))
                .unwrap_or_else(|| jiamishibai(&cuowu::chuangjianshi, miyao))
        }
        "ribao_shanchu" => {
            let canshu: Idcanshu = match serde_json::from_value(qingqiu.canshu) {
                Ok(c) => c,
                Err(_) => return jiamishibai(&cuowu::canshugeshibuzhengque, miyao),
            };
            match shujucaozuo_ribao::shanchu(&canshu.id).await {
                Some(n) if n > 0 => jiamichenggong("删除成功", serde_json::json!({"count": n}), miyao),
                _ => jiamishibai(&cuowu::ribaobucunzai, miyao),
            }
        }
        "ribao_gengxin" => {
            let canshu: Ribaogengxincanshu = match serde_json::from_value(qingqiu.canshu) {
                Ok(c) => c,
                Err(_) => return jiamishibai(&cuowu::canshugeshibuzhengque, miyao),
            };
            let ziduanlie: Vec<(&str, &str)> = canshu.ziduanlie.iter()
                .map(|(k, v)| (k.as_str(), v.as_str()))
                .collect();
            match shujucaozuo_ribao::gengxin(&canshu.id, &ziduanlie).await {
                Some(n) if n > 0 => jiamichenggong("更新成功", serde_json::json!({"count": n}), miyao),
                _ => jiamishibai(&cuowu::ribaobucunzai, miyao),
            }
        }
        "ribao_chaxun_id" => {
            let canshu: Idcanshu = match serde_json::from_value(qingqiu.canshu) {
                Ok(c) => c,
                Err(_) => return jiamishibai(&cuowu::canshugeshibuzhengque, miyao),
            };
            match shujucaozuo_ribao::chaxun_id(&canshu.id).await {
                Some(jieguo) => jiamichenggong("查询成功", jieguo, miyao),
                None => jiamishibai(&cuowu::ribaobucunzai, miyao),
            }
        }
        "ribao_chaxun_yonghuid" => {
            let canshu: Yonghuidcanshu = match serde_json::from_value(qingqiu.canshu) {
                Ok(c) => c,
                Err(_) => return jiamishibai(&cuowu::canshugeshibuzhengque, miyao),
            };
            match shujucaozuo_ribao::chaxun_yonghuid(&canshu.yonghuid).await {
                Some(jieguo) => jiamichenggong("查询成功", serde_json::json!(jieguo), miyao),
                None => jiamishibai(&cuowu::chaxunshibai, miyao),
            }
        }
        "ribao_chaxun_quanbu" => {
            match shujucaozuo_ribao::chaxun_quanbu().await {
                Some(jieguo) => jiamichenggong("查询成功", serde_json::json!(jieguo), miyao),
                None => jiamishibai(&cuowu::chaxunshibai, miyao),
            }
        }
        "ribao_chaxun_fenye" => {
            let canshu: Fenyecanshu = match serde_json::from_value(qingqiu.canshu) {
                Ok(c) => c,
                Err(_) => return jiamishibai(&cuowu::canshugeshibuzhengque, miyao),
            };
            match shujucaozuo_ribao::chaxun_fenye(canshu.yeshu, canshu.meiyetiaoshu).await {
                Some(jieguo) => {
                    let zongshu = shujucaozuo_ribao::tongji_zongshu().await.unwrap_or(0);
                    jiamichenggong("查询成功", serde_json::json!({"liebiao": jieguo, "zongshu": zongshu}), miyao)
                }
                None => jiamishibai(&cuowu::chaxunshibai, miyao),
            }
        }
        "ribao_chaxun_yonghuid_fenye" => {
            let canshu: Yonghuidfenyecanshu = match serde_json::from_value(qingqiu.canshu) {
                Ok(c) => c,
                Err(_) => return jiamishibai(&cuowu::canshugeshibuzhengque, miyao),
            };
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
            let canshu: Yonghuidcanshu = match serde_json::from_value(qingqiu.canshu) {
                Ok(c) => c,
                Err(_) => return jiamishibai(&cuowu::canshugeshibuzhengque, miyao),
            };
            match shujucaozuo_ribao::tongji_yonghuid_zongshu(&canshu.yonghuid).await {
                Some(zongshu) => jiamichenggong("统计成功", serde_json::json!({"count": zongshu}), miyao),
                None => jiamishibai(&cuowu::tongjishibai, miyao),
            }
        }
        "guanlian_xinzeng" => {
            let canshu: Guanlianxinzengcanshu = match serde_json::from_value(qingqiu.canshu) {
                Ok(c) => c,
                Err(_) => return jiamishibai(&cuowu::canshugeshibuzhengque, miyao),
            };
            match shujucaozuo_ribao_biaoqian::xinzeng(&canshu.ribaoid, &canshu.biaoqianid).await {
                Some(n) if n > 0 => jiamichenggong("关联成功", serde_json::json!({"count": n}), miyao),
                _ => jiamishibai(&cuowu::guanlianshibai, miyao),
            }
        }
        "guanlian_shanchu_ribaoid" => {
            let canshu: Ribaoidcanshu = match serde_json::from_value(qingqiu.canshu) {
                Ok(c) => c,
                Err(_) => return jiamishibai(&cuowu::canshugeshibuzhengque, miyao),
            };
            match shujucaozuo_ribao_biaoqian::shanchu_ribaoid(&canshu.ribaoid).await {
                Some(n) => jiamichenggong("删除成功", serde_json::json!({"count": n}), miyao),
                None => jiamishibai(&cuowu::shanchushibai, miyao),
            }
        }
        "guanlian_shanchu" => {
            let canshu: Guanlianshanchu = match serde_json::from_value(qingqiu.canshu) {
                Ok(c) => c,
                Err(_) => return jiamishibai(&cuowu::canshugeshibuzhengque, miyao),
            };
            match shujucaozuo_ribao_biaoqian::shanchu_guanlian(&canshu.ribaoid, &canshu.biaoqianid).await {
                Some(n) if n > 0 => jiamichenggong("删除成功", serde_json::json!({"count": n}), miyao),
                _ => jiamishibai(&cuowu::guanlianbucunzai, miyao),
            }
        }
        "guanlian_chaxun_ribaoid" => {
            let canshu: Ribaoidcanshu = match serde_json::from_value(qingqiu.canshu) {
                Ok(c) => c,
                Err(_) => return jiamishibai(&cuowu::canshugeshibuzhengque, miyao),
            };
            match shujucaozuo_ribao_biaoqian::chaxun_ribaoid(&canshu.ribaoid).await {
                Some(jieguo) => jiamichenggong("查询成功", serde_json::json!(jieguo), miyao),
                None => jiamishibai(&cuowu::chaxunshibai, miyao),
            }
        }
        "guanlian_chaxun_biaoqianid" => {
            let canshu: Biaoqianidcanshu = match serde_json::from_value(qingqiu.canshu) {
                Ok(c) => c,
                Err(_) => return jiamishibai(&cuowu::canshugeshibuzhengque, miyao),
            };
            match shujucaozuo_ribao_biaoqian::chaxun_biaoqianid(&canshu.biaoqianid).await {
                Some(jieguo) => jiamichenggong("查询成功", serde_json::json!(jieguo), miyao),
                None => jiamishibai(&cuowu::chaxunshibai, miyao),
            }
        }
        "guanlian_piliang_xinzeng" => {
            let canshu: Piliangguanliancanshu = match serde_json::from_value(qingqiu.canshu) {
                Ok(c) => c,
                Err(_) => return jiamishibai(&cuowu::canshugeshibuzhengque, miyao),
            };
            let biaoqianidlie: Vec<&str> = canshu.biaoqianidlie.iter().map(|s| s.as_str()).collect();
            match shujucaozuo_ribao_biaoqian::piliang_xinzeng(&canshu.ribaoid, &biaoqianidlie).await {
                Some(n) => jiamichenggong("批量关联成功", serde_json::json!({"count": n}), miyao),
                None => jiamishibai(&cuowu::piliangguanlianshibai, miyao),
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

    let token = match huoqutoken(&req) {
        Some(t) => t,
        None => return jiamishibai(&cuowu::queshouquanlingpai, &miyao),
    };

    let zaiti = match yonghuyanzheng::yanzhenglingpai(&token).await {
        Ok(z) => z,
        Err(Lingpaicuowu::Wuxiao) => return jiamishibai(&cuowu::lingpaiwuxiao, &miyao),
        Err(Lingpaicuowu::Yibeifengjin(yuanyin)) => return jiamishibai_dongtai(403, format!("账号已被封禁：{}", yuanyin), &miyao),
        Err(Lingpaicuowu::Quanxianbuzu) => return jiamishibai(&cuowu::quanxianbuzu, &miyao),
    };

    let shifouquanxian = shifouquanxianyonghu(&zaiti.yonghuzuid).await;
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
