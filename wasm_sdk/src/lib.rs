#![allow(non_upper_case_globals)]

mod jiamihexin;
mod shebeishibie;
pub mod jiekou_nr;

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response, ReadableStreamDefaultReader};
use js_sys::Uint8Array;
use jiekou_nr::xitong::miyaojiaohuanjiekou as miyaojiekou;
use jiekou_nr::xitong::jiankangqingqiu as jiankangqq;
use jiekou_nr::xitong::jiamijiankang as jiamijiankangqq;
use jiekou_nr::xitong::sseceshi as sseceshiqq;
use jiekou_nr::xitong::jiamisseceshi as jiamisseceshiqq;
use jiekou_nr::yonghu::denglujiekou as dengluqq;

const huihua_guoqi_zhuangtaima: u16 = 401;

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

fn shifouhuihuaguoqi(xiangying_wenben: &str) -> bool {
    serde_json::from_str::<serde_json::Value>(xiangying_wenben)
        .ok()
        .and_then(|v| v.get("zhuangtaima")?.as_u64())
        .map_or(false, |ma| ma == huihua_guoqi_zhuangtaima as u64)
}

fn huoquhuidiao(hanming: &str) -> Result<js_sys::Function, JsValue> {
    js_sys::Reflect::get(&js_sys::global(), &JsValue::from_str(hanming))
        .map_err(|_| cuowu("找不到回调函数"))?
        .dyn_into()
        .map_err(|_| cuowu("回调不是函数"))
}

async fn duquliugushu(xiangying: &Response, chuli: impl Fn(&str) -> Result<(), JsValue>) -> Result<(), JsValue> {
    let liuti = xiangying.body().ok_or_else(|| cuowu("无响应体"))?;
    let duquqi: ReadableStreamDefaultReader = liuti.get_reader().dyn_into()?;
    let jiemaqi = web_sys::TextDecoder::new().map_err(|_| cuowu("创建解码器失败"))?;
    loop {
        let jieguo = JsFuture::from(duquqi.read()).await?;
        if js_sys::Reflect::get(&jieguo, &JsValue::from_str("done")).unwrap_or(JsValue::TRUE).is_truthy() {
            break;
        }
        if let Some(shuzhu) = js_sys::Reflect::get(&jieguo, &JsValue::from_str("value")).ok().filter(|v| !v.is_undefined()) {
            let wenben = jiemaqi.decode_with_buffer_source(&Uint8Array::new(&shuzhu)).unwrap_or_default();
            chuli(&wenben)?;
        }
    }
    Ok(())
}

async fn duquliushi(xiangying: &Response, huidiao: &js_sys::Function) -> Result<(), JsValue> {
    duquliugushu(xiangying, |wenben| {
        let _ = huidiao.call1(&JsValue::NULL, &JsValue::from_str(wenben));
        Ok(())
    }).await
}

async fn duqujiamiliushi(xiangying: &Response, miyao: &[u8], huidiao: &js_sys::Function) -> Result<(), JsValue> {
    duquliugushu(xiangying, |yuanwen| {
        for hang in yuanwen.split("data: ").filter(|s| !s.is_empty()) {
            let miwen = hang.trim();
            if miwen.is_empty() { continue; }
            let mingwen = jiemixiangying(miwen, miyao)?;
            let _ = huidiao.call1(&JsValue::NULL, &JsValue::from_str(&format!("data: {}\n\n", mingwen)));
        }
        Ok(())
    }).await
}

#[wasm_bindgen]
pub struct Kehuduanjiami {
    fuwuqidizhi: String,
    huihuaid: Option<String>,
    miyao: Option<Vec<u8>>,
    kehugongyao_b64: Option<String>,
    zhiwen: String,
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
            zhiwen: shebeishibie::shengchengzhiwen(),
        }
    }

    pub async fn jiamiqingqiu(&mut self, fangfa: &str, lujing: &str, qingqiuti: Option<String>) -> Result<String, JsValue> {
        self.quebaoxieshang().await?;
        let jieguo = self.zhixingjiamiqingqiu(fangfa, lujing, qingqiuti.as_deref()).await;
        if self.xuyaochongshi(&jieguo) {
            self.chongxinxieshang().await?;
            return self.zhixingjiamiqingqiu(fangfa, lujing, qingqiuti.as_deref()).await;
        }
        jieguo
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

    pub async fn jiamijiankangqingqiu(&mut self, neirong: Option<String>) -> Result<String, JsValue> {
        self.quebaoxieshang().await?;
        let ti = xuliehua(&jiamijiankangqq::Qingqiuti { neirong })?;
        let jieguo = self.zhixingjiamiqingqiu(jiamijiankangqq::fangshi, jiamijiankangqq::lujing, Some(&ti)).await;
        let jiemi_wenben = if self.xuyaochongshi(&jieguo) {
            self.chongxinxieshang().await?;
            self.zhixingjiamiqingqiu(jiamijiankangqq::fangshi, jiamijiankangqq::lujing, Some(&ti)).await?
        } else {
            jieguo?
        };
        let xiangying: jiamijiankangqq::Xiangying = fanxuliehua(&jiemi_wenben, "解析加密测试响应失败")?;
        xuliehua(&xiangying)
    }

    pub async fn sseputongqingqiu(&self, fangfa: &str, lujing: &str, huidiaohanming: &str) -> Result<(), JsValue> {
        let huidiao = huoquhuidiao(huidiaohanming)?;
        let url = format!("{}{}", self.fuwuqidizhi, lujing);
        let request = goujianqingqiu(fangfa, &url, None, None)?;
        let xiangying = fasongqingqiu(&request).await?;
        duquliushi(&xiangying, &huidiao).await
    }

    pub async fn ssejiamiqingqiu(&mut self, lujing: &str, qingqiuti: Option<String>, huidiaohanming: &str) -> Result<(), JsValue> {
        self.quebaoxieshang().await?;
        let huidiao = huoquhuidiao(huidiaohanming)?;
        let xinxi = self.huoqujiamixinxi()?;
        let jiami_ti = qingqiuti.map(|ti| jiamiqingqiuti(&ti, xinxi.miyao)).transpose()?;
        let url = format!("{}{}", self.fuwuqidizhi, lujing);
        let ewaiqingqiutou = vec![
            ("X-Huihua-Id", xinxi.huihuaid),
            ("X-Kehugongyao", xinxi.kehugongyao),
        ];
        let request = goujianqingqiu("POST", &url, jiami_ti.as_deref(), Some(&ewaiqingqiutou))?;
        let xiangying = fasongqingqiu(&request).await?;
        duqujiamiliushi(&xiangying, xinxi.miyao, &huidiao).await
    }

    pub async fn sseceshiqingqiu(&self, huidiaohanming: &str) -> Result<(), JsValue> {
        self.sseputongqingqiu(sseceshiqq::fangshi, sseceshiqq::lujing, huidiaohanming).await
    }

    pub async fn jiamisseceshiqingqiu(&mut self, huidiaohanming: &str) -> Result<(), JsValue> {
        self.ssejiamiqingqiu(jiamisseceshiqq::lujing, None, huidiaohanming).await
    }

    pub async fn dengluqingqiu(&mut self, zhanghao: &str, mima: &str) -> Result<String, JsValue> {
        self.quebaoxieshang().await?;
        let ti = xuliehua(&dengluqq::Qingqiuti { zhanghao: zhanghao.to_string(), mima: mima.to_string() })?;
        let jieguo = self.zhixingjiamiqingqiu(dengluqq::fangshi, dengluqq::lujing, Some(&ti)).await;
        let jiemi_wenben = if self.xuyaochongshi(&jieguo) {
            self.chongxinxieshang().await?;
            self.zhixingjiamiqingqiu(dengluqq::fangshi, dengluqq::lujing, Some(&ti)).await?
        } else {
            jieguo?
        };
        let xiangying: dengluqq::Xiangying = fanxuliehua(&jiemi_wenben, "解析登录响应失败")?;
        xuliehua(&xiangying)
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

    async fn xieshangmiyao(&mut self) -> Result<(), JsValue> {
        let url = format!("{}{}", self.fuwuqidizhi, miyaojiekou::lujing);
        let ti = xuliehua(&miyaojiekou::Qingqiuti { zhiwen: self.zhiwen.clone() })?;
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

    async fn quebaoxieshang(&mut self) -> Result<(), JsValue> {
        if !self.yixieshang() {
            self.xieshangmiyao().await?;
        }
        Ok(())
    }

    async fn zhixingjiamiqingqiu(&self, fangfa: &str, lujing: &str, qingqiuti: Option<&str>) -> Result<String, JsValue> {
        let xinxi = self.huoqujiamixinxi()?;
        let jiami_ti = qingqiuti.map(|ti| jiamiqingqiuti(ti, xinxi.miyao)).transpose()?;
        let url = format!("{}{}", self.fuwuqidizhi, lujing);
        let ewaiqingqiutou = vec![
            ("X-Huihua-Id", xinxi.huihuaid),
            ("X-Kehugongyao", xinxi.kehugongyao),
        ];
        let xiangying_wenben = putongqingqiu_neibu(fangfa, &url, jiami_ti.as_deref(), Some(&ewaiqingqiutou)).await?;
        jiemixiangying(&xiangying_wenben, xinxi.miyao)
    }

    fn xuyaochongshi(&self, jieguo: &Result<String, JsValue>) -> bool {
        match jieguo {
            Err(e) => e.as_string().map_or(false, |s| s.contains("解密响应失败")),
            Ok(wenben) => shifouhuihuaguoqi(wenben),
        }
    }

    async fn chongxinxieshang(&mut self) -> Result<(), JsValue> {
        self.chongzhihuihua();
        self.xieshangmiyao().await
    }
}
