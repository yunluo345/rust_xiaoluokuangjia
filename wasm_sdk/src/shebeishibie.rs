use sha2::{Sha256, Digest};
use wasm_bindgen::prelude::*;

const fenggefu: &str = "|";

fn sha256hex(shuru: &str) -> String {
    let sanzhi = Sha256::digest(shuru.as_bytes());
    let mut jieguo = String::with_capacity(64);
    for b in sanzhi.iter() {
        jieguo.push(char::from(b"0123456789abcdef"[(b >> 4) as usize]));
        jieguo.push(char::from(b"0123456789abcdef"[(b & 0xf) as usize]));
    }
    jieguo
}

pub fn shengchengzhiwen() -> String {
    let chuangkou = web_sys::window();
    let daohangqi = chuangkou.as_ref().map(|w| w.navigator());
    let pingmu = chuangkou.as_ref().and_then(|w| w.screen().ok());
    let xinxi = [
        daohangqi.as_ref().and_then(|n| n.user_agent().ok()).unwrap_or_default(),
        pingmu.map(|s| format!("{}x{}", s.width().unwrap_or(0), s.height().unwrap_or(0))).unwrap_or_default(),
        js_sys::eval("Intl.DateTimeFormat().resolvedOptions().timeZone").ok().and_then(|v| v.as_string()).unwrap_or_default(),
        daohangqi.as_ref().and_then(|n| n.language()).unwrap_or_default(),
        daohangqi.as_ref().and_then(|n| n.platform().ok()).unwrap_or_default(),
    ].join(fenggefu);
    sha256hex(&xinxi)
}

#[wasm_bindgen]
pub struct Shebeishibie;

#[wasm_bindgen]
impl Shebeishibie {
    #[wasm_bindgen(constructor)]
    pub fn xinjian() -> Self {
        Self
    }

    #[wasm_bindgen(js_name = "shengchengzhiwen")]
    pub fn shengchengzhiwen_wasm(&self) -> String {
        shengchengzhiwen()
    }
}
