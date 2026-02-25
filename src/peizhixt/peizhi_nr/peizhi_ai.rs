#![allow(non_upper_case_globals)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ribaobiaoqian {
    pub mingcheng: String,
    pub miaoshu: String,
    #[serde(default = "moren_bitian")]
    pub bitian: bool,
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
    "你是人物关系分析助手。根据日报内容，分析其中提到的人物之间的关系。\n\
    返回纯JSON，格式：{\"guanxi\":[{\"ren1\":\"姓名1\",\"ren2\":\"姓名2\",\"guanxi\":\"关系类型\",\"miaoshu\":\"关系描述\"}]}\n\
    关系类型包括但不限于：同事、上下级、客户、合作伙伴、同学等。\n\
    注意：\n\
    1. 只分析日报中明确提及的人物\n\
    2. 如果无法确定具体关系，用\"相关\"作为关系类型\n\
    3. miaoshu字段简要描述关系背景\n\
    4. 只返回JSON，不要返回其他内容".to_string()
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
                    biecheng: vec![],
                },
                Ribaobiaoqian {
                    mingcheng: "对方人员".to_string(),
                    miaoshu: "对方公司参与人员姓名".to_string(),
                    bitian: true,
                    biecheng: vec![],
                },
            ],
            siweidaotu_weidu: moren_siweidaotu_weidu(),
            guanxifenxi_tishici: moren_guanxifenxi_tishici(),
        }
    }
}

impl Ai {
    pub fn wenjianming() -> &'static str {
        "ai"
    }
}

