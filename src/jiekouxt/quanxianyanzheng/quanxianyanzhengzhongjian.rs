use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage, http::Method,
};
use std::future::{ready, Ready};
use std::pin::Pin;
use std::task::{Context, Poll};
use crate::jiekouxt::jiekouxtzhuti;
use crate::shujuku::psqlshujuku::shujubiao_nr::yonghu::yonghuyanzheng::{self, Lingpaicuowu};

#[allow(non_upper_case_globals)]
const toubu_shouquan: &str = "Authorization";

pub struct Quanxianyanzheng;

impl<S, B> Transform<S, ServiceRequest> for Quanxianyanzheng
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = Quanxianyanzhengzhongjian<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(Quanxianyanzhengzhongjian { service }))
    }
}

pub struct Quanxianyanzhengzhongjian<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for Quanxianyanzhengzhongjian<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        if req.method() == Method::OPTIONS {
            return Box::pin(self.service.call(req));
        }

        let lujing = req.path().to_string();
        let lingpai = req.headers()
            .get(toubu_shouquan)
            .and_then(|h| h.to_str().ok())
            .and_then(|s| s.strip_prefix("Bearer "))
            .map(|s| s.to_string());

        let fut = self.service.call(req);

        Box::pin(async move {
            if let Some(token) = lingpai {
                match yonghuyanzheng::yanzhenglingpaijiquanxian(&token, &lujing).await {
                    Ok(zaiti) => {
                        let res = fut.await?;
                        res.request().extensions_mut().insert(zaiti);
                        Ok(res)
                    }
                    Err(Lingpaicuowu::Wuxiao) => {
                        Err(actix_web::error::ErrorUnauthorized("令牌无效或已过期"))
                    }
                    Err(Lingpaicuowu::Yibeifengjin(yuanyin)) => {
                        Err(actix_web::error::ErrorForbidden(format!("账号已被封禁：{}", yuanyin)))
                    }
                    Err(Lingpaicuowu::Quanxianbuzu) => {
                        Err(actix_web::error::ErrorForbidden("权限不足，无法访问该接口"))
                    }
                }
            } else {
                Err(actix_web::error::ErrorUnauthorized("缺少授权令牌"))
            }
        })
    }
}
