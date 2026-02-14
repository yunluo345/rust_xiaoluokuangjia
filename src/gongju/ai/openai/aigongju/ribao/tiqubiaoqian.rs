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
                "从日报原文中提取关键信息标签。调用时必须通过 yuanwen 参数传入日报原文，并通过 biaoqianlie 参数传入提取的标签。\n\
                必需的标签类别：{}。\n\
                提取规则：\n\
                1. 优先从 yuanwen 中提取标签。如果某些必需类别在原文中缺失，但用户在后续对话中补充了相关信息，可通过 buchongxinxi 参数传入用户补充的原始内容。\n\
                2. 提取内容不限于专有名词，也包括描述性文本、段落摘要。\n\
                3. 需要语义理解：用户的表述可能与类别名称不完全一致，要根据语义匹配到正确的类别。\n\
                   例如：\"沟通内容\"→\"对话内容\"，\"明日工作计划\"→\"后续计划\"，文末补充说明→\"备注\"。\n\
                4. 每个必需类别至少提取一个标签，否则校验会失败。\n\
                5. 标签的 leibie 必须使用标准类别名称（{}），不要使用用户原文中的表述。\n\
                6. 注意人际关系和角色区分：根据 yuanwen 语境准确判断人物的身份和角色，将其归入正确的类别。\n\
                7. buchongxinxi 仅用于传入用户明确补充的信息，严禁自行编造补充内容。",
                leibie_miaoshu, leibie_miaoshu
            ),
            parameters: json!({
                "type": "object",
                "properties": {
                    "yuanwen": {
                        "type": "string",
                        "description": "日报原文内容。必须传入完整的日报文本，工具仅基于此文本和 buchongxinxi 进行标签提取和验证"
                    },
                    "buchongxinxi": {
                        "type": "string",
                        "description": "用户在对话中补充的信息（可选）。当日报原文缺少某些必需类别的信息，且用户在后续对话中明确补充时，将用户的原始补充内容传入此参数。严禁自行编造"
                    },
                    "biaoqianlie": {
                        "type": "array",
                        "description": format!("从 yuanwen 和 buchongxinxi 中提取出的标签列表。必须覆盖所有必需类别：{}", leibie_miaoshu),
                        "items": {
                            "type": "object",
                            "properties": {
                                "mingcheng": {
                                    "type": "string",
                                    "description": "标签内容，从 yuanwen 或 buchongxinxi 中提取的关键信息（可以是名词、短语或描述性文本）"
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
                "required": ["yuanwen", "biaoqianlie"]
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
pub fn zhixing(canshu_json: &str) -> String {
    let aipeizhi = match duqu_aipeizhi() {
        Some(p) => p,
        None => return xilie_cuowujieguo("无法读取AI配置"),
    };

    let canshu: serde_json::Value = match serde_json::from_str(canshu_json) {
        Ok(v) => v,
        Err(_) => return xilie_cuowujieguo("参数解析失败"),
    };

    let yuanwen = match canshu.get("yuanwen").and_then(|v| v.as_str()) {
        Some(w) if !w.trim().is_empty() => w,
        _ => return xilie_cuowujieguo("缺少必需参数 yuanwen，请传入日报原文"),
    };

    let buchongxinxi = canshu.get("buchongxinxi")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    // 合并原文和补充信息作为验证来源
    let yanzheng_wenben = if buchongxinxi.trim().is_empty() {
        yuanwen.to_string()
    } else {
        format!("{}\n{}", yuanwen, buchongxinxi)
    };

    let biaoqianlie = match jiexi_biaoqianlie_cong_json(&canshu) {
        Some(lie) => lie,
        None => return xilie_cuowujieguo("缺少必需参数 biaoqianlie 或格式错误"),
    };

    if let Some(wupipei) = yanzheng_biaoqian_laiyuan(&biaoqianlie, &yanzheng_wenben) {
        return xilie_cuowujieguo(&format!("以下标签内容在原文和补充信息中均未找到对应信息：{}", wupipei));
    }

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
        "你是一个信息提取助手，从提供的原文中提取与指定类别相关的关键信息。\n\
        \n\
        提取规则：\n\
        1. 允许的类别：{}。严禁提取其他类别的标签。\n\
        2. 必需类别：{}。每个必需类别至少提取一个标签。\n\
        3. 只能从提供的原文中提取信息，严禁使用原文以外的任何信息。\n\
        4. 提取内容不限于专有名词，也包括描述性文本。例如：\n\
           - \"时间\"类别：提取日期、时间段等（如\"2026年2月14日\"）\n\
           - \"工作内容\"类别：提取具体的工作事项描述\n\
           - \"对话内容\"类别：提取沟通、交流、讨论等相关内容（用户可能写作\"沟通内容\"、\"交流记录\"等）\n\
           - \"后续计划\"类别：提取计划、安排等内容（用户可能写作\"明日计划\"、\"下一步\"等）\n\
           - \"备注\"类别：提取备注、附注、补充说明等内容\n\
        5. 需要语义理解：用户的表述可能与类别名称不完全一致，要根据语义匹配到正确的类别。\n\
        6. 标签的 leibie 字段必须使用标准类别名称（即上述允许的类别名），不要使用用户原文中的表述。\n\
        7. 注意人际关系和角色区分：根据原文语境准确判断人物的身份和角色，将其归入正确的类别。\n\
        \n\
        标签合并规则：\n\
        1. 同一类别的多个标签，如果语义上属于同一实体的不同部分，应合并为一个完整标签。\n\
        2. 如果语义上是独立的实体，则保持独立。\n\
        3. 合并时确保语义完整性和准确性。\n\
        \n\
        严禁伪造数据：不得编造、捏造或使用占位符（如无、暂无、N/A等）来满足验证要求。如果原文中确实没有某类别的信息，不要伪造。",
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

/// 验证标签内容是否来源于原文
/// 
/// 验证策略：将标签内容按常见分隔符拆分为片段，
/// 每个片段（去除常见连接词后）必须在原文中找到对应内容。
/// 这样既允许AI合理拼接多个信息，又能防止凭空捏造。
fn yanzheng_biaoqian_laiyuan(biaoqianlie: &[Biaoqian], yuanwen: &str) -> Option<String> {
    let yuanwen_xiaoxie = yuanwen.to_lowercase();
    let wupipei: Vec<String> = biaoqianlie.iter()
        .filter(|b| !pianduan_pipei(&b.mingcheng, &yuanwen_xiaoxie))
        .map(|b| format!("{}({})", b.mingcheng, b.leibie))
        .collect();
    (!wupipei.is_empty()).then(|| wupipei.join("、"))
}

/// 片段匹配：将标签内容拆分为片段，检查每个片段是否在原文中出现
fn pianduan_pipei(biaoqian: &str, yuanwen_xiaoxie: &str) -> bool {
    let biaoqian_xiaoxie = biaoqian.to_lowercase();
    // 先尝试整体匹配
    if yuanwen_xiaoxie.contains(&biaoqian_xiaoxie) {
        return true;
    }
    // 按常见分隔符拆分为片段
    let pianduanlie: Vec<&str> = biaoqian_xiaoxie
        .split(&['；', '、', '，', ';', ',', '\n'][..])
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();
    if pianduanlie.is_empty() {
        return false;
    }
    // 每个片段中至少有一个>=4字符的关键子串在原文中出现
    pianduanlie.iter().all(|pianduan| {
        // 去除常见前缀连接词
        let jinghua = pianduan
            .trim_start_matches("与")
            .trim_start_matches("和")
            .trim_start_matches("及")
            .trim_start_matches("向")
            .trim();
        if jinghua.is_empty() {
            return true; // 纯连接词，跳过
        }
        // 整个片段在原文中
        if yuanwen_xiaoxie.contains(jinghua) {
            return true;
        }
        // 提取片段中>=4字符的连续子串，检查是否在原文中
        let zifuji: Vec<char> = jinghua.chars().collect();
        if zifuji.len() < 4 {
            // 短片段直接检查
            return yuanwen_xiaoxie.contains(jinghua);
        }
        // 滑动窗口：检查是否有>=4字符的子串在原文中
        let chuangkou = 4.min(zifuji.len());
        (0..=zifuji.len() - chuangkou).any(|i| {
            let zichuan: String = zifuji[i..i + chuangkou].iter().collect();
            yuanwen_xiaoxie.contains(&zichuan)
        })
    })
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
