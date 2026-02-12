use actix_web::web;
use super::jiekou_nr;
use crate::gongju::jichugongju;
use crate::shujuku::psqlshujuku::psqlcaozuo;

/// 请求方式
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Qingqiufangshi {
    Get,
    Post,
    Sse,
}

impl Qingqiufangshi {
    pub fn wenben(&self) -> &'static str {
        match self {
            Qingqiufangshi::Get => "GET",
            Qingqiufangshi::Post => "POST",
            Qingqiufangshi::Sse => "SSE",
        }
    }
}

/// 接口定义，所有接口文件必须提供
pub struct Jiekoudinyi {
    pub lujing: &'static str,
    pub nicheng: &'static str,
    pub jieshao: &'static str,
    pub fangshi: Qingqiufangshi,
    pub jiami: bool,
    pub xudenglu: bool,
    pub xuyonghuzu: bool,
    pub yunxuputong: bool,
}

/// 接口注册信息，用于同步到数据库
pub struct Jiekouzhucexinxi {
    pub qianzhui: &'static str,
    pub dinyi: &'static Jiekoudinyi,
}

impl Jiekouzhucexinxi {
    pub fn wanzhenglujing(&self) -> String {
        format!("{}{}", self.qianzhui, self.dinyi.lujing)
    }
}

fn boolzhi(zhi: bool) -> &'static str {
    if zhi { "1" } else { "0" }
}

#[allow(non_upper_case_globals)]
const jiekou_upsert_sql: &str = "\
INSERT INTO \"jiekoujilubiao\" (\"lujing\", \"nicheng\", \"jieshao\", \"fangshi\", \"jiami\", \"xudenglu\", \"xuyonghuzu\", \"yunxuputong\", \"chuangjianshijian\", \"gengxinshijian\") \
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $9) \
ON CONFLICT (\"lujing\") DO UPDATE SET \
\"nicheng\" = $2, \"jieshao\" = $3, \"fangshi\" = $4, \"jiami\" = $5, \"xudenglu\" = $6, \"xuyonghuzu\" = $7, \"yunxuputong\" = $8, \"gengxinshijian\" = $9";

/// 同步所有接口定义到数据库
pub async fn tongbujiekoulie(jiekoulie: &[Jiekouzhucexinxi]) -> bool {
    let shijianchuo = jichugongju::huoqushijianchuo().to_string();
    for jiekou in jiekoulie {
        let lujing = jiekou.wanzhenglujing();
        let d = jiekou.dinyi;
        if psqlcaozuo::zhixing(
            jiekou_upsert_sql,
            &[&lujing, d.nicheng, d.jieshao, d.fangshi.wenben(), boolzhi(d.jiami), boolzhi(d.xudenglu), boolzhi(d.xuyonghuzu), boolzhi(d.yunxuputong), &shijianchuo],
        ).await.is_none() {
            return false;
        }
    }
    true
}

/// 根据接口定义的请求方式返回对应的 actix-web 路由方法
pub fn huoqufangfa(fangshi: Qingqiufangshi) -> fn() -> actix_web::Route {
    match fangshi {
        Qingqiufangshi::Get | Qingqiufangshi::Sse => web::get,
        Qingqiufangshi::Post => web::post,
    }
}

/// 配置所有接口路由，挂载到 App
pub fn peizhi(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/jiekou")
            .configure(jiekou_nr::zhuce)
    );
}
