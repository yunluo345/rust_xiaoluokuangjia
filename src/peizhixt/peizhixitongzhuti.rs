use crate::gongju::wenjiancaozuo::{wenjiancunzai, duquwenjian, xieruwenjian, liebiaowenjian};
use crate::gongju::neicungongju;
use super::peizhi_nr::peizhi_zongpeizhi::Zongpeizhi;
use super::peizhi_nr::peizhi_shujuku::Shujuku;
use super::peizhi_nr::peizhi_ai::Ai;
use serde_json::Value;

#[allow(non_upper_case_globals)]
const peizhi_mulu: &str = "peizhi";

fn huoqulujing(wenjianming: &str) -> String {
    format!("{}/{}.json", peizhi_mulu, wenjianming)
}

fn hebing(moren: &mut Value, xianyou: &Value) {
    if let (Some(moren_obj), Some(xianyou_obj)) = (moren.as_object_mut(), xianyou.as_object()) {
        for (jian, zhi) in xianyou_obj {
            if let Some(moren_zhi) = moren_obj.get_mut(jian) {
                if moren_zhi.is_object() && zhi.is_object() {
                    hebing(moren_zhi, zhi);
                } else {
                    *moren_zhi = zhi.clone();
                }
            }
        }
    }
}

fn tongbupeizhiwenjian<T: Default + serde::Serialize + serde::de::DeserializeOwned>(
    wenjianming: &str,
) -> bool {
    let lujing = huoqulujing(wenjianming);
    let mut moren = match serde_json::to_value(T::default()) {
        Ok(v) => v,
        Err(_) => return false,
    };
    
    if wenjiancunzai(&lujing) {
        if let Some(neirong) = duquwenjian(&lujing) {
            if let Ok(xianyou) = serde_json::from_str::<Value>(&neirong) {
                hebing(&mut moren, &xianyou);
            }
        }
    }
    
    serde_json::to_string_pretty(&moren)
        .ok()
        .map_or(false, |neirong| xieruwenjian(&lujing, &neirong))
}

/// 读取配置文件内容（优先内存缓存，回退到 IO）
pub fn duqupeizhi<T: serde::de::DeserializeOwned>(wenjianming: &str) -> Option<T> {
    let lujing = huoqulujing(wenjianming);
    
    if let Some(neirong) = neicungongju::duqu(&lujing) {
        return serde_json::from_str(&neirong).ok();
    }
    
    if let Some(neirong) = duquwenjian(&lujing) {
        return serde_json::from_str(&neirong).ok();
    }
    
    None
}

/// 将配置文件加载到内存缓存
pub fn jiazaidaohuancun(wenjianming: &str) -> bool {
    neicungongju::jiazaiwenjian(&huoqulujing(wenjianming))
}

/// 将 peizhi 文件夹内所有配置文件加载到内存
pub fn jiazaisuoyoupeizhi() -> bool {
    liebiaowenjian(peizhi_mulu)
        .map(|wenjianlie| {
            wenjianlie.iter().all(|lujing| {
                lujing.to_str()
                    .map_or(false, |lujing_str| neicungongju::jiazaiwenjian(lujing_str))
            })
        })
        .unwrap_or(false)
}

/// 热更新：重新从磁盘加载所有已缓存的配置文件到内存
pub fn regengxinhuancun() -> bool {
    neicungongju::regengxin()
}

/// 初始化配置系统：同步所有配置文件（补充缺失字段）并加载到内存
pub fn chushihua() -> bool {
    tongbupeizhiwenjian::<Zongpeizhi>(Zongpeizhi::wenjianming())
        && tongbupeizhiwenjian::<Shujuku>(Shujuku::wenjianming())
        && tongbupeizhiwenjian::<Ai>(Ai::wenjianming())
        && jiazaisuoyoupeizhi()
}
