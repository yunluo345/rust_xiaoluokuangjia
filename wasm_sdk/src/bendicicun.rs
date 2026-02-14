use wasm_bindgen::JsValue;
use crate::gongju::cuowu;

const lingpai_jian: &str = "wasm_lingpai";

fn huoqucunchuqi() -> Result<web_sys::Storage, JsValue> {
    web_sys::window()
        .ok_or_else(|| cuowu("无法获取window"))?
        .local_storage()
        .map_err(|_| cuowu("无法访问localStorage"))?
        .ok_or_else(|| cuowu("localStorage不可用"))
}

pub fn cunlingpai(lingpai: &str) {
    if let Ok(cunchuqi) = huoqucunchuqi() {
        let _ = cunchuqi.set_item(lingpai_jian, lingpai);
    }
}

pub fn duqulingpai() -> Option<String> {
    huoqucunchuqi().ok()?.get_item(lingpai_jian).ok()?
}

pub fn shanculingpai() {
    if let Ok(cunchuqi) = huoqucunchuqi() {
        let _ = cunchuqi.remove_item(lingpai_jian);
    }
}
