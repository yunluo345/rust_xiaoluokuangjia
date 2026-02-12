use actix_web::{HttpRequest, HttpResponse, web};
use serde::{Deserialize, Serialize};
use crate::jiekouxt::jiekouxtzhuti::{self, Jiekoudinyi, Qingqiufangshi};
use crate::jiekouxt::jiamichuanshu::huihuaguanli;

#[allow(non_upper_case_globals)]
pub const dinyi: Jiekoudinyi = Jiekoudinyi {
    lujing: "/gongyao",
    nicheng: "公钥交换",
    jieshao: "提交设备指纹，获取服务端公钥和会话ID",
    fangshi: Qingqiufangshi::Post,
    jiami: false,
    xudenglu: false,
    xuyonghuzu: false,
    yunxuputong: false,
};

#[allow(non_upper_case_globals)]
const zhiwen_changdu: usize = 64;

#[derive(Deserialize)]
pub struct Qingqiuti {
    zhiwen: String,
}

#[derive(Serialize)]
struct Xiangyingshuju {
    huihuaid: String,
    gongyao: String,
}

fn yanzhengzhiwengeshi(zhiwen: &str) -> bool {
    zhiwen.len() == zhiwen_changdu && zhiwen.chars().all(|c| c.is_ascii_hexdigit())
}

/// 公钥交换接口处理函数
pub async fn chuli(_req: HttpRequest, ti: web::Json<Qingqiuti>) -> HttpResponse {
    if !yanzhengzhiwengeshi(&ti.zhiwen) {
        return jiekouxtzhuti::shibai(400, "指纹格式无效");
    }
    if !huihuaguanli::jiancharate(&ti.zhiwen).await {
        return jiekouxtzhuti::shibai(429, "请求过于频繁");
    }
    match huihuaguanli::huoquhuochuangjian(&ti.zhiwen).await {
        Some((huihuaid, gongyao)) => {
            jiekouxtzhuti::chenggong("公钥交换成功", Xiangyingshuju { huihuaid, gongyao })
        }
        None => jiekouxtzhuti::shibai(500, "会话创建失败"),
    }
}
