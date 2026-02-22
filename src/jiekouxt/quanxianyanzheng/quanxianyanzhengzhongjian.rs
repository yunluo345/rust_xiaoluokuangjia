use actix_web::{
    body::EitherBody,
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage, HttpResponse, http::Method,
};
use std::collections::HashSet;
use std::future::{ready, Ready};
use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;
use std::task::{Context, Poll};
use crate::shujuku::psqlshujuku::shujubiao_nr::yonghu::yonghuyanzheng::{self, Lingpaicuowu};
use crate::gongju::jichugongju;

pub struct Quanxianyanzheng(pub Arc<HashSet<String>>);

impl<S, B> Transform<S, ServiceRequest> for Quanxianyanzheng
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = Quanxianyanzhengzhongjian<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(Quanxianyanzhengzhongjian {
            service: Rc::new(service),
            mianyanzhenglie: Arc::clone(&self.0),
        }))
    }
}

pub struct Quanxianyanzhengzhongjian<S> {
    service: Rc<S>,
    mianyanzhenglie: Arc<HashSet<String>>,
}

impl<S, B> Service<ServiceRequest> for Quanxianyanzhengzhongjian<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let svc = Rc::clone(&self.service);
        let mianyanzheng = Arc::clone(&self.mianyanzhenglie);

        Box::pin(async move {
            if req.method() == Method::OPTIONS {
                return svc.call(req).await.map(|res| res.map_into_left_body());
            }

            let lujing = req.path().to_string();
            if mianyanzheng.contains(&lujing) {
                return svc.call(req).await.map(|res| res.map_into_left_body());
            }

            let lingpai = req.headers()
                .get("Authorization")
                .and_then(|h| h.to_str().ok())
                .and_then(|s| s.strip_prefix("Bearer "))
                .map(|s| s.to_string());

            match lingpai {
                Some(token) => {
                    match yonghuyanzheng::yanzhenglingpaijiquanxian(&token, &lujing).await {
                        Ok(zaiti) => {
                            req.extensions_mut().insert(zaiti);
                            svc.call(req).await.map(|res| res.map_into_left_body())
                        }
                        Err(e) => {
                        let (ma, xi) = match e {
                                Lingpaicuowu::Wuxiao => (401, "令牌无效或已过期".to_string()),
                                Lingpaicuowu::Yibeifengjin(y) => (403, format!("账号已被封禁：{}", y)),
                                Lingpaicuowu::Quanxianbuzu => (403, "权限不足，无法访问该接口".to_string()),
                            };
                            let xiangying = HttpResponse::Ok()
                                .json(serde_json::json!({ "zhuangtaima": ma, "xiaoxi": xi, "shijianchuo": jichugongju::huoqushijianchuo() }));
                            Ok(req.into_response(xiangying).map_into_right_body())
                        }
                    }
                }
                None => {
                    let xiangying = HttpResponse::Ok()
                        .json(serde_json::json!({ "zhuangtaima": 401, "xiaoxi": "缺少授权令牌", "shijianchuo": jichugongju::huoqushijianchuo() }));
                    Ok(req.into_response(xiangying).map_into_right_body())
                }
            }
        })
    }
}
