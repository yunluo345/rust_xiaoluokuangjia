#![allow(non_upper_case_globals)]

use serde::{Deserialize, Serialize};

use super::tishici_moban;

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
pub struct DiaoduqiPeizhi {
    /// 全局AI并发上限（同时发送给AI的请求数）
    #[serde(default = "moren_quanju_bingfa_shangxian")]
    pub quanju_bingfa_shangxian: u32,
    /// 单个请求排队超时（秒），超时返回错误
    #[serde(default = "moren_paidui_chaoshi_miao")]
    pub paidui_chaoshi_miao: u64,
    /// 标签任务：前端断开后是否继续
    #[serde(default = "moren_renwu_houtai_zhixing")]
    pub renwu_houtai_zhixing: bool,
    /// 对话请求：前端断开后是否继续
    #[serde(default)]
    pub duihua_houtai_zhixing: bool,
}

fn moren_quanju_bingfa_shangxian() -> u32 {
    5
}

fn moren_paidui_chaoshi_miao() -> u64 {
    300
}

fn moren_renwu_houtai_zhixing() -> bool {
    true
}

impl Default for DiaoduqiPeizhi {
    fn default() -> Self {
        Self {
            quanju_bingfa_shangxian: moren_quanju_bingfa_shangxian(),
            paidui_chaoshi_miao: moren_paidui_chaoshi_miao(),
            renwu_houtai_zhixing: moren_renwu_houtai_zhixing(),
            duihua_houtai_zhixing: false,
        }
    }
}

fn moren_diaoduqi_peizhi() -> DiaoduqiPeizhi {
    DiaoduqiPeizhi::default()
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
    #[serde(default = "moren_diaoduqi_peizhi")]
    pub diaoduqi: DiaoduqiPeizhi,
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
    #[serde(default = "moren_shendu_fenxi_tishici")]
    pub shendu_fenxi_tishici: String,
    #[serde(default = "moren_guanlian_shendu_tishici")]
    pub guanlian_shendu_tishici: String,
    #[serde(default = "moren_biaoti_shengcheng_tishici")]
    pub biaoti_shengcheng_tishici: String,
    #[serde(default = "moren_zhaiyao_shengcheng_tishici")]
    pub zhaiyao_shengcheng_tishici: String,
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
    tishici_moban::guanxifenxi()
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
    tishici_moban::jiaoliu_fenxi()
}

fn moren_xiangmu_guanlian_tishici() -> String {
    tishici_moban::xiangmu_guanlian()
}

fn moren_shendu_fenxi_tishici() -> String {
    tishici_moban::shendu_fenxi()
}

fn moren_guanlian_shendu_tishici() -> String {
    tishici_moban::guanlian_shendu_fenxi()
}

fn moren_biaoti_shengcheng_tishici() -> String {
    tishici_moban::biaoti_shengcheng()
}

fn moren_zhaiyao_shengcheng_tishici() -> String {
    tishici_moban::zhaiyao_shengcheng()
}

fn moren_fenxi_shiti_leixing() -> Vec<Fenxishitileixing> {
    vec![
        Fenxishitileixing { mingcheng: "项目名称".to_string(), biaoti: "项目".to_string(), guanlianfenxi: true },
        Fenxishitileixing { mingcheng: "客户公司".to_string(), biaoti: "客户".to_string(), guanlianfenxi: true },
        Fenxishitileixing { mingcheng: "客户名字".to_string(), biaoti: "客户人员".to_string(), guanlianfenxi: true },
        Fenxishitileixing { mingcheng: "我方人员".to_string(), biaoti: "我方人员".to_string(), guanlianfenxi: true },
        Fenxishitileixing { mingcheng: "地点".to_string(), biaoti: "地点".to_string(), guanlianfenxi: true },
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
            diaoduqi: DiaoduqiPeizhi::default(),
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
                Ribaobiaoqian {
                    mingcheng: "职位".to_string(),
                    miaoshu: "人员在公司中的职位或职务名称".to_string(),
                    bitian: false,
                    duozhi: false,
                    biecheng: vec![
                        "岗位".to_string(),
                        "职务".to_string(),
                        "头衔".to_string(),
                        "职位信息".to_string(),
                        "岗位信息".to_string(),
                    ],
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
            shendu_fenxi_tishici: moren_shendu_fenxi_tishici(),
            guanlian_shendu_tishici: moren_guanlian_shendu_tishici(),
            biaoti_shengcheng_tishici: moren_biaoti_shengcheng_tishici(),
            zhaiyao_shengcheng_tishici: moren_zhaiyao_shengcheng_tishici(),
        }
    }
}

impl Ai {
    pub fn wenjianming() -> &'static str {
        "ai"
    }

    pub fn duqu_huo_moren() -> Self {
        crate::peizhixt::peizhixitongzhuti::duqupeizhi::<Self>(Self::wenjianming())
            .unwrap_or_default()
    }
}

