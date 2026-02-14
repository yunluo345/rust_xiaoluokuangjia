use actix_web::{HttpRequest, HttpResponse, web};
use serde::Deserialize;
use tokio::sync::mpsc;
use tokio_stream::StreamExt;
use crate::gongju::jiamigongju;
use crate::gongju::ai::openai::aigongju;
use crate::gongju::ai::openai::liushishijian::Liushishijian;
use crate::jiekouxt::jiekouxtzhuti::{self, Jiekoudinyi, Qingqiufangshi};
use crate::jiekouxt::jiamichuanshu::jiamichuanshuzhongjian;
use crate::shujuku::psqlshujuku::shujubiao_nr::ai::shujucaozuo_aiqudao as qudaocaozuo;
use crate::peizhixt::peizhixitongzhuti;
use crate::peizhixt::peizhi_nr::peizhi_ai;

#[allow(non_upper_case_globals)]
pub const dinyi: Jiekoudinyi = Jiekoudinyi {
    lujing: "/duihua",
    nicheng: "AI对话",
    jieshao: "加密SSE流式AI对话接口，支持ReAct工具循环，通过渠道轮训自动选取AI服务",
    fangshi: Qingqiufangshi::Post,
    jiami: true,
    xudenglu: true,
    xuyonghuzu: true,
    yunxuputong: false,
};

#[allow(non_upper_case_globals)]
const wanzhenglujing: &str = "/jiekou/ai/duihua";
#[allow(non_upper_case_globals)]
const api_lujing: &str = "/v1/chat/completions";
#[allow(non_upper_case_globals)]
const chaoshi_miao: u64 = 120;
#[allow(non_upper_case_globals)]
const zuida_xunhuancishu: usize = 10;

#[allow(non_upper_case_globals)]
const quanju_xitongtishici: &str = "你是一个专业的AI日报助手，专注于帮助用户完成日报相关工作。\n\
\n\
严格规则：\n\
1. 只接受与日报相关的工作请求，拒绝任何闲聊、娱乐、无关话题。\n\
2. 当用户提出非日报相关的问题时，礼貌地提醒：\"我是日报助手，只能处理日报相关工作。\"\n\
3. 严禁伪造、编造或捏造任何数据。\n\
4. 当工具调用返回失败（chenggong=false）时，必须仔细分析失败原因：\n\
   - 如果是验证失败（如\"缺少必需的标签类别\"、\"检测到占位符标签\"），说明数据本身不符合要求，不得重试，应直接向用户说明原因。\n\
   - 如果是技术错误（如网络超时、JSON格式错误），可以修正后重试一次。\n\
   - 同一个工具调用，相同参数不得重复调用超过1次。\n\
5. 不得通过添加、修改或伪造数据来绕过工具的验证失败。\n\
6. 工具返回失败后，应立即停止调用工具，直接回复用户说明情况，不要继续尝试。";

#[derive(Deserialize)]
struct Qingqiuti {
    leixing: String,
    #[serde(default)]
    xitongtishici: Option<String>,
    xiaoxilie: Vec<Xiaoxixiang>,
}

#[derive(Deserialize)]
struct Xiaoxixiang {
    jiaose: String,
    neirong: String,
}

struct Qudaopeizhi {
    jiekoudizhi: String,
    miyao: String,
    moxing: String,
    wendu: f64,
}

impl Qudaopeizhi {
    fn cong_shuju(shuju: &serde_json::Value) -> Option<Self> {
        let qu = |ming: &str| shuju.get(ming).and_then(|v| v.as_str()).unwrap_or("").to_string();
        let jiekoudizhi = qu("jiekoudizhi").trim_end_matches('/').to_string();
        if jiekoudizhi.is_empty() { return None; }
        Some(Self {
            jiekoudizhi,
            miyao: qu("miyao"),
            moxing: qu("moxing"),
            wendu: qu("wendu").parse().unwrap_or(0.0),
        })
    }

    fn goujian_url(&self) -> String {
        format!("{}{}", self.jiekoudizhi, api_lujing)
    }
}

struct Gongjudiaoyong {
    id: String,
    mingcheng: String,
    canshu: String,
}

fn jiamissekuai(shijian: &Liushishijian, miyao: &[u8]) -> Option<web::Bytes> {
    let json = serde_json::to_string(shijian).ok()?;
    let miwen = jiamigongju::jiami(json.as_bytes(), miyao)?;
    Some(web::Bytes::from(format!("data: {}\n\n", jiamigongju::zhuanbase64(&miwen))))
}

fn shengcheng_jiamiliushi(
    jieshouqi: mpsc::Receiver<Liushishijian>,
    miyao: Vec<u8>,
) -> impl futures_core::Stream<Item = Result<web::Bytes, actix_web::Error>> {
    tokio_stream::wrappers::ReceiverStream::new(jieshouqi).map(move |shijian| {
        Ok(jiamissekuai(&shijian, &miyao).unwrap_or_else(|| {
            web::Bytes::from(Liushishijian::Cuowu { xinxi: "加密失败".to_string() }.zhuansse())
        }))
    })
}

fn goujian_xiaoxilie(qingqiu: &Qingqiuti) -> Vec<serde_json::Value> {
    let mut jieguo: Vec<serde_json::Value> = vec![];
    
    let mut xitong_neirong = match qingqiu.xitongtishici.as_deref() {
        Some(yonghu_tishici) => format!("{}\n\n{}", quanju_xitongtishici, yonghu_tishici),
        None => quanju_xitongtishici.to_string(),
    };
    
    // 读取AI配置，添加标签提取的类别限制
    if let Some(aipeizhi) = peizhixitongzhuti::duqupeizhi::<peizhi_ai::Aipeizhi>(peizhi_ai::Aipeizhi::wenjianming()) {
        if !aipeizhi.biaoqiantiqu.bixuyou.is_empty() {
            let yunxu_leibie = aipeizhi.biaoqiantiqu.bixuyou.join("、");
            xitong_neirong.push_str(&format!("\n\n工具使用规则：\n调用tiqubiaoqian工具时，只允许提取以下类别的标签：{}。严禁提取其他类别的标签。", yunxu_leibie));
        }
    }
    
    jieguo.push(serde_json::json!({"role": "system", "content": xitong_neirong}));
    
    jieguo.extend(qingqiu.xiaoxilie.iter().filter_map(|x| {
        let jiaose = match x.jiaose.as_str() {
            "yonghu" => "user",
            "zhushou" => "assistant",
            _ => return None,
        };
        Some(serde_json::json!({"role": jiaose, "content": x.neirong}))
    }));
    jieguo
}

fn goujian_gongjulie() -> Option<Vec<serde_json::Value>> {
    let lie = aigongju::huoqu_suoyougongju();
    (!lie.is_empty()).then_some(lie)
}

fn goujian_qingqiuti(peizhi: &Qudaopeizhi, xiaoxilie: &[serde_json::Value]) -> serde_json::Value {
    let mut ti = serde_json::json!({
        "model": peizhi.moxing,
        "messages": xiaoxilie,
        "temperature": peizhi.wendu,
        "stream": true,
    });
    if let Some(gj) = goujian_gongjulie() {
        ti["tools"] = serde_json::Value::Array(gj);
    }
    ti
}

async fn fasong_liushiqingqiu(peizhi: &Qudaopeizhi, ti: &serde_json::Value) -> Option<reqwest::Response> {
    reqwest::Client::new()
        .post(peizhi.goujian_url())
        .header("Authorization", format!("Bearer {}", peizhi.miyao))
        .header("Content-Type", "application/json")
        .timeout(std::time::Duration::from_secs(chaoshi_miao))
        .json(ti)
        .send()
        .await
        .ok()
}

async fn chuli_wenben(fasongqi: &mpsc::Sender<Liushishijian>, xuanze: &serde_json::Value) -> bool {
    if let Some(neirong) = xuanze.get("delta").and_then(|d| d.get("content")).and_then(|c| c.as_str()) {
        if !neirong.is_empty() {
            return fasongqi.send(Liushishijian::Wenbenkuai { neirong: neirong.to_string() }).await.is_ok();
        }
    }
    true
}

fn shouji_gongjudiaoyong(xuanze: &serde_json::Value, huanchong: &mut Vec<Gongjudiaoyong>) {
    let gongjulie = match xuanze.get("delta").and_then(|d| d.get("tool_calls")).and_then(|t| t.as_array()) {
        Some(l) => l,
        None => return,
    };
    for gongju in gongjulie {
        let suoyin = gongju.get("index").and_then(|i| i.as_u64()).unwrap_or(0) as usize;
        let hanshu = gongju.get("function");
        let mingcheng = hanshu.and_then(|f| f.get("name")).and_then(|n| n.as_str()).unwrap_or("");
        let canshu = hanshu.and_then(|f| f.get("arguments")).and_then(|a| a.as_str()).unwrap_or("");
        while huanchong.len() <= suoyin {
            huanchong.push(Gongjudiaoyong {
                id: String::new(),
                mingcheng: String::new(),
                canshu: String::new(),
            });
        }
        if let Some(id) = gongju.get("id").and_then(|i| i.as_str()) {
            if !id.is_empty() { huanchong[suoyin].id = id.to_string(); }
        }
        if !mingcheng.is_empty() { huanchong[suoyin].mingcheng = mingcheng.to_string(); }
        huanchong[suoyin].canshu.push_str(canshu);
    }
}

async fn tuisong_gongjuguocheng(fasongqi: &mpsc::Sender<Liushishijian>, xuanze: &serde_json::Value) {
    let gongjulie = match xuanze.get("delta").and_then(|d| d.get("tool_calls")).and_then(|t| t.as_array()) {
        Some(l) => l,
        None => return,
    };
    for gongju in gongjulie {
        let hanshu = gongju.get("function");
        let suoyin = gongju.get("index").and_then(|i| i.as_u64()).unwrap_or(0) as usize;
        let gongjuming = hanshu.and_then(|f| f.get("name")).and_then(|n| n.as_str()).unwrap_or("");
        if !gongjuming.is_empty() {
            let _ = fasongqi.send(Liushishijian::Gongjukaishi {
                suoyin,
                gongjuid: gongju.get("id").and_then(|i| i.as_str()).unwrap_or("").to_string(),
                gongjuming: gongjuming.to_string(),
            }).await;
        }
        let canshu = hanshu.and_then(|f| f.get("arguments")).and_then(|a| a.as_str()).unwrap_or("");
        if !canshu.is_empty() {
            let _ = fasongqi.send(Liushishijian::Gongjucanshu { suoyin, bufen_json: canshu.to_string() }).await;
        }
    }
}

fn jiancha_wancheng(xuanze: &serde_json::Value) -> Option<String> {
    xuanze.get("finish_reason")
        .and_then(|f| f.as_str())
        .filter(|r| !r.is_empty() && *r != "null")
        .map(|r| r.to_string())
}

async fn xiaofei_liushi(
    xiangying: reqwest::Response,
    fasongqi: &mpsc::Sender<Liushishijian>,
) -> (Vec<Gongjudiaoyong>, Option<String>) {
    let zhuangtai = xiangying.status();
    println!("[AI] HTTP响应状态: {}", zhuangtai);
    let mut liu = xiangying.bytes_stream();
    let mut huanchong = String::new();
    let mut gongjulie: Vec<Gongjudiaoyong> = Vec::new();
    let mut wancheng_yuanyin: Option<String> = None;
    let mut kuaishu: usize = 0;
    while let Some(kuai) = liu.next().await {
        let zijie = match kuai {
            Ok(z) => z,
            Err(e) => {
                println!("[AI] 读取流数据错误: {}", e);
                break;
            }
        };
        kuaishu += 1;
        let wenben = String::from_utf8_lossy(&zijie);
        if kuaishu <= 3 {
            println!("[AI] 原始数据块{}: {}", kuaishu, wenben.chars().take(200).collect::<String>());
        }
        huanchong.push_str(&wenben);
        while let Some(weizhi) = huanchong.find('\n') {
            let hang: String = huanchong.drain(..=weizhi).collect();
            let hang = hang.trim();
            if hang.is_empty() { continue; }
            let shuju = match hang.strip_prefix("data: ") {
                Some(s) => s,
                None => continue,
            };
            if shuju == "[DONE]" {
                if wancheng_yuanyin.is_none() {
                    wancheng_yuanyin = Some("stop".to_string());
                }
                return (gongjulie, wancheng_yuanyin);
            }
            let json: serde_json::Value = match serde_json::from_str(shuju) {
                Ok(j) => j,
                Err(_) => continue,
            };
            let xuanze = match json.get("choices").and_then(|c| c.get(0)) {
                Some(x) => x,
                None => continue,
            };
            if !chuli_wenben(fasongqi, xuanze).await {
                return (gongjulie, Some("stop".to_string()));
            }
            tuisong_gongjuguocheng(fasongqi, xuanze).await;
            shouji_gongjudiaoyong(xuanze, &mut gongjulie);
            if let Some(yuanyin) = jiancha_wancheng(xuanze) {
                wancheng_yuanyin = Some(yuanyin);
            }
        }
    }
    (gongjulie, wancheng_yuanyin)
}

fn goujian_gongjuxiaoxi(diaoyonglie: &[Gongjudiaoyong]) -> serde_json::Value {
    let calls: Vec<serde_json::Value> = diaoyonglie.iter().map(|d| {
        serde_json::json!({
            "id": d.id,
            "type": "function",
            "function": { "name": d.mingcheng, "arguments": d.canshu }
        })
    }).collect();
    serde_json::json!({ "role": "assistant", "tool_calls": calls })
}

async fn zhixing_react_xunhuan(
    peizhi: &Qudaopeizhi,
    mut xiaoxilie: Vec<serde_json::Value>,
    fasongqi: mpsc::Sender<Liushishijian>,
) {
    for lun in 0..zuida_xunhuancishu {
        println!("[AI] ReAct第{}轮开始", lun + 1);
        let ti = goujian_qingqiuti(peizhi, &xiaoxilie);
        let xiangying = match fasong_liushiqingqiu(peizhi, &ti).await {
            Some(r) => r,
            None => {
                println!("[AI] 请求AI服务失败");
                let _ = fasongqi.send(Liushishijian::Cuowu { xinxi: "AI服务请求失败".to_string() }).await;
                return;
            }
        };
        let (gongjulie, yuanyin) = xiaofei_liushi(xiangying, &fasongqi).await;
        println!("[AI] 流消费完毕，工具调用数: {}", gongjulie.len());
        if gongjulie.is_empty() {
            let _ = fasongqi.send(Liushishijian::Wancheng {
                yuanyin: yuanyin.unwrap_or_else(|| "stop".to_string()),
            }).await;
            println!("[AI] ReAct结束，AI决定停止");
            return;
        }
        xiaoxilie.push(goujian_gongjuxiaoxi(&gongjulie));
        for d in &gongjulie {
            println!("[AI] 执行工具: {}", d.mingcheng);
            let diaoyong = llm::ToolCall {
                id: d.id.clone(),
                call_type: "function".to_string(),
                function: llm::FunctionCall {
                    name: d.mingcheng.clone(),
                    arguments: d.canshu.clone(),
                },
            };
            let jieguo = aigongju::zhixing_gongju(&diaoyong);
            println!("[AI] 工具结果: {}", jieguo.function.arguments);
            
            let _ = fasongqi.send(Liushishijian::Gongjuwancheng {
                suoyin: 0,
                gongjuid: d.id.clone(),
                gongjuming: d.mingcheng.clone(),
                canshu: d.canshu.clone(),
            }).await;
            let _ = fasongqi.send(Liushishijian::Gongjujieguo {
                gongjuid: d.id.clone(),
                gongjuming: d.mingcheng.clone(),
                jieguo: jieguo.function.arguments.clone(),
            }).await;
            xiaoxilie.push(serde_json::json!({
                "role": "tool",
                "tool_call_id": d.id,
                "content": jieguo.function.arguments,
            }));
        }
    }
    println!("[AI] ReAct达到最大循环次数");
    let _ = fasongqi.send(Liushishijian::Wancheng { yuanyin: "max_loops".to_string() }).await;
}

fn jiamicuowu(zhuangtaima: u16, xinxi: &str, miyao: &[u8]) -> HttpResponse {
    jiamichuanshuzhongjian::jiamixiangying(jiekouxtzhuti::shibai(zhuangtaima, xinxi), miyao)
}

async fn zhixing_duihua(qingqiu: Qingqiuti, miyao: Vec<u8>) -> HttpResponse {
    let qudaoshuju = match qudaocaozuo::lunxun(&qingqiu.leixing).await {
        Some(s) => s,
        None => return jiamicuowu(404, "没有可用的AI渠道", &miyao),
    };
    let peizhi = match Qudaopeizhi::cong_shuju(&qudaoshuju) {
        Some(p) => p,
        None => return jiamicuowu(500, "渠道配置解析失败", &miyao),
    };
    let xiaoxilie = goujian_xiaoxilie(&qingqiu);
    let (fasongqi, jieshouqi) = mpsc::channel::<Liushishijian>(64);
    let miyao_clone = miyao.clone();
    actix_web::rt::spawn(async move {
        zhixing_react_xunhuan(&peizhi, xiaoxilie, fasongqi).await;
    });
    HttpResponse::Ok()
        .content_type("text/event-stream")
        .insert_header(("Cache-Control", "no-cache"))
        .insert_header(("Connection", "keep-alive"))
        .streaming(shengcheng_jiamiliushi(jieshouqi, miyao_clone))
}

/// AI对话接口处理函数
pub async fn chuli(req: HttpRequest, ti: web::Bytes) -> HttpResponse {
    if let Err(xiangying) = jiekouxtzhuti::jiaoyanquanxian(&req, &dinyi, wanzhenglujing).await {
        return xiangying;
    }
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
        Err(_) => return jiamicuowu(400, "请求参数格式错误", &miyao),
    };
    if !qudaocaozuo::leixingyunxu(&qingqiu.leixing) {
        return jiamicuowu(400, &format!("不支持的渠道类型，仅允许：{}", qudaocaozuo::yunxuleixing.join("、")), &miyao);
    }
    zhixing_duihua(qingqiu, miyao).await
}
