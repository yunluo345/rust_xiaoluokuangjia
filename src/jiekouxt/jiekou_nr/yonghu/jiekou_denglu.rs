use actix_web::{HttpRequest, HttpResponse, web};
use serde::{Deserialize, Serialize};
use crate::jiekouxt::jiekouxtzhuti::{self, Jiekoudinyi, Qingqiufangshi};
use crate::jiekouxt::jiamichuanshu::jiamichuanshuzhongjian;
use crate::shujuku::psqlshujuku::shujubiao_nr::yonghu::shujucaozuo_yonghu;
use crate::gongju::{jwtgongju, jichugongju};

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

#[derive(Serialize)]
struct Xiangyingshuju {
    lingpai: String,
    yonghuid: String,
    nicheng: String,
    yonghuzuid: String,
}

fn jiancha_fengjin(yonghu: &serde_json::Value) -> Option<HttpResponse> {
    let fengjin = yonghu.get("fengjin")?.as_str()?;
    if fengjin != "1" {
        return None;
    }
    let fengjinjieshu = yonghu.get("fengjinjieshu").and_then(|v| v.as_str()).unwrap_or("");
    if !fengjinjieshu.is_empty() {
        if let Ok(jieshu) = fengjinjieshu.parse::<u64>() {
            if jichugongju::huoqushijianchuo() > jieshu {
                return None;
            }
        }
    }
    let yuanyin = yonghu.get("fengjinyuanyin").and_then(|v| v.as_str()).unwrap_or("未知原因");
    Some(jiekouxtzhuti::shibai(403, format!("账号已被封禁：{}", yuanyin)))
}

/// 加密登录接口处理函数
pub async fn chuli(req: HttpRequest, ti: web::Bytes) -> HttpResponse {
    let miyao = match jiamichuanshuzhongjian::paishengyao(&req).await {
        Some(m) => m,
        None => return jiekouxtzhuti::shibai(401, "加密会话无效"),
    };
    let mingwen = match jiamichuanshuzhongjian::jiemiqingqiuti(&ti, &miyao) {
        Some(m) => m,
        None => return jiekouxtzhuti::shibai(400, "解密请求体失败"),
    };
    let qingqiu: Qingqiuti = match serde_json::from_slice(&mingwen) {
        Ok(q) => q,
        Err(_) => return jiamichuanshuzhongjian::jiamixiangying(jiekouxtzhuti::shibai(400, "请求参数格式错误"), &miyao),
    };
    if qingqiu.zhanghao.is_empty() || qingqiu.mima.is_empty() {
        return jiamichuanshuzhongjian::jiamixiangying(jiekouxtzhuti::shibai(400, "账号或密码不能为空"), &miyao);
    }
    let yonghu = match shujucaozuo_yonghu::chaxun_zhanghao(&qingqiu.zhanghao).await {
        Some(y) => y,
        None => return jiamichuanshuzhongjian::jiamixiangying(jiekouxtzhuti::shibai(401, "账号或密码错误"), &miyao),
    };
    let cunchu_mima = match yonghu.get("mima").and_then(|v| v.as_str()) {
        Some(m) => m,
        None => return jiamichuanshuzhongjian::jiamixiangying(jiekouxtzhuti::shibai(500, "用户数据异常"), &miyao),
    };
    if cunchu_mima != qingqiu.mima {
        return jiamichuanshuzhongjian::jiamixiangying(jiekouxtzhuti::shibai(401, "账号或密码错误"), &miyao);
    }
    if let Some(xiangying) = jiancha_fengjin(&yonghu) {
        return jiamichuanshuzhongjian::jiamixiangying(xiangying, &miyao);
    }
    let yonghuid = yonghu.get("id").and_then(|v| v.as_str()).unwrap_or("");
    let nicheng = yonghu.get("nicheng").and_then(|v| v.as_str()).unwrap_or("");
    let yonghuzuid = yonghu.get("yonghuzuid").and_then(|v| v.as_str()).unwrap_or("");
    let lingpai = match jwtgongju::qianfa(yonghuid, &qingqiu.zhanghao, yonghuzuid) {
        Some(l) => l,
        None => return jiamichuanshuzhongjian::jiamixiangying(jiekouxtzhuti::shibai(500, "令牌签发失败"), &miyao),
    };
    let _ = shujucaozuo_yonghu::gengxindenglu(yonghuid).await;
    let xiangying = jiekouxtzhuti::chenggong("登录成功", Xiangyingshuju {
        lingpai,
        yonghuid: yonghuid.to_string(),
        nicheng: nicheng.to_string(),
        yonghuzuid: yonghuzuid.to_string(),
    });
    jiamichuanshuzhongjian::jiamixiangying(xiangying, &miyao)
}
