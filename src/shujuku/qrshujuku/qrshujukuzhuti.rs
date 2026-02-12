use qdrant_client::Qdrant;
use qdrant_client::qdrant::Distance;
use std::sync::OnceLock;
use crate::gongju::neicungongju;
use super::qrshujuku_caozuo::jihe_guanli;

#[allow(non_upper_case_globals)]
const qr_zhuangtai_jian: &str = "qr_lianjie_zhuangtai";
#[allow(non_upper_case_globals)]
static quanju_kehu: OnceLock<Qdrant> = OnceLock::new();

/// Qdrant 连接配置
pub struct Qrpeizhi {
    pub zhiji: String,
    pub duankou: u16,
    pub miyao: String,
    pub jheqianzhui: String,
}

fn goujianurl(peizhi: &Qrpeizhi) -> String {
    format!("http://{}:{}", peizhi.zhiji, peizhi.duankou)
}

fn gengxinzhuangtai(zhuangtai: bool) {
    neicungongju::xieru(qr_zhuangtai_jian, if zhuangtai { "1" } else { "0" });
}

fn goujiankehu(peizhi: &Qrpeizhi) -> Option<Qdrant> {
    let mut goujianqi = Qdrant::from_url(&goujianurl(peizhi));
    if !peizhi.miyao.is_empty() {
        goujianqi = goujianqi.api_key(peizhi.miyao.clone());
    }
    goujianqi.build().ok()
}

/// 拼接前缀和集合名称
pub fn pinjiemingcheng(qianzhui: &str, mingcheng: &str) -> String {
    format!("{}_{}", qianzhui, mingcheng)
}

/// 获取全局 Qdrant 客户端（零开销，无锁）
pub fn huoqukehu() -> Option<&'static Qdrant> {
    quanju_kehu.get()
}

/// 判断 Qdrant 是否已连接（零开销，无锁）
pub fn shifouqiyong() -> bool {
    quanju_kehu.get().is_some()
}

/// 连接并初始化 Qdrant（创建默认集合），连接状态写入内存缓存
pub async fn lianjie(
    peizhi: &Qrpeizhi,
    mingcheng: &str,
    weidu: u64,
    julicedufangshi: Distance,
) -> bool {
    let chenggong = async {
        let kehu = goujiankehu(peizhi)?;
        let wanzhengmingcheng = pinjiemingcheng(&peizhi.jheqianzhui, mingcheng);
        if !jihe_guanli::chuangjian(&kehu, &wanzhengmingcheng, weidu, julicedufangshi).await {
            return None;
        }
        let _ = quanju_kehu.set(kehu);
        Some(())
    }
    .await
    .is_some();

    gengxinzhuangtai(chenggong);
    chenggong
}
