#![allow(non_upper_case_globals)]

mod jiamihexin;
mod shebeishibie;
mod gongju;
mod wangluoqingqiu;
mod jiamiguanli;
mod bendicicun;
pub mod jiekou_nr;

use wasm_bindgen::prelude::*;
use gongju::{cuowu, xuliehua, fanxuliehua};
use jiamiguanli::Huihuazhuangtai;
use jiekou_nr::xitong::jiankangqingqiu as jiankangqq;
use jiekou_nr::xitong::jiamijiankang as jiamijiankangqq;
use jiekou_nr::xitong::sseceshi as sseceshiqq;
use jiekou_nr::xitong::jiamisseceshi as jiamisseceshiqq;
use jiekou_nr::yonghu::denglujiekou as dengluqq;
use jiekou_nr::ai::aiqudaojiekou as aiqudaoqq;
use jiekou_nr::ai::aiduihuajiekou as aiduihuaqq;

#[wasm_bindgen]
pub struct Kehuduanjiami {
    fuwuqidizhi: String,
    huihuazhuangtai: Huihuazhuangtai,
    zhiwen: String,
    lingpai: Option<String>,
}

#[wasm_bindgen]
impl Kehuduanjiami {
    #[wasm_bindgen(constructor)]
    pub fn xinjian(fuwuqidizhi: &str) -> Self {
        Self {
            fuwuqidizhi: fuwuqidizhi.to_string(),
            huihuazhuangtai: Huihuazhuangtai::xingjian(),
            zhiwen: shebeishibie::shengchengzhiwen(),
            lingpai: bendicicun::duqulingpai(),
        }
    }

    pub async fn jiamiqingqiu(&mut self, fangfa: &str, lujing: &str, qingqiuti: Option<String>) -> Result<String, JsValue> {
        jiamiguanli::quebaoxieshang(&self.fuwuqidizhi, &self.zhiwen, &mut self.huihuazhuangtai).await?;
        let jieguo = jiamiguanli::zhixingjiamiqingqiu(&self.fuwuqidizhi, fangfa, lujing, qingqiuti.as_deref(), &self.huihuazhuangtai).await;
        if jiamiguanli::xuyaochongshi(&jieguo) {
            jiamiguanli::xieshangmiyao(&self.fuwuqidizhi, &self.zhiwen, &mut self.huihuazhuangtai).await?;
            return jiamiguanli::zhixingjiamiqingqiu(&self.fuwuqidizhi, fangfa, lujing, qingqiuti.as_deref(), &self.huihuazhuangtai).await;
        }
        jieguo
    }

    pub async fn putongqingqiu(&self, fangfa: &str, lujing: &str, qingqiuti: Option<String>) -> Result<String, JsValue> {
        let url = format!("{}{}", self.fuwuqidizhi, lujing);
        wangluoqingqiu::putongqingqiu(fangfa, &url, qingqiuti.as_deref(), None).await
    }

    pub async fn jiankangqingqiu(&self) -> Result<String, JsValue> {
        let url = format!("{}{}", self.fuwuqidizhi, jiankangqq::lujing);
        let wenben = wangluoqingqiu::putongqingqiu(jiankangqq::fangshi, &url, None, None).await?;
        let xiangying: jiankangqq::Xiangying = fanxuliehua(&wenben, "解析健康检查响应失败")?;
        xuliehua(&xiangying)
    }

    pub async fn jiamijiankangqingqiu(&mut self, neirong: Option<String>) -> Result<String, JsValue> {
        jiamiguanli::quebaoxieshang(&self.fuwuqidizhi, &self.zhiwen, &mut self.huihuazhuangtai).await?;
        let ti = xuliehua(&jiamijiankangqq::Qingqiuti { neirong })?;
        let jieguo = jiamiguanli::zhixingjiamiqingqiu(&self.fuwuqidizhi, jiamijiankangqq::fangshi, jiamijiankangqq::lujing, Some(&ti), &self.huihuazhuangtai).await;
        let jiemi_wenben = if jiamiguanli::xuyaochongshi(&jieguo) {
            jiamiguanli::xieshangmiyao(&self.fuwuqidizhi, &self.zhiwen, &mut self.huihuazhuangtai).await?;
            jiamiguanli::zhixingjiamiqingqiu(&self.fuwuqidizhi, jiamijiankangqq::fangshi, jiamijiankangqq::lujing, Some(&ti), &self.huihuazhuangtai).await?
        } else {
            jieguo?
        };
        let xiangying: jiamijiankangqq::Xiangying = fanxuliehua(&jiemi_wenben, "解析加密测试响应失败")?;
        xuliehua(&xiangying)
    }

    pub async fn sseputongqingqiu(&self, fangfa: &str, lujing: &str, huidiaohanming: &str) -> Result<(), JsValue> {
        let huidiao = wangluoqingqiu::huoquhuidiao(huidiaohanming)?;
        let url = format!("{}{}", self.fuwuqidizhi, lujing);
        let request = wangluoqingqiu::goujianqingqiu(fangfa, &url, None, None, None)?;
        let xiangying = wangluoqingqiu::fasongqingqiu(&request).await?;
        wangluoqingqiu::duquliushi(&xiangying, None, &huidiao).await
    }

    pub async fn ssejiamiqingqiu(&mut self, lujing: &str, qingqiuti: Option<String>, huidiaohanming: &str) -> Result<(), JsValue> {
        jiamiguanli::quebaoxieshang(&self.fuwuqidizhi, &self.zhiwen, &mut self.huihuazhuangtai).await?;
        let huidiao = wangluoqingqiu::huoquhuidiao(huidiaohanming)?;
        let jieguo = jiamiguanli::zhixingssejiamiqingqiu(&self.fuwuqidizhi, lujing, qingqiuti.as_deref(), &huidiao, &self.huihuazhuangtai).await;
        if jieguo.is_err() && jieguo.as_ref().err().and_then(|e| e.as_string()).map_or(false, |s| s.contains("会话已过期")) {
            jiamiguanli::xieshangmiyao(&self.fuwuqidizhi, &self.zhiwen, &mut self.huihuazhuangtai).await?;
            return jiamiguanli::zhixingssejiamiqingqiu(&self.fuwuqidizhi, lujing, qingqiuti.as_deref(), &huidiao, &self.huihuazhuangtai).await;
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
        jiamiguanli::quebaoxieshang(&self.fuwuqidizhi, &self.zhiwen, &mut self.huihuazhuangtai).await?;
        let ti = xuliehua(&dengluqq::Qingqiuti { zhanghao: zhanghao.to_string(), mima: mima.to_string() })?;
        let jieguo = jiamiguanli::zhixingjiamiqingqiu(&self.fuwuqidizhi, dengluqq::fangshi, dengluqq::lujing, Some(&ti), &self.huihuazhuangtai).await;
        let jiemi_wenben = if jiamiguanli::xuyaochongshi(&jieguo) {
            jiamiguanli::xieshangmiyao(&self.fuwuqidizhi, &self.zhiwen, &mut self.huihuazhuangtai).await?;
            jiamiguanli::zhixingjiamiqingqiu(&self.fuwuqidizhi, dengluqq::fangshi, dengluqq::lujing, Some(&ti), &self.huihuazhuangtai).await?
        } else {
            jieguo?
        };
        let xiangying: dengluqq::Xiangying = fanxuliehua(&jiemi_wenben, "解析登录响应失败")?;
        if let Some(ref shuju) = xiangying.shuju {
            self.lingpai = Some(shuju.lingpai.clone());
            bendicicun::cunlingpai(&shuju.lingpai);
        }
        xuliehua(&xiangying)
    }

    pub fn shezhilingpai(&mut self, lingpai: &str) {
        self.lingpai = Some(lingpai.to_string());
        bendicicun::cunlingpai(lingpai);
    }

    pub fn huoqulingpai(&self) -> Option<String> {
        self.lingpai.clone()
    }

    pub fn qingchulingpai(&mut self) {
        self.lingpai = None;
        bendicicun::shanculingpai();
    }

    pub fn yidenglu(&self) -> bool {
        self.lingpai.is_some()
    }

    pub async fn aiqudao_caozuo(&mut self, caozuo_json: &str) -> Result<String, JsValue> {
        let lingpai = self.lingpai.clone().ok_or_else(|| cuowu("未登录，无令牌"))?;
        jiamiguanli::quebaoxieshang(&self.fuwuqidizhi, &self.zhiwen, &mut self.huihuazhuangtai).await?;
        let jieguo = jiamiguanli::zhixingrenzhengjiamiqingqiu(&self.fuwuqidizhi, aiqudaoqq::fangshi, aiqudaoqq::lujing, Some(caozuo_json), &lingpai, &self.huihuazhuangtai).await;
        let jiemi_wenben = if jiamiguanli::xuyaochongshi(&jieguo) {
            jiamiguanli::xieshangmiyao(&self.fuwuqidizhi, &self.zhiwen, &mut self.huihuazhuangtai).await?;
            let lingpai = self.lingpai.clone().ok_or_else(|| cuowu("未登录，无令牌"))?;
            jiamiguanli::zhixingrenzhengjiamiqingqiu(&self.fuwuqidizhi, aiqudaoqq::fangshi, aiqudaoqq::lujing, Some(caozuo_json), &lingpai, &self.huihuazhuangtai).await?
        } else {
            jieguo?
        };
        let xiangying: aiqudaoqq::Xiangying = fanxuliehua(&jiemi_wenben, "解析AI渠道响应失败")?;
        xuliehua(&xiangying)
    }

    pub async fn aiqudao_liebiao(&mut self) -> Result<String, JsValue> {
        let ti = xuliehua(&aiqudaoqq::Qingqiuti::caozuo("liebiao"))?;
        self.aiqudao_caozuo(&ti).await
    }

    pub async fn aiqudao_tianjia(&mut self, mingcheng: &str, leixing: &str, jiekoudizhi: &str, miyao: &str, moxing: &str, wendu: Option<String>, beizhu: Option<String>, zuidatoken: Option<String>, youxianji: Option<String>) -> Result<String, JsValue> {
        let mut ti = aiqudaoqq::Qingqiuti::caozuo("tianjia");
        ti.mingcheng = Some(mingcheng.to_string());
        ti.leixing = Some(leixing.to_string());
        ti.jiekoudizhi = Some(jiekoudizhi.to_string());
        ti.miyao = Some(miyao.to_string());
        ti.moxing = Some(moxing.to_string());
        ti.wendu = wendu;
        ti.beizhu = beizhu;
        ti.zuidatoken = zuidatoken;
        ti.youxianji = youxianji;
        let ti_str = xuliehua(&ti)?;
        self.aiqudao_caozuo(&ti_str).await
    }

    pub async fn aiqudao_shanchu(&mut self, id: &str) -> Result<String, JsValue> {
        let mut ti = aiqudaoqq::Qingqiuti::caozuo("shanchu");
        ti.id = Some(id.to_string());
        let ti_str = xuliehua(&ti)?;
        self.aiqudao_caozuo(&ti_str).await
    }

    pub async fn aiqudao_xiugai(&mut self, caozuo_json: &str) -> Result<String, JsValue> {
        self.aiqudao_caozuo(caozuo_json).await
    }

    #[wasm_bindgen]
    pub fn chuangjian_zhongduanqi() -> Result<web_sys::AbortController, JsValue> {
        web_sys::AbortController::new()
            .map_err(|_| cuowu("创建AbortController失败"))
    }

    #[wasm_bindgen]
    pub async fn aiduihua(
        &mut self,
        leixing: &str,
        xiaoxilie_json: &str,
        huidiaohanming: &str,
        xitongtishici: Option<String>,
        zhongduanqi: Option<web_sys::AbortController>,
    ) -> Result<(), JsValue> {
        let lingpai = self.lingpai.clone().ok_or_else(|| cuowu("未登录，无令牌"))?;
        jiamiguanli::quebaoxieshang(&self.fuwuqidizhi, &self.zhiwen, &mut self.huihuazhuangtai).await?;
        let huidiao = wangluoqingqiu::huoquhuidiao(huidiaohanming)?;
        
        let zhongduanxinhao = zhongduanqi.as_ref().map(|z| z.signal());
        
        let jieguo = self.zhixingaiduihua(
            leixing, 
            xiaoxilie_json, 
            xitongtishici.as_deref(), 
            &lingpai, 
            &huidiao, 
            zhongduanxinhao.as_ref()
        ).await;
        
        if jieguo.is_err() && jieguo.as_ref().err().and_then(|e| e.as_string()).map_or(false, |s| s.contains("会话已过期")) {
            jiamiguanli::xieshangmiyao(&self.fuwuqidizhi, &self.zhiwen, &mut self.huihuazhuangtai).await?;
            let lingpai = self.lingpai.clone().ok_or_else(|| cuowu("未登录，无令牌"))?;
            return self.zhixingaiduihua(
                leixing, 
                xiaoxilie_json, 
                xitongtishici.as_deref(), 
                &lingpai, 
                &huidiao, 
                zhongduanxinhao.as_ref()
            ).await;
        }
        jieguo
    }

    pub fn chongzhihuihua(&mut self) {
        self.huihuazhuangtai.chongzhi();
    }

    #[wasm_bindgen]
    pub fn tingzhishuchu(&self) {
        web_sys::console::log_1(&JsValue::from_str("tingzhishuchu 已废弃，请使用 JavaScript 层面管理 AbortController"));
    }

    pub fn yixieshang(&self) -> bool {
        self.huihuazhuangtai.yixieshang()
    }
}

impl Kehuduanjiami {
    async fn zhixingaiduihua(
        &mut self,
        leixing: &str,
        xiaoxilie_json: &str,
        xitongtishici: Option<&str>,
        shouquan: &str,
        huidiao: &js_sys::Function,
        zhongduanxinhao: Option<&web_sys::AbortSignal>,
    ) -> Result<(), JsValue> {
        let xinxi = self.huihuazhuangtai.huoquxinxi()?;
        let xiaoxilie: Vec<aiduihuaqq::Xiaoxixiang> = fanxuliehua(xiaoxilie_json, "解析消息列表失败")?;
        let ti = aiduihuaqq::Qingqiuti {
            leixing: leixing.to_string(),
            xitongtishici: xitongtishici.map(|s| s.to_string()),
            xiaoxilie,
        };
        let ti_str = xuliehua(&ti)?;
        let jiami_ti = gongju::jiamiqingqiuti(&ti_str, xinxi.miyao)?;
        let url = format!("{}{}", self.fuwuqidizhi, aiduihuaqq::lujing);
        let huihuaid = xinxi.huihuaid.to_string();
        let kehugongyao = xinxi.kehugongyao.to_string();
        let miyao_fuben = xinxi.miyao.to_vec();
        
        web_sys::console::log_1(&JsValue::from_str("AI对话：使用传入的AbortSignal"));
        
        let ewaiqingqiutou = vec![
            ("X-Huihua-Id", huihuaid.as_str()),
            ("X-Kehugongyao", kehugongyao.as_str()),
            ("Authorization", shouquan),
        ];
        
        let request = wangluoqingqiu::goujianqingqiu("POST", &url, Some(&jiami_ti), Some(&ewaiqingqiutou), zhongduanxinhao)?;
        web_sys::console::log_1(&JsValue::from_str("AI对话：请求已构建，开始发送"));
        let xiangying = wangluoqingqiu::fasongqingqiu(&request).await?;
        web_sys::console::log_1(&JsValue::from_str("AI对话：响应已接收，开始读取流"));
        
        let jieguo = wangluoqingqiu::duqujiamiliushi(&xiangying, None, &miyao_fuben, huidiao).await;
        
        web_sys::console::log_1(&JsValue::from_str("AI对话：流读取完成"));
        jieguo
    }
}
