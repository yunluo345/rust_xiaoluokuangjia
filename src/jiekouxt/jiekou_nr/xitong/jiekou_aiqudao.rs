use actix_web::{HttpRequest, HttpResponse, web};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::jiekouxt::jiekouxtzhuti::{self, Jiekoudinyi, Qingqiufangshi};
use crate::jiekouxt::jiamichuanshu::jiamichuanshuzhongjian;
use crate::shujuku::psqlshujuku::shujubiao_nr::ai::shujucaozuo_aiqudao;

#[allow(non_upper_case_globals)]
pub const dinyi: Jiekoudinyi = Jiekoudinyi {
    lujing: "/aiqudao",
    nicheng: "AI渠道管理",
    jieshao: "管理AI渠道的增删改查操作，需要管理员权限",
    fangshi: Qingqiufangshi::Post,
    jiami: true,
    xudenglu: true,
    xuyonghuzu: true,
    yunxuputong: false,
};

#[derive(Deserialize)]
struct Qingqiuti {
    caozuo: String,
    #[serde(flatten)]
    canshu: Value,
}

#[derive(Deserialize)]
struct Xinzengcanshu {
    mingcheng: String,
    leixing: String,
    jiekoudizhi: String,
    miyao: String,
    moxing: String,
    wendu: String,
    zuida_token: Option<String>,
    beizhu: Option<String>,
}

#[derive(Deserialize)]
struct Gengxincanshu {
    id: String,
    ziduanlie: Vec<(String, String)>,
}

#[derive(Deserialize)]
struct Idcanshu {
    id: String,
}

#[derive(Deserialize)]
struct Youxianjicanshu {
    id: String,
    youxianji: String,
}

/// 验证类型是否合法（只允许 openapi、xiangliang、yuyin）
fn yanzheng_leixing(leixing: &str) -> bool {
    matches!(leixing, "openapi" | "xiangliang" | "yuyin")
}

#[derive(Serialize)]
struct Caozuojieguo {
    chenggong: bool,
    shuju: Option<Value>,
}

fn jiamishibai(zhuangtaima: u16, xiaoxi: impl Into<String>, miyao: &[u8]) -> HttpResponse {
    jiamichuanshuzhongjian::jiamixiangying(jiekouxtzhuti::shibai(zhuangtaima, xiaoxi), miyao)
}

fn jiamichenggong(xiaoxi: impl Into<String>, shuju: Value, miyao: &[u8]) -> HttpResponse {
    jiamichuanshuzhongjian::jiamixiangying(jiekouxtzhuti::chenggong(xiaoxi, shuju), miyao)
}

async fn chulicaozuo(mingwen: &[u8], miyao: &[u8]) -> HttpResponse {
    let qingqiu: Qingqiuti = match serde_json::from_slice::<Qingqiuti>(mingwen) {
        Ok(q) => q,
        Err(_) => return jiamishibai(400, "请求参数格式错误", miyao),
    };

    match qingqiu.caozuo.as_str() {
        "chaxun_quanbu" => {
            match shujucaozuo_aiqudao::chaxun_quanbu().await {
                Some(jieguo) => jiamichenggong("查询成功", serde_json::json!(jieguo), miyao),
                None => jiamishibai(500, "查询失败", miyao),
            }
        }
        "chaxun_qiyong" => {
            match shujucaozuo_aiqudao::chaxun_qiyong().await {
                Some(jieguo) => jiamichenggong("查询成功", serde_json::json!(jieguo), miyao),
                None => jiamishibai(500, "查询失败", miyao),
            }
        }
        "chaxun_id" => {
            let canshu: Idcanshu = match serde_json::from_value(qingqiu.canshu) {
                Ok(c) => c,
                Err(_) => return jiamishibai(400, "参数格式错误", miyao),
            };
            match shujucaozuo_aiqudao::chaxun_id(&canshu.id).await {
                Some(jieguo) => jiamichenggong("查询成功", jieguo, miyao),
                None => jiamishibai(404, "渠道不存在", miyao),
            }
        }
        "xinzeng" => {
            let canshu: Xinzengcanshu = match serde_json::from_value(qingqiu.canshu) {
                Ok(c) => c,
                Err(_) => return jiamishibai(400, "参数格式错误", miyao),
            };
            if !yanzheng_leixing(&canshu.leixing) {
                return jiamishibai(400, "类型只能是 openapi、xiangliang 或 yuyin", miyao);
            }
            if shujucaozuo_aiqudao::mingchengcunzai(&canshu.mingcheng).await {
                return jiamishibai(400, "渠道名称已存在", miyao);
            }
            let zuida_token = canshu.zuida_token.as_deref().unwrap_or("0");
            match shujucaozuo_aiqudao::xinzeng(
                &canshu.mingcheng,
                &canshu.leixing,
                &canshu.jiekoudizhi,
                &canshu.miyao,
                &canshu.moxing,
                &canshu.wendu,
                zuida_token,
                canshu.beizhu.as_deref(),
            ).await {
                Some(id) => jiamichenggong("新增成功", serde_json::json!({"id": id}), miyao),
                None => jiamishibai(500, "新增失败", miyao),
            }
        }
        "gengxin" => {
            let canshu: Gengxincanshu = match serde_json::from_value(qingqiu.canshu) {
                Ok(c) => c,
                Err(_) => return jiamishibai(400, "参数格式错误", miyao),
            };
            if canshu.ziduanlie.is_empty() {
                return jiamishibai(400, "更新字段不能为空", miyao);
            }
            // 检查是否更新了类型字段，如果有则验证
            for (key, value) in &canshu.ziduanlie {
                if key == "leixing" && !yanzheng_leixing(value) {
                    return jiamishibai(400, "类型只能是 openapi、xiangliang 或 yuyin", miyao);
                }
            }
            let ziduanlie: Vec<(&str, &str)> = canshu.ziduanlie.iter()
                .map(|(k, v)| (k.as_str(), v.as_str()))
                .collect();
            match shujucaozuo_aiqudao::gengxin(&canshu.id, &ziduanlie).await {
                Some(n) if n > 0 => jiamichenggong("更新成功", serde_json::json!({"yingxiang": n}), miyao),
                Some(_) => jiamishibai(404, "渠道不存在", miyao),
                None => jiamishibai(500, "更新失败", miyao),
            }
        }
        "shanchu" => {
            let canshu: Idcanshu = match serde_json::from_value(qingqiu.canshu) {
                Ok(c) => c,
                Err(_) => return jiamishibai(400, "参数格式错误", miyao),
            };
            match shujucaozuo_aiqudao::shanchu(&canshu.id).await {
                Some(n) if n > 0 => jiamichenggong("删除成功", serde_json::json!({"yingxiang": n}), miyao),
                Some(_) => jiamishibai(404, "渠道不存在", miyao),
                None => jiamishibai(500, "删除失败", miyao),
            }
        }
        "qiehuanzhuangtai" => {
            let canshu: Idcanshu = match serde_json::from_value(qingqiu.canshu) {
                Ok(c) => c,
                Err(_) => return jiamishibai(400, "参数格式错误", miyao),
            };
            match shujucaozuo_aiqudao::qiehuanzhuangtai(&canshu.id).await {
                Some(n) if n > 0 => jiamichenggong("状态切换成功", serde_json::json!({"yingxiang": n}), miyao),
                Some(_) => jiamishibai(404, "渠道不存在", miyao),
                None => jiamishibai(500, "操作失败", miyao),
            }
        }
        "gengxinyouxianji" => {
            let canshu: Youxianjicanshu = match serde_json::from_value(qingqiu.canshu) {
                Ok(c) => c,
                Err(_) => return jiamishibai(400, "参数格式错误", miyao),
            };
            match shujucaozuo_aiqudao::gengxinyouxianji(&canshu.id, &canshu.youxianji).await {
                Some(n) if n > 0 => jiamichenggong("优先级更新成功", serde_json::json!({"yingxiang": n}), miyao),
                Some(_) => jiamishibai(404, "渠道不存在", miyao),
                None => jiamishibai(500, "更新失败", miyao),
            }
        }
        _ => jiamishibai(400, "不支持的操作类型", miyao),
    }
}

/// AI渠道管理接口处理函数
pub async fn chuli(req: HttpRequest, ti: web::Bytes) -> HttpResponse {
    let miyao = match jiamichuanshuzhongjian::paishengyao(&req).await {
        Some(m) => m,
        None => return jiekouxtzhuti::shibai(401, "加密会话无效"),
    };
    match jiamichuanshuzhongjian::jiemiqingqiuti(&ti, &miyao) {
        Some(mingwen) => chulicaozuo(&mingwen, &miyao).await,
        None => jiekouxtzhuti::shibai(400, "解密请求体失败"),
    }
}
