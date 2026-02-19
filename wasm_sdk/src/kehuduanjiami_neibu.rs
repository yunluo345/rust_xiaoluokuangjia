use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use crate::jiami_gongju::{Jiamixinxi, cuowu, xuliehua, fanxuliehua, jiamiqingqiuti, jiemixiangying, shifouhuihuaguoqi};
use crate::http_gongju::{goujianqingqiu, fasongqingqiu, putongqingqiu_neibu, duqujiamiliushi};
use crate::jiekou_nr::xitong::miyaojiaohuanjiekou as miyaojiekou;
use crate::jiamihexin;

pub struct Kehuduanjiami {
    pub fuwuqidizhi: String,
    pub huihuaid: Option<String>,
    pub miyao: Option<Vec<u8>>,
    pub kehugongyao_b64: Option<String>,
    pub zhiwen: String,
    pub lingpai: Option<String>,
}

impl Kehuduanjiami {
    pub fn huoqujiamixinxi(&self) -> Result<Jiamixinxi<'_>, JsValue> {
        Ok(Jiamixinxi {
            miyao: self.miyao.as_ref().ok_or_else(|| cuowu("尚未协商密钥"))?,
            huihuaid: self.huihuaid.as_deref().ok_or_else(|| cuowu("尚未协商密钥"))?,
            kehugongyao: self.kehugongyao_b64.as_deref().ok_or_else(|| cuowu("尚未协商密钥"))?,
        })
    }

    pub fn baocunlingpai(&self) {
        if let Some(ref lingpai) = self.lingpai {
            if let Some(chuangkou) = web_sys::window() {
                if let Ok(Some(cunchu)) = chuangkou.local_storage() {
                    let _ = cunchu.set_item("lingpai", lingpai);
                }
            }
        }
    }

    pub fn huifulingpai(&mut self) {
        if let Some(chuangkou) = web_sys::window() {
            if let Ok(Some(cunchu)) = chuangkou.local_storage() {
                if let Ok(Some(lingpai)) = cunchu.get_item("lingpai") {
                    if !lingpai.is_empty() {
                        self.lingpai = Some(lingpai);
                    }
                }
            }
        }
    }

    #[allow(dead_code)]
    pub fn qingchulingpai(&self) {
        if let Some(chuangkou) = web_sys::window() {
            if let Ok(Some(cunchu)) = chuangkou.local_storage() {
                let _ = cunchu.remove_item("lingpai");
            }
        }
    }

    pub async fn xieshangmiyao(&mut self) -> Result<(), JsValue> {
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

    pub async fn quebaoxieshang(&mut self) -> Result<(), JsValue> {
        if !self.yixieshang() {
            self.xieshangmiyao().await?;
        }
        Ok(())
    }

    pub fn yixieshang(&self) -> bool {
        self.miyao.is_some() && self.huihuaid.is_some()
    }

    pub fn chongzhihuihua(&mut self) {
        self.huihuaid = None;
        self.miyao = None;
        self.kehugongyao_b64 = None;
    }

    pub async fn zhixingjiamiqingqiu(&self, fangfa: &str, lujing: &str, qingqiuti: Option<&str>) -> Result<String, JsValue> {
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

    pub fn xuyaochongshi(&self, jieguo: &Result<String, JsValue>) -> bool {
        match jieguo {
            Err(e) => e.as_string().map_or(false, |s| s.contains("解密响应失败") || s.contains("base64解码失败")),
            Ok(wenben) => shifouhuihuaguoqi(wenben),
        }
    }

    pub async fn chongxinxieshang(&mut self) -> Result<(), JsValue> {
        self.chongzhihuihua();
        self.xieshangmiyao().await
    }

    pub async fn zhixingrenzhengjiamiqingqiu(&self, fangfa: &str, lujing: &str, qingqiuti: Option<&str>, lingpai: &str) -> Result<String, JsValue> {
        let xinxi = self.huoqujiamixinxi()?;
        let jiami_ti = qingqiuti.map(|ti| jiamiqingqiuti(ti, xinxi.miyao)).transpose()?;
        let url = format!("{}{}", self.fuwuqidizhi, lujing);
        let auth_header = format!("Bearer {}", lingpai);
        let ewaiqingqiutou = vec![
            ("X-Huihua-Id", xinxi.huihuaid),
            ("X-Kehugongyao", xinxi.kehugongyao),
            ("Authorization", auth_header.as_str()),
        ];
        let xiangying_wenben = putongqingqiu_neibu(fangfa, &url, jiami_ti.as_deref(), Some(&ewaiqingqiutou)).await?;
        jiemixiangying(&xiangying_wenben, xinxi.miyao)
    }

    #[allow(dead_code)]
    pub async fn zhixingrenzhengjiamiqingqiu_with_abort(&self, fangfa: &str, lujing: &str, qingqiuti: Option<&str>, lingpai: &str, abort_signal: &web_sys::AbortSignal) -> Result<String, JsValue> {
        let xinxi = self.huoqujiamixinxi()?;
        let jiami_ti = qingqiuti.map(|ti| jiamiqingqiuti(ti, xinxi.miyao)).transpose()?;
        let url = format!("{}{}", self.fuwuqidizhi, lujing);
        let auth_header = format!("Bearer {}", lingpai);
        let ewaiqingqiutou = vec![
            ("X-Huihua-Id", xinxi.huihuaid),
            ("X-Kehugongyao", xinxi.kehugongyao),
            ("Authorization", auth_header.as_str()),
        ];
        let request = goujianqingqiu(fangfa, &url, jiami_ti.as_deref(), Some(&ewaiqingqiutou), Some(abort_signal))?;
        let xiangying = fasongqingqiu(&request).await?;
        let wenben = JsFuture::from(xiangying.text()?).await?;
        let xiangying_wenben = wenben.as_string().ok_or_else(|| cuowu("响应不是文本"))?;
        jiemixiangying(&xiangying_wenben, xinxi.miyao)
    }

    pub async fn zhixingssejiamiqingqiu(&self, lujing: &str, qingqiuti: Option<&str>, huidiao: &js_sys::Function) -> Result<(), JsValue> {
        let xinxi = self.huoqujiamixinxi()?;
        let jiami_ti = qingqiuti.map(|ti| jiamiqingqiuti(ti, xinxi.miyao)).transpose()?;
        let url = format!("{}{}", self.fuwuqidizhi, lujing);
        let ewaiqingqiutou = vec![
            ("X-Huihua-Id", xinxi.huihuaid),
            ("X-Kehugongyao", xinxi.kehugongyao),
        ];
        let request = goujianqingqiu("POST", &url, jiami_ti.as_deref(), Some(&ewaiqingqiutou), None)?;
        let xiangying = fasongqingqiu(&request).await?;
        duqujiamiliushi(&xiangying, xinxi.miyao, huidiao, None).await
    }

    #[allow(dead_code)]
    pub async fn zhixingsserenzhengjiamiqingqiu_with_abort(&self, lujing: &str, qingqiuti: Option<&str>, huidiao: &js_sys::Function, lingpai: &str, abort_signal: &web_sys::AbortSignal) -> Result<(), JsValue> {
        let xinxi = self.huoqujiamixinxi()?;
        let jiami_ti = qingqiuti.map(|ti| jiamiqingqiuti(ti, xinxi.miyao)).transpose()?;
        let url = format!("{}{}", self.fuwuqidizhi, lujing);
        let auth_header = format!("Bearer {}", lingpai);
        let ewaiqingqiutou = vec![
            ("X-Huihua-Id", xinxi.huihuaid),
            ("X-Kehugongyao", xinxi.kehugongyao),
            ("Authorization", auth_header.as_str()),
        ];
        let request = goujianqingqiu("POST", &url, jiami_ti.as_deref(), Some(&ewaiqingqiutou), Some(abort_signal))?;
        let xiangying = fasongqingqiu(&request).await?;
        duqujiamiliushi(&xiangying, xinxi.miyao, huidiao, None).await
    }

    pub async fn zhixingrenzhengputongqingqiu_with_abort(&self, fangfa: &str, lujing: &str, qingqiuti: Option<&str>, lingpai: &str, abort_signal: &web_sys::AbortSignal) -> Result<String, JsValue> {
        let url = format!("{}{}", self.fuwuqidizhi, lujing);
        let auth_header = format!("Bearer {}", lingpai);
        let ewaiqingqiutou = vec![
            ("Authorization", auth_header.as_str()),
        ];
        let request = goujianqingqiu(fangfa, &url, qingqiuti, Some(&ewaiqingqiutou), Some(abort_signal))?;
        let xiangying = fasongqingqiu(&request).await?;
        let wenben = JsFuture::from(xiangying.text()?).await?;
        wenben.as_string().ok_or_else(|| cuowu("响应不是文本"))
    }

    pub async fn zhixingsserenzhengputongqingqiu_with_abort(&self, lujing: &str, qingqiuti: Option<&str>, huidiao: &js_sys::Function, lingpai: &str, abort_signal: &web_sys::AbortSignal) -> Result<(), JsValue> {
        let url = format!("{}{}", self.fuwuqidizhi, lujing);
        let auth_header = format!("Bearer {}", lingpai);
        let ewaiqingqiutou = vec![
            ("Authorization", auth_header.as_str()),
        ];
        let request = goujianqingqiu("POST", &url, qingqiuti, Some(&ewaiqingqiutou), Some(abort_signal))?;
        let xiangying = fasongqingqiu(&request).await?;
        crate::http_gongju::duquliushi(&xiangying, huidiao).await
    }
}
