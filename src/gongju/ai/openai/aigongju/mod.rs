pub mod ribao;
pub mod yasuoxiaoxi;
pub mod gongjuzhuce;
pub mod gongjusousuo;

use gongjuzhuce::Gongjuyuanshuju;

/// 初始化工具注册表（应用启动时调用一次）
/// 
/// 所有工具在此注册，声明元数据、分组、关键词
pub fn chushihua() {
    gongjuzhuce::chushihua(vec![
        // ===== 核心工具（始终加载，不需要发现） =====
        Gongjuyuanshuju {
            mingcheng: yasuoxiaoxi::mingcheng(),
            miaoshu: "当对话历史过长超过token限制时，总结并压缩历史消息",
            fenzu: "xitong",
            guanjianci: &["压缩", "总结", "历史", "token", "消息"],
            changjingci: &["对话太长", "token超限", "需要压缩"],
            goujianqi: yasuoxiaoxi::goujian_gongju,
            zhixingqi: yasuoxiaoxi::zhixing,
            hexingongju: true,
        },
        // ===== 可发现工具（通过搜索发现后动态加载） =====
        Gongjuyuanshuju {
            mingcheng: ribao::tiqubiaoqian::mingcheng(),
            miaoshu: "从文本中提取关键信息标签，如人名、地名、时间等实体",
            fenzu: "wenben",
            guanjianci: &["标签", "提取", "实体", "人名", "地名", "时间", "NER", "信息提取"],
            changjingci: &["日报", "周报", "工作汇报", "会议纪要", "文本分析", "提取信息", "提取标签", "关键信息", "实体识别"],
            goujianqi: ribao::tiqubiaoqian::goujian_gongju,
            zhixingqi: ribao::tiqubiaoqian::zhixing,
            hexingongju: false,
        },
        Gongjuyuanshuju {
            mingcheng: ribao::xieribao::mingcheng(),
            miaoshu: "帮助用户撰写工作日报并审查日报内容，收集人名、地名、时间、对话内容等必需信息后生成日报，同时审查日报是否符合要求",
            fenzu: "wenben",
            guanjianci: &["日报", "写日报", "工作日报", "工作汇报", "每日总结", "今日工作", "审查日报", "日报审核"],
            changjingci: &["写日报", "帮我写日报", "生成日报", "今天的工作日报", "工作总结", "每日汇报", "日报撰写", "审查日报", "检查日报"],
            goujianqi: ribao::xieribao::goujian_gongju,
            zhixingqi: ribao::xieribao::zhixing,
            hexingongju: false,
        },
        Gongjuyuanshuju {
            mingcheng: ribao::ribaozhiyin::mingcheng(),
            miaoshu: "日报处理流程指南，当用户发来日报相关内容时必须首先调用此工具，按步骤指导完成审查、标签提取等完整流程",
            fenzu: "wenben",
            guanjianci: &["日报", "日报流程", "日报指南", "日报处理", "工作日报", "工作汇报"],
            changjingci: &["写日报", "帮我写日报", "日报", "工作日报", "今天的日报", "日报相关", "每日汇报", "工作总结", "日报审查"],
            goujianqi: ribao::ribaozhiyin::goujian_gongju,
            zhixingqi: ribao::ribaozhiyin::zhixing,
            hexingongju: false,
        },
    ]);
}

/// 生成可发现工具的目录描述（注入系统提示词，让AI知道有哪些工具可用）
pub fn shengcheng_gongjumulu() -> String {
    let faxian = gongjuzhuce::huoqu_faxiangongju();
    if faxian.is_empty() {
        return String::new();
    }
    let mut mulu = String::from("\n\n可用工具目录（这些工具已根据你的需求自动加载，可直接调用）：\n");
    for g in &faxian {
        mulu.push_str(&format!("- {}：{}\n", g.mingcheng, g.miaoshu));
    }
    mulu
}

/// 获取核心工具的 JSON 列表（始终注入给AI）
pub fn huoqu_hexingongju_json() -> Vec<serde_json::Value> {
    gongjuzhuce::huoqu_hexingongju()
        .into_iter()
        .filter_map(|g| serde_json::to_value((g.goujianqi)()).ok())
        .collect()
}

/// 获取核心工具的 Tool 列表
pub fn huoqu_hexingongju_lie() -> Vec<llm::chat::Tool> {
    gongjuzhuce::huoqu_hexingongju()
        .into_iter()
        .map(|g| (g.goujianqi)())
        .collect()
}

/// 按名称列表获取工具的 JSON 列表（用于动态加载发现的工具）
pub fn huoqu_gongju_json_anming(mingchenglie: &[&str]) -> Vec<serde_json::Value> {
    gongjuzhuce::piliang_chazhao(mingchenglie)
        .into_iter()
        .filter_map(|g| serde_json::to_value((g.goujianqi)()).ok())
        .collect()
}

/// 获取所有工具的 JSON 列表（向后兼容）
pub fn huoqu_suoyougongju() -> Vec<serde_json::Value> {
    gongjuzhuce::huoqu_quanbu()
        .iter()
        .filter_map(|g| serde_json::to_value((g.goujianqi)()).ok())
        .collect()
}

/// 获取所有工具的 Tool 列表（向后兼容）
pub fn huoqu_gongjulie() -> Vec<llm::chat::Tool> {
    gongjuzhuce::huoqu_quanbu()
        .iter()
        .map(|g| (g.goujianqi)())
        .collect()
}

/// 执行工具调用，通过注册表分发
pub fn zhixing_gongju(diaoyong: &llm::ToolCall) -> llm::ToolCall {
    let jieguo = match gongjuzhuce::anming_chazhao(&diaoyong.function.name) {
        Some(gongju) => (gongju.zhixingqi)(&diaoyong.function.arguments),
        None => format!("{{\"cuowu\": \"未知工具: {}\"}}", diaoyong.function.name),
    };
    llm::ToolCall {
        id: diaoyong.id.clone(),
        call_type: "function".to_string(),
        function: llm::FunctionCall {
            name: diaoyong.function.name.clone(),
            arguments: jieguo,
        },
    }
}

/// 批量执行工具调用
pub async fn pizhixing(diaoyonglie: Vec<llm::ToolCall>) -> Vec<llm::ToolCall> {
    diaoyonglie.iter().map(zhixing_gongju).collect()
}

/// 搜索工具：根据意图描述查找相关工具
pub fn sousuo_gongju(yitu: &str, zuida: usize) -> Vec<gongjusousuo::Sousuojieguo> {
    gongjusousuo::sousuo(yitu, zuida)
}
