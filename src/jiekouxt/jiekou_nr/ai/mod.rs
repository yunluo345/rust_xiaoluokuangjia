pub mod jiekou_aiduihua;
pub mod jiekou_aiduihualiushi;
pub mod ceshi;

use serde::Deserialize;
use crate::gongju::ai::openai::{aipeizhi, aixiaoxiguanli, gongjuji, openaizhuti};
use crate::gongju::ai::openai::openaizhuti::ReactJieguo;
use crate::shujuku::psqlshujuku::shujubiao_nr::ai::shujucaozuo_aiqudao;
use crate::peizhixt::peizhixitongzhuti;
use crate::peizhixt::peizhi_nr::peizhi_ai::Ai;

#[allow(non_upper_case_globals)]
pub const xitongtishici: &str = "\
你是AI日报助手，基于xiaoluo-B3框架。\
你的职责是帮助员工处理工作日报相关事务，包括日报的撰写、整理、总结等。\
你只能处理与工作相关的问题，不允许回答与工作无关的内容。\
对于简单的问候（如你好、早上好等），你可以友好地回复。\
严禁随机生成或伪造虚假日报内容：不得为用户编造姓名、对接人、工作内容等信息。工作日报必须基于用户提供的真实信息。\
工具使用规则：\
- 获取工具执行结果后，必须直接使用结果回答用户问题，不要重复调用相同工具；\
- 避免无意义地重复调用相同工具。如果用户明确要求重新执行操作，可以再次调用。\
当用户反馈任务未处理、处理失败、未完成时，必须优先调用可用工具自动处理，不要要求用户提供任务ID列表。\
区分询问与执行：\
- 询问性问题（能不能、可以吗、是否可以、怎么做）：直接回答，不调用工具；\
- 执行命令（提交、检查、处理、完成、帮我做）：先调用工具，再基于工具结果回复。\
处理任务后必须基于工具返回结果回复：\
- 若返回zongshu为0，明确告知当前无可处理任务；\
- 若返回chenggong为true且zongshu大于0，明确告知已处理数量；\
- 若返回cuowu，直接说明失败原因并建议用户重试。\
不要输出“请提供任务ID列表”或“请指示我是否再次执行”这类反问。";

#[allow(non_upper_case_globals)]
pub const yitu_tishici: &str = "\
你是意图分析助手。根据用户消息判断意图类型，并提取多个语义关键词。\
只返回JSON，不要返回其他任何内容。\
格式：{\"leixing\":\"gongjudiaoyong\"或\"putongduihua\",\"guanjianci\":[\"关键词1\",\"关键词2\",\"关键词3\"]}\
意图判断规则：\
- 数据补充（仅提供姓名、日期、数字等简短信息，无完整句子）→ gongjudiaoyong\
- 询问性问题（能不能、可以吗、是否可以、怎么做、如何、为什么）→ putongduihua\
- 执行命令（帮我、请、立即、马上 + 动词）→ gongjudiaoyong\
- 查询实时信息（几点了、今天日期、当前时间）→ gongjudiaoyong\
- 普通问候、闲聊、知识问答 → putongduihua\
- 指代性表达（上面、刚才、那个）结合上下文判断 → 优先 gongjudiaoyong\
关键词提取规则：\
1. 结合上下文摘要和当前消息提取关键词\
2. 从用户消息中提取核心语义词，包括原词和同义词/近义词\
3. 例如'现在几点了'应提取['时间','几点','当前时间']，'帮我检查日报'应提取['日报','检查','验证']\
4. 每条消息提取2-5个关键词，覆盖用户意图的各个维度";

#[allow(non_upper_case_globals)]
pub const zhaiyao_tishici: &str = "\
你是上下文摘要助手。将对话历史压缩为简短摘要，保留关键信息。\
只返回摘要文本，不要返回其他内容。\
摘要规则：\
1. 提取关键实体：人名、日期、客户名、地点等\
2. 概括当前任务：用户正在做什么（提交日报、检查信息、补充数据等）\
3. 标注待补充信息：缺少哪些字段或数据\
4. 控制长度：50-150字\
示例输出：\
用户正在提交2026年2月14日的工作日报，汇报人林哲，服务客户阿里巴巴和联想。\
已提供完整日报内容，但缺少对方人员姓名。用户后续补充了张婷、黄伟两个姓名。";

#[allow(non_upper_case_globals)]
const chongfu_yuzhi: u32 = 1;

#[allow(non_upper_case_globals)]
const zhaiyao_xiaoxishuliang: usize = 5;

#[derive(Deserialize)]
pub struct Xiaoxi {
    pub juese: String,
    pub neirong: String,
}

#[derive(Deserialize)]
pub struct Qingqiuti {
    pub xiaoxilie: Vec<Xiaoxi>,
}

pub struct YituJieguo {
    pub leixing: String,
    pub guanjianci: Vec<String>,
    pub yuanwen: String,
}

/// 构建消息管理器：设置系统提示词、注册工具、填充历史消息
pub fn goujian_guanli(qingqiu: &Qingqiuti) -> aixiaoxiguanli::Xiaoxiguanli {
    let mut guanli = aixiaoxiguanli::Xiaoxiguanli::xingjian()
        .shezhi_xitongtishici(xitongtishici);
    for gongju in gongjuji::huoqu_suoyougongju() {
        guanli = guanli.tianjia_gongju(gongju);
    }
    for xiaoxi in &qingqiu.xiaoxilie {
        match xiaoxi.juese.as_str() {
            "user" => guanli.zhuijia_yonghuxiaoxi(&xiaoxi.neirong),
            "assistant" => {
                let neirong = xiaoxi.neirong.trim();
                if !neirong.starts_with('[') {
                    guanli.zhuijia_zhushouneirong(&xiaoxi.neirong);
                }
            }
            _ => {}
        }
    }
    guanli
}

/// 根据意图构建消息管理器：设置系统提示词、注册筛选后的工具、填充历史消息
pub fn goujian_guanli_anyitu(qingqiu: &Qingqiuti, gongjulie: Vec<llm::chat::Tool>) -> aixiaoxiguanli::Xiaoxiguanli {
    let mut guanli = aixiaoxiguanli::Xiaoxiguanli::xingjian()
        .shezhi_xitongtishici(xitongtishici);
    for gongju in gongjulie {
        guanli = guanli.tianjia_gongju(gongju);
    }
    for xiaoxi in &qingqiu.xiaoxilie {
        match xiaoxi.juese.as_str() {
            "user" => guanli.zhuijia_yonghuxiaoxi(&xiaoxi.neirong),
            "assistant" => {
                let neirong = xiaoxi.neirong.trim();
                if !neirong.starts_with('[') {
                    guanli.zhuijia_zhushouneirong(&xiaoxi.neirong);
                }
            }
            _ => {}
        }
    }
    guanli
}

/// 获取渠道并解析配置
pub async fn huoqu_peizhi() -> Option<aipeizhi::Aipeizhi> {
    let qudao = shujucaozuo_aiqudao::suiji_huoqu_qudao("openapi").await?;
    println!("获取到的渠道数据: {}", qudao);
    aipeizhi::Aipeizhi::cong_qudaoshuju(&qudao)
}

/// 生成上下文摘要：将历史对话压缩为简短摘要
async fn shengcheng_zhaiyao(peizhi: &aipeizhi::Aipeizhi, xiaoxilie: &[Xiaoxi]) -> Option<String> {
    let changdu = xiaoxilie.len();
    if changdu <= 2 {
        return None;
    }
    let kaishi = changdu.saturating_sub(zhaiyao_xiaoxishuliang);
    let lishi_xiaoxi = &xiaoxilie[kaishi..changdu.saturating_sub(1)];
    let mut guanli = aixiaoxiguanli::Xiaoxiguanli::xingjian()
        .shezhi_xitongtishici(zhaiyao_tishici);
    for xiaoxi in lishi_xiaoxi {
        match xiaoxi.juese.as_str() {
            "user" => guanli.zhuijia_yonghuxiaoxi(&xiaoxi.neirong),
            "assistant" => guanli.zhuijia_zhushouneirong(&xiaoxi.neirong),
            _ => {}
        }
    }
    println!("[上下文摘要] 开始生成摘要，历史消息数: {}", lishi_xiaoxi.len());
    let zhaiyao = openaizhuti::putongqingqiu(peizhi, &guanli).await?;
    println!("[上下文摘要] 摘要: {}", zhaiyao);
    Some(zhaiyao)
}

/// 意图分析：用AI判断用户本次消息的意图
async fn fenxi_yitu(peizhi: &aipeizhi::Aipeizhi, xiaoxilie: &[Xiaoxi]) -> Option<YituJieguo> {
    let benci_neirong = xiaoxilie.last().map(|x| x.neirong.as_str()).unwrap_or("");
    let zhaiyao = shengcheng_zhaiyao(peizhi, xiaoxilie).await;
    let tishici = match &zhaiyao {
        Some(z) => format!("{}\n\n上下文摘要：{}", yitu_tishici, z),
        None => yitu_tishici.to_string()
    };
    let mut guanli = aixiaoxiguanli::Xiaoxiguanli::xingjian()
        .shezhi_xitongtishici(&tishici);
    if !benci_neirong.is_empty() {
        guanli.zhuijia_yonghuxiaoxi(benci_neirong);
    }
    println!("[意图分析] 开始分析: {}", benci_neirong);
    let huifu = openaizhuti::putongqingqiu(peizhi, &guanli).await?;
    println!("[意图分析] AI返回: {}", huifu);
    let jinghua = huifu.trim()
        .trim_start_matches("```json")
        .trim_start_matches("```")
        .trim_end_matches("```")
        .trim();
    if let Ok(json) = serde_json::from_str::<serde_json::Value>(jinghua) {
        let leixing = json["leixing"].as_str().unwrap_or("putongduihua").to_string();
        // 支持数组和字符串两种格式
        let guanjianci = if let Some(shuzu) = json["guanjianci"].as_array() {
            shuzu.iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect()
        } else if let Some(danzi) = json["guanjianci"].as_str() {
            if danzi.is_empty() {
                vec![]
            } else {
                vec![danzi.to_string()]
            }
        } else {
            vec![]
        };
        println!("[意图分析] 类型: {} 关键词: {:?}", leixing, guanjianci);
        Some(YituJieguo { leixing, guanjianci, yuanwen: huifu })
    } else {
        println!("[意图分析] JSON解析失败");
        None
    }
}

/// 多关键词匹配工具：逐个关键词匹配，合并去重，匹配不到再用原文兜底
fn zhineng_tiqu_gongju_youxian(guanjianci_lie: &[String], yuanwen: &str) -> Vec<llm::chat::Tool> {
    // 1. 用每个关键词分别匹配，收集工具名和命中次数
    let mut gongjuming_defen: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    for ci in guanjianci_lie {
        let pipei = gongjuji::zhineng_tiqu_gongjuming(ci);
        for (ming, defen) in pipei {
            *gongjuming_defen.entry(ming).or_insert(0) += defen;
        }
    }
    // 也把所有关键词拼接起来整体匹配一次
    if !guanjianci_lie.is_empty() {
        let pingjie = guanjianci_lie.join(" ");
        let pipei = gongjuji::zhineng_tiqu_gongjuming(&pingjie);
        for (ming, defen) in pipei {
            *gongjuming_defen.entry(ming).or_insert(0) += defen;
        }
    }
    if !gongjuming_defen.is_empty() {
        // 按得分降序排列，取所有匹配到的工具
        let mut paixu: Vec<(String, usize)> = gongjuming_defen.into_iter().collect();
        paixu.sort_by(|a, b| b.1.cmp(&a.1));
        println!("[意图] 多关键词匹配结果: {:?}", paixu);
        let gongjuming_lie: Vec<String> = paixu.into_iter().map(|(ming, _)| ming).collect();
        let gongju = gongjuji::huoqu_suoyougongju().into_iter()
            .filter(|g| gongjuming_lie.contains(&g.function.name))
            .collect::<Vec<_>>();
        if !gongju.is_empty() {
            return gongju;
        }
    }
    // 2. 关键词都没匹配到，用原文兜底
    let gongju = gongjuji::zhineng_tiqu_gongju(yuanwen);
    if !gongju.is_empty() {
        return gongju;
    }
    gongjuji::huoqu_suoyougongju()
}

/// 意图分析 + 工具筛选：先AI分析，失败则降级关键词匹配，再失败则无工具
pub async fn huoqu_yitu_gongju(peizhi: &aipeizhi::Aipeizhi, xiaoxilie: &[Xiaoxi]) -> (Vec<llm::chat::Tool>, String) {
    let benci_neirong = xiaoxilie.last().map(|x| x.neirong.as_str()).unwrap_or("");
    // 1. 尝试AI意图分析
    if let Some(yitu) = fenxi_yitu(peizhi, xiaoxilie).await {
        if yitu.leixing == "gongjudiaoyong" {
            let guanjianci_miaoshu = yitu.guanjianci.join(", ");
            let gongju = zhineng_tiqu_gongju_youxian(&yitu.guanjianci, benci_neirong);
            println!("[意图] 工具调用，关键词: [{}] 匹配到 {} 个工具", guanjianci_miaoshu, gongju.len());
            return (gongju, format!("工具调用: [{}]", guanjianci_miaoshu));
        }
        // AI判断为普通对话，但用关键词做兜底校验，防止AI误判
        let doudigongju = gongjuji::zhineng_tiqu_gongju(benci_neirong);
        if !doudigongju.is_empty() {
            println!("[意图] AI判断普通对话，但关键词兜底匹配到 {} 个工具，覆盖为工具调用", doudigongju.len());
            return (doudigongju, "工具调用(关键词兜底)".to_string());
        }
        println!("[意图] 普通对话");
        return (vec![], "普通对话".to_string());
    }
    // 2. AI分析失败，降级为直接关键词匹配
    println!("[意图] AI分析失败，降级关键词匹配");
    let gongju = gongjuji::zhineng_tiqu_gongju(benci_neirong);
    if !gongju.is_empty() {
        println!("[意图] 降级匹配到 {} 个工具", gongju.len());
        return (gongju, "工具调用(降级匹配)".to_string());
    }
    // 3. 都失败，无工具直接对话
    println!("[意图] 降级无结果，普通对话");
    (vec![], "普通对话(降级)".to_string())
}

/// 并发执行工具调用
async fn zhixing_gongjudiaoyong(qz: &str, lie: &[llm::ToolCall], lingpai: &str) -> Vec<llm::ToolCall> {
    let mut jieguolie: Vec<llm::ToolCall> = Vec::with_capacity(lie.len());
    for d in lie {
        let mut dan = d.clone();
        println!("[{}] 执行工具: {} 参数: {}", qz, dan.function.name, dan.function.arguments);
        dan.function.arguments = gongjuji::zhixing(&dan.function.name, &dan.function.arguments, lingpai).await;
        jieguolie.push(dan);
    }
    jieguolie
}

/// 工具调用签名，用于重复检测
fn gongju_qianming(lie: &[llm::ToolCall]) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    for d in lie {
        d.function.name.hash(&mut h);
        d.function.arguments.hash(&mut h);
    }
    h.finish()
}

/// ReAct循环：处理工具调用，返回最终结果（成功为Some，失败为None）
pub async fn react_xunhuan(
    peizhi: &aipeizhi::Aipeizhi,
    guanli: &mut aixiaoxiguanli::Xiaoxiguanli,
    qz: &str,
    lingpai: &str,
    _qingqiu: &Qingqiuti,
) -> Option<ReactJieguo> {
    let zuida = peizhixitongzhuti::duqupeizhi::<Ai>(Ai::wenjianming())
        .map(|p| p.zuida_xunhuancishu).unwrap_or(20);
    let mut shangci_hash: u64 = 0;
    let mut chongfu: u32 = 0;

    for cishu in 1..=zuida {
        guanli.caijian_shangxiawen(peizhi.zuida_token);
        println!("[{}] 第 {} 轮循环 (token: {})", qz, cishu, guanli.dangqian_token());
        match openaizhuti::putongqingqiu_react(peizhi, guanli).await {
            Some(ReactJieguo::Wenben(huifu)) => return Some(ReactJieguo::Wenben(huifu)),
            Some(ReactJieguo::Gongjudiaoyong(lie)) => {
                let hash = gongju_qianming(&lie);
                if hash == shangci_hash && shangci_hash != 0 {
                    chongfu += 1;
                    if chongfu >= chongfu_yuzhi {
                        println!("[{}] 工具重复调用 {} 次，移除工具做最终回复", qz, chongfu + 1);
                        guanli.qingkong_gongjulie();
                        if let Some(huifu) = openaizhuti::putongqingqiu(peizhi, guanli).await {
                            return Some(ReactJieguo::Wenben(huifu));
                        }
                        return None;
                    }
                } else {
                    chongfu = 0;
                }
                shangci_hash = hash;
                guanli.zhuijia_zhushou_gongjudiaoyong(lie.clone());
                guanli.zhuijia_gongjujieguo(zhixing_gongjudiaoyong(qz, &lie, lingpai).await);
            }
            None => return None,
        }
    }
    None
}

use actix_web::web;
use crate::jiekouxt::jiekouxtzhuti::huoqufangfa;

pub fn zhuce(cfg: &mut web::ServiceConfig, qianzhui: &str) {
    cfg.service(
        web::scope(qianzhui)
            .route(jiekou_aiduihua::dinyi.lujing, huoqufangfa(jiekou_aiduihua::dinyi.fangshi)().to(jiekou_aiduihua::chuli))
            .route(jiekou_aiduihualiushi::dinyi.lujing, huoqufangfa(jiekou_aiduihualiushi::dinyi.fangshi)().to(jiekou_aiduihualiushi::chuli))
    );
}
