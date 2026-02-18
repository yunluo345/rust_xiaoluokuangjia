#![allow(non_upper_case_globals)]

mod jiamihexin;
mod shebeishibie;
mod http_gongju;
mod jiami_gongju;
mod kehuduanjiami_neibu;
pub mod jiekou_nr;

use wasm_bindgen::prelude::*;
use jiekou_nr::xitong::jiankangqingqiu as jiankangqq;
use jiekou_nr::xitong::jiamijiankang as jiamijiankangqq;
use jiekou_nr::xitong::sseceshi as sseceshiqq;
use jiekou_nr::xitong::jiamisseceshi as jiamisseceshiqq;
use jiekou_nr::xitong::aiqudaoguanli as aiqudaoqq;
use jiekou_nr::yonghu::denglujiekou as dengluqq;
use jiekou_nr::ai::duihua as aiduihuaqq;
use jiekou_nr::ai::duihualiushi as aiduihualiushiqq;

use jiami_gongju::{xuliehua, fanxuliehua};
use http_gongju::{putongqingqiu_neibu, huoquhuidiao, duquliushi};
use kehuduanjiami_neibu::Kehuduanjiami as KehuduanjiamiNeibu;

const huihua_guoqi_zhuangtaima: u16 = 401;

#[wasm_bindgen]
pub struct Kehuduanjiami {
    neibu: KehuduanjiamiNeibu,
}

#[wasm_bindgen]
impl Kehuduanjiami {
    #[wasm_bindgen(constructor)]
    pub fn xinjian(fuwuqidizhi: &str) -> Self {
        let mut neibu = KehuduanjiamiNeibu {
            fuwuqidizhi: fuwuqidizhi.to_string(),
            huihuaid: None,
            miyao: None,
            kehugongyao_b64: None,
            zhiwen: shebeishibie::shengchengzhiwen(),
            lingpai: None,
        };
        neibu.huifulingpai();
        Self { neibu }
    }

    pub async fn jiamiqingqiu(&mut self, fangfa: &str, lujing: &str, qingqiuti: Option<String>) -> Result<String, JsValue> {
        self.neibu.quebaoxieshang().await?;
        let jieguo = self.neibu.zhixingjiamiqingqiu(fangfa, lujing, qingqiuti.as_deref()).await;
        if self.neibu.xuyaochongshi(&jieguo) {
            self.neibu.chongxinxieshang().await?;
            return self.neibu.zhixingjiamiqingqiu(fangfa, lujing, qingqiuti.as_deref()).await;
        }
        jieguo
    }

    pub async fn putongqingqiu(&self, fangfa: &str, lujing: &str, qingqiuti: Option<String>) -> Result<String, JsValue> {
        let url = format!("{}{}", self.neibu.fuwuqidizhi, lujing);
        putongqingqiu_neibu(fangfa, &url, qingqiuti.as_deref(), None).await
    }

    pub async fn jiankangqingqiu(&self) -> Result<String, JsValue> {
        let url = format!("{}{}", self.neibu.fuwuqidizhi, jiankangqq::lujing);
        let wenben = putongqingqiu_neibu(jiankangqq::fangshi, &url, None, None).await?;
        let xiangying: jiankangqq::Xiangying = fanxuliehua(&wenben, "解析健康检查响应失败")?;
        xuliehua(&xiangying)
    }

    pub async fn jiamijiankangqingqiu(&mut self, neirong: Option<String>) -> Result<String, JsValue> {
        self.neibu.quebaoxieshang().await?;
        let ti = xuliehua(&jiamijiankangqq::Qingqiuti { neirong })?;
        let jieguo = self.neibu.zhixingjiamiqingqiu(jiamijiankangqq::fangshi, jiamijiankangqq::lujing, Some(&ti)).await;
        let jiemi_wenben = if self.neibu.xuyaochongshi(&jieguo) {
            self.neibu.chongxinxieshang().await?;
            self.neibu.zhixingjiamiqingqiu(jiamijiankangqq::fangshi, jiamijiankangqq::lujing, Some(&ti)).await?
        } else {
            jieguo?
        };
        let xiangying: jiamijiankangqq::Xiangying = fanxuliehua(&jiemi_wenben, "解析加密测试响应失败")?;
        xuliehua(&xiangying)
    }

    pub async fn sseputongqingqiu(&self, fangfa: &str, lujing: &str, huidiaohanming: &str) -> Result<(), JsValue> {
        let huidiao = huoquhuidiao(huidiaohanming)?;
        let url = format!("{}{}", self.neibu.fuwuqidizhi, lujing);
        let request = http_gongju::goujianqingqiu(fangfa, &url, None, None, None)?;
        let xiangying = http_gongju::fasongqingqiu(&request).await?;
        duquliushi(&xiangying, &huidiao).await
    }

    pub async fn ssejiamiqingqiu(&mut self, lujing: &str, qingqiuti: Option<String>, huidiaohanming: &str) -> Result<(), JsValue> {
        self.neibu.quebaoxieshang().await?;
        let huidiao = huoquhuidiao(huidiaohanming)?;
        let jieguo = self.neibu.zhixingssejiamiqingqiu(lujing, qingqiuti.as_deref(), &huidiao).await;
        if jieguo.as_ref().err().and_then(|e| e.as_string()).map_or(false, |s| s.contains("base64解码失败") || s.contains("解密响应失败")) {
            self.neibu.chongxinxieshang().await?;
            return self.neibu.zhixingssejiamiqingqiu(lujing, qingqiuti.as_deref(), &huidiao).await;
        }
        jieguo
    }

    pub async fn sseceshiqingqiu(&self, huidiaohanming: &str) -> Result<(), JsValue> {
        self.sseputongqingqiu(sseceshiqq::fangshi, sseceshiqq::lujing, huidiaohanming).await
    }

    pub async fn jiamisseceshiqingqiu(&mut self, huidiaohanming: &str) -> Result<(), JsValue> {
        self.ssejiamiqingqiu(jiamisseceshiqq::lujing, None, huidiaohanming).await
    }

    pub async fn dengluqingqiu(&mut self, zhanghao: &str, mima: &str) -> Result<String, JsValue> {
        self.neibu.quebaoxieshang().await?;
        let ti = xuliehua(&dengluqq::Qingqiuti { zhanghao: zhanghao.to_string(), mima: mima.to_string() })?;
        let jieguo = self.neibu.zhixingjiamiqingqiu(dengluqq::fangshi, dengluqq::lujing, Some(&ti)).await;
        let jiemi_wenben = if self.neibu.xuyaochongshi(&jieguo) {
            self.neibu.chongxinxieshang().await?;
            self.neibu.zhixingjiamiqingqiu(dengluqq::fangshi, dengluqq::lujing, Some(&ti)).await?
        } else {
            jieguo?
        };
        let xiangying: dengluqq::Xiangying = fanxuliehua(&jiemi_wenben, "解析登录响应失败")?;
        if xiangying.zhuangtaima == 200 {
            if let Some(ref shuju) = xiangying.shuju {
                self.neibu.lingpai = Some(shuju.lingpai.clone());
                self.neibu.baocunlingpai();
            }
        }
        xuliehua(&xiangying)
    }

    pub fn yixieshang(&self) -> bool {
        self.neibu.yixieshang()
    }

    pub fn yidenglu(&self) -> bool {
        self.neibu.lingpai.is_some()
    }

    pub fn chongzhihuihua(&mut self) {
        self.neibu.chongzhihuihua();
    }

    pub async fn aiqudaoqingqiu(&mut self, caozuo: &str, canshu: Option<String>) -> Result<String, JsValue> {
        let lingpai = self.neibu.lingpai.as_ref().ok_or_else(|| jiami_gongju::cuowu("尚未登录，请先登录"))?.clone();
        self.neibu.quebaoxieshang().await?;
        let canshu_zhi: serde_json::Value = match canshu {
            Some(ref s) => fanxuliehua(s, "解析参数失败")?,
            None => serde_json::json!({}),
        };
        let ti = xuliehua(&aiqudaoqq::Qingqiuti { caozuo: caozuo.to_string(), canshu: canshu_zhi })?;
        let jieguo = self.neibu.zhixingrenzhengjiamiqingqiu(aiqudaoqq::fangshi, aiqudaoqq::lujing, Some(&ti), &lingpai).await;
        let jiemi_wenben = if self.neibu.xuyaochongshi(&jieguo) {
            self.neibu.chongxinxieshang().await?;
            self.neibu.zhixingrenzhengjiamiqingqiu(aiqudaoqq::fangshi, aiqudaoqq::lujing, Some(&ti), &lingpai).await?
        } else {
            jieguo?
        };
        Ok(jiemi_wenben)
    }

    pub async fn aiduihuaqingqiu(&mut self, xiaoxilie_json: &str, baocunduquqi_hanming: &str) -> Result<String, JsValue> {
        let lingpai = self.neibu.lingpai.as_ref().ok_or_else(|| jiami_gongju::cuowu("未登录"))?.clone();
        self.neibu.quebaoxieshang().await?;
        
        let xiaoxilie: Vec<aiduihuaqq::Xiaoxi> = fanxuliehua(xiaoxilie_json, "解析消息列表失败")?;
        let ti = xuliehua(&aiduihuaqq::Qingqiuti { xiaoxilie })?;
        
        let baocunduquqi = huoquhuidiao(baocunduquqi_hanming)?;
        let abort_controller = web_sys::AbortController::new().map_err(|_| jiami_gongju::cuowu("创建 AbortController 失败"))?;
        let abort_signal = abort_controller.signal();
        let _ = baocunduquqi.call1(&JsValue::NULL, &abort_controller);
        
        let jieguo = self.neibu.zhixingrenzhengjiamiqingqiu_with_abort(aiduihuaqq::fangshi, aiduihuaqq::lujing, Some(&ti), &lingpai, &abort_signal).await;
        
        let jiemi_wenben = if self.neibu.xuyaochongshi(&jieguo) {
            self.neibu.chongxinxieshang().await?;
            self.neibu.zhixingrenzhengjiamiqingqiu_with_abort(aiduihuaqq::fangshi, aiduihuaqq::lujing, Some(&ti), &lingpai, &abort_signal).await?
        } else {
            jieguo?
        };
        
        let xiangying: aiduihuaqq::Xiangying = fanxuliehua(&jiemi_wenben, "解析AI对话响应失败")?;
        if xiangying.zhuangtaima == huihua_guoqi_zhuangtaima {
            self.neibu.lingpai = None;
            self.neibu.baocunlingpai();
            return Err(jiami_gongju::cuowu("会话已过期，请重新登录"));
        }
        xuliehua(&xiangying)
    }

    pub async fn aiduihualiushiqingqiu(&mut self, xiaoxilie_json: &str, huidiaohanming: &str, baocunduquqi_hanming: &str) -> Result<(), JsValue> {
        let lingpai = self.neibu.lingpai.as_ref().ok_or_else(|| jiami_gongju::cuowu("未登录"))?.clone();
        self.neibu.quebaoxieshang().await?;
        
        let xiaoxilie: Vec<aiduihualiushiqq::Xiaoxi> = fanxuliehua(xiaoxilie_json, "解析消息列表失败")?;
        let ti = xuliehua(&aiduihualiushiqq::Qingqiuti { xiaoxilie })?;
        let huidiao = huoquhuidiao(huidiaohanming)?;
        
        let baocunduquqi = huoquhuidiao(baocunduquqi_hanming)?;
        let abort_controller = web_sys::AbortController::new().map_err(|_| jiami_gongju::cuowu("创建 AbortController 失败"))?;
        let abort_signal = abort_controller.signal();
        let _ = baocunduquqi.call1(&JsValue::NULL, &abort_controller);
        
        let jieguo = self.neibu.zhixingsserenzhengjiamiqingqiu_with_abort(aiduihualiushiqq::lujing, Some(&ti), &huidiao, &lingpai, &abort_signal).await;
        
        if jieguo.as_ref().err().and_then(|e| e.as_string()).map_or(false, |s| s.contains("base64解码失败") || s.contains("解密响应失败")) {
            self.neibu.chongxinxieshang().await?;
            return self.neibu.zhixingsserenzhengjiamiqingqiu_with_abort(aiduihualiushiqq::lujing, Some(&ti), &huidiao, &lingpai, &abort_signal).await;
        }
        jieguo
    }

    pub fn huoqulingpai(&self) -> Option<String> {
        self.neibu.lingpai.clone()
    }

    pub fn shezhilingpai(&mut self, lingpai: Option<String>) {
        self.neibu.lingpai = lingpai;
        self.neibu.baocunlingpai();
    }

    pub fn huoqufuwuqidizhi(&self) -> String {
        self.neibu.fuwuqidizhi.clone()
    }

    pub fn shezhifuwuqidizhi(&mut self, dizhi: &str) {
        self.neibu.fuwuqidizhi = dizhi.to_string();
    }

    pub fn huoquhuihuaid(&self) -> Option<String> {
        self.neibu.huihuaid.clone()
    }

    pub fn huoquzhiwen(&self) -> String {
        self.neibu.zhiwen.clone()
    }

    pub async fn chongxinxieshangmiyao(&mut self) -> Result<(), JsValue> {
        self.neibu.chongxinxieshang().await
    }

    pub fn qingchulingpai(&mut self) {
        self.neibu.lingpai = None;
        self.neibu.qingchulingpai();
    }

    pub fn qingchuhuihua(&mut self) {
        self.neibu.chongzhihuihua();
    }

    pub fn quanbuqingchu(&mut self) {
        self.qingchulingpai();
        self.qingchuhuihua();
    }

    pub fn huoquzhuangtai(&self) -> String {
        format!(
            "{{\"yixieshang\":{},\"yidenglu\":{},\"youhuihuaid\":{},\"youlingpai\":{}}}",
            self.yixieshang(),
            self.yidenglu(),
            self.neibu.huihuaid.is_some(),
            self.neibu.lingpai.is_some()
        )
    }

    pub fn shengchengxinzhiwen(&mut self) {
        self.neibu.zhiwen = shebeishibie::shengchengzhiwen();
    }

    pub async fn xieshangmiyao(&mut self) -> Result<(), JsValue> {
        self.neibu.xieshangmiyao().await
    }

    pub fn miyaoshifoucunzai(&self) -> bool {
        self.neibu.miyao.is_some()
    }

    pub fn huihuaidshifoucunzai(&self) -> bool {
        self.neibu.huihuaid.is_some()
    }

    pub fn lingpaishifoucunzai(&self) -> bool {
        self.neibu.lingpai.is_some()
    }

    pub fn huoqumiyaochangdu(&self) -> usize {
        self.neibu.miyao.as_ref().map_or(0, |m| m.len())
    }

    pub fn huoqukehugongyao(&self) -> Option<String> {
        self.neibu.kehugongyao_b64.clone()
    }
}
