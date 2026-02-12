use crate::gongju::jiamigongju;
use crate::gongju::jichugongju;
use crate::shujuku::redisshujuku::rediscaozuo;

#[allow(non_upper_case_globals)]
const huihua_qianzhui: &str = "jiami:huihua:";
#[allow(non_upper_case_globals)]
const zhiwen_qianzhui: &str = "jiami:zhiwen:";
#[allow(non_upper_case_globals)]
const pinlv_qianzhui: &str = "jiami:pinlv:";
#[allow(non_upper_case_globals)]
const huihua_ttl_miao: u64 = 1800;
#[allow(non_upper_case_globals)]
const pinlv_chuangkou_miao: u64 = 60;
#[allow(non_upper_case_globals)]
const pinlv_shangxian: i64 = 30;
#[allow(non_upper_case_globals)]
const huihuaid_changdu: usize = 32;

fn huihuajian(huihuaid: &str) -> String {
    format!("{}{}", huihua_qianzhui, huihuaid)
}

fn zhiwenjian(zhiwen: &str) -> String {
    format!("{}{}", zhiwen_qianzhui, zhiwen)
}

fn pinlvjian(zhiwen: &str) -> String {
    format!("{}{}", pinlv_qianzhui, zhiwen)
}

/// 检查指纹频率是否超限
pub async fn jiancharate(zhiwen: &str) -> bool {
    rediscaozuo::zizengdaiguoqi(&pinlvjian(zhiwen), pinlv_chuangkou_miao)
        .await
        .map_or(false, |cishu| cishu <= pinlv_shangxian)
}

/// 根据指纹获取已有会话或创建新会话，返回 (会话ID, 公钥base64)
pub async fn huoquhuochuangjian(zhiwen: &str) -> Option<(String, String)> {
    if let Some(yiyou_huihuaid) = rediscaozuo::huoqu::<String>(&zhiwenjian(zhiwen)).await {
        if let Some(gongyao) = rediscaozuo::hhuoqu::<String>(&huihuajian(&yiyou_huihuaid), "gongyao").await {
            xuqihuihua(&yiyou_huihuaid).await;
            return Some((yiyou_huihuaid, gongyao));
        }
    }
    chuangjiaxinhuihua(zhiwen).await
}

async fn chuangjiaxinhuihua(zhiwen: &str) -> Option<(String, String)> {
    let peizhi = jichugongju::Suijipeizhi::xin()
        .shezhi_changdu(huihuaid_changdu)
        .shezhi_hunluan(true);
    let huihuaid = jichugongju::shengchengsuijizifuchuan(&peizhi);
    let (siyao, gongyao) = jiamigongju::shengchengyaodui();
    let siyao_b64 = jiamigongju::zhuanbase64(&siyao);
    let gongyao_b64 = jiamigongju::zhuanbase64(&gongyao);
    let jian = huihuajian(&huihuaid);
    rediscaozuo::hpiliangshezhi(&jian, &[
        ("siyao", &siyao_b64),
        ("gongyao", &gongyao_b64),
        ("zhiwen", zhiwen),
    ]).await.then_some(())?;
    rediscaozuo::shezhiguoqi(&jian, huihua_ttl_miao).await;
    rediscaozuo::shezhidaiguoqi(&zhiwenjian(zhiwen), &huihuaid, huihua_ttl_miao).await;
    Some((huihuaid, gongyao_b64))
}

/// 从 Redis 取出服务端私钥字节
pub async fn huoqusiyao(huihuaid: &str) -> Option<Vec<u8>> {
    let siyao_b64: String = rediscaozuo::hhuoqu(&huihuajian(huihuaid), "siyao").await?;
    jiamigongju::congbase64(&siyao_b64)
}

/// 续期会话 TTL
pub async fn xuqihuihua(huihuaid: &str) {
    let jian = huihuajian(huihuaid);
    rediscaozuo::shezhiguoqi(&jian, huihua_ttl_miao).await;
    if let Some(zhiwen) = rediscaozuo::hhuoqu::<String>(&jian, "zhiwen").await {
        rediscaozuo::shezhiguoqi(&zhiwenjian(&zhiwen), huihua_ttl_miao).await;
    }
}
