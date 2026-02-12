mod jiamihexin;
pub mod jiekou_nr;

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response, ReadableStreamDefaultReader};
use js_sys::Uint8Array;
use jiekou_nr::miyaojiaohuanjiekou as miyaojiekou;
use jiekou_nr::jiankangqingqiu as jiankangqq;
use jiekou_nr::jiamijiankang as jiamijiankangqq;

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

    fn yijingxieshang(&self) -> bool {
        self.huihuaid.is_some() && self.miyao.is_some()
    }

    pub async fn xieshangmiyao(&mut self, zhiwen: &str) -> Result<(), JsValue> {
        let url = format!("{}{}", self.fuwuqidizhi, miyaojiekou::lujing);
        let qingqiuti = miyaojiekou::Qingqiuti { zhiwen: zhiwen.to_string() };
        let ti = serde_json::to_string(&qingqiuti).map_err(|e| JsValue::from_str(&format!("序列化失败: {}", e)))?;
        let xiangying_wenben = putongqingqiu_neibu(miyaojiekou::fangshi, &url, Some(&ti), None).await?;
        let xiangying: miyaojiekou::Xiangying = serde_json::from_str(&xiangying_wenben)
            .map_err(|e| JsValue::from_str(&format!("解析响应失败: {}", e)))?;
        let shuju = xiangying.shuju.ok_or_else(|| JsValue::from_str("服务端未返回公钥数据"))?;
        let fuwuqigongyao = jiamihexin::congbase64(&shuju.gongyao)
            .ok_or_else(|| JsValue::from_str("服务端公钥base64解码失败"))?;
        let (kehusiyao, kehugongyao) = jiamihexin::shengchengyaodui();
        let gongxiangyao = jiamihexin::xieshanggongxiangyao(&kehusiyao, &fuwuqigongyao)
            .ok_or_else(|| JsValue::from_str("ECDH协商失败"))?;
        let miyao = jiamihexin::paishengyao(&gongxiangyao, jiamihexin::yanfen)
            .ok_or_else(|| JsValue::from_str("密钥派生失败"))?;
        self.huihuaid = Some(shuju.huihuaid);
        self.miyao = Some(miyao);
        self.kehugongyao_b64 = Some(jiamihexin::zhuanbase64(&kehugongyao));
        Ok(())
    }

    pub async fn jiamiqingqiu(&self, fangfa: &str, lujing: &str, qingqiuti: Option<String>) -> Result<String, JsValue> {
        let miyao = self.miyao.as_ref().ok_or_else(|| JsValue::from_str("尚未协商密钥"))?;
        let huihuaid = self.huihuaid.as_ref().ok_or_else(|| JsValue::from_str("尚未协商密钥"))?;
        let kehugongyao = self.kehugongyao_b64.as_ref().ok_or_else(|| JsValue::from_str("尚未协商密钥"))?;
        let jiami_ti = qingqiuti.map(|ti| {
            let miwen = jiamihexin::jiami(ti.as_bytes(), miyao)
                .ok_or_else(|| JsValue::from_str("加密请求体失败"))?;
            Ok::<String, JsValue>(jiamihexin::zhuanbase64(&miwen))
        }).transpose()?;
        let url = format!("{}{}", self.fuwuqidizhi, lujing);
        let ewaiqingqiutou = vec![
            ("X-Huihua-Id", huihuaid.as_str()),
            ("X-Kehugongyao", kehugongyao.as_str()),
        ];
        let xiangying_wenben = putongqingqiu_neibu(fangfa, &url, jiami_ti.as_deref(), Some(&ewaiqingqiutou)).await?;
        let miwen_zijie = jiamihexin::congbase64(&xiangying_wenben)
            .ok_or_else(|| JsValue::from_str("响应base64解码失败"))?;
        let mingwen = jiamihexin::jiemi(&miwen_zijie, miyao)
            .ok_or_else(|| JsValue::from_str("解密响应失败"))?;
        String::from_utf8(mingwen).map_err(|_| JsValue::from_str("响应不是有效UTF-8"))
    }

    pub async fn putongqingqiu(&self, fangfa: &str, lujing: &str, qingqiuti: Option<String>) -> Result<String, JsValue> {
        let url = format!("{}{}", self.fuwuqidizhi, lujing);
        putongqingqiu_neibu(fangfa, &url, qingqiuti.as_deref(), None).await
    }

    pub async fn jiankangqingqiu(&self) -> Result<String, JsValue> {
        let url = format!("{}{}", self.fuwuqidizhi, jiankangqq::lujing);
        let wenben = putongqingqiu_neibu(jiankangqq::fangshi, &url, None, None).await?;
        let xiangying: jiankangqq::Xiangying = serde_json::from_str(&wenben)
            .map_err(|e| JsValue::from_str(&format!("解析健康检查响应失败: {}", e)))?;
        serde_json::to_string(&serde_json::json!({
            "zhuangtaima": xiangying.zhuangtaima,
            "xiaoxi": xiangying.xiaoxi,
            "shuju": xiangying.shuju.map(|s| s.zhuangtai)
        })).map_err(|e| JsValue::from_str(&format!("序列化失败: {}", e)))
    }

    pub async fn jiamijiankangqingqiu(&self, neirong: Option<String>) -> Result<String, JsValue> {
        let ti = serde_json::to_string(&jiamijiankangqq::Qingqiuti { neirong })
            .map_err(|e| JsValue::from_str(&format!("序列化失败: {}", e)))?;
        let jiemi_wenben = self.jiamiqingqiu(jiamijiankangqq::fangshi, jiamijiankangqq::lujing, Some(ti)).await?;
        let xiangying: jiamijiankangqq::Xiangying = serde_json::from_str(&jiemi_wenben)
            .map_err(|e| JsValue::from_str(&format!("解析加密测试响应失败: {}", e)))?;
        serde_json::to_string(&serde_json::json!({
            "zhuangtaima": xiangying.zhuangtaima,
            "xiaoxi": xiangying.xiaoxi,
            "shuju": xiangying.shuju.map(|s| serde_json::json!({
                "huifu": s.huifu,
                "yuanshishuju": s.yuanshishuju
            }))
        })).map_err(|e| JsValue::from_str(&format!("序列化失败: {}", e)))
    }

    pub async fn ssejiamiqingqiu(&self, lujing: &str, qingqiuti: Option<String>, huidiaohanming: &str) -> Result<(), JsValue> {
        let miyao = self.miyao.as_ref().ok_or_else(|| JsValue::from_str("尚未协商密钥"))?;
        let huihuaid = self.huihuaid.as_ref().ok_or_else(|| JsValue::from_str("尚未协商密钥"))?;
        let kehugongyao = self.kehugongyao_b64.as_ref().ok_or_else(|| JsValue::from_str("尚未协商密钥"))?;
        let jiami_ti = qingqiuti.map(|ti| {
            let miwen = jiamihexin::jiami(ti.as_bytes(), miyao)
                .ok_or_else(|| JsValue::from_str("加密请求体失败"))?;
            Ok::<String, JsValue>(jiamihexin::zhuanbase64(&miwen))
        }).transpose()?;
        let url = format!("{}{}", self.fuwuqidizhi, lujing);
        let opts = RequestInit::new();
        opts.set_method("POST");
        opts.set_mode(RequestMode::Cors);
        if let Some(ref ti) = jiami_ti {
            opts.set_body(&JsValue::from_str(ti));
        }
        let request = Request::new_with_str_and_init(&url, &opts)
            .map_err(|_| JsValue::from_str("创建请求失败"))?;
        let toubu = request.headers();
        toubu.set("Content-Type", "application/json").ok();
        toubu.set("X-Huihua-Id", huihuaid).ok();
        toubu.set("X-Kehugongyao", kehugongyao).ok();
        let chuangkou = web_sys::window().ok_or_else(|| JsValue::from_str("无法获取window"))?;
        let xiangying: Response = JsFuture::from(chuangkou.fetch_with_request(&request)).await?.dyn_into()?;
        let liuti = xiangying.body().ok_or_else(|| JsValue::from_str("无响应体"))?;
        let duquqi: ReadableStreamDefaultReader = liuti.get_reader().dyn_into()?;
        let quanju = js_sys::global();
        let huidiao_fn: js_sys::Function = js_sys::Reflect::get(&quanju, &JsValue::from_str(huidiaohanming))
            .map_err(|_| JsValue::from_str("找不到回调函数"))?
            .dyn_into()
            .map_err(|_| JsValue::from_str("回调不是函数"))?;
        let jiemaqi = web_sys::TextDecoder::new().map_err(|_| JsValue::from_str("创建解码器失败"))?;
        loop {
            let jieguo = JsFuture::from(duquqi.read()).await?;
            let wancheng = js_sys::Reflect::get(&jieguo, &JsValue::from_str("done"))
                .unwrap_or(JsValue::TRUE);
            if wancheng.is_truthy() {
                break;
            }
            let zhi = js_sys::Reflect::get(&jieguo, &JsValue::from_str("value"))
                .ok().filter(|v| !v.is_undefined());
            if let Some(shuzhu) = zhi {
                let uint8 = Uint8Array::new(&shuzhu);
                let wenben = jiemaqi.decode_with_buffer_source(&uint8)
                    .unwrap_or_default();
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
        self.yijingxieshang()
    }
}

async fn putongqingqiu_neibu(fangfa: &str, url: &str, ti: Option<&str>, ewaiqingqiutou: Option<&[(&str, &str)]>) -> Result<String, JsValue> {
    let opts = RequestInit::new();
    opts.set_method(fangfa);
    opts.set_mode(RequestMode::Cors);
    if let Some(neirong) = ti {
        opts.set_body(&JsValue::from_str(neirong));
    }
    let request = Request::new_with_str_and_init(url, &opts)
        .map_err(|_| JsValue::from_str("创建请求失败"))?;
    let toubu = request.headers();
    toubu.set("Content-Type", "application/json").ok();
    if let Some(tou_lie) = ewaiqingqiutou {
        for (ming, zhi) in tou_lie {
            toubu.set(ming, zhi).ok();
        }
    }
    let chuangkou = web_sys::window().ok_or_else(|| JsValue::from_str("无法获取window"))?;
    let xiangying: Response = JsFuture::from(chuangkou.fetch_with_request(&request)).await?.dyn_into()?;
    let wenben = JsFuture::from(xiangying.text()?).await?;
    wenben.as_string().ok_or_else(|| JsValue::from_str("响应不是文本"))
}
