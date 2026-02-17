use actix_web::{HttpRequest, HttpResponse, web};
use crate::jiekouxt::jiekouxtzhuti::{self, Jiekoudinyi, Qingqiufangshi};
use crate::jiekouxt::jiamichuanshu::jiamichuanshuzhongjian;
use crate::gongju::ai::openai::openaizhuti::ReactJieguo;

#[allow(non_upper_case_globals)]
pub const dinyi: Jiekoudinyi = Jiekoudinyi {
    lujing: "/duihua",
    nicheng: "AI对话",
    jieshao: "非流式AI对话接口，自动选择可用渠道",
    fangshi: Qingqiufangshi::Post,
    jiami: true,
    xudenglu: true,
    xuyonghuzu: false,
    yunxuputong: false,
};

fn jiamishibai(zhuangtaima: u16, xiaoxi: impl Into<String>, miyao: &[u8]) -> HttpResponse {
    jiamichuanshuzhongjian::jiamixiangying(jiekouxtzhuti::shibai(zhuangtaima, xiaoxi), miyao)
}

async fn chuliqingqiu(mingwen: &[u8], miyao: &[u8]) -> HttpResponse {
    let qingqiu: super::Qingqiuti = match serde_json::from_slice::<super::Qingqiuti>(mingwen) {
        Ok(q) if !q.xiaoxilie.is_empty() => q,
        Ok(_) => return jiamishibai(400, "消息列表不能为空", miyao),
        Err(_) => return jiamishibai(400, "请求参数格式错误", miyao),
    };

    let peizhi = match super::huoqu_peizhi().await {
        Some(p) => p,
        None => return jiamishibai(500, "暂无可用AI渠道或配置错误", miyao),
    };

    let mut guanli = super::goujian_guanli(qingqiu);

    match super::react_xunhuan(&peizhi, &mut guanli, "ReAct").await {
        Some(ReactJieguo::Wenben(huifu)) => {
            let shuju = serde_json::json!({ "huifu": huifu });
            jiamichuanshuzhongjian::jiamixiangying(
                jiekouxtzhuti::chenggong("对话成功", shuju), miyao,
            )
        }
        _ => jiamishibai(500, "AI服务调用失败或处理超时", miyao),
    }
}

pub async fn chuli(req: HttpRequest, ti: web::Bytes) -> HttpResponse {
    if let Some(lingpai) = jiekouxtzhuti::tiqulingpai(&req) {
        println!("[AI对话] 用户令牌: {}", lingpai);
    }
    let miyao = match jiamichuanshuzhongjian::paishengyao(&req).await {
        Some(m) => m,
        None => return jiekouxtzhuti::shibai(401, "加密会话无效"),
    };
    match jiamichuanshuzhongjian::jiemiqingqiuti(&ti, &miyao) {
        Some(mingwen) => chuliqingqiu(&mingwen, &miyao).await,
        None => jiekouxtzhuti::shibai(400, "解密请求体失败"),
    }
}
