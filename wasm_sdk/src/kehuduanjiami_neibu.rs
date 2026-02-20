use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use std::rc::Rc;
use std::cell::RefCell;
use crate::jiami_gongju::{Jiamixinxi, cuowu, xuliehua, fanxuliehua, jiamiqingqiuti, jiemixiangying, shifouhuihuaguoqi};
use crate::http_gongju::{goujianqingqiu, fasongqingqiu, putongqingqiu_neibu, duqujiamiliushi};
use crate::jiekou_nr::xitong::miyaojiaohuanjiekou as miyaojiekou;
use crate::jiamihexin;

struct Zhuangtai {
    huihuaid: Option<String>,
    miyao: Option<Vec<u8>>,
    kehugongyao_b64: Option<String>,
    lingpai: Option<String>,
    fuwuqidizhi: String,
    zhiwen: String,
}

pub struct Kehuduanjiami {
    zhuangtai: Rc<RefCell<Zhuangtai>>,
}

impl Kehuduanjiami {
    pub fn xinjian(fuwuqidizhi: String, zhiwen: String) -> Self {
        Self {
            zhuangtai: Rc::new(RefCell::new(Zhuangtai {
                huihuaid: None,
                miyao: None,
                kehugongyao_b64: None,
                lingpai: None,
                fuwuqidizhi,
                zhiwen,
            })),
        }
    }

    pub fn huoqufuwuqidizhi(&self) -> String {
        self.zhuangtai.borrow().fuwuqidizhi.clone()
    }

    pub fn huoquzhiwen(&self) -> String {
        self.zhuangtai.borrow().zhiwen.clone()
    }

    pub fn shezhizhiwen(&self, zhiwen: String) {
        self.zhuangtai.borrow_mut().zhiwen = zhiwen;
    }

    pub fn shezhilingpai(&self, lingpai: String) {
        self.zhuangtai.borrow_mut().lingpai = Some(lingpai);
    }

    pub fn huoqulingpai(&self) -> Option<String> {
        self.zhuangtai.borrow().lingpai.clone()
    }

    pub fn huoquhuihuaid(&self) -> Option<String> {
        self.zhuangtai.borrow().huihuaid.clone()
    }

    pub fn qingchulingpai_neibu(&self) {
        self.zhuangtai.borrow_mut().lingpai = None;
    }

    pub fn shezhifuwuqidizhi(&self, dizhi: String) {
        self.zhuangtai.borrow_mut().fuwuqidizhi = dizhi;
    }

    pub fn miyaoshifoucunzai(&self) -> bool {
        self.zhuangtai.borrow().miyao.is_some()
    }

    pub fn huoqumiyaochangdu(&self) -> usize {
        self.zhuangtai.borrow().miyao.as_ref().map_or(0, |m| m.len())
    }

    pub fn huoqukehugongyao(&self) -> Option<String> {
        self.zhuangtai.borrow().kehugongyao_b64.clone()
    }

    pub fn huoqujiamixinxi(&self) -> Result<Jiamixinxi, JsValue> {
        let (miyao, huihuaid, kehugongyao) = {
            let zhuangtai = self.zhuangtai.borrow();
            (
                zhuangtai.miyao.as_ref().ok_or_else(|| cuowu("尚未协商密钥"))?.clone(),
                zhuangtai.huihuaid.as_ref().ok_or_else(|| cuowu("尚未协商密钥"))?.clone(),
                zhuangtai.kehugongyao_b64.as_ref().ok_or_else(|| cuowu("尚未协商密钥"))?.clone(),
            )
        };
        Ok(Jiamixinxi { miyao, huihuaid, kehugongyao })
    }

    pub fn baocunlingpai(&self) {
        let lingpai = self.zhuangtai.borrow().lingpai.clone();
        if let Some(lingpai) = lingpai {
            web_sys::window()
                .and_then(|chuangkou| chuangkou.local_storage().ok().flatten())
                .and_then(|cunchu| cunchu.set_item("lingpai", &lingpai).ok());
        }
    }

    pub fn huifulingpai(&self) {
        let lingpai = web_sys::window()
            .and_then(|chuangkou| chuangkou.local_storage().ok().flatten())
            .and_then(|cunchu| cunchu.get_item("lingpai").ok().flatten())
            .filter(|lingpai| !lingpai.is_empty());
        
        if let Some(lingpai) = lingpai {
            self.zhuangtai.borrow_mut().lingpai = Some(lingpai);
        }
    }

    #[allow(dead_code)]
    pub fn qingchulingpai(&self) {
        web_sys::window()
            .and_then(|chuangkou| chuangkou.local_storage().ok().flatten())
            .map(|cunchu| cunchu.remove_item("lingpai"));
    }

    pub async fn xieshangmiyao(&self) -> Result<(), JsValue> {
        let fuwuqidizhi = self.huoqufuwuqidizhi();
        let zhiwen = self.huoquzhiwen();
        let url = format!("{}{}", fuwuqidizhi, miyaojiekou::lujing);
        let ti = xuliehua(&miyaojiekou::Qingqiuti { zhiwen })?;
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
        
        let mut zhuangtai = self.zhuangtai.borrow_mut();
        zhuangtai.huihuaid = Some(shuju.huihuaid);
        zhuangtai.miyao = Some(miyao);
        zhuangtai.kehugongyao_b64 = Some(jiamihexin::zhuanbase64(&kehugongyao));
        Ok(())
    }

    pub async fn quebaoxieshang(&self) -> Result<(), JsValue> {
        if !self.yixieshang() {
            self.xieshangmiyao().await?;
        }
        Ok(())
    }

    pub fn yixieshang(&self) -> bool {
        let (miyao, huihuaid) = {
            let zhuangtai = self.zhuangtai.borrow();
            (zhuangtai.miyao.is_some(), zhuangtai.huihuaid.is_some())
        };
        miyao && huihuaid
    }

    pub fn chongzhihuihua(&self) {
        {
            let mut zhuangtai = self.zhuangtai.borrow_mut();
            zhuangtai.huihuaid = None;
            zhuangtai.miyao = None;
            zhuangtai.kehugongyao_b64 = None;
        }
    }

    pub async fn zhixingjiamiqingqiu(&self, fangfa: &str, lujing: &str, qingqiuti: Option<&str>) -> Result<String, JsValue> {
        let xinxi = self.huoqujiamixinxi()?;
        let jiami_ti = qingqiuti.map(|ti| jiamiqingqiuti(ti, &xinxi.miyao)).transpose()?;
        let url = format!("{}{}", self.huoqufuwuqidizhi(), lujing);
        let ewaiqingqiutou = vec![
            ("X-Huihua-Id", xinxi.huihuaid.as_str()),
            ("X-Kehugongyao", xinxi.kehugongyao.as_str()),
        ];
        let xiangying_wenben = putongqingqiu_neibu(fangfa, &url, jiami_ti.as_deref(), Some(&ewaiqingqiutou)).await?;
        jiemixiangying(&xiangying_wenben, &xinxi.miyao)
    }

    pub fn xuyaochongshi(&self, jieguo: &Result<String, JsValue>) -> bool {
        match jieguo {
            Err(e) => e.as_string().is_some_and(|s| s.contains("解密响应失败") || s.contains("base64解码失败")),
            Ok(wenben) => shifouhuihuaguoqi(wenben),
        }
    }

    pub async fn chongxinxieshang(&self) -> Result<(), JsValue> {
        self.chongzhihuihua();
        self.xieshangmiyao().await
    }

    pub async fn zhixingrenzhengjiamiqingqiu(&self, fangfa: &str, lujing: &str, qingqiuti: Option<&str>, lingpai: &str) -> Result<String, JsValue> {
        let xinxi = self.huoqujiamixinxi()?;
        let jiami_ti = qingqiuti.map(|ti| jiamiqingqiuti(ti, &xinxi.miyao)).transpose()?;
        let url = format!("{}{}", self.huoqufuwuqidizhi(), lujing);
        let auth_header = format!("Bearer {}", lingpai);
        let ewaiqingqiutou = vec![
            ("X-Huihua-Id", xinxi.huihuaid.as_str()),
            ("X-Kehugongyao", xinxi.kehugongyao.as_str()),
            ("Authorization", auth_header.as_str()),
        ];
        let xiangying_wenben = putongqingqiu_neibu(fangfa, &url, jiami_ti.as_deref(), Some(&ewaiqingqiutou)).await?;
        jiemixiangying(&xiangying_wenben, &xinxi.miyao)
    }

    #[allow(dead_code)]
    pub async fn zhixingrenzhengjiamiqingqiu_with_abort(&self, fangfa: &str, lujing: &str, qingqiuti: Option<&str>, lingpai: &str, abort_signal: &web_sys::AbortSignal) -> Result<String, JsValue> {
        let xinxi = self.huoqujiamixinxi()?;
        let jiami_ti = qingqiuti.map(|ti| jiamiqingqiuti(ti, &xinxi.miyao)).transpose()?;
        let url = format!("{}{}", self.huoqufuwuqidizhi(), lujing);
        let auth_header = format!("Bearer {}", lingpai);
        let ewaiqingqiutou = vec![
            ("X-Huihua-Id", xinxi.huihuaid.as_str()),
            ("X-Kehugongyao", xinxi.kehugongyao.as_str()),
            ("Authorization", auth_header.as_str()),
        ];
        let request = goujianqingqiu(fangfa, &url, jiami_ti.as_deref(), Some(&ewaiqingqiutou), Some(abort_signal))?;
        let xiangying = fasongqingqiu(&request).await?;
        let wenben = JsFuture::from(xiangying.text()?).await?;
        let xiangying_wenben = wenben.as_string().ok_or_else(|| cuowu("响应不是文本"))?;
        jiemixiangying(&xiangying_wenben, &xinxi.miyao)
    }

    pub async fn zhixingssejiamiqingqiu(&self, lujing: &str, qingqiuti: Option<&str>, huidiao: &js_sys::Function) -> Result<(), JsValue> {
        let xinxi = self.huoqujiamixinxi()?;
        let jiami_ti = qingqiuti.map(|ti| jiamiqingqiuti(ti, &xinxi.miyao)).transpose()?;
        let url = format!("{}{}", self.huoqufuwuqidizhi(), lujing);
        let ewaiqingqiutou = vec![
            ("X-Huihua-Id", xinxi.huihuaid.as_str()),
            ("X-Kehugongyao", xinxi.kehugongyao.as_str()),
        ];
        let request = goujianqingqiu("POST", &url, jiami_ti.as_deref(), Some(&ewaiqingqiutou), None)?;
        let xiangying = fasongqingqiu(&request).await?;
        duqujiamiliushi(&xiangying, &xinxi.miyao, huidiao, None).await
    }

    #[allow(dead_code)]
    pub async fn zhixingsserenzhengjiamiqingqiu_with_abort(&self, lujing: &str, qingqiuti: Option<&str>, huidiao: &js_sys::Function, lingpai: &str, abort_signal: &web_sys::AbortSignal) -> Result<(), JsValue> {
        let xinxi = self.huoqujiamixinxi()?;
        let jiami_ti = qingqiuti.map(|ti| jiamiqingqiuti(ti, &xinxi.miyao)).transpose()?;
        let url = format!("{}{}", self.huoqufuwuqidizhi(), lujing);
        let auth_header = format!("Bearer {}", lingpai);
        let ewaiqingqiutou = vec![
            ("X-Huihua-Id", xinxi.huihuaid.as_str()),
            ("X-Kehugongyao", xinxi.kehugongyao.as_str()),
            ("Authorization", auth_header.as_str()),
        ];
        let request = goujianqingqiu("POST", &url, jiami_ti.as_deref(), Some(&ewaiqingqiutou), Some(abort_signal))?;
        let xiangying = fasongqingqiu(&request).await?;
        duqujiamiliushi(&xiangying, &xinxi.miyao, huidiao, None).await
    }

    pub async fn zhixingrenzhengputongqingqiu_with_abort(&self, fangfa: &str, lujing: &str, qingqiuti: Option<&str>, lingpai: &str, abort_signal: &web_sys::AbortSignal) -> Result<String, JsValue> {
        let url = format!("{}{}", self.huoqufuwuqidizhi(), lujing);
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
        let url = format!("{}{}", self.huoqufuwuqidizhi(), lujing);
        let auth_header = format!("Bearer {}", lingpai);
        let ewaiqingqiutou = vec![
            ("Authorization", auth_header.as_str()),
        ];
        let request = goujianqingqiu("POST", &url, qingqiuti, Some(&ewaiqingqiutou), Some(abort_signal))?;
        let xiangying = fasongqingqiu(&request).await?;
        crate::http_gongju::duquliushi(&xiangying, huidiao).await
    }
}
