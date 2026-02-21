use crate::gongju::jwtgongju;
use crate::peizhixt::peizhi_nr::peizhi_ai::Ai;
use crate::peizhixt::peizhixitongzhuti;
use crate::shujuku::psqlshujuku::shujubiao_nr::ribao::{
    shujucaozuo_ribao,
    shujucaozuo_ribao_biaoqian,
    shujucaozuo_ribao_biaoqianrenwu,
};
use llm::chat::Tool;
use serde::Deserialize;
use serde_json::{json, Value};
use super::{gongju_ribaorenwuchuli, gongju_ribaojiancha};

#[derive(Debug, Clone)]
pub enum Gongjufenzu {
    Guanli,
    Xitong,
}

#[derive(Deserialize)]
struct Qingqiucanshu {
    neirong: String,
    fabushijian: String,
    biaoqianidlie: Option<Vec<String>>,
}

/// 获取工具关键词
pub fn huoqu_guanjianci() -> Vec<String> {
    vec![
        "提交日报".to_string(),
        "发布日报".to_string(),
        "保存日报".to_string(),
        "新增日报".to_string(),
        "写日报".to_string(),
        "report".to_string(),
        "submit".to_string(),
        "daily report".to_string(),
    ]
}

/// 获取工具分组
pub fn huoqu_fenzu() -> Gongjufenzu {
    Gongjufenzu::Xitong
}

/// 工具定义
pub fn dinyi() -> Tool {
    Tool {
        tool_type: "function".to_string(),
        function: llm::chat::FunctionTool {
            name: "ribao_tijiao".to_string(),
            description: "提交日报内容并创建标签提取任务。提交成功后必须立即调用任务处理工具完成标签提取。".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "neirong": {
                        "type": "string",
                        "description": "日报完整内容"
                    },
                    "fabushijian": {
                        "type": "string",
                        "description": "发布时间（时间戳字符串）"
                    },
                    "biaoqianidlie": {
                        "type": "array",
                        "items": {"type": "string"},
                        "description": "可选：要关联的标签ID列表"
                    }
                },
                "required": ["neirong", "fabushijian"]
            }),
        },
    }
}

/// 工具执行
pub async fn zhixing(canshu: &str, lingpai: &str) -> String {
    let zaiti = match jwtgongju::yanzheng(lingpai).await {
        Some(z) => z,
        None => return json!({"cuowu": "令牌无效或已过期"}).to_string(),
    };

    let qingqiu: Qingqiucanshu = match serde_json::from_str(canshu) {
        Ok(q) => q,
        Err(_) => return json!({"cuowu": "参数格式错误"}).to_string(),
    };

    let peizhi = match peizhixitongzhuti::duqupeizhi::<Ai>(Ai::wenjianming()) {
        Some(p) => p,
        None => return json!({"cuowu": "无法读取配置"}).to_string(),
    };

    let aiyijian = gongju_ribaojiancha::ai_jiancha(&qingqiu.neirong, &peizhi).await;
    let ai_hege = aiyijian.as_ref()
        .map(|yj| yj.contains("内容完整规范"))
        .unwrap_or(false);

    if !ai_hege {
        return json!({
            "cuowu": "日报审核未通过",
            "yuanyin": aiyijian.unwrap_or_else(|| "内容不符合规范，请补充完整信息后重试".to_string())
        }).to_string();
    }

    let ribaoid = match shujucaozuo_ribao::xinzeng(&zaiti.yonghuid, &qingqiu.neirong, &qingqiu.fabushijian).await {
        Some(id) => id,
        None => return json!({"cuowu": "日报提交失败"}).to_string(),
    };

    let zuidachangshicishu = peizhixitongzhuti::duqupeizhi::<Ai>(Ai::wenjianming())
        .map(|p| p.ribao_biaoqianrenwu_chongshi_cishu as i64)
        .unwrap_or(3);

    let renwuid = match shujucaozuo_ribao_biaoqianrenwu::faburenwu(&ribaoid, &zaiti.yonghuid, zuidachangshicishu).await {
        Some(id) => id,
        None => return json!({"cuowu": "日报提交成功但任务发布失败", "ribaoid": ribaoid}).to_string(),
    };

    json!({"chenggong": true, "ribaoid": ribaoid, "renwuid": renwuid}).to_string()
}
