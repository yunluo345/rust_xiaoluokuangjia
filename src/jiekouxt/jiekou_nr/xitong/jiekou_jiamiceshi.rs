use actix_web::{HttpRequest, HttpResponse, web};
use serde::{Deserialize, Serialize};
use crate::jiekouxt::jiekouxtzhuti::{self, Jiekoudinyi, Qingqiufangshi};
use crate::jiekouxt::jiamichuanshu::jiamichuanshuzhongjian;

#[allow(non_upper_case_globals)]
pub const dinyi: Jiekoudinyi = Jiekoudinyi {
    lujing: "/jiamiceshi",
    nicheng: "加密测试",
    jieshao: "测试加密传输的完整链路",
    fangshi: Qingqiufangshi::Post,
    jiami: true,
    xudenglu: false,
    xuyonghuzu: false,
    yunxuputong: false,
};

#[derive(Deserialize)]
pub struct Qingqiuti {
    neirong: Option<String>,
}

#[derive(Serialize)]
struct Xiangyingshuju {
    huifu: String,
    yuanshishuju: Option<String>,
}

/// 加密测试接口处理函数
pub async fn chuli(req: HttpRequest, ti: web::Bytes) -> HttpResponse {
    let miyao = match jiamichuanshuzhongjian::paishengyao(&req).await {
        Some(m) => m,
        None => return jiekouxtzhuti::shibai(401, "加密会话无效"),
    };
    let mingwen = match jiamichuanshuzhongjian::jiemiqingqiuti(&ti, &miyao) {
        Some(m) => m,
        None => return jiekouxtzhuti::shibai(400, "解密请求体失败"),
    };
    let yuanshi: Option<String> = serde_json::from_slice(&mingwen)
        .ok()
        .and_then(|v: Qingqiuti| v.neirong);
    let xiangying = jiekouxtzhuti::chenggong("加密测试成功", Xiangyingshuju {
        huifu: "服务端已收到并解密你的数据".to_string(),
        yuanshishuju: yuanshi,
    });
    jiamichuanshuzhongjian::jiamixiangying(xiangying, &miyao)
}
