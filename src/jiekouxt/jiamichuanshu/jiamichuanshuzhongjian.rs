use actix_web::{HttpRequest, HttpResponse};
use actix_web::body::MessageBody;
use crate::gongju::jiamigongju;
use crate::jiekouxt::jiekouxtzhuti;
use super::huihuaguanli;

#[allow(non_upper_case_globals)]
const toubu_huihuaid: &str = "X-Huihua-Id";
#[allow(non_upper_case_globals)]
const toubu_kehugongyao: &str = "X-Kehugongyao";

fn huoqutoubu<'a>(req: &'a HttpRequest, mingcheng: &str) -> Option<&'a str> {
    req.headers().get(mingcheng)?.to_str().ok()
}

/// 从请求头中提取会话信息并派生 AES 密钥
pub async fn paishengyao(req: &HttpRequest) -> Option<Vec<u8>> {
    let huihuaid = huoqutoubu(req, toubu_huihuaid)?;
    let kehugongyao_b64 = huoqutoubu(req, toubu_kehugongyao)?;
    let fuwuqisiyao = huihuaguanli::huoqusiyao(huihuaid).await?;
    let kehugongyao = jiamigongju::congbase64(kehugongyao_b64)?;
    let gongxiangyao = jiamigongju::xieshanggongxiangyao(&fuwuqisiyao, &kehugongyao)?;
    let miyao = jiamigongju::paishengyao(&gongxiangyao, jiamigongju::yanfen)?;
    huihuaguanli::xuqihuihua(huihuaid).await;
    Some(miyao)
}

/// 解密请求体
pub fn jiemiqingqiuti(jiamishuju: &[u8], miyao: &[u8]) -> Option<Vec<u8>> {
    let miwen = jiamigongju::congbase64(std::str::from_utf8(jiamishuju).ok()?)?;
    jiamigongju::jiemi(&miwen, miyao)
}

/// 加密响应体并返回加密后的 HttpResponse
pub fn jiamixiangying(xiangying: HttpResponse, miyao: &[u8]) -> HttpResponse {
    let (_head, body) = xiangying.into_parts();
    let mingwen = match body.try_into_bytes() {
        Ok(zijie) => zijie,
        Err(_) => return jiekouxtzhuti::shibai(500, "读取响应体失败"),
    };
    match jiamigongju::jiami(&mingwen, miyao) {
        Some(miwen) => {
            let base64_miwen = jiamigongju::zhuanbase64(&miwen);
            HttpResponse::Ok()
                .content_type("text/plain")
                .body(base64_miwen)
        }
        None => jiekouxtzhuti::shibai(500, "加密响应失败"),
    }
}
