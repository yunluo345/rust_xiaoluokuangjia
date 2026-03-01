use actix_web::{HttpRequest, HttpResponse, web};
use crate::jiekouxt::jiekouxtzhuti::{self, Jiekoudinyi, Qingqiufangshi};
use crate::gongju::ai::openai::{diaoduqi, gongjuji, openaizhuti};
use crate::gongju::jichugongju;
use crate::peizhixt::peizhi_nr::peizhi_ai::Ai;

#[allow(non_upper_case_globals)]
pub const dinyi: Jiekoudinyi = Jiekoudinyi {
    lujing: "/duihualiushi",
    nicheng: "AI对话流式",
    jieshao: "流式AI对话接口，自动选择可用渠道，实时推送响应",
    fangshi: Qingqiufangshi::Post,
    jiami: false,
    xudenglu: true,
    xuyonghuzu: false,
    yunxuputong: false,
};

#[allow(non_upper_case_globals)]
const duihua_renwuzu_mingcheng: &str = "对话流式";

fn shengcheng_duihua_renwuzu_id() -> String {
    format!(
        "duihua_{}_{}",
        jichugongju::huoqushijianchuo(),
        fastrand::u16(0..10000),
    )
}

fn cuowu_sse(xinxi: &str) -> HttpResponse {
    let neirong = serde_json::json!({"cuowu": xinxi}).to_string();
    HttpResponse::Ok()
        .content_type("text/event-stream")
        .insert_header(("Cache-Control", "no-cache"))
        .insert_header(("Connection", "keep-alive"))
        .body(format!("data: {}\n\n", neirong))
}

fn shengchengsse(shuju: &serde_json::Value) -> web::Bytes {
    web::Bytes::from(format!("data: {}\n\n", shuju))
}

fn fasongshuju(
    fasongqi: &futures::channel::mpsc::UnboundedSender<Result<web::Bytes, actix_web::Error>>,
    shuju: serde_json::Value,
) -> bool {
    fasongqi.unbounded_send(Ok(shengchengsse(&shuju))).is_ok()
}

/// 逐字发送文本内容
async fn zhuzi_fasong(
    fasongqi: &futures::channel::mpsc::UnboundedSender<Result<web::Bytes, actix_web::Error>>,
    wenben: &str,
) -> bool {
    for zi in wenben.chars() {
        if !fasongshuju(fasongqi, serde_json::json!({"neirong": zi.to_string()})) {
            return false;
        }
        actix_web::rt::time::sleep(std::time::Duration::from_millis(20)).await;
    }
    fasongshuju(fasongqi, serde_json::json!({"wancheng": true}))
}


async fn zhixing_gongju_liushi(
    qz: &str,
    lie: &[llm::ToolCall],
    lingpai: &str,
    fasongqi: &futures::channel::mpsc::UnboundedSender<Result<web::Bytes, actix_web::Error>>,
) -> Vec<llm::ToolCall> {
    let mut jieguolie: Vec<llm::ToolCall> = Vec::with_capacity(lie.len());
    for d in lie {
        let mut dan = d.clone();
        println!("[{}] 执行工具: {} 参数: {}", qz, dan.function.name, dan.function.arguments);
        dan.function.arguments = gongjuji::zhixing(&dan.function.name, &dan.function.arguments, lingpai).await;
        if !fasongshuju(fasongqi, serde_json::json!({
            "shijian": "gongjujieguo",
            "mingcheng": dan.function.name,
            "neirong": format!("工具{}执行完成", dan.function.name),
        })) {
            jieguolie.push(dan);
            return jieguolie;
        }
        jieguolie.push(dan);
    }
    jieguolie
}

async fn chuliqingqiu(ti: &[u8], lingpai: &str) -> HttpResponse {
    let qingqiu: super::Qingqiuti = match serde_json::from_slice::<super::Qingqiuti>(ti) {
        Ok(q) if !q.xiaoxilie.is_empty() => q,
        Ok(_) => return cuowu_sse("消息列表不能为空"),
        Err(_) => return cuowu_sse("请求参数格式错误"),
    };

    let peizhi = match super::huoqu_peizhi().await {
        Some(p) => p,
        None => return cuowu_sse("暂无可用AI渠道或配置错误"),
    };

    let (gongjulie, yitu_miaoshu, yitu_sikao) = super::huoqu_yitu_gongju(&peizhi, &qingqiu.xiaoxilie).await;
    println!("[AI对话流式] 意图: {} 工具数: {}", yitu_miaoshu, gongjulie.len());

    let (fasongqi, jieshouqi) = futures::channel::mpsc::unbounded::<Result<web::Bytes, actix_web::Error>>();
    let lingpai = lingpai.to_string();

    actix_web::rt::spawn(async move {
        let ai_peizhi = Ai::duqu_huo_moren();
        let duihua_houtai = ai_peizhi.diaoduqi.duihua_houtai_zhixing;
        let zuida = ai_peizhi.zuida_xunhuancishu;
        let renwuzu = diaoduqi::Renwuzu::xingjian(
            shengcheng_duihua_renwuzu_id(),
            duihua_renwuzu_mingcheng,
            duihua_houtai,
        );

        diaoduqi::zai_renwuzu_zhong(renwuzu.clone(), async move {
            let mut yitu_json = serde_json::json!({"shijian": "yitu", "yitu": yitu_miaoshu});
            if let Some(s) = yitu_sikao {
                yitu_json["sikao"] = serde_json::json!(s);
            }
            if !fasongshuju(&fasongqi, yitu_json) {
                return;
            }

            let mut guanli = super::goujian_guanli_anyitu(&qingqiu, gongjulie);
            let mut shangci_hash: u64 = 0;
            let mut chongfu: u32 = 0;

            for cishu in 1..=zuida {
                if !duihua_houtai && fasongqi.is_closed() {
                    renwuzu.quxiao();
                    println!("[流式ReAct] 前端已断开，停止AI调用");
                    return;
                }
                guanli.caijian_shangxiawen(peizhi.zuida_token);
                if !fasongshuju(&fasongqi, serde_json::json!({
                    "shijian": "xunhuan",
                    "lun": cishu,
                    "neirong": format!("第{}轮思考中", cishu),
                })) {
                    return;
                }

                match openaizhuti::putongqingqiu_react(&peizhi, &guanli).await {
                    Some(openaizhuti::ReactJieguo::Wenben { neirong, sikao }) => {
                        if let Some(s) = sikao {
                            fasongshuju(&fasongqi, serde_json::json!({"shijian": "sikao", "neirong": s}));
                        }
                        zhuzi_fasong(&fasongqi, &neirong).await;
                        return;
                    }
                    Some(openaizhuti::ReactJieguo::Gongjudiaoyong(lie)) => {
                    let hash = super::gongju_qianming(&lie);
                    if hash == shangci_hash && shangci_hash != 0 {
                        chongfu += 1;
                    if chongfu >= 2 {
                            println!("[流式ReAct] 工具重复调用，移除工具做最终回复");
                            guanli.qingkong_gongjulie();
                                if let Some((neirong, sikao)) = openaizhuti::putongqingqiu_daisikao(&peizhi, &guanli).await {
                                    if let Some(s) = sikao {
                                        fasongshuju(&fasongqi, serde_json::json!({"shijian": "sikao", "neirong": s}));
                                    }
                                    zhuzi_fasong(&fasongqi, &neirong).await;
                                } else {
                                    let _ = fasongshuju(&fasongqi, serde_json::json!({"cuowu": "AI服务调用失败"}));
                                }
                                return;
                            }
                        } else {
                            chongfu = 0;
                        }
                        shangci_hash = hash;

                        if !fasongshuju(&fasongqi, serde_json::json!({
                            "shijian": "gongjudiaoyong",
                            "lun": cishu,
                            "gongju": lie.iter().map(|d| d.function.name.clone()).collect::<Vec<_>>(),
                            "neirong": format!("第{}轮调用工具", cishu),
                        })) {
                            return;
                        }

                        guanli.zhuijia_zhushou_gongjudiaoyong(lie.clone());
                        let jieguolie = zhixing_gongju_liushi("流式ReAct", &lie, &lingpai, &fasongqi).await;

                        // 检测询问信号：若工具返回 xunwen，发送问题给前端并终止循环
                        if let Some(xinhao) = gongjuji::jiancha_xunwen(&jieguolie) {
                            println!("[流式ReAct] 检测到询问信号: {}", xinhao.wenti);
                            let shuju = if xinhao.xuanxiang.is_empty() {
                                serde_json::json!(null)
                            } else {
                                serde_json::json!({"xuanxiang": xinhao.xuanxiang})
                            };
                            fasongshuju(&fasongqi, serde_json::json!({
                                "aihuifu": {
                                    "huifu": xinhao.wenti,
                                    "leixing": "xunwen",
                                    "shuju": shuju,
                                }
                            }));
                            let _ = fasongshuju(&fasongqi, serde_json::json!({"wancheng": true}));
                            return;
                        }

                        guanli.zhuijia_gongjujieguo(jieguolie);
                    }
                    None => {
                        let _ = fasongshuju(&fasongqi, serde_json::json!({"cuowu": "AI服务调用失败或处理超时"}));
                        return;
                    }
                }
            }

            let _ = fasongshuju(&fasongqi, serde_json::json!({"cuowu": "超过最大循环次数，已终止"}));
        }).await;
    });

    HttpResponse::Ok()
        .content_type("text/event-stream")
        .insert_header(("Cache-Control", "no-cache"))
        .insert_header(("Connection", "keep-alive"))
        .streaming(jieshouqi)
}

pub async fn chuli(req: HttpRequest, ti: web::Bytes) -> HttpResponse {
    let lingpai = jiekouxtzhuti::tiqulingpai(&req).unwrap_or_default();
    println!("[AI对话流式] 用户令牌: {}", lingpai);
    println!("[AI对话流式] 前端请求内容: {}", String::from_utf8_lossy(&ti));
    if let Ok(json) = serde_json::from_slice::<serde_json::Value>(&ti) {
        if let Some(zuihou) = json["xiaoxilie"].as_array().and_then(|arr| arr.last()) {
            println!("[AI对话流式] 本次发送内容: {}", zuihou["neirong"].as_str().unwrap_or(""));
        }
    }
    
    chuliqingqiu(&ti, &lingpai).await
}
