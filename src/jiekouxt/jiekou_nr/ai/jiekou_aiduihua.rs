use actix_web::{HttpRequest, HttpResponse, web};
use serde::Deserialize;
use crate::jiekouxt::jiekouxtzhuti::{self, Jiekoudinyi, Qingqiufangshi};
use crate::jiekouxt::jiamichuanshu::jiamichuanshuzhongjian;
use crate::shujuku::psqlshujuku::shujubiao_nr::ai::shujucaozuo_aiqudao;
use crate::gongju::ai::openai::{aipeizhi, aixiaoxiguanli, gongjuji, openaizhuti};
use crate::gongju::ai::openai::openaizhuti::ReactJieguo;
use crate::peizhixt::peizhixitongzhuti;
use crate::peizhixt::peizhi_nr::peizhi_ai::Ai;

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

/// 执行工具调用，将每个 ToolCall 的结果填入后返回
async fn zhixing_gongjudiaoyong(diaoyonglie: &[llm::ToolCall]) -> Vec<llm::ToolCall> {
    let mut jieguolie = Vec::new();
    for diaoyong in diaoyonglie {
        let gongjuming = &diaoyong.function.name;
        let canshu = &diaoyong.function.arguments;
        println!("[ReAct] 执行工具: {} 参数: {}", gongjuming, canshu);

        let jieguo = gongjuji::zhixing(gongjuming, canshu).await;

        jieguolie.push(llm::ToolCall {
            id: diaoyong.id.clone(),
            call_type: diaoyong.call_type.clone(),
            function: llm::FunctionCall {
                name: gongjuming.clone(),
                arguments: jieguo,
            },
        });
    }
    jieguolie
}

async fn chuliqingqiu(mingwen: &[u8], miyao: &[u8]) -> HttpResponse {
    let qingqiu: Qingqiuti = match serde_json::from_slice::<Qingqiuti>(mingwen) {
        Ok(q) if !q.xiaoxilie.is_empty() => q,
        Ok(_) => return jiamishibai(400, "消息列表不能为空", miyao),
        Err(_) => return jiamishibai(400, "请求参数格式错误", miyao),
    };

    let qudao = match shujucaozuo_aiqudao::suiji_huoqu_qudao("openapi").await {
        Some(q) => {
            println!("获取到的渠道数据: {}", q);
            q
        }
        None => return jiamishibai(500, "暂无可用AI渠道", miyao),
    };

    let peizhi = match aipeizhi::Aipeizhi::cong_qudaoshuju(&qudao) {
        Some(p) => p,
        None => return jiamishibai(500, "AI渠道配置错误", miyao),
    };

    let mut guanli = aixiaoxiguanli::Xiaoxiguanli::xingjian()
        .shezhi_xitongtishici(super::xitongtishici);
    for gongju in gongjuji::huoqu_suoyougongju() {
        guanli = guanli.tianjia_gongju(gongju);
    }
    for xiaoxi in qingqiu.xiaoxilie {
        match xiaoxi.juese.as_str() {
            "user" => guanli.zhuijia_yonghuxiaoxi(xiaoxi.neirong),
            "assistant" => guanli.zhuijia_zhushouneirong(xiaoxi.neirong),
            _ => {}
        }
    }

    let zuida_cishu = peizhixitongzhuti::duqupeizhi::<Ai>(Ai::wenjianming())
        .map(|p| p.zuida_xunhuancishu)
        .unwrap_or(20);

    for cishu in 1..=zuida_cishu {
        println!("[ReAct] 第 {} 轮循环", cishu);
        match openaizhuti::putongqingqiu_react(&peizhi, &guanli).await {
            Some(ReactJieguo::Wenben(huifu)) => {
                let shuju = serde_json::json!({ "huifu": huifu });
                return jiamichuanshuzhongjian::jiamixiangying(
                    jiekouxtzhuti::chenggong("对话成功", shuju), miyao,
                );
            }
            Some(ReactJieguo::Gongjudiaoyong(diaoyonglie)) => {
                guanli.zhuijia_zhushou_gongjudiaoyong(diaoyonglie.clone());
                let jieguo = zhixing_gongjudiaoyong(&diaoyonglie).await;
                guanli.zhuijia_gongjujieguo(jieguo);
            }
            None => return jiamishibai(500, "AI服务调用失败", miyao),
        }
    }
    jiamishibai(500, "AI处理超时，已达最大循环次数", miyao)
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
