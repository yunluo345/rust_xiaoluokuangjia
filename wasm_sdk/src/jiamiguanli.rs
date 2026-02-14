use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::JsValue;
use crate::jiamihexin;
use crate::gongju::{cuowu, xuliehua, fanxuliehua, jiamiqingqiuti, jiemixiangying, shifouhuihuaguoqi};
use crate::wangluoqingqiu;
use crate::jiekou_nr::xitong::miyaojiaohuanjiekou as miyaojiekou;

// ==================== 数据结构 ====================

pub struct Jiamixinxi<'a> {
    pub miyao: &'a [u8],
    pub huihuaid: &'a str,
    pub kehugongyao: &'a str,
}

/// Owned 版本，用于从 RefCell 中取出数据后使用
pub struct JiamixinxiOwned {
    pub miyao: Vec<u8>,
    pub huihuaid: String,
    pub kehugongyao: String,
}

pub struct Huihuazhuangtai {
    pub huihuaid: Option<String>,
    pub miyao: Option<Vec<u8>>,
    pub kehugongyao_b64: Option<String>,
}

impl Huihuazhuangtai {
    pub fn xingjian() -> Self {
        Self {
            huihuaid: None,
            miyao: None,
            kehugongyao_b64: None,
        }
    }

    pub fn yixieshang(&self) -> bool {
        self.huihuaid.is_some() && self.miyao.is_some()
    }

    pub fn chongzhi(&mut self) {
        self.huihuaid = None;
        self.miyao = None;
        self.kehugongyao_b64 = None;
    }

    pub fn huoquxinxi(&self) -> Result<Jiamixinxi<'_>, JsValue> {
        Ok(Jiamixinxi {
            miyao: self.miyao.as_ref().ok_or_else(|| cuowu("尚未协商密钥"))?,
            huihuaid: self.huihuaid.as_deref().ok_or_else(|| cuowu("尚未协商密钥"))?,
            kehugongyao: self.kehugongyao_b64.as_deref().ok_or_else(|| cuowu("尚未协商密钥"))?,
        })
    }

    pub fn huoquxinxi_clone(&self) -> Result<JiamixinxiOwned, JsValue> {
        Ok(JiamixinxiOwned {
            miyao: self.miyao.clone().ok_or_else(|| cuowu("尚未协商密钥"))?,
            huihuaid: self.huihuaid.clone().ok_or_else(|| cuowu("尚未协商密钥"))?,
            kehugongyao: self.kehugongyao_b64.clone().ok_or_else(|| cuowu("尚未协商密钥"))?,
        })
    }
}

// ==================== 密钥协商 ====================

pub async fn xieshangmiyao(fuwuqidizhi: &str, zhiwen: &str, zhuangtai: &Rc<RefCell<Huihuazhuangtai>>) -> Result<(), JsValue> {
    let url = format!("{}{}", fuwuqidizhi, miyaojiekou::lujing);
    let ti = xuliehua(&miyaojiekou::Qingqiuti { zhiwen: zhiwen.to_string() })?;
    let xiangying_wenben = wangluoqingqiu::putongqingqiu(miyaojiekou::fangshi, &url, Some(&ti), None).await?;
    let xiangying: miyaojiekou::Xiangying = fanxuliehua(&xiangying_wenben, "解析响应失败")?;
    let shuju = xiangying.shuju.ok_or_else(|| cuowu("服务端未返回公钥数据"))?;
    let fuwuqigongyao = jiamihexin::congbase64(&shuju.gongyao)
        .ok_or_else(|| cuowu("服务端公钥base64解码失败"))?;
    let (kehusiyao, kehugongyao) = jiamihexin::shengchengyaodui();
    let gongxiangyao = jiamihexin::xieshanggongxiangyao(&kehusiyao, &fuwuqigongyao)
        .ok_or_else(|| cuowu("ECDH协商失败"))?;
    let miyao = jiamihexin::paishengyao(&gongxiangyao, jiamihexin::yanfen)
        .ok_or_else(|| cuowu("密钥派生失败"))?;
    let mut zt = zhuangtai.borrow_mut();
    zt.huihuaid = Some(shuju.huihuaid);
    zt.miyao = Some(miyao);
    zt.kehugongyao_b64 = Some(jiamihexin::zhuanbase64(&kehugongyao));
    Ok(())
}

pub async fn quebaoxieshang(fuwuqidizhi: &str, zhiwen: &str, zhuangtai: &Rc<RefCell<Huihuazhuangtai>>) -> Result<(), JsValue> {
    if !zhuangtai.borrow().yixieshang() {
        xieshangmiyao(fuwuqidizhi, zhiwen, zhuangtai).await?;
    }
    Ok(())
}

// ==================== 加密请求 ====================

pub async fn zhixingjiamiqingqiu(
    fuwuqidizhi: &str,
    fangfa: &str,
    lujing: &str,
    qingqiuti: Option<&str>,
    zhuangtai: &Rc<RefCell<Huihuazhuangtai>>
) -> Result<String, JsValue> {
    let (jiami_ti, url, huihuaid, kehugongyao, miyao_fuben) = {
        let zt = zhuangtai.borrow();
        let xinxi = zt.huoquxinxi()?;
        let jiami_ti = qingqiuti.map(|ti| jiamiqingqiuti(ti, xinxi.miyao)).transpose()?;
        let url = format!("{}{}", fuwuqidizhi, lujing);
        let huihuaid = xinxi.huihuaid.to_string();
        let kehugongyao = xinxi.kehugongyao.to_string();
        let miyao_fuben = xinxi.miyao.to_vec();
        (jiami_ti, url, huihuaid, kehugongyao, miyao_fuben)
    };
    let ewaiqingqiutou = vec![
        ("X-Huihua-Id", huihuaid.as_str()),
        ("X-Kehugongyao", kehugongyao.as_str()),
    ];
    let xiangying_wenben = wangluoqingqiu::putongqingqiu(fangfa, &url, jiami_ti.as_deref(), Some(&ewaiqingqiutou)).await?;
    jiemixiangying(&xiangying_wenben, &miyao_fuben)
}

pub async fn zhixingrenzhengjiamiqingqiu(
    fuwuqidizhi: &str,
    fangfa: &str,
    lujing: &str,
    qingqiuti: Option<&str>,
    lingpai: &str,
    zhuangtai: &Rc<RefCell<Huihuazhuangtai>>
) -> Result<String, JsValue> {
    let (jiami_ti, url, huihuaid, kehugongyao, miyao_fuben) = {
        let zt = zhuangtai.borrow();
        let xinxi = zt.huoquxinxi()?;
        let jiami_ti = qingqiuti.map(|ti| jiamiqingqiuti(ti, xinxi.miyao)).transpose()?;
        let url = format!("{}{}", fuwuqidizhi, lujing);
        let huihuaid = xinxi.huihuaid.to_string();
        let kehugongyao = xinxi.kehugongyao.to_string();
        let miyao_fuben = xinxi.miyao.to_vec();
        (jiami_ti, url, huihuaid, kehugongyao, miyao_fuben)
    };
    let shouquan = format!("Bearer {}", lingpai);
    let ewaiqingqiutou = vec![
        ("X-Huihua-Id", huihuaid.as_str()),
        ("X-Kehugongyao", kehugongyao.as_str()),
        ("Authorization", shouquan.as_str()),
    ];
    let xiangying_wenben = wangluoqingqiu::putongqingqiu(fangfa, &url, jiami_ti.as_deref(), Some(&ewaiqingqiutou)).await?;
    jiemixiangying(&xiangying_wenben, &miyao_fuben)
}

pub async fn zhixingssejiamiqingqiu(
    fuwuqidizhi: &str,
    lujing: &str,
    qingqiuti: Option<&str>,
    huidiao: &js_sys::Function,
    zhuangtai: &Rc<RefCell<Huihuazhuangtai>>
) -> Result<(), JsValue> {
    let (jiami_ti, url, huihuaid, kehugongyao, miyao_fuben) = {
        let zt = zhuangtai.borrow();
        let xinxi = zt.huoquxinxi()?;
        let jiami_ti = qingqiuti.map(|ti| jiamiqingqiuti(ti, xinxi.miyao)).transpose()?;
        let url = format!("{}{}", fuwuqidizhi, lujing);
        let huihuaid = xinxi.huihuaid.to_string();
        let kehugongyao = xinxi.kehugongyao.to_string();
        let miyao_fuben = xinxi.miyao.to_vec();
        (jiami_ti, url, huihuaid, kehugongyao, miyao_fuben)
    };
    let ewaiqingqiutou = vec![
        ("X-Huihua-Id", huihuaid.as_str()),
        ("X-Kehugongyao", kehugongyao.as_str()),
    ];
    let request = wangluoqingqiu::goujianqingqiu("POST", &url, jiami_ti.as_deref(), Some(&ewaiqingqiutou), None)?;
    let xiangying = wangluoqingqiu::fasongqingqiu(&request).await?;
    wangluoqingqiu::duqujiamiliushi(&xiangying, None, &miyao_fuben, huidiao).await
}

// ==================== 重试逻辑 ====================

pub fn xuyaochongshi(jieguo: &Result<String, JsValue>) -> bool {
    match jieguo {
        Err(e) => e.as_string().map_or(false, |s| s.contains("解密响应失败")),
        Ok(wenben) => shifouhuihuaguoqi(wenben),
    }
}
