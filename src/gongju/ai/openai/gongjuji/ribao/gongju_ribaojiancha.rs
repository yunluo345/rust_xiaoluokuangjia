use crate::peizhixt::peizhixitongzhuti;
use crate::peizhixt::peizhi_nr::peizhi_ai::Ai;
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
}

/// 获取工具关键词
pub fn huoqu_guanjianci() -> Vec<String> {
    vec![
        "日报".to_string(),
        "检查".to_string(),
        "日报检查".to_string(),
        "验证".to_string(),
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
            description: "检查日报原文中是否明确包含所有必需字段的信息。\
            严格规则：只有原文中明确写出的信息才算存在，不得推断、脑补或用模糊表述代替。\
            例如：'对方公司参与人员姓名'必须是具体人名（如张三），'客户方'、'相关负责人'等模糊表述不算；\
            '客户姓名'必须是具体人名，公司名不算。\
            将原文中能明确找到的字段填入对应key，找不到的字段留空字符串\"\"，由系统判断是否缺失。".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "ribaoneirong": {
                        "type": "string",
                        "description": "从日报原文中严格提取的JSON字符串。只填写原文中明确出现的信息，找不到的字段必须留空字符串\"\"。字段包括：我方人员（我方人员姓名）、对方人员（对方人员具体姓名，非公司名、非职位）、日报时间（日期）、交流内容（交流内容）、客户名字（客户具体姓名，非公司名）、地点（地点）、工作内容（工作内容）"
                    }
                },
                "required": ["ribaoneirong"]
            }),
        },
    }
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

    let jieguo = Jianchajieguo {
        hege: queshaoziduanlie.is_empty(),
        queshaoziduanlie,
    };

    json!({"chenggong": true, "shuju": jieguo}).to_string()
}
