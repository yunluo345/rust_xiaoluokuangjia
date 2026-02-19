use llm::chat::{ChatMessage, MessageType};
use tiktoken_rs::o200k_base;

/// 计算单条文本的 token 数
pub fn jisuan_wenben(wenben: &str) -> usize {
    match o200k_base() {
        Ok(bpe) => bpe.encode_with_special_tokens(wenben).len(),
        Err(_) => guji_token(wenben),
    }
}

/// 粗略估算 token 数（降级方案）
fn guji_token(wenben: &str) -> usize {
    let mut zhongwen = 0usize;
    let mut qita = 0usize;
    for c in wenben.chars() {
        if c > '\u{4E00}' && c < '\u{9FFF}' {
            zhongwen += 1;
        } else {
            qita += 1;
        }
    }
    zhongwen * 2 + (qita + 3) / 4
}

/// 计算单条消息的 token 数（含角色开销约 4 token）
pub fn jisuan_xiaoxi(xiaoxi: &ChatMessage) -> usize {
    let neirong_token = match &xiaoxi.message_type {
        MessageType::ToolUse(diaoyonglie) => {
            let mut zong = 0;
            for d in diaoyonglie {
                zong += jisuan_wenben(&d.function.name);
                zong += jisuan_wenben(&d.function.arguments);
            }
            zong
        }
        MessageType::ToolResult(jieguolie) => {
            let mut zong = 0;
            for j in jieguolie {
                zong += jisuan_wenben(&j.function.arguments);
            }
            zong
        }
        _ => jisuan_wenben(&xiaoxi.content),
    };
    neirong_token + 4 // 每条消息额外开销
}

/// 计算消息列表总 token 数
pub fn jisuan_xiaoxilie(xitongtishici: Option<&str>, xiaoxilie: &[ChatMessage]) -> usize {
    let mut zong = 0usize;
    if let Some(tishici) = xitongtishici {
        zong += jisuan_wenben(tishici) + 4;
    }
    for xiaoxi in xiaoxilie {
        zong += jisuan_xiaoxi(xiaoxi);
    }
    zong + 3 // 回复引导开销
}
