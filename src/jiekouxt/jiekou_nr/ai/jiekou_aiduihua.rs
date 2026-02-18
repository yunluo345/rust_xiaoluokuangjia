use actix_web::{HttpRequest, HttpResponse, web};
use crate::jiekouxt::jiekouxtzhuti::{self, Jiekoudinyi, Qingqiufangshi};
use crate::gongju::ai::openai::openaizhuti::ReactJieguo;

#[allow(non_upper_case_globals)]
pub const dinyi: Jiekoudinyi = Jiekoudinyi {
    lujing: "/duihua",
    nicheng: "AI对话",
    jieshao: "非流式AI对话接口，自动选择可用渠道",
    fangshi: Qingqiufangshi::Post,
    jiami: false,
    xudenglu: true,
    xuyonghuzu: false,
    yunxuputong: false,
};

async fn chuliqingqiu(ti: &[u8], lingpai: &str) -> HttpResponse {
    let qingqiu: super::Qingqiuti = match serde_json::from_slice::<super::Qingqiuti>(ti) {
        Ok(q) if !q.xiaoxilie.is_empty() => q,
        Ok(_) => return jiekouxtzhuti::shibai(400, "消息列表不能为空"),
        Err(_) => return jiekouxtzhuti::shibai(400, "请求参数格式错误"),
    };

    let peizhi = match super::huoqu_peizhi().await {
        Some(p) => p,
        None => return jiekouxtzhuti::shibai(500, "暂无可用AI渠道或配置错误"),
    };

    let benci_neirong = qingqiu.xiaoxilie.iter()
        .rev()
        .find(|x| x.juese == "user")
        .map(|x| x.neirong.as_str())
        .unwrap_or("");

    let (gongjulie, yitu_miaoshu) = super::huoqu_yitu_gongju(&peizhi, benci_neirong).await;
    println!("[AI对话] 意图: {} 工具数: {}", yitu_miaoshu, gongjulie.len());

    let mut guanli = super::goujian_guanli_anyitu(&qingqiu, gongjulie);

    match super::react_xunhuan(&peizhi, &mut guanli, "ReAct", lingpai, &qingqiu).await {
        Some(ReactJieguo::Wenben(huifu)) => {
            let shuju = serde_json::json!({ "huifu": huifu, "yitu": yitu_miaoshu });
            jiekouxtzhuti::chenggong("对话成功", shuju)
        }
        _ => jiekouxtzhuti::shibai(500, "AI服务调用失败或处理超时"),
    }
}

pub async fn chuli(req: HttpRequest, ti: web::Bytes) -> HttpResponse {
    let lingpai = match jiekouxtzhuti::tiqulingpai(&req) {
        Some(l) => {
            println!("[AI对话] 用户令牌: {}", l);
            l
        }
        None => return jiekouxtzhuti::shibai(401, "缺少授权令牌"),
    };
    
    println!("[AI对话] 前端请求内容: {}", String::from_utf8_lossy(&ti));
    if let Ok(json) = serde_json::from_slice::<serde_json::Value>(&ti) {
        if let Some(zuihou) = json["xiaoxilie"].as_array().and_then(|arr| arr.last()) {
            println!("[AI对话] 本次发送内容: {}", zuihou["neirong"].as_str().unwrap_or(""));
        }
    }
    
    chuliqingqiu(&ti, &lingpai).await
}
