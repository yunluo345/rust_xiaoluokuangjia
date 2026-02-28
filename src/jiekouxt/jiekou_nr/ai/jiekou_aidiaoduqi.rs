use actix_web::{HttpResponse, HttpRequest};
use crate::jiekouxt::jiekouxtzhuti::{self, Jiekoudinyi, Qingqiufangshi};
use crate::gongju::ai::openai::diaoduqi;

#[allow(non_upper_case_globals)]
pub const dinyi: Jiekoudinyi = Jiekoudinyi {
    lujing: "/diaoduqi",
    nicheng: "AI调度器状态",
    jieshao: "查询AI调度器当前队列状态：并发数、等待数、剩余位置",
    fangshi: Qingqiufangshi::Get,
    jiami: false,
    xudenglu: true,
    xuyonghuzu: true,
    yunxuputong: false,
};

pub async fn chuli(_req: HttpRequest) -> HttpResponse {
    let zt = diaoduqi::chaxun_zhuangtai();
    jiekouxtzhuti::chenggong("查询成功", serde_json::json!({
        "quanju_shangxian": zt.quanju_shangxian,
        "dangqian_bingfashu": zt.dangqian_bingfashu,
        "dengdaishu": zt.dengdaishu,
        "shengyu_weizhi": zt.shengyu_weizhi(),
    }))
}
