use actix_web::{HttpRequest, HttpResponse};
use crate::jiekouxt::jiekouxtzhuti::{Jiekoudinyi, Qingqiufangshi};
use crate::gongju::ai::openai::diaoduqi;

#[allow(non_upper_case_globals)]
pub const dinyi: Jiekoudinyi = Jiekoudinyi {
    lujing: "/diaoduqi/jiankong",
    nicheng: "AI调度器监控指标",
    jieshao: "输出AI调度器Prometheus监控指标，需要管理员权限",
    fangshi: Qingqiufangshi::Get,
    jiami: false,
    xudenglu: true,
    xuyonghuzu: true,
    yunxuputong: false,
};

pub async fn chuli(_req: HttpRequest) -> HttpResponse {
    let zt = diaoduqi::chaxun_zhuangtai();
    let paidui_chaoshi = diaoduqi::huoqu_paidui_chaoshi_miao();
    let shuju = format!(
        "# HELP ai_diaoduqi_quanju_shangxian AI调度器全局并发上限\n\
# TYPE ai_diaoduqi_quanju_shangxian gauge\n\
ai_diaoduqi_quanju_shangxian {}\n\
# HELP ai_diaoduqi_dangqian_bingfashu AI调度器当前并发执行数\n\
# TYPE ai_diaoduqi_dangqian_bingfashu gauge\n\
ai_diaoduqi_dangqian_bingfashu {}\n\
# HELP ai_diaoduqi_dengdaishu AI调度器当前排队等待数\n\
# TYPE ai_diaoduqi_dengdaishu gauge\n\
ai_diaoduqi_dengdaishu {}\n\
# HELP ai_diaoduqi_shengyu_weizhi AI调度器剩余可用并发位\n\
# TYPE ai_diaoduqi_shengyu_weizhi gauge\n\
ai_diaoduqi_shengyu_weizhi {}\n\
# HELP ai_diaoduqi_paidui_chaoshi_miao AI调度器排队超时秒数\n\
# TYPE ai_diaoduqi_paidui_chaoshi_miao gauge\n\
ai_diaoduqi_paidui_chaoshi_miao {}\n",
        zt.quanju_shangxian,
        zt.dangqian_bingfashu,
        zt.dengdaishu,
        zt.shengyu_weizhi(),
        paidui_chaoshi,
    );
    HttpResponse::Ok()
        .content_type("text/plain; version=0.0.4; charset=utf-8")
        .body(shuju)
}
