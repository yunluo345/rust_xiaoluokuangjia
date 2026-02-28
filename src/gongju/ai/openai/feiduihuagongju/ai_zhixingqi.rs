use serde_json::Value;
use super::gongyong::{ai_putongqingqiu_wenben_chongshi, jinghua_json_huifu};

/// AI JSON 执行结果
pub enum AiZhixingJieguo {
    /// 成功：包含已验证的 JSON 字符串
    Chenggong(String),
    /// 提示词为空
    TishiciWeikongzhi,
    /// AI 调用失败
    DiaoyongShibai,
    /// 返回内容非法 JSON
    JsonJiexiShibai(String),
}

/// AI JSON 执行器配置
pub struct AiJsonPeizhi<'a> {
    /// 系统提示词
    pub xitong_tishici: &'a str,
    /// 用户消息内容
    pub yonghu_xiaoxi: String,
    /// 超时秒数
    pub chaoshi_miao: u64,
    /// 任务标识（用于日志）
    pub renwu_biaoshi: &'a str,
}

/// 统一 AI JSON 执行管道：提示词 → AI 调用 → JSON 清洗 → 反序列化校验
///
/// 收敛了 kuaribaofenxi 中多个分析函数的重复流程：
/// 1. 检查提示词非空
/// 2. 调用 AI 获取文本回复
/// 3. 清洗 markdown 代码块标记
/// 4. 校验为合法 JSON
pub async fn zhixing_ai_json(peizhi: AiJsonPeizhi<'_>) -> AiZhixingJieguo {
    if peizhi.xitong_tishici.is_empty() {
        println!("[AI执行器] {} 提示词未配置", peizhi.renwu_biaoshi);
        return AiZhixingJieguo::TishiciWeikongzhi;
    }

    println!("[AI执行器] {} 开始调用", peizhi.renwu_biaoshi);

    let huifu = match ai_putongqingqiu_wenben_chongshi(
        peizhi.xitong_tishici,
        peizhi.yonghu_xiaoxi,
        peizhi.chaoshi_miao,
        3,
    ).await {
        Some(h) => h,
        None => {
            println!("[AI执行器] {} AI调用失败", peizhi.renwu_biaoshi);
            return AiZhixingJieguo::DiaoyongShibai;
        }
    };

    let jinghua = jinghua_json_huifu(&huifu);

    match serde_json::from_str::<Value>(jinghua) {
        Ok(_) => {
            println!("[AI执行器] {} 成功 长度={}", peizhi.renwu_biaoshi, jinghua.len());
            AiZhixingJieguo::Chenggong(jinghua.to_string())
        }
        Err(e) => {
            println!("[AI执行器] {} JSON解析失败: {}", peizhi.renwu_biaoshi, e);
            AiZhixingJieguo::JsonJiexiShibai(format!("JSON解析失败: {}", e))
        }
    }
}

/// 便捷方法：执行 AI JSON 调用，成功返回 Some(json_string)，失败返回 None
/// 兼容现有 Option<String> 签名，方便渐进迁移
pub async fn zhixing_ai_json_jianbian(
    xitong_tishici: &str,
    yonghu_xiaoxi: String,
    chaoshi_miao: u64,
    renwu_biaoshi: &str,
) -> Option<String> {
    let peizhi = AiJsonPeizhi {
        xitong_tishici,
        yonghu_xiaoxi,
        chaoshi_miao,
        renwu_biaoshi,
    };
    match zhixing_ai_json(peizhi).await {
        AiZhixingJieguo::Chenggong(s) => Some(s),
        _ => None,
    }
}
