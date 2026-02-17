use actix_web::{HttpRequest, HttpResponse, web};
use serde::Deserialize;
use crate::jiekouxt::jiekouxtzhuti::{self, Jiekoudinyi, Qingqiufangshi};
use crate::jiekouxt::jiamichuanshu::jiamichuanshuzhongjian;
use crate::shujuku::psqlshujuku::shujubiao_nr::ai::shujucaozuo_aiqudao;
use crate::gongju::ai::openai::{aipeizhi, aixiaoxiguanli, openaizhuti};

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

#[derive(Deserialize)]
struct Xiaoxi {
    juese: String,
    neirong: String,
}

#[derive(Deserialize)]
struct Qingqiuti {
    xiaoxilie: Vec<Xiaoxi>,
}

fn jiamishibai(zhuangtaima: u16, xiaoxi: impl Into<String>, miyao: &[u8]) -> HttpResponse {
    jiamichuanshuzhongjian::jiamixiangying(jiekouxtzhuti::shibai(zhuangtaima, xiaoxi), miyao)
}

async fn chuliqingqiu(mingwen: &[u8], miyao: &[u8]) -> HttpResponse {
    let qingqiu: Qingqiuti = match serde_json::from_slice::<Qingqiuti>(mingwen) {
        Ok(q) if !q.xiaoxilie.is_empty() => q,
        Ok(_) => return jiamishibai(400, "消息列表不能为空", miyao),
        Err(_) => return jiamishibai(400, "请求参数格式错误", miyao),
    };

    let qudao = match shujucaozuo_aiqudao::suiji_huoqu_qudao("openapi").await {
        Some(q) => q,
        None => return jiamishibai(500, "暂无可用AI渠道", miyao),
    };

    let peizhi = match aipeizhi::Aipeizhi::cong_qudaoshuju(&qudao) {
        Some(p) => p,
        None => return jiamishibai(500, "AI渠道配置错误", miyao),
    };

    let mut guanli = aixiaoxiguanli::Xiaoxiguanli::xingjian();
    for xiaoxi in qingqiu.xiaoxilie {
        match xiaoxi.juese.as_str() {
            "user" => guanli.zhuijia_yonghuxiaoxi(xiaoxi.neirong),
            "assistant" => guanli.zhuijia_zhushouneirong(xiaoxi.neirong),
            _ => {}
        }
    }

    match openaizhuti::putongqingqiu(&peizhi, &guanli).await {
        Some(huifu) => {
            let shuju = serde_json::json!({ "huifu": huifu });
            jiamichuanshuzhongjian::jiamixiangying(jiekouxtzhuti::chenggong("对话成功", shuju), miyao)
        }
        None => jiamishibai(500, "AI服务调用失败", miyao),
    }
}

pub async fn chuli(req: HttpRequest, ti: web::Bytes) -> HttpResponse {
    let miyao = match jiamichuanshuzhongjian::paishengyao(&req).await {
        Some(m) => m,
        None => return jiekouxtzhuti::shibai(401, "加密会话无效"),
    };
    match jiamichuanshuzhongjian::jiemiqingqiuti(&ti, &miyao) {
        Some(mingwen) => chuliqingqiu(&mingwen, &miyao).await,
        None => jiekouxtzhuti::shibai(400, "解密请求体失败"),
    }
}
