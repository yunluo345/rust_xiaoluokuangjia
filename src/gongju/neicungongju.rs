use std::collections::HashMap;
use std::sync::{OnceLock, RwLock};
use super::wenjiancaozuo::duquwenjian;

type Huancun = RwLock<HashMap<String, String>>;

#[allow(non_upper_case_globals)]
static quanju: OnceLock<Huancun> = OnceLock::new();

fn huoqu() -> &'static Huancun {
    quanju.get_or_init(|| RwLock::new(HashMap::new()))
}

/// 将指定文件加载到内存缓存，键为文件路径
#[allow(dead_code)]
pub fn jiazaiwenjian(lujing: &str) -> bool {
    duquwenjian(lujing).map_or(false, |neirong| xieru(lujing, &neirong))
}

/// 批量加载多个文件到内存缓存
#[allow(dead_code)]
pub fn piliangjiaizai(lujinglie: &[&str]) -> bool {
    lujinglie.iter().all(|lujing| jiazaiwenjian(lujing))
}

/// 从内存缓存读取内容
#[allow(dead_code)]
pub fn duqu(jian: &str) -> Option<String> {
    huoqu().read().ok()?.get(jian).cloned()
}

/// 写入或更新缓存内容
#[allow(dead_code)]
pub fn xieru(jian: &str, zhi: &str) -> bool {
    huoqu()
        .write()
        .ok()
        .map(|mut xie| { xie.insert(jian.to_string(), zhi.to_string()); })
        .is_some()
}

/// 移除缓存中的指定键
#[allow(dead_code)]
pub fn yichu(jian: &str) -> bool {
    huoqu()
        .write()
        .ok()
        .and_then(|mut xie| xie.remove(jian))
        .is_some()
}

/// 清空全部缓存
#[allow(dead_code)]
pub fn qingkong() -> bool {
    huoqu()
        .write()
        .ok()
        .map(|mut xie| xie.clear())
        .is_some()
}

/// 检查缓存中是否存在指定键
#[allow(dead_code)]
pub fn cunzai(jian: &str) -> bool {
    huoqu().read().ok().map_or(false, |du| du.contains_key(jian))
}

/// 热更新：重新从磁盘加载所有已缓存的文件
#[allow(dead_code)]
pub fn regengxin() -> bool {
    huoqu()
        .read()
        .ok()
        .map(|du| du.keys().cloned().collect::<Vec<_>>())
        .map_or(false, |jianlie| jianlie.iter().all(|jian| jiazaiwenjian(jian)))
}
