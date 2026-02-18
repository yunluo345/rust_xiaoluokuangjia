use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response, ReadableStreamDefaultReader};
use js_sys::Uint8Array;

pub fn cuowu(xinxi: &str) -> JsValue {
    JsValue::from_str(xinxi)
}

pub fn goujianqingqiu(fangfa: &str, url: &str, ti: Option<&str>, ewaiqingqiutou: Option<&[(&str, &str)]>, abort_signal: Option<&web_sys::AbortSignal>) -> Result<Request, JsValue> {
    let opts = RequestInit::new();
    opts.set_method(fangfa);
    opts.set_mode(RequestMode::Cors);
    
    if let Some(signal) = abort_signal {
        opts.set_signal(Some(signal));
    }

    let tou = web_sys::Headers::new()?;
    tou.set("Content-Type", "application/json")?;
    if let Some(ewai) = ewaiqingqiutou {
        for (jian, zhi) in ewai {
            tou.set(jian, zhi)?;
        }
    }
    opts.set_headers(&tou);

    if let Some(neirong) = ti {
        opts.set_body(&JsValue::from_str(neirong));
    }

    Request::new_with_str_and_init(url, &opts)
}

pub async fn fasongqingqiu(request: &Request) -> Result<Response, JsValue> {
    let chuangkou = web_sys::window().ok_or_else(|| cuowu("无法获取window对象"))?;
    JsFuture::from(chuangkou.fetch_with_request(request)).await?.dyn_into()
}

pub async fn putongqingqiu_neibu(fangfa: &str, url: &str, ti: Option<&str>, ewaiqingqiutou: Option<&[(&str, &str)]>) -> Result<String, JsValue> {
    let request = goujianqingqiu(fangfa, url, ti, ewaiqingqiutou, None)?;
    let xiangying = fasongqingqiu(&request).await?;
    let wenben = JsFuture::from(xiangying.text()?).await?;
    wenben.as_string().ok_or_else(|| cuowu("响应不是文本"))
}

pub fn huoquhuidiao(hanming: &str) -> Result<js_sys::Function, JsValue> {
    js_sys::Reflect::get(&js_sys::global(), &JsValue::from_str(hanming))?
        .dyn_into::<js_sys::Function>()
        .map_err(|_| cuowu(&format!("回调函数 {} 不存在或不是函数", hanming)))
}

async fn duquliugushu(xiangying: &Response, duquqi_huidiao: Option<&js_sys::Function>, chuli: impl Fn(&str) -> Result<(), JsValue>) -> Result<(), JsValue> {
    let ti = xiangying.body().ok_or_else(|| cuowu("响应没有body"))?;
    let duquqi = ti.get_reader().dyn_into::<ReadableStreamDefaultReader>()?;
    
    if let Some(huidiao) = duquqi_huidiao {
        let _ = huidiao.call1(&JsValue::NULL, &duquqi);
    }

    let jiemaqi = web_sys::TextDecoder::new()?;
    loop {
        let jieguo = JsFuture::from(duquqi.read()).await?;
        let wancheng = js_sys::Reflect::get(&jieguo, &JsValue::from_str("done"))?;
        if wancheng.as_bool().unwrap_or(false) { break; }

        let zhi = js_sys::Reflect::get(&jieguo, &JsValue::from_str("value"))?;
        let shuju = zhi.dyn_into::<Uint8Array>()?;
        let wenben = jiemaqi.decode_with_buffer_source(&shuju)?;
        
        for hang in wenben.split('\n') {
            if !hang.is_empty() {
                chuli(hang)?;
            }
        }
    }
    Ok(())
}

pub async fn duquliushi(xiangying: &Response, huidiao: &js_sys::Function) -> Result<(), JsValue> {
    duquliugushu(xiangying, None, |hang| {
        let _ = huidiao.call1(&JsValue::NULL, &JsValue::from_str(hang));
        Ok(())
    }).await
}

pub async fn duqujiamiliushi(xiangying: &Response, miyao: &[u8], huidiao: &js_sys::Function, duquqi_huidiao: Option<&js_sys::Function>) -> Result<(), JsValue> {
    duquliugushu(xiangying, duquqi_huidiao, |hang| {
        let jiemi = super::jiami_gongju::jiemixiangying(hang, miyao)?;
        let _ = huidiao.call1(&JsValue::NULL, &JsValue::from_str(&jiemi));
        Ok(())
    }).await
}
