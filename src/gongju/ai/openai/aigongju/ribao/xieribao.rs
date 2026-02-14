use llm::chat::{FunctionTool, Tool};
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::peizhixt::peizhixitongzhuti;
use crate::peizhixt::peizhi_nr::peizhi_ai::Aipeizhi as Aipeizhiwenjian;

// ==================== 常量定义 ====================

/// 工具名称
#[allow(non_upper_case_globals)]
const gongju_mingcheng: &str = "xieribao";

// ==================== 数据结构 ====================

/// 日报信息条目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ribaoxinxi {
    /// 信息类别（如：人名、地名、时间、对话内容）
    pub leibie: String,
    /// 具体内容
    pub neirong: String,
}

/// 日报生成结果
#[derive(Debug, Serialize, Deserialize)]
pub struct Ribaojieguo {
    pub chenggong: bool,
    pub xiaoxi: String,
    /// 生成的日报文本（仅成功时有值）
    pub ribao: Option<String>,
    /// 缺少的必需信息类别（仅校验失败时有值）
    pub queshi_xinxi: Option<Vec<String>>,
    /// 审查结果详情（仅审查模式有值）
    pub shenchajieguo: Option<Shenchajieguo>,
}

/// 日报审查结果
#[derive(Debug, Serialize, Deserialize)]
pub struct Shenchajieguo {
    pub tongguo: bool,
    pub wenti: Vec<String>,
}

// ==================== 公共接口 ====================

/// 构建日报生成工具定义
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
                "帮助用户撰写工作日报并审查日报内容。支持两种模式：\n\
                1. 撰写模式（moshi=xiezuo）：收集完整的必需信息（{}）后生成日报，信息不完整会返回缺少的类别。\n\
                2. 审查模式（moshi=shencha）：审查已有日报内容的基本格式（如内容长度等）。信息类别的完整性由后续标签提取阶段严格校验。",
                leibie_miaoshu
            ),
            parameters: json!({
                "type": "object",
                "properties": {
                    "moshi": {
                        "type": "string",
                        "description": "操作模式：xiezuo（撰写日报）或 shencha（审查日报）",
                        "enum": ["xiezuo", "shencha"]
                    },
                    "xinxilie": {
                        "type": "array",
                        "description": format!("撰写模式下使用：用户提供的日报信息列表。支持的信息类别：{}", leibie_miaoshu),
                        "items": {
                            "type": "object",
                            "properties": {
                                "leibie": {
                                    "type": "string",
                                    "description": format!("信息类别，可选值：{}", leibie_miaoshu)
                                },
                                "neirong": {
                                    "type": "string",
                                    "description": "该类别对应的具体内容"
                                }
                            },
                            "required": ["leibie", "neirong"]
                        }
                    },
                    "ribaoneirong": {
                        "type": "string",
                        "description": "审查模式下使用：待审查的日报文本内容"
                    }
                },
                "required": ["moshi"]
            }),
        },
    }
}

/// 执行日报工具（同步接口，用于ReAct循环）
pub fn zhixing(canshu_json: &str) -> String {
    let peizhi = match duqu_aipeizhi() {
        Some(p) => p,
        None => return xilie_cuowujieguo("无法读取AI配置"),
    };

    let canshu: serde_json::Value = match serde_json::from_str(canshu_json) {
        Ok(v) => v,
        Err(_) => return xilie_cuowujieguo("参数解析失败"),
    };

    let moshi = canshu.get("moshi").and_then(|v| v.as_str()).unwrap_or("xiezuo");
    match moshi {
        "shencha" => zhixing_shencha(&canshu, &peizhi),
        _ => zhixing_xiezuo(&canshu, &peizhi),
    }
}

/// 获取工具名称
pub fn mingcheng() -> &'static str {
    gongju_mingcheng
}

// ==================== 私有辅助函数 ====================

/// 执行撰写模式
fn zhixing_xiezuo(canshu: &serde_json::Value, peizhi: &Aipeizhiwenjian) -> String {
    let xinxilie = match jiexi_xinxilie(canshu) {
        Some(lie) => lie,
        None => return xilie_cuowujieguo("撰写模式需要提供 xinxilie 参数"),
    };

    let bixu_leibie: Vec<String> = peizhi.ribaoshengcheng.xinxi_yingshe.keys().cloned().collect();
    if let Some(queshi) = jiancha_bixu_xinxi(&xinxilie, &bixu_leibie) {
        return xilie_jieguo(false, &format!("日报信息不完整，请补充以下内容：{}", queshi.join("、")), None, Some(queshi), None);
    }

    if let Some(kongbei) = jiancha_kongneirong(&xinxilie) {
        return xilie_jieguo(false, &format!("以下信息内容为空，请补充：{}", kongbei.join("、")), None, Some(kongbei), None);
    }

    let ribao = shengcheng_ribao(&xinxilie, &peizhi.ribaoshengcheng.shuchu_moban, &peizhi.ribaoshengcheng.xinxi_yingshe);
    xilie_jieguo(true, "日报生成成功", Some(ribao), None, None)
}

/// 执行审查模式
/// 
/// 只做基本格式检查（如内容长度）。
/// 信息类别的完整性由后续 tiqubiaoqian 阶段通过语义理解严格校验。
fn zhixing_shencha(canshu: &serde_json::Value, peizhi: &Aipeizhiwenjian) -> String {
    let neirong = match canshu.get("ribaoneirong").and_then(|v| v.as_str()) {
        Some(n) if !n.trim().is_empty() => n,
        _ => return xilie_cuowujieguo("审查模式需要提供 ribaoneirong 参数"),
    };

    let mut wenti: Vec<String> = Vec::new();

    if neirong.len() < 50 {
        wenti.push("日报内容过短，建议补充更多细节".to_string());
    }

    let bixu_leibie: Vec<&str> = peizhi.ribaoshengcheng.xinxi_yingshe.keys().map(|s| s.as_str()).collect();
    let tongguo = wenti.is_empty();
    let xiaoxi = if tongguo {
        format!("日报格式审查通过。后续标签提取阶段将严格校验以下必需信息类别是否涵盖：{}", bixu_leibie.join("、"))
    } else {
        format!("日报审查未通过，发现 {} 个问题", wenti.len())
    };
    let shencha = Shenchajieguo { tongguo, wenti };
    xilie_jieguo(tongguo, &xiaoxi, Some(neirong.to_string()), None, Some(shencha))
}

/// 读取AI配置文件
fn duqu_aipeizhi() -> Option<Aipeizhiwenjian> {
    peizhixitongzhuti::duqupeizhi::<Aipeizhiwenjian>(Aipeizhiwenjian::wenjianming())
}

/// 从参数中解析信息列表
fn jiexi_xinxilie(canshu: &serde_json::Value) -> Option<Vec<Ribaoxinxi>> {
    let xinxilie = canshu.get("xinxilie")?.as_array()?;
    let jieguo: Vec<Ribaoxinxi> = xinxilie.iter()
        .filter_map(|v| serde_json::from_value(v.clone()).ok())
        .collect();
    (!jieguo.is_empty()).then_some(jieguo)
}

/// 检查是否缺少必需信息类别
fn jiancha_bixu_xinxi(xinxilie: &[Ribaoxinxi], bixu: &[String]) -> Option<Vec<String>> {
    let queshi: Vec<String> = bixu.iter()
        .filter(|leibie| !xinxilie.iter().any(|x| &x.leibie == *leibie))
        .cloned()
        .collect();
    (!queshi.is_empty()).then_some(queshi)
}

/// 检查是否存在空内容的条目
fn jiancha_kongneirong(xinxilie: &[Ribaoxinxi]) -> Option<Vec<String>> {
    let kongbei: Vec<String> = xinxilie.iter()
        .filter(|x| x.neirong.trim().is_empty())
        .map(|x| x.leibie.clone())
        .collect();
    (!kongbei.is_empty()).then_some(kongbei)
}

/// 根据模板和配置映射生成日报文本
fn shengcheng_ribao(xinxilie: &[Ribaoxinxi], moban: &str, yingshe: &std::collections::HashMap<String, String>) -> String {
    let zhaoxinxi = |leibie: &str| -> String {
        xinxilie.iter()
            .filter(|x| x.leibie == leibie)
            .map(|x| x.neirong.as_str())
            .collect::<Vec<_>>()
            .join("、")
    };

    let mut jieguo = moban.to_string();
    for (leibie, zhankuofu) in yingshe {
        let zhanwei = format!("{{{}}}", zhankuofu);
        jieguo = jieguo.replace(&zhanwei, &zhaoxinxi(leibie));
    }
    jieguo
}

/// 序列化结果为JSON字符串
fn xilie_jieguo(chenggong: bool, xiaoxi: &str, ribao: Option<String>, queshi: Option<Vec<String>>, shencha: Option<Shenchajieguo>) -> String {
    let jieguo = Ribaojieguo {
        chenggong,
        xiaoxi: xiaoxi.to_string(),
        ribao,
        queshi_xinxi: queshi,
        shenchajieguo: shencha,
    };
    serde_json::to_string(&jieguo).unwrap_or_default()
}

/// 序列化错误结果为JSON字符串
fn xilie_cuowujieguo(xiaoxi: &str) -> String {
    xilie_jieguo(false, xiaoxi, None, None, None)
}
