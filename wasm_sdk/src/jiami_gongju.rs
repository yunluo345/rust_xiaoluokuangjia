use wasm_bindgen::prelude::*;

pub fn cuowu(xinxi: &str) -> JsValue {
    JsValue::from_str(xinxi)
}

pub fn cuowu_fmt(qianzhui: &str, e: impl std::fmt::Display) -> JsValue {
    JsValue::from_str(&format!("{}: {}", qianzhui, e))
}

pub fn xuliehua<T: serde::Serialize>(zhi: &T) -> Result<String, JsValue> {
    serde_json::to_string(zhi).map_err(|e| cuowu_fmt("序列化失败", e))
}

pub fn fanxuliehua<T: serde::de::DeserializeOwned>(wenben: &str, miaoshu: &str) -> Result<T, JsValue> {
    serde_json::from_str(wenben).map_err(|e| cuowu_fmt(miaoshu, e))
}

pub fn jiamiqingqiuti(mingwen: &str, miyao: &[u8]) -> Result<String, JsValue> {
    let miwen = super::jiamihexin::jiami(mingwen.as_bytes(), miyao)
        .ok_or_else(|| cuowu("加密请求体失败"))?;
    Ok(super::jiamihexin::zhuanbase64(&miwen))
}

pub fn jiemixiangying(miwen_b64: &str, miyao: &[u8]) -> Result<String, JsValue> {
    let miwen = super::jiamihexin::congbase64(miwen_b64)
        .ok_or_else(|| cuowu("base64解码失败"))?;
    let mingwen = super::jiamihexin::jiemi(&miwen, miyao)
        .ok_or_else(|| cuowu("解密响应失败"))?;
    String::from_utf8(mingwen).map_err(|_| cuowu("解密后的数据不是有效的UTF-8"))
}

pub fn shifouhuihuaguoqi(xiangying_wenben: &str) -> bool {
    xiangying_wenben.contains("\"zhuangtaima\":401") || xiangying_wenben.contains("\"zhuangtaima\": 401")
}

pub struct Jiamixinxi<'a> {
    pub miyao: &'a [u8],
    pub huihuaid: &'a str,
    pub kehugongyao: &'a str,
}
