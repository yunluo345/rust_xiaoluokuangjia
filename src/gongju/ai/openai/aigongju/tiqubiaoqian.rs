use llm::chat::{FunctionTool, Tool};
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::gongju::ai::openai::aipeizhi::Aipeizhi;
use crate::gongju::ai::openai::aixiaoxiguanli::Xiaoxiguanli;
use crate::gongju::ai::openai::openaizhuti;

#[allow(non_upper_case_globals)]
const gongjumingcheng: &str = "tiqubiaoqian";

#[allow(non_upper_case_globals)]
const xitongtishici: &str = "你是一个专业的信息提取助手。你的任务是从用户提交的文本中提取关键信息作为标签。\
标签包括但不限于：人名、地名、组织名、项目名、技术术语、时间节点等重要信息。\
你必须调用提供的工具函数来返回结果，每个标签需要有名称和类别。\
类别包括：人名、地名、组织、项目、技术、时间、其他。\
只提取有意义的关键信息，忽略无关紧要的内容。";

#[derive(Debug, Serialize, Deserialize)]
pub struct Biaoqian {
    pub mingcheng: String,
    pub leibie: String,
}

/// 构建标签提取工具的 Tool 定义
pub fn goujian_gongju() -> Tool {
    Tool {
        tool_type: "function".to_string(),
        function: FunctionTool {
            name: gongjumingcheng.to_string(),
            description: "从文本中提取关键信息标签，包括人名、地名、组织名等".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "biaoqianlie": {
                        "type": "array",
                        "description": "提取出的标签列表",
                        "items": {
                            "type": "object",
                            "properties": {
                                "mingcheng": {
                                    "type": "string",
                                    "description": "标签名称，如具体的人名、地名等"
                                },
                                "leibie": {
                                    "type": "string",
                                    "description": "标签类别",
                                    "enum": ["人名", "地名", "组织", "项目", "技术", "时间", "其他"]
                                }
                            },
                            "required": ["mingcheng", "leibie"]
                        }
                    }
                },
                "required": ["biaoqianlie"]
            }),
        },
    }
}

/// 从文本中提取标签，返回标签列表
pub async fn tiqu(peizhi: &Aipeizhi, wenben: &str) -> Option<Vec<Biaoqian>> {
    let mut guanli = Xiaoxiguanli::xingjian()
        .shezhi_xitongtishici(xitongtishici)
        .tianjia_gongju(goujian_gongju());
    guanli.zhuijia_yonghuxiaoxi(wenben);
    let jieguo = openaizhuti::gongjuqingqiu(peizhi, &mut guanli).await?;
    match jieguo {
        openaizhuti::Aijieguo::Gongjudiaoyong(diaoyonglie) => {
            jiexi_gongjujieguo(&diaoyonglie)
        }
        openaizhuti::Aijieguo::Wenben(_) => None,
    }
}

/// 解析工具调用结果中的标签数据
fn jiexi_gongjujieguo(diaoyonglie: &[llm::ToolCall]) -> Option<Vec<Biaoqian>> {
    let diaoyong = diaoyonglie.iter()
        .find(|d| d.function.name == gongjumingcheng)?;
    let canshu: serde_json::Value = serde_json::from_str(&diaoyong.function.arguments).ok()?;
    let biaoqianlie = canshu.get("biaoqianlie")?.as_array()?;
    let jieguo: Vec<Biaoqian> = biaoqianlie.iter()
        .filter_map(|v| serde_json::from_value(v.clone()).ok())
        .collect();
    (!jieguo.is_empty()).then_some(jieguo)
}
