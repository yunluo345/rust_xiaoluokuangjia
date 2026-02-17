/// 根据AI类型，智能补全网关地址
pub fn buquan_wangguandizhi(leixing: &str, wangguandizhi: &str) -> Option<String> {
    let qianzhui = match leixing {
        "openai" => "/v1",
        _ => return None,
    };
    let dizhi = wangguandizhi.trim_end_matches('/');
    if dizhi.is_empty() {
        return None;
    }
    match dizhi.find(qianzhui) {
        Some(_) => Some(dizhi.to_string()),
        None => Some(format!("{}{}", dizhi, qianzhui)),
    }
}
