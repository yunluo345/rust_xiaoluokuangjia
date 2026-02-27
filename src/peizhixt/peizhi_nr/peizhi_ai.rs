#![allow(non_upper_case_globals)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ribaobiaoqian {
    pub mingcheng: String,
    pub miaoshu: String,
    #[serde(default = "moren_bitian")]
    pub bitian: bool,
    #[serde(default)]
    pub duozhi: bool,
    #[serde(default)]
    pub biecheng: Vec<String>,
}

fn moren_bitian() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Siweidaotuzijiedian {
    pub mingcheng: String,
    pub neirong: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Siweidaotuweidu {
    pub mingcheng: String,
    pub zijiedian: Vec<Siweidaotuzijiedian>,
    #[serde(default)]
    pub beizhu: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuanxifenxiYanse {
    pub zhu: String,
    pub qian: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuanxifenxiLeixing {
    pub mingcheng: String,
    #[serde(default)]
    pub biecheng: Vec<String>,
    pub yanse: GuanxifenxiYanse,
    #[serde(default)]
    pub fumian: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fenxishitileixing {
    pub mingcheng: String,
    pub biaoti: String,
    #[serde(default)]
    pub guanlianfenxi: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ai {
    pub zuida_xunhuancishu: u32,
    #[serde(default = "moren_ribao_biaoqianrenwu_chongshi_cishu")]
    pub ribao_biaoqianrenwu_chongshi_cishu: u32,
    #[serde(default = "moren_ribao_biaoqianrenwu_bingfashuliang")]
    pub ribao_biaoqianrenwu_bingfashuliang: u32,
    #[serde(default = "moren_bingxingrenwushu")]
    pub bingxingrenwushu: u32,
    pub ribao_biaoqian: Vec<Ribaobiaoqian>,
    #[serde(default = "moren_siweidaotu_weidu")]
    pub siweidaotu_weidu: Vec<Siweidaotuweidu>,
    #[serde(default = "moren_guanxifenxi_tishici")]
    pub guanxifenxi_tishici: String,
    #[serde(default = "moren_guanxifenxi_danpian_zifushangxian")]
    pub guanxifenxi_danpian_zifushangxian: usize,
    #[serde(default = "moren_guanxifenxi_fenduan_daxiao")]
    pub guanxifenxi_fenduan_daxiao: usize,
    #[serde(default = "moren_guanxifenxi_fenduan_zhongdie")]
    pub guanxifenxi_fenduan_zhongdie: usize,
    #[serde(default = "moren_guanxifenxi_zuida_fenduanshu")]
    pub guanxifenxi_zuida_fenduanshu: usize,
    #[serde(default = "moren_guanxifenxi_leixing")]
    pub guanxifenxi_leixing: Vec<GuanxifenxiLeixing>,
    #[serde(default = "moren_jiaoliu_fenxi_tishici")]
    pub jiaoliu_fenxi_tishici: String,
    #[serde(default = "moren_xiangmu_guanlian_tishici")]
    pub xiangmu_guanlian_tishici: String,
    #[serde(default = "moren_fenxi_shiti_leixing")]
    pub fenxi_shiti_leixing: Vec<Fenxishitileixing>,
}

fn moren_ribao_biaoqianrenwu_chongshi_cishu() -> u32 {
    3
}

fn moren_ribao_biaoqianrenwu_bingfashuliang() -> u32 {
    1
}

fn moren_bingxingrenwushu() -> u32 {
    5
}

fn moren_guanxifenxi_tishici() -> String {
    "你是日报关系分析助手。根据日报内容，分析其中提到的人物之间、公司之间的关系，包括正面和负面关系。\n\
    返回纯JSON，格式：{\"guanxi\":[{\"ren1\":\"名称1\",\"ren2\":\"名称2\",\"guanxi\":\"关系类型\",\"miaoshu\":\"关系描述\",\"xindu\":0.9,\"zhengjupianduan\":\"原文证据片段\",\"juese\":{\"ren1\":\"角色\",\"ren2\":\"角色\"},\"qinggan_qingxiang\":\"正面|负面|中性\"}]}\n\
    人物关系类型：同事、上下级、客户、合作伙伴、同学、对立方、竞争者等。\n\
    公司关系类型：合作方、竞争对手、上下游、客户供应商、子母公司、对立方等。\n\
    上下级判定规则：出现\"汇报\"\"审批\"\"负责人\"\"直属\"\"经理安排\"\"向X汇报\"\"X负责\"\"X安排\"\"X分配任务\"\"管理\"\"带领\"时优先判定为上下级关系，ren1为上级。\n\
    竞争/敌对判定规则：出现\"竞争\"\"抢单\"\"竞标\"\"对手\"\"争夺\"\"比稿\"\"PK\"\"挖客户\"\"压价\"\"低价竞争\"\"流失\"\"被抢\"\"冲突\"\"纠纷\"\"投诉\"\"不满\"\"拒绝\"\"终止合作\"\"撤单\"\"违约\"\"威胁\"\"施压\"\"敌意\"\"针对\"\"排挤\"\"打压\"\"刁难\"\"推诿\"\"甩锅\"\"吵架\"\"争执\"\"翻脸\"\"闹僵\"时应判定为竞争对手或对立方关系。\n\
    情绪/心情判定规则：关注日报中流露的情绪信号，如\"不开心\"\"郁闷\"\"烦躁\"\"焦虑\"\"压力大\"\"受挫\"\"委屈\"\"失望\"\"愤怒\"\"沮丧\"\"无奈\"\"抱怨\"\"不满\"\"难受\"\"崩溃\"\"心累\"\"很烦\"\"不爽\"\"生气\"\"窝火\"表示负面情绪，\"顺利\"\"高兴\"\"开心\"\"满意\"\"有成就感\"\"进展良好\"\"配合默契\"表示正面情绪。将情绪信号融入关系的 miaoshu 和 qinggan_qingxiang 中。\n\
    注意：\n\
    1. 同时分析人物关系和公司关系，放在同一个 guanxi 数组中\n\
    2. ren1/ren2 可以是人名也可以是公司名\n\
    3. 只分析日报中明确提及的实体\n\
    4. 必须同时识别正面关系（合作、客户等）和负面关系（竞争、对立、冲突等），不要回避负面关系\n\
    5. miaoshu 简要描述关系背景，如有情绪信号需纳入描述\n\
    6. xindu 为0到1之间的置信度数值，表示对该关系判断的确定程度\n\
    7. zhengjupianduan 摘录日报原文中支撑该关系的关键语句（不超过50字）\n\
    8. juese 为两个实体在该关系中的角色（如项目经理、技术负责人、客户联系人等，无明确角色可省略）\n\
    9. qinggan_qingxiang 为该关系的情感倾向，取值：正面、负面、中性。根据日报中对该关系的描述语气和用词判断\n\
    10. 只返回JSON，不要返回其他内容\n\
    11. 如果两个实体之间没有关系，不要返回该条目，禁止使用\"无关联\"\"无关系\"\"无\"等作为关系类型\n\
    12. 无法确定具体关系时用\"相关\"".to_string()
}

fn moren_guanxifenxi_danpian_zifushangxian() -> usize {
    4000
}

fn moren_guanxifenxi_fenduan_daxiao() -> usize {
    2500
}

fn moren_guanxifenxi_fenduan_zhongdie() -> usize {
    300
}

fn moren_guanxifenxi_zuida_fenduanshu() -> usize {
    8
}

fn moren_jiaoliu_fenxi_tishici() -> String {
    "你是跨日报交流内容分析助手。你将收到某个客户或项目下多篇日报的交流内容摘要（按时间排序）。\n\
    请分析沟通的整体脉络、演变趋势和关键议题。\n\
    返回纯JSON，格式：{\"zhutihuizong\":[{\"zhuti\":\"主题名\",\"miaoshu\":\"该主题的概述\",\"cishu\":3}],\
    \"yanbianguiji\":\"沟通从...逐步演变为...\",\
    \"guanjianwenti\":[{\"wenti\":\"问题描述\",\"yanzhongchengdu\":\"高|中|低\"}],\
    \"jianyi\":\"后续跟进建议\"}\n\
    注意：\n\
    1. zhutihuizong 按出现频率从高到低排列\n\
    2. yanbianguiji 描述沟通内容随时间的变化趋势\n\
    3. guanjianwenti 提取沟通中暴露的关键问题或风险\n\
    4. jianyi 给出具体可操作的后续跟进建议\n\
    5. 只返回JSON，不要返回其他内容".to_string()
}

fn moren_xiangmu_guanlian_tishici() -> String {
    "你是跨项目关联分析助手。你将收到多个项目的标签汇总数据（包含各项目的人员、客户、工作内容等）。\n\
    请分析这些项目之间的关联关系、共享资源和潜在风险。\n\
    返回纯JSON，格式：{\"xiangmuguanxi\":[{\"xm1\":\"项目A\",\"xm2\":\"项目B\",\"guanxi\":\"关联类型\",\
    \"gongxiangziyuan\":[\"共享人员/客户/资源\"],\"miaoshu\":\"关联描述\"}],\
    \"fengxiantishi\":[{\"neirong\":\"风险描述\",\"yanzhongchengdu\":\"高|中|低\",\"shejiXiangmu\":[\"项目名\"]}],\
    \"ziyuanfenbu\":{\"gaofuzairenyuan\":[{\"xingming\":\"人名\",\"canyu_xiangmu\":[\"项目名\"],\"shuoming\":\"说明\"}]}}\n\
    注意：\n\
    1. xiangmuguanxi 分析每对项目之间的关联\n\
    2. fengxiantishi 识别跨项目的资源冲突、进度风险等\n\
    3. ziyuanfenbu.gaofuzairenyuan 找出参与多个项目的高负载人员\n\
    4. 只返回JSON，不要返回其他内容".to_string()
}

fn moren_fenxi_shiti_leixing() -> Vec<Fenxishitileixing> {
    vec![
        Fenxishitileixing { mingcheng: "项目名称".to_string(), biaoti: "项目".to_string(), guanlianfenxi: true },
        Fenxishitileixing { mingcheng: "客户公司".to_string(), biaoti: "客户".to_string(), guanlianfenxi: false },
        Fenxishitileixing { mingcheng: "客户名字".to_string(), biaoti: "客户人员".to_string(), guanlianfenxi: false },
        Fenxishitileixing { mingcheng: "我方人员".to_string(), biaoti: "我方人员".to_string(), guanlianfenxi: false },
        Fenxishitileixing { mingcheng: "地点".to_string(), biaoti: "地点".to_string(), guanlianfenxi: false },
    ]
}

fn moren_guanxifenxi_leixing() -> Vec<GuanxifenxiLeixing> {
    vec![
        GuanxifenxiLeixing { mingcheng: "同事".to_string(), biecheng: vec![], yanse: GuanxifenxiYanse { zhu: "#6366F1".to_string(), qian: "#EEF2FF".to_string() }, fumian: false },
        GuanxifenxiLeixing { mingcheng: "上下级".to_string(), biecheng: vec![], yanse: GuanxifenxiYanse { zhu: "#06B6D4".to_string(), qian: "#ECFEFF".to_string() }, fumian: false },
        GuanxifenxiLeixing { mingcheng: "客户".to_string(), biecheng: vec!["客户供应商".to_string()], yanse: GuanxifenxiYanse { zhu: "#10B981".to_string(), qian: "#ECFDF5".to_string() }, fumian: false },
        GuanxifenxiLeixing { mingcheng: "合作伙伴".to_string(), biecheng: vec!["合作方".to_string()], yanse: GuanxifenxiYanse { zhu: "#F59E0B".to_string(), qian: "#FFFBEB".to_string() }, fumian: false },
        GuanxifenxiLeixing { mingcheng: "同学".to_string(), biecheng: vec![], yanse: GuanxifenxiYanse { zhu: "#EC4899".to_string(), qian: "#FDF2F8".to_string() }, fumian: false },
        GuanxifenxiLeixing { mingcheng: "相关".to_string(), biecheng: vec![], yanse: GuanxifenxiYanse { zhu: "#8B5CF6".to_string(), qian: "#F5F3FF".to_string() }, fumian: false },
        GuanxifenxiLeixing { mingcheng: "竞争对手".to_string(), biecheng: vec!["竞争者".to_string(), "对立方".to_string(), "敌对".to_string()], yanse: GuanxifenxiYanse { zhu: "#EF4444".to_string(), qian: "#FEF2F2".to_string() }, fumian: true },
        GuanxifenxiLeixing { mingcheng: "上下游".to_string(), biecheng: vec![], yanse: GuanxifenxiYanse { zhu: "#14B8A6".to_string(), qian: "#F0FDFA".to_string() }, fumian: false },
        GuanxifenxiLeixing { mingcheng: "子母公司".to_string(), biecheng: vec![], yanse: GuanxifenxiYanse { zhu: "#7C3AED".to_string(), qian: "#F5F3FF".to_string() }, fumian: false },
    ]
}

fn moren_siweidaotu_weidu() -> Vec<Siweidaotuweidu> {
    vec![
        Siweidaotuweidu {
            mingcheng: "客户分析".to_string(),
            zijiedian: vec![
                Siweidaotuzijiedian { mingcheng: "客户名称".to_string(), neirong: "具体客户名".to_string() },
                Siweidaotuzijiedian { mingcheng: "客户特征".to_string(), neirong: "分析客户是什么样的人，沟通风格、关注点、决策偏好等".to_string() },
                Siweidaotuzijiedian { mingcheng: "合作状态".to_string(), neirong: "当前合作关系评估".to_string() },
            ],
            beizhu: "可根据实际客户数量动态增减子节点".to_string(),
        },
        Siweidaotuweidu {
            mingcheng: "员工表现".to_string(),
            zijiedian: vec![
                Siweidaotuzijiedian { mingcheng: "任务完成度".to_string(), neirong: "各项工作完成情况评估".to_string() },
                Siweidaotuzijiedian { mingcheng: "专业能力".to_string(), neirong: "展现的技术/业务能力".to_string() },
                Siweidaotuzijiedian { mingcheng: "沟通协作".to_string(), neirong: "与客户、同事的协作表现".to_string() },
                Siweidaotuzijiedian { mingcheng: "综合评分".to_string(), neirong: "打分及理由".to_string() },
            ],
            beizhu: String::new(),
        },
        Siweidaotuweidu {
            mingcheng: "工作内容".to_string(),
            zijiedian: vec![
                Siweidaotuzijiedian { mingcheng: "核心任务".to_string(), neirong: "今日最重要的工作".to_string() },
                Siweidaotuzijiedian { mingcheng: "技术细节".to_string(), neirong: "涉及的技术方案和实现".to_string() },
                Siweidaotuzijiedian { mingcheng: "工作成果".to_string(), neirong: "实际产出和交付物".to_string() },
            ],
            beizhu: String::new(),
        },
        Siweidaotuweidu {
            mingcheng: "风险与待办".to_string(),
            zijiedian: vec![
                Siweidaotuzijiedian { mingcheng: "当前风险".to_string(), neirong: "存在的问题或潜在风险".to_string() },
                Siweidaotuzijiedian { mingcheng: "待跟进事项".to_string(), neirong: "需要后续处理的事项".to_string() },
                Siweidaotuzijiedian { mingcheng: "改进建议".to_string(), neirong: "对工作流程或方法的建议".to_string() },
            ],
            beizhu: String::new(),
        },
    ]
}

impl Default for Ai {
    fn default() -> Self {
        Self {
            zuida_xunhuancishu: 20,
            ribao_biaoqianrenwu_chongshi_cishu: 3,
            ribao_biaoqianrenwu_bingfashuliang: 1,
            bingxingrenwushu: 5,
            ribao_biaoqian: vec![
                Ribaobiaoqian {
                    mingcheng: "我方人员".to_string(),
                    miaoshu: "我方公司参与人员姓名".to_string(),
                    bitian: true,
                    duozhi: true,
                    biecheng: vec![],
                },
                Ribaobiaoqian {
                    mingcheng: "对方人员".to_string(),
                    miaoshu: "对方公司参与人员姓名".to_string(),
                    bitian: true,
                    duozhi: true,
                    biecheng: vec![],
                },
            ],
            siweidaotu_weidu: moren_siweidaotu_weidu(),
            guanxifenxi_tishici: moren_guanxifenxi_tishici(),
            guanxifenxi_danpian_zifushangxian: moren_guanxifenxi_danpian_zifushangxian(),
            guanxifenxi_fenduan_daxiao: moren_guanxifenxi_fenduan_daxiao(),
            guanxifenxi_fenduan_zhongdie: moren_guanxifenxi_fenduan_zhongdie(),
            guanxifenxi_zuida_fenduanshu: moren_guanxifenxi_zuida_fenduanshu(),
            guanxifenxi_leixing: moren_guanxifenxi_leixing(),
            jiaoliu_fenxi_tishici: moren_jiaoliu_fenxi_tishici(),
            xiangmu_guanlian_tishici: moren_xiangmu_guanlian_tishici(),
            fenxi_shiti_leixing: moren_fenxi_shiti_leixing(),
        }
    }
}

impl Ai {
    pub fn wenjianming() -> &'static str {
        "ai"
    }
}

