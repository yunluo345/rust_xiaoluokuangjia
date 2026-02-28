use actix_web::{HttpRequest, HttpResponse, web};
use serde::Deserialize;
use crate::jiekouxt::jiekouxtzhuti::{self, Jiekoudinyi, Qingqiufangshi};
use crate::gongju::ai::openai::diaoduqi;

#[allow(non_upper_case_globals)]
pub const dinyi: Jiekoudinyi = Jiekoudinyi {
    lujing: "/diaoduqi/gengxin",
    nicheng: "AI调度器热更新",
    jieshao: "在线更新AI调度器并发上限与排队超时，需要管理员权限",
    fangshi: Qingqiufangshi::Post,
    jiami: false,
    xudenglu: true,
    xuyonghuzu: true,
    yunxuputong: false,
};

#[derive(Deserialize)]
struct Qingqiuti {
    quanju_shangxian: Option<u32>,
    paidui_chaoshi_miao: Option<u64>,
}

#[allow(non_upper_case_globals)]
const zuixiao_quanju_shangxian: u32 = 1;
#[allow(non_upper_case_globals)]
const zuixiao_paidui_chaoshi_miao: u64 = 1;

pub async fn chuli(_req: HttpRequest, ti: web::Bytes) -> HttpResponse {
    let qingqiu = match serde_json::from_slice::<Qingqiuti>(&ti) {
        Ok(q) => q,
        Err(_) => return jiekouxtzhuti::shibai(400, "请求参数格式错误"),
    };

    let mut yigengxin = false;

    if let Some(shangxian) = qingqiu.quanju_shangxian {
        if shangxian < zuixiao_quanju_shangxian {
            return jiekouxtzhuti::shibai(400, "全局并发上限必须大于等于1");
        }
        diaoduqi::regengxin_shangxian(shangxian);
        yigengxin = true;
    }

    if let Some(chaoshi) = qingqiu.paidui_chaoshi_miao {
        if chaoshi < zuixiao_paidui_chaoshi_miao {
            return jiekouxtzhuti::shibai(400, "排队超时秒数必须大于等于1");
        }
        diaoduqi::regengxin_chaoshi(chaoshi);
        yigengxin = true;
    }

    if !yigengxin {
        return jiekouxtzhuti::shibai(400, "至少提供一个可更新字段");
    }

    let zt = diaoduqi::chaxun_zhuangtai();
    jiekouxtzhuti::chenggong("更新成功", serde_json::json!({
        "quanju_shangxian": zt.quanju_shangxian,
        "dangqian_bingfashu": zt.dangqian_bingfashu,
        "dengdaishu": zt.dengdaishu,
        "shengyu_weizhi": zt.shengyu_weizhi(),
        "paidui_chaoshi_miao": diaoduqi::huoqu_paidui_chaoshi_miao(),
    }))
}
