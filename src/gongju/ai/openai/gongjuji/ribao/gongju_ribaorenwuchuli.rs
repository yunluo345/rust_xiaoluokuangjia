use crate::peizhixt::peizhi_nr::peizhi_ai::Ai;
use crate::peizhixt::peizhixitongzhuti;
use llm::chat::Tool;
use serde_json::json;

use super::super::Gongjufenzu;

// 从 feiduihuagongju 模块 re-export 公开函数，保持外部调用兼容
pub use crate::gongju::ai::openai::feiduihuagongju::renwuchuli::{zhixing, zhixing_neibu, zhixing_dange_renwu_neibu};
pub use crate::gongju::ai::openai::feiduihuagongju::kuaribaofenxi::{ai_jiaoliu_fenxi, ai_ribao_shendu_fenxi, ai_xiangmu_guanlian_fenxi};

pub fn huoqu_guanjianci() -> Vec<String> {
    vec![
        "日报标签任务".to_string(),
        "处理日报任务".to_string(),
        "标签提取任务".to_string(),
    ]
}

pub fn huoqu_fenzu() -> Gongjufenzu {
    Gongjufenzu::Xitong
}

pub fn dinyi() -> Tool {
    let peizhi = peizhixitongzhuti::duqupeizhi::<Ai>(Ai::wenjianming()).unwrap_or_default();

    let biaoqian_tishi = peizhi.ribao_biaoqian.iter()
        .map(|bq| {
            let biecheng_str = bq.biecheng.join("、");
            format!("{}（{}，别名：{}）", bq.mingcheng, bq.miaoshu, biecheng_str)
        })
        .collect::<Vec<_>>()
        .join("；");

    let miaoshu = format!(
        "处理日报标签提取任务，从日报内容中提取标签并绑定。支持的标签：{}",
        biaoqian_tishi
    );

    Tool {
        tool_type: "function".to_string(),
        function: llm::chat::FunctionTool {
            name: "ribao_renwubiaoqian_chuli".to_string(),
            description: miaoshu,
            parameters: json!({
                "type": "object",
                "properties": {
                    "shuliang": {
                        "type": "integer",
                        "description": "本次处理任务数量，未传时使用系统配置并发数量"
                    }
                }
            }),
        },
    }
}
