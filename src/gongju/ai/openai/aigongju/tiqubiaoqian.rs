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
    Tool {
        tool_type: "function".to_string(),
        function: FunctionTool {
            name: gongju_mingcheng.to_string(),
            description: "从文本中提取关键信息标签。严格按照系统提示词中的全局引导执行，只提取指定类别的实体。".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "biaoqianlie": {
                        "type": "array",
                        "description": "提取出的标签列表，严格遵循系统提示词中的全局引导，只提取允许的类别",
                        "items": {
                            "type": "object",
                            "properties": {
                                "mingcheng": {
                                    "type": "string",
                                    "description": "标签名称，必须是文本中真实存在的专有名词"
                                },
                                "leibie": {
                                    "type": "string",
                                    "description": "标签类别，必须符合系统提示词中允许的类别"
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
    let yunxu_leibie = peizhi.biaoqiantiqu.bixuyou.join("、");
    
    format!(
        "提取规则：只提取文本中真实存在的专有名词实体。\n\
        允许的类别：{}。严禁提取其他类别的标签。\n\
        必需类别：{}。如果文本中确实没有这些类别的实体，不要伪造、编造或使用占位符（如无、暂无、N/A等），应直接返回空列表。\n\
        标签合并规则：\n\
        1. 对于同一类别的多个标签，先进行语义检查，判断它们是否可以合并。\n\
        2. 如果多个标签在语义上属于同一实体的不同部分（如\"2026年2月14日\"和\"下午2点\"都是描述同一时间），应合并为一个完整标签（如\"2026年2月14日下午2点\"）。\n\
        3. 如果多个标签在语义上是独立的实体（如\"张三\"和\"李四\"是不同的人），则不应合并，保持独立。\n\
        4. 合并时要确保语义完整性和准确性，不要强行拼接无关内容。\n\
        严禁伪造数据：不得编造、捏造或使用占位符来满足验证要求。",
        yunxu_leibie,
        yunxu_leibie
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
    if let Some(queshi) = jiancha_bixu_leibie(biaoqianlie, &peizhi.biaoqiantiqu.bixuyou) {
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
