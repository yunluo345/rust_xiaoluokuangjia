use actix_web::{HttpRequest, HttpResponse, web};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use tokio_stream::StreamExt;
use crate::gongju::jiamigongju;
use crate::gongju::ai::openai::liushishijian::Liushishijian;
use crate::jiekouxt::jiekouxtzhuti::{self, Jiekoudinyi, Qingqiufangshi};
use crate::jiekouxt::jiamichuanshu::jiamichuanshuzhongjian;
use crate::shujuku::psqlshujuku::shujubiao_nr::ai::shujucaozuo_aiqudao as qudaocaozuo;

#[allow(non_upper_case_globals)]
pub const dinyi: Jiekoudinyi = Jiekoudinyi {
    lujing: "/duihua",
    nicheng: "AI对话",
    jieshao: "加密SSE流式AI对话接口，支持工具调用，通过渠道轮训自动选取AI服务",
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

#[derive(Deserialize)]
struct Qingqiuti {
    leixing: String,
    #[serde(default)]
    xitongtishici: Option<String>,
    xiaoxilie: Vec<Xiaoxixiang>,
    #[serde(default)]
    gongjulie: Option<Vec<Gongjuxiang>>,
}

#[derive(Deserialize)]
struct Xiaoxixiang {
    jiaose: String,
    neirong: String,
}

#[derive(Deserialize, Serialize)]
struct Gongjuxiang {
    mingcheng: String,
    miaoshu: String,
    canshu: serde_json::Value,
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
    let mut jieguo: Vec<serde_json::Value> = qingqiu.xitongtishici.as_deref()
        .map(|t| vec![serde_json::json!({"role": "system", "content": t})])
        .unwrap_or_default();
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

fn goujian_gongjulie(qingqiu: &Qingqiuti) -> Option<Vec<serde_json::Value>> {
    qingqiu.gongjulie.as_ref().map(|lie| {
        lie.iter().map(|gj| serde_json::json!({
            "type": "function",
            "function": { "name": gj.mingcheng, "description": gj.miaoshu, "parameters": gj.canshu }
        })).collect()
    })
}

fn goujian_qingqiuti(peizhi: &Qudaopeizhi, qingqiu: &Qingqiuti) -> serde_json::Value {
    let mut ti = serde_json::json!({
        "model": peizhi.moxing,
        "messages": goujian_xiaoxilie(qingqiu),
        "temperature": peizhi.wendu,
        "stream": true,
    });
    if let Some(gj) = goujian_gongjulie(qingqiu) {
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

async fn chuli_gongjudiaoyong(fasongqi: &mpsc::Sender<Liushishijian>, xuanze: &serde_json::Value) {
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

fn jiancha_wancheng(xuanze: &serde_json::Value) -> bool {
    xuanze.get("finish_reason").and_then(|f| f.as_str()).is_some_and(|r| !r.is_empty() && r != "null")
}

async fn jiexi_ssehang(hang: &str, fasongqi: &mpsc::Sender<Liushishijian>) -> Option<bool> {
    let shuju = hang.strip_prefix("data: ")?;
    if shuju == "[DONE]" {
        let _ = fasongqi.send(Liushishijian::Wancheng { yuanyin: "stop".to_string() }).await;
        return Some(true);
    }
    let json: serde_json::Value = serde_json::from_str(shuju).ok()?;
    let xuanze = json.get("choices")?.get(0)?;
    if !chuli_wenben(fasongqi, xuanze).await {
        return Some(true);
    }
    chuli_gongjudiaoyong(fasongqi, xuanze).await;
    if jiancha_wancheng(xuanze) {
        let yuanyin = xuanze.get("finish_reason").and_then(|f| f.as_str()).unwrap_or("stop").to_string();
        let _ = fasongqi.send(Liushishijian::Wancheng { yuanyin }).await;
        return Some(true);
    }
    Some(false)
}

async fn xiaofei_liushi(xiangying: reqwest::Response, fasongqi: mpsc::Sender<Liushishijian>) {
    let mut liu = xiangying.bytes_stream();
    let mut huanchong = String::new();
    while let Some(kuai) = liu.next().await {
        let zijie = match kuai {
            Ok(z) => z,
            Err(_) => break,
        };
        huanchong.push_str(&String::from_utf8_lossy(&zijie));
        while let Some(weizhi) = huanchong.find('\n') {
            let hang: String = huanchong.drain(..=weizhi).collect();
            let hang = hang.trim();
            if hang.is_empty() { continue; }
            if jiexi_ssehang(hang, &fasongqi).await == Some(true) {
                return;
            }
        }
    }
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
    let ti = goujian_qingqiuti(&peizhi, &qingqiu);
    let xiangying = match fasong_liushiqingqiu(&peizhi, &ti).await {
        Some(r) => r,
        None => return jiamicuowu(502, "AI服务请求失败", &miyao),
    };
    let (fasongqi, jieshouqi) = mpsc::channel::<Liushishijian>(64);
    let miyao_clone = miyao.clone();
    actix_web::rt::spawn(xiaofei_liushi(xiangying, fasongqi));
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
