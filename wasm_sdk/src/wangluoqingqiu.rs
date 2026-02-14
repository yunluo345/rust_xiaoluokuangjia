use std::rc::Rc;
use std::cell::Cell;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response, ReadableStreamDefaultReader};
use js_sys::Uint8Array;
use crate::gongju::{cuowu, jiemixiangying, shifouhuihuaguoqi};

// ==================== 请求构建 ====================

pub fn goujianqingqiu(
    fangfa: &str,
    url: &str,
    ti: Option<&str>,
    ewaiqingqiutou: Option<&[(&str, &str)]>,
    zhongduanxinhao: Option<&web_sys::AbortSignal>
) -> Result<Request, JsValue> {
    let opts = RequestInit::new();
    opts.set_method(fangfa);
    opts.set_mode(RequestMode::Cors);
    if let Some(xinhao) = zhongduanxinhao {
        opts.set_signal(Some(xinhao));
    }
    if let Some(neirong) = ti {
        opts.set_body(&JsValue::from_str(neirong));
    }
    let request = Request::new_with_str_and_init(url, &opts)
        .map_err(|_| cuowu("创建请求失败"))?;
    let toubu = request.headers();
    toubu.set("Content-Type", "application/json").ok();
    if let Some(tou_lie) = ewaiqingqiutou {
        for (ming, zhi) in tou_lie {
            toubu.set(ming, zhi).ok();
        }
    }
    Ok(request)
}

// ==================== 请求发送 ====================

pub async fn fasongqingqiu(request: &Request) -> Result<Response, JsValue> {
    let chuangkou = web_sys::window().ok_or_else(|| cuowu("无法获取window"))?;
    JsFuture::from(chuangkou.fetch_with_request(request))
        .await?
        .dyn_into()
        .map_err(|_| cuowu("响应类型错误"))
}

pub async fn putongqingqiu(
    fangfa: &str,
    url: &str,
    ti: Option<&str>,
    ewaiqingqiutou: Option<&[(&str, &str)]>
) -> Result<String, JsValue> {
    let request = goujianqingqiu(fangfa, url, ti, ewaiqingqiutou, None)?;
    let xiangying = fasongqingqiu(&request).await?;
    let wenben = JsFuture::from(xiangying.text()?).await?;
    wenben.as_string().ok_or_else(|| cuowu("响应不是文本"))
}

// ==================== 回调获取 ====================

pub fn huoquhuidiao(hanming: &str) -> Result<js_sys::Function, JsValue> {
    js_sys::Reflect::get(&js_sys::global(), &JsValue::from_str(hanming))
        .map_err(|_| cuowu("找不到回调函数"))?
        .dyn_into()
        .map_err(|_| cuowu("回调不是函数"))
}

// ==================== 流处理 ====================

async fn duquliugushu(
    xiangying: &Response,
    zhongduan_biaozhi: Option<Rc<Cell<bool>>>,
    chuli: impl Fn(&str) -> Result<(), JsValue>
) -> Result<(), JsValue> {
    let liuti = xiangying.body().ok_or_else(|| cuowu("无响应体"))?;
    let duquqi: ReadableStreamDefaultReader = liuti.get_reader().dyn_into()?;
    let jiemaqi = web_sys::TextDecoder::new().map_err(|_| cuowu("创建解码器失败"))?;
    loop {
        if let Some(ref biaozhi) = zhongduan_biaozhi {
            if biaozhi.get() {
                web_sys::console::log_1(&JsValue::from_str("检测到中断标志，停止读取流"));
                let _ = duquqi.cancel();
                return Err(cuowu("请求已中断"));
            }
        }
        
        let jieguo = JsFuture::from(duquqi.read()).await?;
        if js_sys::Reflect::get(&jieguo, &JsValue::from_str("done"))
            .unwrap_or(JsValue::TRUE)
            .is_truthy() {
            break;
        }
        if let Some(shuzhu) = js_sys::Reflect::get(&jieguo, &JsValue::from_str("value"))
            .ok()
            .filter(|v| !v.is_undefined()) {
            let wenben = jiemaqi.decode_with_buffer_source(&Uint8Array::new(&shuzhu))
                .unwrap_or_default();
            chuli(&wenben)?;
        }
    }
    Ok(())
}

pub async fn duquliushi(
    xiangying: &Response,
    zhongduan_biaozhi: Option<Rc<Cell<bool>>>,
    huidiao: &js_sys::Function
) -> Result<(), JsValue> {
    duquliugushu(xiangying, zhongduan_biaozhi, |wenben| {
        let _ = huidiao.call1(&JsValue::NULL, &JsValue::from_str(wenben));
        Ok(())
    }).await
}

pub async fn duqujiamiliushi(
    xiangying: &Response,
    zhongduan_biaozhi: Option<Rc<Cell<bool>>>,
    miyao: &[u8],
    huidiao: &js_sys::Function
) -> Result<(), JsValue> {
    let content_type = xiangying.headers().get("content-type")
        .ok()
        .flatten()
        .unwrap_or_default();
    
    if content_type.contains("text/plain") {
        let wenben = JsFuture::from(xiangying.text().map_err(|_| cuowu("读取响应失败"))?)
            .await?
            .as_string()
            .ok_or_else(|| cuowu("响应不是文本"))?;
        
        let mingwen = jiemixiangying(&wenben, miyao)?;
        
        let json: serde_json::Value = serde_json::from_str(&mingwen)
            .map_err(|_| cuowu("解析错误响应失败"))?;
        
        let zhuangtaima = json.get("zhuangtaima").and_then(|v| v.as_u64()).unwrap_or(500);
        let xiaoxi = json.get("xiaoxi").and_then(|v| v.as_str()).unwrap_or("未知错误");
        
        return Err(JsValue::from_str(&format!("[错误 {}] {}", zhuangtaima, xiaoxi)));
    }
    
    duquliugushu(xiangying, zhongduan_biaozhi, |yuanwen| {
        for hang in yuanwen.split("data: ").filter(|s| !s.is_empty()) {
            let miwen = hang.trim();
            if miwen.is_empty() { continue; }
            let mingwen = jiemixiangying(miwen, miyao)?;
            if shifouhuihuaguoqi(&mingwen) {
                return Err(cuowu("会话已过期"));
            }
            let _ = huidiao.call1(&JsValue::NULL, &JsValue::from_str(&format!("data: {}\n\n", mingwen)));
        }
        Ok(())
    }).await
}
