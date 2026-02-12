#![allow(non_upper_case_globals)]

mod jiamihexin;
pub mod jiekou_nr;

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response, ReadableStreamDefaultReader};
use js_sys::Uint8Array;
use jiekou_nr::xitong::miyaojiaohuanjiekou as miyaojiekou;
use jiekou_nr::xitong::jiankangqingqiu as jiankangqq;
use jiekou_nr::xitong::jiamijiankang as jiamijiankangqq;

struct Jiamixinxi<'a> {
    miyao: &'a [u8],
    huihuaid: &'a str,
    kehugongyao: &'a str,
}

fn cuowu(xinxi: &str) -> JsValue {
    JsValue::from_str(xinxi)
}

fn cuowu_fmt(qianzhui: &str, e: impl std::fmt::Display) -> JsValue {
    JsValue::from_str(&format!("{}: {}", qianzhui, e))
}

fn xuliehua<T: serde::Serialize>(zhi: &T) -> Result<String, JsValue> {
    serde_json::to_string(zhi).map_err(|e| cuowu_fmt("序列化失败", e))
}

fn fanxuliehua<T: serde::de::DeserializeOwned>(wenben: &str, miaoshu: &str) -> Result<T, JsValue> {
    serde_json::from_str(wenben).map_err(|e| cuowu_fmt(miaoshu, e))
}

fn jiamiqingqiuti(mingwen: &str, miyao: &[u8]) -> Result<String, JsValue> {
    let miwen = jiamihexin::jiami(mingwen.as_bytes(), miyao)
        .ok_or_else(|| cuowu("加密请求体失败"))?;
    Ok(jiamihexin::zhuanbase64(&miwen))
}

fn jiemixiangying(miwen_b64: &str, miyao: &[u8]) -> Result<String, JsValue> {
    let miwen_zijie = jiamihexin::congbase64(miwen_b64)
        .ok_or_else(|| cuowu("响应base64解码失败"))?;
    let mingwen = jiamihexin::jiemi(&miwen_zijie, miyao)
        .ok_or_else(|| cuowu("解密响应失败"))?;
    String::from_utf8(mingwen).map_err(|_| cuowu("响应不是有效UTF-8"))
}

fn goujianqingqiu(fangfa: &str, url: &str, ti: Option<&str>, ewaiqingqiutou: Option<&[(&str, &str)]>) -> Result<Request, JsValue> {
    let opts = RequestInit::new();
    opts.set_method(fangfa);
    opts.set_mode(RequestMode::Cors);
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

async fn fasongqingqiu(request: &Request) -> Result<Response, JsValue> {
    let chuangkou = web_sys::window().ok_or_else(|| cuowu("无法获取window"))?;
    JsFuture::from(chuangkou.fetch_with_request(request)).await?.dyn_into().map_err(|_| cuowu("响应类型错误"))
}

async fn putongqingqiu_neibu(fangfa: &str, url: &str, ti: Option<&str>, ewaiqingqiutou: Option<&[(&str, &str)]>) -> Result<String, JsValue> {
    let request = goujianqingqiu(fangfa, url, ti, ewaiqingqiutou)?;
    let xiangying = fasongqingqiu(&request).await?;
    let wenben = JsFuture::from(xiangying.text()?).await?;
    wenben.as_string().ok_or_else(|| cuowu("响应不是文本"))
}

#[wasm_bindgen]
pub struct Kehuduanjiami {
    fuwuqidizhi: String,
    huihuaid: Option<String>,
    miyao: Option<Vec<u8>>,
    kehugongyao_b64: Option<String>,
}

#[wasm_bindgen]
impl Kehuduanjiami {
    #[wasm_bindgen(constructor)]
    pub fn xinjian(fuwuqidizhi: &str) -> Self {
        Self {
            fuwuqidizhi: fuwuqidizhi.to_string(),
            huihuaid: None,
            miyao: None,
            kehugongyao_b64: None,
        }
    }

    pub async fn xieshangmiyao(&mut self, zhiwen: &str) -> Result<(), JsValue> {
        let url = format!("{}{}", self.fuwuqidizhi, miyaojiekou::lujing);
        let ti = xuliehua(&miyaojiekou::Qingqiuti { zhiwen: zhiwen.to_string() })?;
        let xiangying_wenben = putongqingqiu_neibu(miyaojiekou::fangshi, &url, Some(&ti), None).await?;
        let xiangying: miyaojiekou::Xiangying = fanxuliehua(&xiangying_wenben, "解析响应失败")?;
        let shuju = xiangying.shuju.ok_or_else(|| cuowu("服务端未返回公钥数据"))?;
        let fuwuqigongyao = jiamihexin::congbase64(&shuju.gongyao)
            .ok_or_else(|| cuowu("服务端公钥base64解码失败"))?;
        let (kehusiyao, kehugongyao) = jiamihexin::shengchengyaodui();
        let gongxiangyao = jiamihexin::xieshanggongxiangyao(&kehusiyao, &fuwuqigongyao)
            .ok_or_else(|| cuowu("ECDH协商失败"))?;
        let miyao = jiamihexin::paishengyao(&gongxiangyao, jiamihexin::yanfen)
            .ok_or_else(|| cuowu("密钥派生失败"))?;
        self.huihuaid = Some(shuju.huihuaid);
        self.miyao = Some(miyao);
        self.kehugongyao_b64 = Some(jiamihexin::zhuanbase64(&kehugongyao));
        Ok(())
    }

    pub async fn jiamiqingqiu(&self, fangfa: &str, lujing: &str, qingqiuti: Option<String>) -> Result<String, JsValue> {
        let xinxi = self.huoqujiamixinxi()?;
        let jiami_ti = qingqiuti.map(|ti| jiamiqingqiuti(&ti, xinxi.miyao)).transpose()?;
        let url = format!("{}{}", self.fuwuqidizhi, lujing);
        let ewaiqingqiutou = vec![
            ("X-Huihua-Id", xinxi.huihuaid),
            ("X-Kehugongyao", xinxi.kehugongyao),
        ];
        let xiangying_wenben = putongqingqiu_neibu(fangfa, &url, jiami_ti.as_deref(), Some(&ewaiqingqiutou)).await?;
        jiemixiangying(&xiangying_wenben, xinxi.miyao)
    }

    pub async fn putongqingqiu(&self, fangfa: &str, lujing: &str, qingqiuti: Option<String>) -> Result<String, JsValue> {
        let url = format!("{}{}", self.fuwuqidizhi, lujing);
        putongqingqiu_neibu(fangfa, &url, qingqiuti.as_deref(), None).await
    }

    pub async fn jiankangqingqiu(&self) -> Result<String, JsValue> {
        let url = format!("{}{}", self.fuwuqidizhi, jiankangqq::lujing);
        let wenben = putongqingqiu_neibu(jiankangqq::fangshi, &url, None, None).await?;
        let xiangying: jiankangqq::Xiangying = fanxuliehua(&wenben, "解析健康检查响应失败")?;
        xuliehua(&xiangying)
    }

    pub async fn jiamijiankangqingqiu(&self, neirong: Option<String>) -> Result<String, JsValue> {
        let ti = xuliehua(&jiamijiankangqq::Qingqiuti { neirong })?;
        let jiemi_wenben = self.jiamiqingqiu(jiamijiankangqq::fangshi, jiamijiankangqq::lujing, Some(ti)).await?;
        let xiangying: jiamijiankangqq::Xiangying = fanxuliehua(&jiemi_wenben, "解析加密测试响应失败")?;
        xuliehua(&xiangying)
    }

    pub async fn ssejiamiqingqiu(&self, lujing: &str, qingqiuti: Option<String>, huidiaohanming: &str) -> Result<(), JsValue> {
        let xinxi = self.huoqujiamixinxi()?;
        let jiami_ti = qingqiuti.map(|ti| jiamiqingqiuti(&ti, xinxi.miyao)).transpose()?;
        let url = format!("{}{}", self.fuwuqidizhi, lujing);
        let ewaiqingqiutou = vec![
            ("X-Huihua-Id", xinxi.huihuaid),
            ("X-Kehugongyao", xinxi.kehugongyao),
        ];
        let request = goujianqingqiu("POST", &url, jiami_ti.as_deref(), Some(&ewaiqingqiutou))?;
        let xiangying = fasongqingqiu(&request).await?;
        let liuti = xiangying.body().ok_or_else(|| cuowu("无响应体"))?;
        let duquqi: ReadableStreamDefaultReader = liuti.get_reader().dyn_into()?;
        let quanju = js_sys::global();
        let huidiao_fn: js_sys::Function = js_sys::Reflect::get(&quanju, &JsValue::from_str(huidiaohanming))
            .map_err(|_| cuowu("找不到回调函数"))?
            .dyn_into()
            .map_err(|_| cuowu("回调不是函数"))?;
        let jiemaqi = web_sys::TextDecoder::new().map_err(|_| cuowu("创建解码器失败"))?;
        loop {
            let jieguo = JsFuture::from(duquqi.read()).await?;
            let wancheng = js_sys::Reflect::get(&jieguo, &JsValue::from_str("done"))
                .unwrap_or(JsValue::TRUE);
            if wancheng.is_truthy() {
                break;
            }
            if let Some(shuzhu) = js_sys::Reflect::get(&jieguo, &JsValue::from_str("value")).ok().filter(|v| !v.is_undefined()) {
                let wenben = jiemaqi.decode_with_buffer_source(&Uint8Array::new(&shuzhu)).unwrap_or_default();
                let _ = huidiao_fn.call1(&JsValue::NULL, &JsValue::from_str(&wenben));
            }
        }
        Ok(())
    }

    pub fn chongzhihuihua(&mut self) {
        self.huihuaid = None;
        self.miyao = None;
        self.kehugongyao_b64 = None;
    }

    pub fn yixieshang(&self) -> bool {
        self.huihuaid.is_some() && self.miyao.is_some()
    }
}

impl Kehuduanjiami {
    fn huoqujiamixinxi(&self) -> Result<Jiamixinxi<'_>, JsValue> {
        Ok(Jiamixinxi {
            miyao: self.miyao.as_ref().ok_or_else(|| cuowu("尚未协商密钥"))?,
            huihuaid: self.huihuaid.as_deref().ok_or_else(|| cuowu("尚未协商密钥"))?,
            kehugongyao: self.kehugongyao_b64.as_deref().ok_or_else(|| cuowu("尚未协商密钥"))?,
        })
    }
}
