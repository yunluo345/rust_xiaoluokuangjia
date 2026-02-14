use wasm_bindgen::JsValue;
use crate::jiamihexin;

// ==================== 错误处理 ====================

pub fn cuowu(xinxi: &str) -> JsValue {
    JsValue::from_str(xinxi)
}

pub fn cuowu_fmt(qianzhui: &str, e: impl std::fmt::Display) -> JsValue {
    JsValue::from_str(&format!("{}: {}", qianzhui, e))
}

// ==================== 序列化工具 ====================

pub fn xuliehua<T: serde::Serialize>(zhi: &T) -> Result<String, JsValue> {
    serde_json::to_string(zhi).map_err(|e| cuowu_fmt("序列化失败", e))
}

pub fn fanxuliehua<T: serde::de::DeserializeOwned>(wenben: &str, miaoshu: &str) -> Result<T, JsValue> {
    serde_json::from_str(wenben).map_err(|e| cuowu_fmt(miaoshu, e))
}

// ==================== 加密工具 ====================

pub fn jiamiqingqiuti(mingwen: &str, miyao: &[u8]) -> Result<String, JsValue> {
    let miwen = jiamihexin::jiami(mingwen.as_bytes(), miyao)
        .ok_or_else(|| cuowu("加密请求体失败"))?;
    Ok(jiamihexin::zhuanbase64(&miwen))
}

pub fn jiemixiangying(miwen_b64: &str, miyao: &[u8]) -> Result<String, JsValue> {
    let miwen_zijie = jiamihexin::congbase64(miwen_b64)
        .ok_or_else(|| cuowu("响应base64解码失败"))?;
    let mingwen = jiamihexin::jiemi(&miwen_zijie, miyao)
        .ok_or_else(|| cuowu("解密响应失败"))?;
    String::from_utf8(mingwen).map_err(|_| cuowu("响应不是有效UTF-8"))
}

// ==================== 会话检查 ====================

const huihua_guoqi_zhuangtaima: u16 = 401;

pub fn shifouhuihuaguoqi(xiangying_wenben: &str) -> bool {
    serde_json::from_str::<serde_json::Value>(xiangying_wenben)
        .ok()
        .and_then(|v| v.get("zhuangtaima")?.as_u64())
        .map_or(false, |ma| ma == huihua_guoqi_zhuangtaima as u64)
}
