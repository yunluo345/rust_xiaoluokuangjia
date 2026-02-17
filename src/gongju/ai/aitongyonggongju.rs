/// 根据AI类型，智能补全网关地址
pub fn buquan_wangguandizhi(leixing: &str, wangguandizhi: &str) -> Option<String> {
    let houzhui = match leixing {
        "openai" | "openapi" => "/v1",
        _ => return None,
    };
    let dizhi = wangguandizhi.trim_end_matches('/');
    if dizhi.is_empty() {
        return None;
    }
    let dizhi = dizhi.trim_end_matches("/chat/completions").trim_end_matches("/v1");
    Some(format!("{}{}", dizhi, houzhui))
}
