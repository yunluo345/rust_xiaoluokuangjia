use actix_web::{HttpRequest, HttpResponse, web};
use serde::Deserialize;
use crate::jiekouxt::jiekouxtzhuti::{self, Jiekoudinyi, Qingqiufangshi};
use crate::jiekouxt::jiamichuanshu::jiamichuanshuzhongjian;
use crate::shujuku::psqlshujuku::shujubiao_nr::ai::shujucaozuo_aiqudao as qudaocaozuo;

#[allow(non_upper_case_globals)]
pub const dinyi: Jiekoudinyi = Jiekoudinyi {
    lujing: "/aiqudao",
    nicheng: "AI渠道管理",
    jieshao: "AI渠道的增删改查及状态管理，通过caozuo字段区分操作",
    fangshi: Qingqiufangshi::Post,
    jiami: true,
    xudenglu: true,
    xuyonghuzu: true,
    yunxuputong: false,
};

#[allow(non_upper_case_globals)]
const wanzhenglujing: &str = "/jiekou/ai/aiqudao";

#[derive(Deserialize)]
struct Qingqiuti {
    caozuo: String,
    #[serde(default)]
    id: Option<String>,
    #[serde(default)]
    mingcheng: Option<String>,
    #[serde(default)]
    leixing: Option<String>,
    #[serde(default)]
    jiekoudizhi: Option<String>,
    #[serde(default)]
    miyao: Option<String>,
    #[serde(default)]
    moxing: Option<String>,
    #[serde(default)]
    wendu: Option<String>,
    #[serde(default)]
    beizhu: Option<String>,
    #[serde(default)]
    zuidatoken: Option<String>,
    #[serde(default)]
    youxianji: Option<String>,
}

fn jiamishibai(zhuangtaima: u16, xiaoxi: impl Into<String>, yao: &[u8]) -> HttpResponse {
    jiamichuanshuzhongjian::jiamixiangying(jiekouxtzhuti::shibai(zhuangtaima, xiaoxi), yao)
}

fn jiamichenggong<T: serde::Serialize>(xiaoxi: impl Into<String>, shuju: T, yao: &[u8]) -> HttpResponse {
    jiamichuanshuzhongjian::jiamixiangying(jiekouxtzhuti::chenggong(xiaoxi, shuju), yao)
}

async fn chuli_liebiao(yao: &[u8]) -> HttpResponse {
    match qudaocaozuo::chaxun_quanbu().await {
        Some(lie) => jiamichenggong("查询成功", lie, yao),
        None => jiamishibai(500, "查询渠道列表失败", yao),
    }
}

async fn chuli_xiangqing(qingqiu: &Qingqiuti, yao: &[u8]) -> HttpResponse {
    let id = match qingqiu.id.as_deref().filter(|s| !s.is_empty()) {
        Some(v) => v,
        None => return jiamishibai(400, "缺少渠道ID", yao),
    };
    match qudaocaozuo::chaxun_id(id).await {
        Some(shuju) => jiamichenggong("查询成功", shuju, yao),
        None => jiamishibai(404, "渠道不存在", yao),
    }
}

async fn chuli_xinzeng(qingqiu: &Qingqiuti, yao: &[u8]) -> HttpResponse {
    let (mingcheng, leixing, jiekoudizhi, miyao_zhi, moxing) = match (
        qingqiu.mingcheng.as_deref().filter(|s| !s.is_empty()),
        qingqiu.leixing.as_deref().filter(|s| !s.is_empty()),
        qingqiu.jiekoudizhi.as_deref().filter(|s| !s.is_empty()),
        qingqiu.miyao.as_deref().filter(|s| !s.is_empty()),
        qingqiu.moxing.as_deref().filter(|s| !s.is_empty()),
    ) {
        (Some(a), Some(b), Some(c), Some(d), Some(e)) => (a, b, c, d, e),
        _ => return jiamishibai(400, "名称、类型、接口地址、密钥、模型为必填项", yao),
    };
    if !qudaocaozuo::leixingyunxu(leixing) {
        return jiamishibai(400, format!("不支持的渠道类型，仅允许：{}", qudaocaozuo::yunxuleixing.join("、")), yao);
    }
    if qudaocaozuo::mingchengcunzai(mingcheng).await {
        return jiamishibai(409, "渠道名称已存在", yao);
    }
    let wendu = qingqiu.wendu.as_deref().unwrap_or("0");
    match qudaocaozuo::xinzeng(mingcheng, leixing, jiekoudizhi, miyao_zhi, moxing, wendu, qingqiu.beizhu.as_deref(), qingqiu.zuidatoken.as_deref()).await {
        Some(id) => jiamichenggong("新增成功", serde_json::json!({"id": id}), yao),
        None => jiamishibai(500, "新增渠道失败", yao),
    }
}

async fn chuli_gengxin(qingqiu: &Qingqiuti, yao: &[u8]) -> HttpResponse {
    let id = match qingqiu.id.as_deref().filter(|s| !s.is_empty()) {
        Some(v) => v,
        None => return jiamishibai(400, "缺少渠道ID", yao),
    };
    if let Some(lx) = qingqiu.leixing.as_deref().filter(|s| !s.is_empty()) {
        if !qudaocaozuo::leixingyunxu(lx) {
            return jiamishibai(400, format!("不支持的渠道类型，仅允许：{}", qudaocaozuo::yunxuleixing.join("、")), yao);
        }
    }
    let duiying: &[(&str, &Option<String>)] = &[
        ("mingcheng", &qingqiu.mingcheng),
        ("leixing", &qingqiu.leixing),
        ("jiekoudizhi", &qingqiu.jiekoudizhi),
        ("miyao", &qingqiu.miyao),
        ("moxing", &qingqiu.moxing),
        ("wendu", &qingqiu.wendu),
        ("beizhu", &qingqiu.beizhu),
        ("zuidatoken", &qingqiu.zuidatoken),
        ("youxianji", &qingqiu.youxianji),
    ];
    let ziduanlie: Vec<(&str, &str)> = duiying.iter()
        .filter_map(|(ming, zhi)| {
            // 备注字段允许为空字符串，其他字段必须非空
            if *ming == "beizhu" {
                zhi.as_deref().map(|v| (*ming, v))
            } else {
                zhi.as_deref().filter(|s| !s.is_empty()).map(|v| (*ming, v))
            }
        })
        .collect();
    if ziduanlie.is_empty() {
        return jiamishibai(400, "没有需要更新的字段", yao);
    }
    match qudaocaozuo::gengxin(id, &ziduanlie).await {
        Some(0) => jiamishibai(404, "渠道不存在", yao),
        Some(_) => jiamichenggong("更新成功", serde_json::json!({}), yao),
        None => jiamishibai(500, "更新渠道失败", yao),
    }
}

async fn chuli_shanchu(qingqiu: &Qingqiuti, yao: &[u8]) -> HttpResponse {
    let id = match qingqiu.id.as_deref().filter(|s| !s.is_empty()) {
        Some(v) => v,
        None => return jiamishibai(400, "缺少渠道ID", yao),
    };
    match qudaocaozuo::shanchu(id).await {
        Some(0) => jiamishibai(404, "渠道不存在", yao),
        Some(_) => jiamichenggong("删除成功", serde_json::json!({}), yao),
        None => jiamishibai(500, "删除渠道失败", yao),
    }
}

async fn chuli_qiehuanzhuangtai(qingqiu: &Qingqiuti, yao: &[u8]) -> HttpResponse {
    let id = match qingqiu.id.as_deref().filter(|s| !s.is_empty()) {
        Some(v) => v,
        None => return jiamishibai(400, "缺少渠道ID", yao),
    };
    match qudaocaozuo::qiehuanzhuangtai(id).await {
        Some(0) => jiamishibai(404, "渠道不存在", yao),
        Some(_) => jiamichenggong("状态切换成功", serde_json::json!({}), yao),
        None => jiamishibai(500, "切换状态失败", yao),
    }
}

async fn fenfa(mingwen: &[u8], yao: &[u8]) -> HttpResponse {
    let qingqiu: Qingqiuti = match serde_json::from_slice(mingwen) {
        Ok(q) => q,
        Err(_) => return jiamishibai(400, "请求参数格式错误", yao),
    };
    match qingqiu.caozuo.as_str() {
        "liebiao" => chuli_liebiao(yao).await,
        "xiangqing" => chuli_xiangqing(&qingqiu, yao).await,
        "xinzeng" | "tianjia" => chuli_xinzeng(&qingqiu, yao).await,
        "gengxin" => chuli_gengxin(&qingqiu, yao).await,
        "shanchu" => chuli_shanchu(&qingqiu, yao).await,
        "qiehuanzhuangtai" => chuli_qiehuanzhuangtai(&qingqiu, yao).await,
        _ => jiamishibai(400, "不支持的操作类型", yao),
    }
}

/// AI渠道管理接口处理函数
pub async fn chuli(req: HttpRequest, ti: web::Bytes) -> HttpResponse {
    if let Err(xiangying) = jiekouxtzhuti::jiaoyanquanxian(&req, &dinyi, wanzhenglujing).await {
        return xiangying;
    }
    let yao = match jiamichuanshuzhongjian::paishengyao(&req).await {
        Some(m) => m,
        None => return jiekouxtzhuti::shibai(401, "加密会话无效"),
    };
    match jiamichuanshuzhongjian::jiemiqingqiuti(&ti, &yao) {
        Some(mingwen) => fenfa(&mingwen, &yao).await,
        None => jiekouxtzhuti::shibai(400, "解密请求体失败"),
    }
}
