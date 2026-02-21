use crate::peizhixt::peizhixitongzhuti;
use crate::peizhixt::peizhi_nr::peizhi_ai::Ai;
use crate::gongju::ai::openai::{aipeizhi, aixiaoxiguanli, openaizhuti};
use llm::chat::Tool;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Clone)]
pub enum Gongjufenzu {
    Guanli,
    Xitong,
}

#[derive(Deserialize)]
struct Qingqiucanshu {
    ribaoneirong: String,
}

#[derive(Serialize)]
struct Jianchajieguo {
    hege: bool,
    queshaoziduanlie: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    aiyijian: Option<String>,
}

/// 获取工具关键词
pub fn huoqu_guanjianci() -> Vec<String> {
    vec![
        "日报检查".to_string(),
        "检查日报".to_string(),
        "验证日报".to_string(),
        "日报验证".to_string(),
        "标签".to_string(),
        "日报标签".to_string(),
        "合格".to_string(),
        "report".to_string(),
        "check".to_string(),
        "validate".to_string(),
    ]
}

/// 获取工具分组
pub fn huoqu_fenzu() -> Gongjufenzu {
    Gongjufenzu::Xitong
}

/// 工具定义
pub fn dinyi() -> Tool {
    Tool {
        tool_type: "function".to_string(),
        function: llm::chat::FunctionTool {
            name: "ribao_jiancha".to_string(),
            description: "检查日报内容是否包含必填字段，返回缺失字段列表".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "ribaoneirong": {
                        "type": "string",
                        "description": "日报内容的JSON字符串，包含字段：我方人员、对方人员、日报时间、交流内容、客户名字、地点、工作内容"
                    }
                },
                "required": ["ribaoneirong"]
            }),
        },
    }
}

pub async fn ai_jiancha(neirong: &str, peizhi: &Ai) -> Option<String> {
    let bitian_biaoqian: Vec<String> = peizhi.ribao_biaoqian.iter()
        .filter(|bq| bq.bitian)
        .map(|bq| format!("【{}】{}", bq.mingcheng, bq.miaoshu))
        .collect();
    
    let xitongtishici = format!(
        "你是严格的日报审核助手。请逐项检查以下必填标签是否都有具体内容：\n\n{}\n\n\
        审核标准：\n\
        1. 每个必填标签必须有明确的具体信息（如姓名必须是真实姓名，不能是「客户方」「相关负责人」等泛指）\n\
        2. 日期必须是完整日期格式\n\
        3. 地点必须是具体地点\n\
        4. 内容描述必须清晰具体\n\n\
        如果所有必填标签都完整且具体，回复「内容完整规范」。\n\
        如果有任何必填标签缺失或不够具体，明确指出问题（30字以内）。",
        bitian_biaoqian.join("\n")
    );
    
    let aipeizhi = crate::jiekouxt::jiekou_nr::ai::huoqu_peizhi().await?
        .shezhi_chaoshi(30).shezhi_chongshi(1);
    
    let mut guanli = aixiaoxiguanli::Xiaoxiguanli::xingjian()
        .shezhi_xitongtishici(&xitongtishici);
    guanli.zhuijia_yonghuxiaoxi(format!("日报内容：\n{}", neirong));
    
    openaizhuti::putongqingqiu(&aipeizhi, &guanli).await
}

/// 工具执行
pub async fn zhixing(canshu: &str, _lingpai: &str) -> String {
    let qingqiu: Qingqiucanshu = match serde_json::from_str(canshu) {
        Ok(q) => q,
        Err(_) => return json!({"cuowu": "参数格式错误"}).to_string(),
    };

    let ribaoshuju: Value = match serde_json::from_str(&qingqiu.ribaoneirong) {
        Ok(v) => v,
        Err(_) => return json!({"cuowu": "日报内容格式错误"}).to_string(),
    };

    let peizhi = match peizhixitongzhuti::duqupeizhi::<Ai>(Ai::wenjianming()) {
        Some(p) => p,
        None => return json!({"cuowu": "无法读取配置"}).to_string(),
    };

    let queshaoziduanlie: Vec<String> = peizhi
        .ribao_biaoqian
        .iter()
        .filter(|biaoqian| biaoqian.bitian)
        .filter_map(|biaoqian| {
            let zhi = ribaoshuju.get(&biaoqian.mingcheng);
            match zhi {
                Some(v) if !v.is_null() && !v.as_str().unwrap_or("").trim().is_empty() => None,
                _ => Some(biaoqian.miaoshu.clone()),
            }
        })
        .collect();

    let aiyijian = ai_jiancha(&qingqiu.ribaoneirong, &peizhi).await;
    
    let ai_hege = aiyijian.as_ref()
        .map(|yj| yj.contains("内容完整规范"))
        .unwrap_or(true);

    let jieguo = Jianchajieguo {
        hege: queshaoziduanlie.is_empty() && ai_hege,
        queshaoziduanlie,
        aiyijian,
    };

    json!({"chenggong": true, "shuju": jieguo}).to_string()
}
