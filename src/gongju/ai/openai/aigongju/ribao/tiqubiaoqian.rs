use llm::chat::{FunctionTool, Tool};
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::gongju::ai::openai::aipeizhi::Aipeizhi;
use crate::gongju::ai::openai::aixiaoxiguanli::Xiaoxiguanli;
use crate::gongju::ai::openai::openaizhuti;
use crate::peizhixt::peizhixitongzhuti;
use crate::peizhixt::peizhi_nr::peizhi_ai::Aipeizhi as Aipeizhiwenjian;

// ==================== 常量定义 ====================

/// 工具名称
#[allow(non_upper_case_globals)]
const gongju_mingcheng: &str = "tiqubiaoqian";

// ==================== 数据结构 ====================

/// 标签实体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Biaoqian {
    pub mingcheng: String,
    pub leibie: String,
}

/// 标签提取结果
#[derive(Debug, Serialize, Deserialize)]
pub struct Tiqujieguo {
    pub chenggong: bool,
    pub xiaoxi: String,
    pub biaoqianlie: Vec<Biaoqian>,
}

// ==================== 公共接口 ====================

/// 构建标签提取工具定义
pub fn goujian_gongju() -> Tool {
    let peizhi = duqu_aipeizhi();
    let leibie_miaoshu = match &peizhi {
        Some(p) => {
            let suoyou: Vec<&str> = p.ribaoshengcheng.xinxi_yingshe.keys().map(|s| s.as_str()).collect();
            suoyou.join("、")
        }
        None => "人名、地名、时间、对话内容".to_string(),
    };

    Tool {
        tool_type: "function".to_string(),
        function: FunctionTool {
            name: gongju_mingcheng.to_string(),
            description: format!(
                "从文本中提取关键信息标签。必需的标签类别：{}。\n\
                提取规则：\n\
                1. 提取内容不限于专有名词，也包括描述性文本、段落摘要。\n\
                2. 需要语义理解：用户的表述可能与类别名称不完全一致，要根据语义匹配到正确的类别。\n\
                   例如：\"沟通内容\"→\"对话内容\"，\"明日工作计划\"→\"后续计划\"，文末补充说明→\"备注\"。\n\
                3. 每个必需类别至少提取一个标签，否则校验会失败。\n\
                4. 标签的 leibie 必须使用标准类别名称（{}），不要使用用户原文中的表述。\n\
                5. 注意人际关系和角色区分：根据上下文语境准确判断人物的身份和角色，将其归入正确的类别。\
                例如，文中提到的人物可能是作者本人、内部协作者、外部客户等不同角色，需根据语境判断，不要混淆。",
                leibie_miaoshu, leibie_miaoshu
            ),
            parameters: json!({
                "type": "object",
                "properties": {
                    "biaoqianlie": {
                        "type": "array",
                        "description": format!("提取出的标签列表。必须覆盖所有必需类别：{}", leibie_miaoshu),
                        "items": {
                            "type": "object",
                            "properties": {
                                "mingcheng": {
                                    "type": "string",
                                    "description": "标签内容，从文本中提取的关键信息（可以是名词、短语或描述性文本）"
                                },
                                "leibie": {
                                    "type": "string",
                                    "description": format!("标签类别，必须是以下标准名称之一：{}", leibie_miaoshu)
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

/// 从文本中提取标签（异步接口）
/// 
/// # 参数
/// - `peizhi`: OpenAI配置
/// - `wenben`: 待提取的文本内容
/// 
/// # 返回
/// - `Some(Tiqujieguo)`: 提取成功或验证失败
/// - `None`: 配置读取失败或AI调用失败
pub async fn tiqu(peizhi: &Aipeizhi, wenben: &str) -> Option<Tiqujieguo> {
    let aipeizhi = duqu_aipeizhi()?;
    let xitongtishici = goujian_xitongtishici(&aipeizhi);
    
    let mut guanli = Xiaoxiguanli::xingjian()
        .shezhi_xitongtishici(&xitongtishici)
        .tianjia_gongju(goujian_gongju());
    guanli.zhuijia_yonghuxiaoxi(wenben);
    
    let jieguo = openaizhuti::gongjuqingqiu(peizhi, &mut guanli).await?;
    match jieguo {
        openaizhuti::Aijieguo::Gongjudiaoyong(diaoyonglie) => {
            let biaoqianlie = jiexi_gongjujieguo(&diaoyonglie)?;
            yanzheng_biaoqian(&biaoqianlie, &aipeizhi)
        }
        openaizhuti::Aijieguo::Wenben(_) => None,
    }
}

/// 执行标签提取工具（同步接口，用于ReAct循环）
/// 
/// # 参数
/// - `canshu_json`: 工具调用参数的JSON字符串
/// 
/// # 返回
/// - 提取结果的JSON字符串
pub fn zhixing(canshu_json: &str) -> String {
    let aipeizhi = match duqu_aipeizhi() {
        Some(p) => p,
        None => return xilie_cuowujieguo("无法读取AI配置"),
    };
    
    let biaoqianlie = match jiexi_canshu(canshu_json) {
        Some(lie) => lie,
        None => return xilie_cuowujieguo("参数解析失败"),
    };
    
    let jieguo = yanzheng_biaoqian(&biaoqianlie, &aipeizhi)
        .unwrap_or_else(|| Tiqujieguo {
            chenggong: false,
            xiaoxi: "标签验证失败".to_string(),
            biaoqianlie: vec![],
        });
    
    serde_json::to_string(&jieguo).unwrap_or_default()
}

/// 获取工具名称
pub fn mingcheng() -> &'static str {
    gongju_mingcheng
}

// ==================== 私有辅助函数 ====================

/// 读取AI配置文件
fn duqu_aipeizhi() -> Option<Aipeizhiwenjian> {
    peizhixitongzhuti::duqupeizhi::<Aipeizhiwenjian>(Aipeizhiwenjian::wenjianming())
}

/// 构建系统提示词
fn goujian_xitongtishici(peizhi: &Aipeizhiwenjian) -> String {
    let yunxu_leibie: Vec<&str> = peizhi.ribaoshengcheng.xinxi_yingshe.keys().map(|s| s.as_str()).collect();
    let yunxu_leibie_str = yunxu_leibie.join("、");
    
    format!(
        "你是一个信息提取助手，从文本中提取与指定类别相关的关键信息。\n\
        \n\
        提取规则：\n\
        1. 允许的类别：{}。严禁提取其他类别的标签。\n\
        2. 必需类别：{}。每个必需类别至少提取一个标签。\n\
        3. 提取内容不限于专有名词，也包括描述性文本。例如：\n\
           - \"时间\"类别：提取日期、时间段等（如\"2026年2月14日\"）\n\
           - \"工作内容\"类别：提取具体的工作事项描述\n\
           - \"对话内容\"类别：提取沟通、交流、讨论等相关内容（用户可能写作\"沟通内容\"、\"交流记录\"等）\n\
           - \"后续计划\"类别：提取计划、安排等内容（用户可能写作\"明日计划\"、\"下一步\"等）\n\
           - \"备注\"类别：提取备注、附注、补充说明等内容\n\
        4. 需要语义理解：用户的表述可能与类别名称不完全一致，要根据语义匹配到正确的类别。\n\
        5. 标签的 leibie 字段必须使用标准类别名称（即上述允许的类别名），不要使用用户原文中的表述。\n\
        6. 注意人际关系和角色区分：根据上下文语境准确判断人物的身份和角色，将其归入正确的类别。\
        例如，文中提到的人物可能是作者本人、内部协作者、外部客户等不同角色，需根据语境判断，不要混淆。\n\
        \n\
        标签合并规则：\n\
        1. 同一类别的多个标签，如果语义上属于同一实体的不同部分，应合并为一个完整标签。\n\
        2. 如果语义上是独立的实体，则保持独立。\n\
        3. 合并时确保语义完整性和准确性。\n\
        \n\
        严禁伪造数据：不得编造、捏造或使用占位符（如无、暂无、N/A等）来满足验证要求。如果文本中确实没有某类别的信息，不要伪造。",
        yunxu_leibie_str,
        yunxu_leibie_str
    )
}

/// 解析工具调用结果
fn jiexi_gongjujieguo(diaoyonglie: &[llm::ToolCall]) -> Option<Vec<Biaoqian>> {
    let diaoyong = diaoyonglie.iter()
        .find(|d| d.function.name == gongju_mingcheng)?;
    let canshu: serde_json::Value = serde_json::from_str(&diaoyong.function.arguments).ok()?;
    jiexi_biaoqianlie_cong_json(&canshu)
}

/// 解析工具参数JSON字符串
fn jiexi_canshu(canshu_json: &str) -> Option<Vec<Biaoqian>> {
    let canshu: serde_json::Value = serde_json::from_str(canshu_json).ok()?;
    jiexi_biaoqianlie_cong_json(&canshu)
}

/// 从JSON对象中提取标签列表
fn jiexi_biaoqianlie_cong_json(json: &serde_json::Value) -> Option<Vec<Biaoqian>> {
    let biaoqianlie = json.get("biaoqianlie")?.as_array()?;
    let jieguo: Vec<Biaoqian> = biaoqianlie.iter()
        .filter_map(|v| serde_json::from_value(v.clone()).ok())
        .collect();
    (!jieguo.is_empty()).then_some(jieguo)
}

/// 序列化错误结果为JSON字符串
fn xilie_cuowujieguo(xiaoxi: &str) -> String {
    let jieguo = Tiqujieguo {
        chenggong: false,
        xiaoxi: xiaoxi.to_string(),
        biaoqianlie: vec![],
    };
    serde_json::to_string(&jieguo).unwrap_or_default()
}

/// 验证标签列表
fn yanzheng_biaoqian(biaoqianlie: &[Biaoqian], peizhi: &Aipeizhiwenjian) -> Option<Tiqujieguo> {
    // 检查占位符
    if let Some(zhanweifu) = jiancha_zhanweifu(biaoqianlie) {
        return Some(Tiqujieguo {
            chenggong: false,
            xiaoxi: format!("检测到占位符标签：{}，请勿伪造数据", zhanweifu),
            biaoqianlie: vec![],
        });
    }
    
    // 检查必需类别
    let bixu_leibie: Vec<String> = peizhi.ribaoshengcheng.xinxi_yingshe.keys().cloned().collect();
    if let Some(queshi) = jiancha_bixu_leibie(biaoqianlie, &bixu_leibie) {
        return Some(Tiqujieguo {
            chenggong: false,
            xiaoxi: format!("缺少必需的标签类别：{}", queshi),
            biaoqianlie: vec![],
        });
    }
    
    Some(Tiqujieguo {
        chenggong: true,
        xiaoxi: "标签提取成功".to_string(),
        biaoqianlie: biaoqianlie.to_vec(),
    })
}

/// 检查是否包含占位符标签
fn jiancha_zhanweifu(biaoqianlie: &[Biaoqian]) -> Option<String> {
    #[allow(non_upper_case_globals)]
    const zhanweifu: &[&str] = &[
        "无", "暂无", "无人", "无人员", 
        "N/A", "n/a", "NA", "na",
        "null", "NULL", "None", "none",
        "未知", "不详", "空"
    ];
    
    biaoqianlie.iter()
        .find(|b| zhanweifu.contains(&b.mingcheng.as_str()))
        .map(|b| b.mingcheng.clone())
}

/// 检查是否缺少必需类别
fn jiancha_bixu_leibie(biaoqianlie: &[Biaoqian], bixuyou: &[String]) -> Option<String> {
    let queshi: Vec<String> = bixuyou.iter()
        .filter(|leibie| !biaoqianlie.iter().any(|b| &b.leibie == *leibie))
        .cloned()
        .collect();
    
    (!queshi.is_empty()).then(|| queshi.join("、"))
}
