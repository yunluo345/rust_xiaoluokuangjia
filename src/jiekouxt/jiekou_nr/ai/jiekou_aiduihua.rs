use actix_web::{HttpRequest, HttpResponse, web};
use serde::Deserialize;
use tokio::sync::mpsc;
use tokio_stream::StreamExt;
use crate::gongju::jiamigongju;
use crate::gongju::ai::openai::aigongju;
use crate::gongju::ai::openai::liushishijian::{Liushishijian, FaxianGongju};
use crate::jiekouxt::jiekouxtzhuti::{self, Jiekoudinyi, Qingqiufangshi};
use crate::jiekouxt::jiamichuanshu::jiamichuanshuzhongjian;
use crate::shujuku::psqlshujuku::shujubiao_nr::ai::shujucaozuo_aiqudao as qudaocaozuo;
use crate::peizhixt::peizhixitongzhuti;
use crate::peizhixt::peizhi_nr::peizhi_ai;
use tiktoken_rs::CoreBPE;
use std::sync::OnceLock;

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
const quanju_xitongtishici: &str = "你是一个专业的AI助手，擅长日报相关工作，同时也能回答用户的其他合理问题。\n\
\n\
基本规则：\n\
1. 优先处理日报相关工作，但也可以回答用户的其他问题，保持友好和耐心。\n\
2. 严禁伪造、编造或捏造任何数据。\n\
3. 当工具调用返回失败（chenggong=false）时，必须仔细分析失败原因：\n\
   - 如果是验证失败（如\"缺少必需的标签类别\"、\"检测到占位符标签\"），说明数据本身不符合要求，不得重试，应直接向用户说明原因。\n\
   - 如果是技术错误（如网络超时、JSON格式错误），可以修正后重试一次。\n\
   - 同一个工具调用，相同参数不得重复调用超过1次。\n\
4. 不得通过添加、修改或伪造数据来绕过工具的验证失败。\n\
5. 工具返回失败后，应立即停止调用工具，直接回复用户说明情况，不要继续尝试。\n\
\n\
标签提取注意事项：\n\
- 提取标签时，标签内容必须直接引用或忠实概括原文中的表述，严禁对原文进行改写、缩写或重新组织语言。如果原文较长，可以合理截取关键段落，但措辞必须与原文一致。\n\
- 当工具返回验证失败时，应检查自己传入的标签内容是否偏离了原文表述，优先自行修正后重试（使用原文原始措辞），而不是要求用户重新提供已有的信息。\n\
- 如果原文中确实缺少某个必需类别的信息，不要用描述性文字（如\"未明确提及\"、\"文中未提到\"）作为标签内容，这会被验证拒绝。\n\
- 正确做法：直接停止标签提取，向用户说明缺少哪些类别的信息，请用户补充。\n\
- 用户补充后，将补充内容通过 buchongxinxi 参数传入，重新提取。\n\
\n\
工具调用数据隔离规则：\n\
- 调用工具时，必须通过工具参数传入该工具所需的全部数据，工具只能看到你传入的参数，无法访问对话上下文。\n\
- 例如调用 tiqubiaoqian 时，必须通过 yuanwen 参数传入日报原文。如果用户在对话中补充了原文中缺失的信息，必须通过 buchongxinxi 参数传入用户补充的原始内容。\n\
- 例如调用 xieribao 审查模式时，必须通过 ribaoneirong 参数传入日报内容。\n\
- 严禁假设工具能看到对话历史中的任何内容，所有工具所需数据必须显式传入参数。\n\
\n\
消息压缩规则：\n\
- 只有当你收到以\"⚠️\"开头的系统消息明确要求压缩时，才可以调用yasuoxiaoxi工具。\n\
- 在没有收到⚠️压缩指令的情况下，严禁主动调用yasuoxiaoxi工具。\n\
- 收到压缩指令后，必须立即调用yasuoxiaoxi工具，提供一个简洁的总结，包含：\n\
  1. 用户的主要问题和需求\n\
  2. 已完成的操作和结果\n\
  3. 当前状态和待解决问题\n\
  4. 重要的上下文信息\n
- 总结后，历史消息将被替换为你的总结，然后继续对话。";

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
    zuidatoken: usize,
}

impl Qudaopeizhi {
    fn cong_shuju(shuju: &serde_json::Value) -> Option<Self> {
        println!("[AI配置] 原始数据: {}", shuju);
        let qu = |ming: &str| shuju.get(ming).and_then(|v| v.as_str()).unwrap_or("").to_string();
        let jiekoudizhi = qu("jiekoudizhi").trim_end_matches('/').to_string();
        if jiekoudizhi.is_empty() { return None; }
        
        let zuidatoken_value = shuju.get("zuidatoken");
        println!("[AI配置] zuidatoken字段值: {:?}", zuidatoken_value);
        
        let zuidatoken = zuidatoken_value
            .and_then(|v| {
                let result = v.as_i64().or_else(|| v.as_str()?.parse().ok());
                println!("[AI配置] zuidatoken解析结果: {:?}", result);
                result
            })
            .unwrap_or(0)
            .max(0) as usize;
        
        println!("[AI配置] 最终zuidatoken: {}", zuidatoken);
        
        Some(Self {
            jiekoudizhi,
            miyao: qu("miyao"),
            moxing: qu("moxing"),
            wendu: qu("wendu").parse().unwrap_or(0.0),
            zuidatoken,
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

#[allow(non_upper_case_globals)]
static fenciqi: OnceLock<CoreBPE> = OnceLock::new();

fn jisuan_tokenshu(wenben: &str) -> usize {
    fenciqi
        .get_or_init(|| tiktoken_rs::o200k_base().unwrap_or_else(|_| tiktoken_rs::cl100k_base().unwrap()))
        .encode_with_special_tokens(wenben)
        .len()
}

fn guji_xiaoxi_token(xiaoxi: &serde_json::Value) -> usize {
    xiaoxi.get("content")
        .and_then(|c| c.as_str())
        .map(jisuan_tokenshu)
        .unwrap_or_else(|| jisuan_tokenshu(&xiaoxi.to_string()))
}

fn yasuo_xiaoxilie(xiaoxilie: &mut Vec<serde_json::Value>, xitongtishici: &str, zuidatoken: usize) {
    if zuidatoken == 0 || xiaoxilie.is_empty() {
        return;
    }
    
    let tishici_tokenshu = jisuan_tokenshu(xitongtishici);
    let keyong = zuidatoken.saturating_sub(tishici_tokenshu);
    
    let meitian_tokenshu: Vec<usize> = xiaoxilie.iter().map(guji_xiaoxi_token).collect();
    let zong: usize = meitian_tokenshu.iter().sum();
    
    println!("[AI压缩] 系统提示词token: {}, 可用token: {}, 最大token: {}", tishici_tokenshu, keyong, zuidatoken);
    println!("[AI压缩] 当前消息总token: {}, 消息数: {}", zong, xiaoxilie.len());
    
    if zong <= keyong {
        println!("[AI压缩] 无需压缩");
        return;
    }
    
    let mut yaoshan = 0usize;
    let mut yishantoken = 0usize;
    let baoliu_zuishao = 1;
    
    while yaoshan < xiaoxilie.len() - baoliu_zuishao && zong - yishantoken > keyong {
        yishantoken += meitian_tokenshu[yaoshan];
        yaoshan += 1;
    }
    
    if yaoshan > 0 {
        println!("[AI压缩] 删除最旧的{}条消息，原始{}条，压缩后{}条", yaoshan, xiaoxilie.len(), xiaoxilie.len() - yaoshan);
        xiaoxilie.drain(..yaoshan);
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

fn goujian_xiaoxilie(qingqiu: &Qingqiuti, zuidatoken: usize) -> Vec<serde_json::Value> {
    let mut jieguo: Vec<serde_json::Value> = vec![];
    
    let mut xitong_neirong = match qingqiu.xitongtishici.as_deref() {
        Some(yonghu_tishici) => format!("{}\n\n{}", quanju_xitongtishici, yonghu_tishici),
        None => quanju_xitongtishici.to_string(),
    };
    
    // 读取AI配置，添加标签提取的类别限制
    if let Some(aipeizhi) = peizhixitongzhuti::duqupeizhi::<peizhi_ai::Aipeizhi>(peizhi_ai::Aipeizhi::wenjianming()) {
        let yunxu_leibie: Vec<&str> = aipeizhi.ribaoshengcheng.xinxi_yingshe.keys().map(|s| s.as_str()).collect();
        if !yunxu_leibie.is_empty() {
            let yunxu_leibie_str = yunxu_leibie.join("、");
            xitong_neirong.push_str(&format!("\n\n工具使用规则：\n调用tiqubiaoqian工具时，只允许提取以下类别的标签：{}。严禁提取其他类别的标签。", yunxu_leibie_str));
        }
    }
    
    jieguo.push(serde_json::json!({"role": "system", "content": xitong_neirong}));
    
    let mut yonghu_xiaoxilie: Vec<serde_json::Value> = qingqiu.xiaoxilie.iter().filter_map(|x| {
        let jiaose = match x.jiaose.as_str() {
            "yonghu" => "user",
            "zhushou" => "assistant",
            _ => return None,
        };
        Some(serde_json::json!({"role": jiaose, "content": x.neirong}))
    }).collect();
    
    // 如果设置了token限制，执行压缩
    if zuidatoken > 0 && !yonghu_xiaoxilie.is_empty() {
        yasuo_xiaoxilie(&mut yonghu_xiaoxilie, &xitong_neirong, zuidatoken);
    }
    
    jieguo.extend(yonghu_xiaoxilie);
    jieguo
}

fn goujian_gongjulie(ewaigongju: &[&str]) -> Option<Vec<serde_json::Value>> {
    let mut lie = aigongju::huoqu_hexingongju_json();
    if !ewaigongju.is_empty() {
        lie.extend(aigongju::huoqu_gongju_json_anming(ewaigongju));
    }
    (!lie.is_empty()).then_some(lie)
}

fn goujian_qingqiuti(peizhi: &Qudaopeizhi, xiaoxilie: &[serde_json::Value], ewaigongju: &[&str]) -> serde_json::Value {
    let mut ti = serde_json::json!({
        "model": peizhi.moxing,
        "messages": xiaoxilie,
        "temperature": peizhi.wendu,
        "stream": true,
    });
    if let Some(gj) = goujian_gongjulie(ewaigongju) {
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
            if fasongqi.send(Liushishijian::Wenbenkuai { neirong: neirong.to_string() }).await.is_err() {
                println!("[AI] 客户端已断开连接，停止发送文本");
                return false;
            }
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

async fn tuisong_gongjuguocheng(fasongqi: &mpsc::Sender<Liushishijian>, xuanze: &serde_json::Value) -> bool {
    let gongjulie = match xuanze.get("delta").and_then(|d| d.get("tool_calls")).and_then(|t| t.as_array()) {
        Some(l) => l,
        None => return true,
    };
    for gongju in gongjulie {
        let hanshu = gongju.get("function");
        let suoyin = gongju.get("index").and_then(|i| i.as_u64()).unwrap_or(0) as usize;
        let gongjuming = hanshu.and_then(|f| f.get("name")).and_then(|n| n.as_str()).unwrap_or("");
        if !gongjuming.is_empty() {
            if fasongqi.send(Liushishijian::Gongjukaishi {
                suoyin,
                gongjuid: gongju.get("id").and_then(|i| i.as_str()).unwrap_or("").to_string(),
                gongjuming: gongjuming.to_string(),
            }).await.is_err() {
                println!("[AI] 客户端已断开连接，停止发送工具开始事件");
                return false;
            }
        }
        let canshu = hanshu.and_then(|f| f.get("arguments")).and_then(|a| a.as_str()).unwrap_or("");
        if !canshu.is_empty() {
            if fasongqi.send(Liushishijian::Gongjucanshu { suoyin, bufen_json: canshu.to_string() }).await.is_err() {
                println!("[AI] 客户端已断开连接，停止发送工具参数");
                return false;
            }
        }
    }
    true
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
                println!("[AI] 客户端断开，停止消费流");
                return (gongjulie, Some("client_disconnected".to_string()));
            }
            if !tuisong_gongjuguocheng(fasongqi, xuanze).await {
                println!("[AI] 客户端断开，停止消费流");
                return (gongjulie, Some("client_disconnected".to_string()));
            }
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

/// 意图分析提示词：让AI分析用户意图并生成搜索关键词
#[allow(non_upper_case_globals)]
const yitu_fenxi_tishici: &str = "你是一个意图分析助手。分析用户的最新消息，结合对话历史上下文，判断需要使用什么工具来处理。\n\
\n\
请输出JSON格式（不要输出其他内容）：\n\
{\"yitu\": \"一句话描述用户意图\", \"guanjianci\": [\"关键词1\", \"关键词2\", ...]}\n\
\n\
关键词要求：\n\
- 生成3-8个与所需工具功能相关的关键词\n\
- 包含动作词（如：提取、分析、搜索、生成）\n\
- 包含对象词（如：标签、实体、人名、时间、日报）\n\
- 包含场景词（如：文本分析、信息提取、数据处理）\n\
\n\
上下文感知规则（非常重要）：\n\
如姓名、客户公司、对方派遣人员等
- 判断依据：对话历史中有日报内容、有工具调用失败记录（如缺少标签类别）、AI曾要求用户补充信息，而用户当前消息正在回应这个请求。\n\
- 不要把对未完成流程的补充信息当作\"日常对话\"。\n\
\n\
示例：\n\
用户发了一篇日报 → {\"yitu\": \"处理日报，提取关键信息\", \"guanjianci\": [\"日报\", \"标签\", \"提取\", \"实体识别\", \"人名\", \"时间\", \"文本分析\"]}\n\
对话历史中日报标签提取失败缺少姓名，用户说\"我叫张三\" → {\"yitu\": \"补充日报缺失信息，继续日报流程\", \"guanjianci\": [\"日报\", \"补充\", \"标签\", \"提取\", \"实体识别\", \"信息提取\"]}\n\
用户问天气 → {\"yitu\": \"查询天气信息\", \"guanjianci\": [\"天气\", \"查询\", \"气象\"]}\n\
用户闲聊 → {\"yitu\": \"日常对话\", \"guanjianci\": []}";

/// 意图分析结果
struct Yitujieguo {
    yitu: String,
    guanjianci: Vec<String>,
}

/// 第一阶段：调用AI进行意图分析，生成搜索关键词
async fn fenxi_yitu(peizhi: &Qudaopeizhi, yonghu_xiaoxi: &str, duihua_shangxiawen: &str) -> Option<Yitujieguo> {
    // 将对话上下文和当前消息组合传入，让AI能感知未完成的流程
    let fenxi_neirong = if duihua_shangxiawen.is_empty() {
        yonghu_xiaoxi.to_string()
    } else {
        format!("【近期对话上下文】\n{}\n\n【用户最新消息】\n{}", duihua_shangxiawen, yonghu_xiaoxi)
    };

    let xiaoxilie = serde_json::json!([
        {"role": "system", "content": yitu_fenxi_tishici},
        {"role": "user", "content": fenxi_neirong}
    ]);

    let ti = serde_json::json!({
        "model": peizhi.moxing,
        "messages": xiaoxilie,
        "temperature": 0.1,
        "max_tokens": 200,
        "stream": false,
    });

    let xiangying = reqwest::Client::new()
        .post(peizhi.goujian_url())
        .header("Authorization", format!("Bearer {}", peizhi.miyao))
        .header("Content-Type", "application/json")
        .timeout(std::time::Duration::from_secs(15))
        .json(&ti)
        .send()
        .await
        .ok()?;

    let json: serde_json::Value = xiangying.json().await.ok()?;
    let neirong = json.get("choices")?.get(0)?
        .get("message")?.get("content")?.as_str()?;

    println!("[AI意图分析] 原始返回: {}", neirong);

    // 解析JSON（兼容markdown代码块包裹）
    let jinghua = neirong.trim()
        .trim_start_matches("```json").trim_start_matches("```")
        .trim_end_matches("```").trim();

    let jieguo: serde_json::Value = serde_json::from_str(jinghua).ok()?;
    let yitu = jieguo.get("yitu")?.as_str()?.to_string();
    let guanjianci: Vec<String> = jieguo.get("guanjianci")?
        .as_array()?
        .iter()
        .filter_map(|v| v.as_str().map(|s| s.to_string()))
        .collect();

    Some(Yitujieguo { yitu, guanjianci })
}

/// 工具发现阶段：先AI分析意图生成关键词，再用关键词搜索工具
async fn gongju_faxian(
    peizhi: &Qudaopeizhi,
    xiaoxilie: &[serde_json::Value],
    fasongqi: &mpsc::Sender<Liushishijian>,
) -> Vec<String> {
    // 提取用户最后一条消息
    let yonghu_xiaoxi = xiaoxilie.iter().rev()
        .find(|x| x.get("role").and_then(|r| r.as_str()) == Some("user"))
        .and_then(|x| x.get("content").and_then(|c| c.as_str()))
        .unwrap_or("");

    if yonghu_xiaoxi.is_empty() {
        return vec![];
    }

    // 提取最近的对话上下文（最多取最后6条非system消息），用于意图分析感知未完成的流程
    let duihua_shangxiawen: String = xiaoxilie.iter()
        .filter(|x| x.get("role").and_then(|r| r.as_str()) != Some("system"))
        .rev()
        .skip(1) // 跳过最后一条（即当前用户消息，已单独传入）
        .take(6)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .filter_map(|x| {
            let jiaose = x.get("role").and_then(|r| r.as_str()).unwrap_or("?");
            let neirong = x.get("content").and_then(|c| c.as_str()).unwrap_or("");
            if neirong.is_empty() { return None; }
            // 截断过长的内容，只保留前200字符用于意图判断（字符安全截断）
            let duanneirong: String = if neirong.chars().count() > 200 {
                neirong.chars().take(200).collect()
            } else {
                neirong.to_string()
            };
            Some(format!("[{}]: {}", jiaose, duanneirong))
        })
        .collect::<Vec<_>>()
        .join("\n");

    // 推送：开始分析意图
    let _ = fasongqi.send(Liushishijian::Sikaoguocheng {
        neirong: "正在分析用户意图...".to_string(),
    }).await;

    // 第一步：AI意图分析（传入对话上下文）
    let yitu_jieguo = match fenxi_yitu(peizhi, yonghu_xiaoxi, &duihua_shangxiawen).await {
        Some(j) => j,
        None => {
            println!("[AI意图分析] 分析失败，跳过工具发现");
            let _ = fasongqi.send(Liushishijian::Sikaoguocheng {
                neirong: "意图分析未完成，使用基础能力回答".to_string(),
            }).await;
            return vec![];
        }
    };

    println!("[AI意图分析] 意图: {}, 关键词: {:?}", yitu_jieguo.yitu, yitu_jieguo.guanjianci);

    // 推送：意图分析结果
    let _ = fasongqi.send(Liushishijian::Yitufenxi {
        yitu: yitu_jieguo.yitu.clone(),
        guanjianci: yitu_jieguo.guanjianci.clone(),
    }).await;

    // 如果没有关键词，说明不需要工具
    if yitu_jieguo.guanjianci.is_empty() {
        let _ = fasongqi.send(Liushishijian::Sikaoguocheng {
            neirong: "无需专用工具，直接回答".to_string(),
        }).await;
        return vec![];
    }

    // 推送：开始搜索工具
    let _ = fasongqi.send(Liushishijian::Sikaoguocheng {
        neirong: format!("根据关键词「{}」搜索可用工具...", yitu_jieguo.guanjianci.join("、")),
    }).await;

    // 第二步：用AI生成的关键词搜索工具
    let sousuo_wenben = yitu_jieguo.guanjianci.join(" ");
    let sousuojieguo = aigongju::sousuo_gongju(&sousuo_wenben, 5);

    if sousuojieguo.is_empty() {
        let _ = fasongqi.send(Liushishijian::Sikaoguocheng {
            neirong: "未找到匹配的工具，使用基础能力回答".to_string(),
        }).await;
        return vec![];
    }

    let faxian_lie: Vec<FaxianGongju> = sousuojieguo.iter().map(|s| FaxianGongju {
        mingcheng: s.gongju.mingcheng.to_string(),
        miaoshu: s.gongju.miaoshu.to_string(),
        defen: s.defen,
        yuanyin: s.yuanyin.clone(),
    }).collect();

    let gongjuming_lie: Vec<String> = sousuojieguo.iter()
        .map(|s| s.gongju.mingcheng.to_string())
        .collect();

    println!("[AI工具发现] 关键词: {:?}, 发现工具: {:?}", yitu_jieguo.guanjianci, gongjuming_lie);

    // 推送：发现了哪些工具
    let miaoshu_lie: Vec<String> = sousuojieguo.iter()
        .map(|s| format!("{}({})", s.gongju.mingcheng, s.gongju.miaoshu))
        .collect();
    let _ = fasongqi.send(Liushishijian::Sikaoguocheng {
        neirong: format!("发现{}个相关工具：{}，正在加载...", gongjuming_lie.len(), miaoshu_lie.join("、")),
    }).await;

    // 推送：工具发现详情
    let _ = fasongqi.send(Liushishijian::Gongjufaxian {
        yitu: yitu_jieguo.yitu,
        jieguo: faxian_lie,
    }).await;

    // 推送：加载完成
    let _ = fasongqi.send(Liushishijian::Sikaoguocheng {
        neirong: "工具已加载完成，开始处理请求".to_string(),
    }).await;

    gongjuming_lie
}

async fn zhixing_react_xunhuan(
    peizhi: &Qudaopeizhi,
    mut xiaoxilie: Vec<serde_json::Value>,
    fasongqi: mpsc::Sender<Liushishijian>,
) {
    // 第一阶段：工具发现
    let faxian_gongjulie = gongju_faxian(peizhi, &xiaoxilie, &fasongqi).await;
    let ewai_refs: Vec<&str> = faxian_gongjulie.iter().map(|s| s.as_str()).collect();

    // 如果发现了工具，将工具目录追加到系统提示词中
    if !faxian_gongjulie.is_empty() {
        let gongjumulu = aigongju::shengcheng_gongjumulu();
        if !gongjumulu.is_empty() {
            if let Some(xitong_xiaoxi) = xiaoxilie.first_mut() {
                if let Some(neirong) = xitong_xiaoxi.get("content").and_then(|c| c.as_str()).map(|s| s.to_string()) {
                    xitong_xiaoxi["content"] = serde_json::Value::String(format!("{}{}", neirong, gongjumulu));
                }
            }
        }
    }

    for lun in 0..zuida_xunhuancishu {
        println!("[AI] ReAct第{}轮开始，当前消息数: {}, 可用工具: 核心+{:?}", lun + 1, xiaoxilie.len(), faxian_gongjulie);
        
        if peizhi.zuidatoken > 0 && xiaoxilie.len() > 1 {
            let tishici_tokenshu = xiaoxilie.first()
                .and_then(|x| x.get("content"))
                .and_then(|c| c.as_str())
                .map(jisuan_tokenshu)
                .unwrap_or(0);
            let keyong = peizhi.zuidatoken.saturating_sub(tishici_tokenshu);
            let zong_token: usize = xiaoxilie.iter().skip(1).map(guji_xiaoxi_token).sum();
            
            if zong_token > keyong {
                println!("[AI压缩] ReAct轮次token超限: 当前{}, 可用{}, 需要AI总结压缩", zong_token, keyong);
                xiaoxilie.retain(|x| {
                    x.get("role").and_then(|r| r.as_str()) != Some("system")
                        || x.get("content").and_then(|c| c.as_str()).map_or(true, |s| !s.starts_with("⚠️"))
                });
                xiaoxilie.push(serde_json::json!({
                    "role": "system",
                    "content": format!("⚠️ 对话历史已超过token限制（当前{}，限制{}），请立即调用yasuoxiaoxi工具总结历史对话。", zong_token, keyong)
                }));
            }
        }
        
        // 第二阶段：使用核心工具 + 发现的工具
        let ti = goujian_qingqiuti(peizhi, &xiaoxilie, &ewai_refs);
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
        
        // 检查客户端是否断开
        if yuanyin.as_deref() == Some("client_disconnected") {
            println!("[AI] 检测到客户端断开，停止ReAct循环");
            return;
        }
        
        if gongjulie.is_empty() {
            let _ = fasongqi.send(Liushishijian::Wancheng {
                yuanyin: yuanyin.unwrap_or_else(|| "stop".to_string()),
            }).await;
            println!("[AI] ReAct结束，AI决定停止");
            return;
        }
        xiaoxilie.push(goujian_gongjuxiaoxi(&gongjulie));
        
        let mut gongjujieguo_lie: Vec<(String, String)> = Vec::new();
        let mut yasuo_zongjie: Option<String> = None;
        
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
            gongjujieguo_lie.push((d.id.clone(), jieguo.function.arguments.clone()));
            
            if d.mingcheng == "yasuoxiaoxi" {
                if let Ok(j) = serde_json::from_str::<serde_json::Value>(&jieguo.function.arguments) {
                    if j.get("chenggong").and_then(|v| v.as_bool()).unwrap_or(false) {
                        yasuo_zongjie = j.get("zongjie").and_then(|v| v.as_str()).map(|s| s.to_string());
                    }
                }
            }
            
            if fasongqi.send(Liushishijian::Gongjuwancheng {
                suoyin: 0,
                gongjuid: d.id.clone(),
                gongjuming: d.mingcheng.clone(),
                canshu: d.canshu.clone(),
            }).await.is_err() {
                return;
            }
            if fasongqi.send(Liushishijian::Gongjujieguo {
                gongjuid: d.id.clone(),
                gongjuming: d.mingcheng.clone(),
                jieguo: jieguo.function.arguments.clone(),
            }).await.is_err() {
                return;
            }
        }
        
        if let Some(zongjie) = &yasuo_zongjie {
            println!("[AI压缩] AI总结: {}", zongjie);
            let xitong_xiaoxi = xiaoxilie[0].clone();
            xiaoxilie = vec![
                xitong_xiaoxi,
                serde_json::json!({
                    "role": "user",
                    "content": format!("【历史对话总结】\n{}", zongjie)
                })
            ];
            println!("[AI压缩] 历史消息已替换为总结，当前消息数: {}", xiaoxilie.len());
            let _ = fasongqi.send(Liushishijian::Yasuowancheng {
                zongjie: zongjie.clone(),
            }).await;
            continue; // 压缩完成后直接进入下一轮正常对话，避免重复压缩
        } else {
            for (id, jieguo_str) in &gongjujieguo_lie {
                xiaoxilie.push(serde_json::json!({
                    "role": "tool",
                    "tool_call_id": id,
                    "content": jieguo_str,
                }));
            }
        }
    }
    println!("[AI] ReAct达到最大循环次数");
    let _ = fasongqi.send(Liushishijian::Wancheng { yuanyin: "max_loops".to_string() }).await;
}

fn jiamicuowu(zhuangtaima: u16, xinxi: &str, miyao: &[u8]) -> HttpResponse {
    println!("[加密错误] 准备加密错误响应: 状态码={}, 消息={}", zhuangtaima, xinxi);
    let xiangying = jiekouxtzhuti::shibai(zhuangtaima, xinxi);
    let jiami_xiangying = jiamichuanshuzhongjian::jiamixiangying(xiangying, miyao);
    println!("[加密错误] 错误响应已加密并返回");
    jiami_xiangying
}

async fn zhixing_duihua(qingqiu: Qingqiuti, miyao: Vec<u8>) -> HttpResponse {
    let qudaoshuju = match qudaocaozuo::lunxun_daichongshi(&qingqiu.leixing).await {
        Ok(s) => s,
        Err(e) => {
            println!("[接口层] 渠道获取失败，返回错误: {} (状态码: {})", e.xiaoxi(), e.zhuangtaima());
            return jiamicuowu(e.zhuangtaima(), e.xiaoxi(), &miyao);
        }
    };
    let peizhi = match Qudaopeizhi::cong_shuju(&qudaoshuju) {
        Some(p) => p,
        None => return jiamicuowu(500, "渠道配置解析失败", &miyao),
    };
    let xiaoxilie = goujian_xiaoxilie(&qingqiu, peizhi.zuidatoken);
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
