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
use jiekou_nr::xitong::ribaoguanli as ribaoqq;
use jiekou_nr::yonghu::denglujiekou as dengluqq;
use jiekou_nr::yonghu::yonghuguanlijiekou as yonghuguanliqq;
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
        let neibu = KehuduanjiamiNeibu::xinjian(
            fuwuqidizhi.to_string(),
            shebeishibie::shengchengzhiwen()
        );
        neibu.huifulingpai();
        Self { neibu }
    }

    async fn zhixingdaichongshi<F, Fut>(&self, caozuo: F) -> Result<String, JsValue>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = Result<String, JsValue>>,
    {
        self.neibu.quebaoxieshang().await?;
        let jieguo = caozuo().await;
        match self.neibu.xuyaochongshi(&jieguo) {
            true => {
                self.neibu.chongxinxieshang().await?;
                caozuo().await
            }
            false => jieguo,
        }
    }

    fn jieximoren_canshu(&self, canshu: Option<String>) -> Result<serde_json::Value, JsValue> {
        canshu.map_or(Ok(serde_json::json!({})), |s| fanxuliehua(&s, "解析参数失败"))
    }

    fn huoqubixu_lingpai(&self) -> Result<String, JsValue> {
        self.neibu.huoqulingpai().ok_or_else(|| jiami_gongju::cuowu("尚未登录，请先登录"))
    }

    pub async fn jiamiqingqiu(&self, fangfa: &str, lujing: &str, qingqiuti: Option<String>) -> Result<String, JsValue> {
        self.zhixingdaichongshi(|| self.neibu.zhixingjiamiqingqiu(fangfa, lujing, qingqiuti.as_deref())).await
    }

    pub async fn putongqingqiu(&self, fangfa: &str, lujing: &str, qingqiuti: Option<String>) -> Result<String, JsValue> {
        let url = format!("{}{}", self.neibu.huoqufuwuqidizhi(), lujing);
        putongqingqiu_neibu(fangfa, &url, qingqiuti.as_deref(), None).await
    }

    pub async fn jiankangqingqiu(&self) -> Result<String, JsValue> {
        let url = format!("{}{}", self.neibu.huoqufuwuqidizhi(), jiankangqq::lujing);
        let wenben = putongqingqiu_neibu(jiankangqq::fangshi, &url, None, None).await?;
        let xiangying: jiankangqq::Xiangying = fanxuliehua(&wenben, "解析健康检查响应失败")?;
        xuliehua(&xiangying)
    }

    pub async fn jiamijiankangqingqiu(&self, neirong: Option<String>) -> Result<String, JsValue> {
        let ti = xuliehua(&jiamijiankangqq::Qingqiuti { neirong })?;
        let jiemi_wenben = self.zhixingdaichongshi(|| self.neibu.zhixingjiamiqingqiu(jiamijiankangqq::fangshi, jiamijiankangqq::lujing, Some(&ti))).await?;
        let xiangying: jiamijiankangqq::Xiangying = fanxuliehua(&jiemi_wenben, "解析加密测试响应失败")?;
        xuliehua(&xiangying)
    }

    pub async fn sseputongqingqiu(&self, fangfa: &str, lujing: &str, huidiaohanming: &str) -> Result<(), JsValue> {
        let huidiao = huoquhuidiao(huidiaohanming)?;
        let url = format!("{}{}", self.neibu.huoqufuwuqidizhi(), lujing);
        let request = http_gongju::goujianqingqiu(fangfa, &url, None, None, None)?;
        let xiangying = http_gongju::fasongqingqiu(&request).await?;
        duquliushi(&xiangying, &huidiao).await
    }

    pub async fn ssejiamiqingqiu(&self, lujing: &str, qingqiuti: Option<String>, huidiaohanming: &str) -> Result<(), JsValue> {
        self.neibu.quebaoxieshang().await?;
        let huidiao = huoquhuidiao(huidiaohanming)?;
        let jieguo = self.neibu.zhixingssejiamiqingqiu(lujing, qingqiuti.as_deref(), &huidiao).await;
        if jieguo.as_ref().err().and_then(|e| e.as_string()).is_some_and(|s| s.contains("base64解码失败") || s.contains("解密响应失败")) {
            self.neibu.chongxinxieshang().await?;
            return self.neibu.zhixingssejiamiqingqiu(lujing, qingqiuti.as_deref(), &huidiao).await;
        }
        jieguo
    }

    pub async fn sseceshiqingqiu(&self, huidiaohanming: &str) -> Result<(), JsValue> {
        self.sseputongqingqiu(sseceshiqq::fangshi, sseceshiqq::lujing, huidiaohanming).await
    }

    pub async fn jiamisseceshiqingqiu(&self, huidiaohanming: &str) -> Result<(), JsValue> {
        self.ssejiamiqingqiu(jiamisseceshiqq::lujing, None, huidiaohanming).await
    }

    pub async fn dengluqingqiu(&self, zhanghao: &str, mima: &str) -> Result<String, JsValue> {
        let ti = xuliehua(&dengluqq::Qingqiuti { zhanghao: zhanghao.to_string(), mima: mima.to_string() })?;
        let jiemi_wenben = self.zhixingdaichongshi(|| self.neibu.zhixingjiamiqingqiu(dengluqq::fangshi, dengluqq::lujing, Some(&ti))).await?;
        let xiangying: dengluqq::Xiangying = fanxuliehua(&jiemi_wenben, "解析登录响应失败")?;
        if let Some(shuju) = xiangying.shuju.as_ref().filter(|_| xiangying.zhuangtaima == 200) {
            self.neibu.shezhilingpai(shuju.lingpai.clone());
            self.neibu.baocunlingpai();
        }
        xuliehua(&xiangying)
    }

    pub async fn yonghuguanliqingqiu(&self, caozuo: &str, canshu: Option<String>) -> Result<String, JsValue> {
        let lingpai = self.huoqubixu_lingpai()?;
        let canshu_zhi = self.jieximoren_canshu(canshu)?;
        let ti = xuliehua(&yonghuguanliqq::Qingqiuti {
            caozuo: caozuo.to_string(),
            dangqianyeshu: canshu_zhi.get("dangqianyeshu").and_then(|v| v.as_i64()).map(|v| v as i32),
            meiyeshuliang: canshu_zhi.get("meiyeshuliang").and_then(|v| v.as_i64()).map(|v| v as i32),
            guanjianci: canshu_zhi.get("guanjianci").and_then(|v| v.as_str()).map(String::from),
            id: canshu_zhi.get("id").and_then(|v| v.as_str()).map(String::from),
        })?;
        self.zhixingdaichongshi(|| self.neibu.zhixingrenzhengjiamiqingqiu(yonghuguanliqq::fangshi, yonghuguanliqq::lujing, Some(&ti), &lingpai)).await
    }

    pub fn yixieshang(&self) -> bool {
        self.neibu.yixieshang()
    }

    pub fn yidenglu(&self) -> bool {
        self.neibu.huoqulingpai().is_some()
    }

    pub fn chongzhihuihua(&self) {
        self.neibu.chongzhihuihua();
    }

    pub async fn aiqudaoqingqiu(&self, caozuo: &str, canshu: Option<String>) -> Result<String, JsValue> {
        let lingpai = self.huoqubixu_lingpai()?;
        let canshu_zhi = self.jieximoren_canshu(canshu)?;
        let ti = xuliehua(&aiqudaoqq::Qingqiuti { caozuo: caozuo.to_string(), canshu: canshu_zhi })?;
        self.zhixingdaichongshi(|| self.neibu.zhixingrenzhengjiamiqingqiu(aiqudaoqq::fangshi, aiqudaoqq::lujing, Some(&ti), &lingpai)).await
    }

    pub async fn ribaoqingqiu(&self, caozuo: &str, canshu: Option<String>) -> Result<String, JsValue> {
        let lingpai = self.huoqubixu_lingpai()?;
        let canshu_zhi = self.jieximoren_canshu(canshu)?;
        let ti = xuliehua(&ribaoqq::Qingqiuti { caozuo: caozuo.to_string(), canshu: canshu_zhi })?;
        self.zhixingdaichongshi(|| self.neibu.zhixingrenzhengjiamiqingqiu(ribaoqq::fangshi, ribaoqq::lujing, Some(&ti), &lingpai)).await
    }

    pub async fn aiduihuaqingqiu(&self, xiaoxilie_json: &str, baocunduquqi_hanming: &str) -> Result<String, JsValue> {
        let lingpai = self.neibu.huoqulingpai().ok_or_else(|| jiami_gongju::cuowu("未登录"))?;
        
        let xiaoxilie: Vec<aiduihuaqq::Xiaoxi> = fanxuliehua(xiaoxilie_json, "解析消息列表失败")?;
        let ti = xuliehua(&aiduihuaqq::Qingqiuti { xiaoxilie })?;
        
        let baocunduquqi = huoquhuidiao(baocunduquqi_hanming)?;
        let abort_controller = web_sys::AbortController::new().map_err(|_| jiami_gongju::cuowu("创建 AbortController 失败"))?;
        let abort_signal = abort_controller.signal();
        let _ = baocunduquqi.call1(&JsValue::NULL, &abort_controller);
        
        let jiemi_wenben = self.neibu.zhixingrenzhengputongqingqiu_with_abort(aiduihuaqq::fangshi, aiduihuaqq::lujing, Some(&ti), &lingpai, &abort_signal).await?;
        
        let xiangying: aiduihuaqq::Xiangying = fanxuliehua(&jiemi_wenben, "解析AI对话响应失败")?;
        if xiangying.zhuangtaima == huihua_guoqi_zhuangtaima {
            self.neibu.qingchulingpai_neibu();
            self.neibu.baocunlingpai();
            return Err(jiami_gongju::cuowu("会话已过期，请重新登录"));
        }
        xuliehua(&xiangying)
    }

    pub async fn aiduihualiushiqingqiu(&self, xiaoxilie_json: &str, huidiaohanming: &str, baocunduquqi_hanming: &str) -> Result<(), JsValue> {
        let lingpai = self.neibu.huoqulingpai().ok_or_else(|| jiami_gongju::cuowu("未登录"))?;
        
        let xiaoxilie: Vec<aiduihualiushiqq::Xiaoxi> = fanxuliehua(xiaoxilie_json, "解析消息列表失败")?;
        let ti = xuliehua(&aiduihualiushiqq::Qingqiuti { xiaoxilie })?;
        let huidiao = huoquhuidiao(huidiaohanming)?;
        
        let baocunduquqi = huoquhuidiao(baocunduquqi_hanming)?;
        let abort_controller = web_sys::AbortController::new().map_err(|_| jiami_gongju::cuowu("创建 AbortController 失败"))?;
        let abort_signal = abort_controller.signal();
        let _ = baocunduquqi.call1(&JsValue::NULL, &abort_controller);
        
        self.neibu.zhixingsserenzhengputongqingqiu_with_abort(aiduihualiushiqq::lujing, Some(&ti), &huidiao, &lingpai, &abort_signal).await
    }

    pub fn huoqulingpai(&self) -> Option<String> {
        self.neibu.huoqulingpai()
    }

    pub fn shezhilingpai(&self, lingpai: Option<String>) {
        if let Some(lp) = lingpai {
            self.neibu.shezhilingpai(lp);
        } else {
            self.neibu.qingchulingpai_neibu();
        }
        self.neibu.baocunlingpai();
    }

    pub fn huoqufuwuqidizhi(&self) -> String {
        self.neibu.huoqufuwuqidizhi()
    }

    pub fn shezhifuwuqidizhi(&self, dizhi: &str) {
        self.neibu.shezhifuwuqidizhi(dizhi.to_string());
    }

    pub fn huoquhuihuaid(&self) -> Option<String> {
        self.neibu.huoquhuihuaid()
    }

    pub fn huoquzhiwen(&self) -> String {
        self.neibu.huoquzhiwen()
    }

    pub async fn chongxinxieshangmiyao(&self) -> Result<(), JsValue> {
        self.neibu.chongxinxieshang().await
    }

    pub fn qingchulingpai(&self) {
        self.neibu.qingchulingpai_neibu();
        self.neibu.qingchulingpai();
    }

    pub fn qingchuhuihua(&self) {
        self.neibu.chongzhihuihua();
    }

    pub fn quanbuqingchu(&self) {
        self.qingchulingpai();
        self.qingchuhuihua();
    }

    pub fn huoquzhuangtai(&self) -> String {
        format!(
            "{{\"yixieshang\":{},\"yidenglu\":{},\"youhuihuaid\":{},\"youlingpai\":{}}}",
            self.yixieshang(),
            self.yidenglu(),
            self.neibu.huoquhuihuaid().is_some(),
            self.neibu.huoqulingpai().is_some()
        )
    }

    pub fn shengchengxinzhiwen(&self) {
        self.neibu.shezhizhiwen(shebeishibie::shengchengzhiwen());
    }

    pub async fn xieshangmiyao(&self) -> Result<(), JsValue> {
        self.neibu.xieshangmiyao().await
    }

    pub fn miyaoshifoucunzai(&self) -> bool {
        self.neibu.miyaoshifoucunzai()
    }

    pub fn huihuaidshifoucunzai(&self) -> bool {
        self.neibu.huoquhuihuaid().is_some()
    }

    pub fn lingpaishifoucunzai(&self) -> bool {
        self.neibu.huoqulingpai().is_some()
    }

    pub fn huoqumiyaochangdu(&self) -> usize {
        self.neibu.huoqumiyaochangdu()
    }

    pub fn huoqukehugongyao(&self) -> Option<String> {
        self.neibu.huoqukehugongyao()
    }
}
