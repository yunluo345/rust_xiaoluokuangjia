use actix_web::{HttpRequest, HttpResponse, web};
use serde::Deserialize;
use crate::jiekouxt::jiekouxtzhuti::{self, Jiekoudinyi, Qingqiufangshi};
use crate::jiekouxt::jiamichuanshu::jiamichuanshuzhongjian;
use crate::shujuku::psqlshujuku::shujubiao_nr::yonghu::yonghuyanzheng::{self, Denglucuowu};

#[allow(non_upper_case_globals)]
pub const dinyi: Jiekoudinyi = Jiekoudinyi {
    lujing: "/denglu",
    nicheng: "用户登录",
    jieshao: "通过账号密码登录，返回JWT令牌",
    fangshi: Qingqiufangshi::Post,
    jiami: true,
    xudenglu: false,
    xuyonghuzu: false,
    yunxuputong: false,
};

#[derive(Deserialize)]
struct Qingqiuti {
    zhanghao: String,
    mima: String,
}

fn jiamishibai(zhuangtaima: u16, xiaoxi: impl Into<String>, miyao: &[u8]) -> HttpResponse {
    jiamichuanshuzhongjian::jiamixiangying(jiekouxtzhuti::shibai(zhuangtaima, xiaoxi), miyao)
}

async fn yanzhengdenglu(mingwen: &[u8], miyao: &[u8]) -> HttpResponse {
    let qingqiu: Qingqiuti = match serde_json::from_slice::<Qingqiuti>(mingwen) {
        Ok(q) if !q.zhanghao.is_empty() && !q.mima.is_empty() => q,
        Ok(_) => return jiamishibai(400, "账号或密码不能为空", miyao),
        Err(_) => return jiamishibai(400, "请求参数格式错误", miyao),
    };
    match yonghuyanzheng::denglu(&qingqiu.zhanghao, &qingqiu.mima).await {
        Ok(jieguo) => jiamichuanshuzhongjian::jiamixiangying(jiekouxtzhuti::chenggong("登录成功", jieguo), miyao),
        Err(Denglucuowu::Zhanghaomimacuowu) => jiamishibai(401, "账号或密码错误", miyao),
        Err(Denglucuowu::Yibeifengjin(yuanyin)) => jiamishibai(403, format!("账号已被封禁：{}", yuanyin), miyao),
        Err(Denglucuowu::Lingpaishibai) => jiamishibai(500, "令牌签发失败", miyao),
    }
}

/// 加密登录接口处理函数
pub async fn chuli(req: HttpRequest, ti: web::Bytes) -> HttpResponse {
    let miyao = match jiamichuanshuzhongjian::paishengyao(&req).await {
        Some(m) => m,
        None => return jiekouxtzhuti::shibai(401, "加密会话无效"),
    };
    match jiamichuanshuzhongjian::jiemiqingqiuti(&ti, &miyao) {
        Some(mingwen) => yanzhengdenglu(&mingwen, &miyao).await,
        None => jiekouxtzhuti::shibai(400, "解密请求体失败"),
    }
}
