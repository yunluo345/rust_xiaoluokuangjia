use crate::gongju::ai::openai::{aipeizhi, aixiaoxiguanli, openaizhuti};
use crate::gongju::jwtgongju;
use crate::peizhixt::peizhi_nr::peizhi_ai::Ai;
use crate::peizhixt::peizhixitongzhuti;
use crate::shujuku::psqlshujuku::shujubiao_nr::ribao::{
    shujucaozuo_ribao,
    shujucaozuo_ribao_biaoqianrenwu,
};
use llm::chat::Tool;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::time::{SystemTime, UNIX_EPOCH};
use super::super::Gongjufenzu;

#[derive(Deserialize)]
struct Qingqiucanshu {
    neirong: Option<String>,
    ribaoneirong: Option<String>,
    fabushijian: Option<String>,
    buchongbiaoqian: Option<std::collections::HashMap<String, String>>,
}

#[derive(Serialize)]
struct Jianchajieguo {
    hege: bool,
    queshaoziduanlie: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    aiyijian: Option<String>,
}

pub fn huoqu_guanjianci() -> Vec<String> {
    vec![
        "日报检查".to_string(),
        "检查日报".to_string(),
        "验证日报".to_string(),
        "日报验证".to_string(),
        "标签".to_string(),
        "日报标签".to_string(),
        "合格".to_string(),
        "提交日报".to_string(),
        "发布日报".to_string(),
        "保存日报".to_string(),
        "新增日报".to_string(),
        "写日报".to_string(),
    ]
}

pub fn huoqu_fenzu() -> Gongjufenzu {
    Gongjufenzu::Xitong
}

pub fn dinyi() -> Tool {
    Tool {
        tool_type: "function".to_string(),
        function: llm::chat::FunctionTool {
            name: "ribao_jiancha".to_string(),
            description: "审核日报内容，审核通过后自动提交并创建标签任务；审核不通过返回错误".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "neirong": {"type": "string", "description": "日报完整内容（优先使用）"},
                    "ribaoneirong": {"type": "string", "description": "兼容字段：日报内容"},
                    "fabushijian": {"type": "string", "description": "可选：发布时间时间戳字符串，未传则自动使用当前时间"}
                }
            }),
        },
    }
}

async fn tiqu_biaoqian(neirong: &str, peizhi: &Ai) -> Option<String> {
    let biaoqian_shuoming: Vec<String> = peizhi.ribao_biaoqian.iter()
        .map(|bq| format!("{}（别称：{}）", bq.mingcheng, bq.biecheng.join("、")))
        .collect();
    let xitongtishici = format!(
        "从日报文本中提取以下标签信息，返回纯JSON格式（不要markdown代码块）：\n\n{}\n\n严格提取规则：\n1. 标签名使用配置中的「标准名称」（如\"我方人员\"而非\"汇报人\"）\n2. 只提取文中**明确写出的具体信息**（如\"张三\"\"2026-02-14\"\"北京市海淀区\"）\n3. 如果文中只有泛指词汇（\"客户方\"\"相关负责人\"\"对方\"\"联系人\"\"办公室\"\"公司\"等），该字段值设为null\n4. 绝对禁止推测、编造或从上下文猜测未明确写出的信息\n5. 只返回JSON对象，格式：{{\"我方人员\":\"张三\",\"对方人员\":null,\"日报时间\":\"2026-02-14\",...}}",
        biaoqian_shuoming.join("\n")
    );
    let aipeizhi = crate::jiekouxt::jiekou_nr::ai::huoqu_peizhi().await?.shezhi_chaoshi(30).shezhi_chongshi(1);
    let mut guanli = aixiaoxiguanli::Xiaoxiguanli::xingjian().shezhi_xitongtishici(&xitongtishici);
    guanli.zhuijia_yonghuxiaoxi(format!("日报内容：\n{}", neirong));
    openaizhuti::putongqingqiu(&aipeizhi, &guanli).await
}

pub async fn ai_jiancha(neirong: &str, peizhi: &Ai) -> Option<String> {
    let bitian_biaoqian: Vec<String> = peizhi.ribao_biaoqian.iter().filter(|bq| bq.bitian).map(|bq| format!("【{}】{}", bq.mingcheng, bq.miaoshu)).collect();
    let xitongtishici = format!(
        "检查以下必填标签的值是否有效：\n\n{}\n\n判断标准：\n1. 值不能为空、null、\"无\"、\"待定\"等无效内容\n2. 日期格式合理（如2026-02-14）\n3. 姓名必须是具体姓名，不能是「客户方」「相关负责人」「对方」「甲方」「乙方」「联系人」等泛指\n4. 地点必须是具体地点，不能是「办公室」「公司」等泛指\n\n如果所有必填标签都有有效值，回复「内容完整规范」。\n如果有标签值是泛指或无效，明确指出问题（30字以内）。",
        bitian_biaoqian.join("\n")
    );
    let aipeizhi = crate::jiekouxt::jiekou_nr::ai::huoqu_peizhi().await?.shezhi_chaoshi(30).shezhi_chongshi(1);
    let mut guanli = aixiaoxiguanli::Xiaoxiguanli::xingjian().shezhi_xitongtishici(&xitongtishici);
    guanli.zhuijia_yonghuxiaoxi(format!("提取后的日报数据：\n{}", neirong));
    openaizhuti::putongqingqiu(&aipeizhi, &guanli).await
}

fn huoqu_neirong(qingqiu: &Qingqiucanshu) -> Option<String> {
    qingqiu.neirong.as_deref().or(qingqiu.ribaoneirong.as_deref()).map(str::trim).filter(|v| !v.is_empty()).map(ToString::to_string)
}

fn huoqu_fabushijian(qingqiu: &Qingqiucanshu) -> String {
    qingqiu.fabushijian.as_deref()
        .map(str::trim)
        .filter(|v| !v.is_empty())
        .map(ToString::to_string)
        .unwrap_or_else(|| {
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_millis().to_string())
                .unwrap_or_else(|_| "0".to_string())
        })
}

fn shifou_youxiao(v: Option<&Value>) -> bool {
    match v {
        Some(z) if !z.is_null() => z.as_str().map(|s| !s.trim().is_empty()).or_else(|| z.as_i64().map(|_| true)).or_else(|| z.as_u64().map(|_| true)).unwrap_or(false),
        _ => false,
    }
}

fn jiancha_fanzhici(v: Option<&Value>) -> Option<String> {
    const FANZHICI: &[&str] = &["客户方", "相关负责人", "对方", "联系人", "甲方", "乙方", "待定", "无", "暂无"];
    v.and_then(|z| z.as_str()).and_then(|s| {
        let neirong = s.trim();
        // 只检测纯泛指：如果内容只是泛指词本身或包含泛指但没有具体信息
        FANZHICI.iter().find(|&&fc| {
            neirong == fc || (neirong.contains(fc) && neirong.len() < fc.len() + 5)
        }).map(|fc| fc.to_string())
    })
}

pub async fn zhixing(canshu: &str, lingpai: &str) -> String {
    let zaiti = match jwtgongju::yanzheng(lingpai).await {
        Some(z) => z,
        None => return json!({"cuowu": "令牌无效或已过期"}).to_string(),
    };
    let qingqiu: Qingqiucanshu = match serde_json::from_str(canshu) {
        Ok(q) => q,
        Err(_) => return json!({"cuowu": "参数格式错误"}).to_string(),
    };
    let neirong = match huoqu_neirong(&qingqiu) {
        Some(v) => v,
        None => return json!({"cuowu": "日报内容不能为空"}).to_string(),
    };
    let peizhi = match peizhixitongzhuti::duqupeizhi::<Ai>(Ai::wenjianming()) {
        Some(p) => p,
        None => return json!({"cuowu": "无法读取配置"}).to_string(),
    };
    let mut ribaoshuju: Value = match serde_json::from_str(&neirong) {
        Ok(v) => v,
        Err(_) => {
            let json_str = match tiqu_biaoqian(&neirong, &peizhi).await {
                Some(s) => s.trim().trim_start_matches("```json").trim_end_matches("```").trim().to_string(),
                None => return json!({"cuowu": "AI提取标签失败，请稍后重试"}).to_string(),
            };
            match serde_json::from_str(&json_str) {
                Ok(v) => v,
                Err(_) => return json!({"cuowu": "AI返回格式错误，请检查日报内容"}).to_string(),
            }
        }
    };
    if let Some(buchong) = &qingqiu.buchongbiaoqian {
        if let Some(obj) = ribaoshuju.as_object_mut() {
            for (k, v) in buchong {
                obj.insert(k.clone(), Value::String(v.clone()));
            }
        }
    }
    let mut queshaoziduanlie: Vec<String> = peizhi.ribao_biaoqian.iter()
        .filter(|b| b.bitian)
        .filter_map(|b| {
            (!shifou_youxiao(ribaoshuju.get(&b.mingcheng)))
                .then_some(b.miaoshu.clone())
        })
        .collect();
    let fanzhiciziduanlie: Vec<String> = peizhi.ribao_biaoqian.iter()
        .filter(|b| b.bitian)
        .filter_map(|b| {
            jiancha_fanzhici(ribaoshuju.get(&b.mingcheng))
                .map(|fc| format!("{}包含泛指「{}」", b.miaoshu, fc))
        })
        .collect();
    queshaoziduanlie.extend(fanzhiciziduanlie);
    let jiegouhua_neirong = serde_json::to_string_pretty(&ribaoshuju).unwrap_or_default();
    let aiyijian = ai_jiancha(&jiegouhua_neirong, &peizhi).await;
    let ai_hege = aiyijian.as_ref()
        .map(|v| v.contains("内容完整规范"))
        .unwrap_or(false);
    let hege = queshaoziduanlie.is_empty() && ai_hege;
    let yuanyin = if hege {
        String::new()
    } else if !queshaoziduanlie.is_empty() {
        format!("缺少必填字段：{}", queshaoziduanlie.join("、"))
    } else {
        aiyijian.clone().unwrap_or_else(|| "内容不符合规范，请补充完整信息后重试".to_string())
    };
    let jianchajieguo = Jianchajieguo { hege, queshaoziduanlie, aiyijian };
    if !hege {
        return json!({"cuowu": "日报审核未通过", "yuanyin": yuanyin, "shuju": jianchajieguo}).to_string();
    }
    let fabushijian = huoqu_fabushijian(&qingqiu);
    let ribaoid = match shujucaozuo_ribao::xinzeng(&zaiti.yonghuid, &neirong, &fabushijian).await {
        Some(id) => id,
        None => return json!({"cuowu": "日报提交失败"}).to_string(),
    };
    let renwuchongshi = peizhi.ribao_biaoqianrenwu_chongshi_cishu as i64;
    let renwuid = match shujucaozuo_ribao_biaoqianrenwu::faburenwu(&ribaoid, &zaiti.yonghuid, renwuchongshi).await {
        Some(id) => id,
        None => return json!({"cuowu": "日报提交成功但任务发布失败", "ribaoid": ribaoid}).to_string(),
    };
    json!({"chenggong": true, "shuju": jianchajieguo, "ribaoid": ribaoid, "renwuid": renwuid}).to_string()
}
