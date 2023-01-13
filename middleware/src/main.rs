// use std::pin::Pin;
// use std::task::{Context, Poll};
//
// use actix_service::{Service, Transform};
// use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error};
// use futures::future::{ok, Ready};
// use futures::Future;
//
// pub struct SayHi;
//
// impl<S, B> Transform<S> for SayHi
// where
//     S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
//     S::Future: 'static,
//     B: 'static,
// {
//     type Request = ServiceRequest;
//     type Response = ServiceResponse<B>;
//     type Error = Error;
//     type InitError = ();
//     type Transform = SayHiMiddleware<S>;
//     type Future = Ready<Result<Self::Transform, Self::InitError>>;
//
//     fn new_transform(&self, service: S) -> Self::Future {
//         ok(SayHiMiddleware { service })
//     }
// }
//
// pub struct SayHiMiddleware<S> {
//     service: S,
// }
//
// impl<S, B> Service for SayHiMiddleware<S>
// where
//     S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
//     S::Future: 'static,
//     B: 'static,
// {
//     type Request = ServiceRequest;
//     type Response = ServiceResponse<B>;
//     type Error = Error;
//     type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;
//
//     fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
//         self.service.poll_ready(cx)
//     }
//
//     fn call(&mut self, req: ServiceRequest) -> Self::Future {
//         println!("Hi from start. You requested: {}", req.path());
//
//         let fut = self.service.call(req);
//
//         Box::pin(async move {
//             let res = fut.await?;
//
//             println!("Hi from response");
//             Ok(res)
//         })
//     }
// }

// use actix_service::Service;
// use actix_web::{web, App};
// use futures::future::FutureExt;
//
// #[actix_rt::main]
// async fn main() {
//     let app = App::new()
//         .wrap_fn(|req, srv| {
//             println!("Hi from start. You requested: {}", req.path());
//             srv.call(req).map(|res| {
//                 println!("Hi from response");
//                 res
//             })
//         })
//         .route(
//             "/index.html",
//             web::get().to(|| async {
//                 "Hello, middle!"
//             })
//         );
// }

// use actix_web::middleware::Logger;
// use env_logger::Env;
//
// #[actix_rt::main]
// async fn main() -> std::io::Result<()> {
//     use actix_web::{App, HttpServer};
//
//     env_logger::from_env(Env::default().default_filter_or("info")).init();
//
//     HttpServer::new(|| {
//         App::new()
//             .wrap(Logger::default())
//             .wrap(Logger::new("%a %{User-Agent}i"))
//     })
//     .bind("127.0.0.1:8088")?
//     .run()
//     .await
// }

// use actix_web::{http, middleware, HttpResponse};
//
// #[actix_rt::main]
// async fn main() -> std::io::Result<()> {
//     use actix_web::{web, App, HttpServer};
//
//     HttpServer::new(|| {
//         App::new()
//             .wrap(middleware::DefaultHeaders::new().header("X-Version", "0.2"))
//             .service(
//                 web::resource("/test")
//                     .route(web::get().to(|| HttpResponse::Ok()))
//                     .route(
//                         web::method(http::Method::HEAD)
//                             .to(|| HttpResponse::MethodNotAllowed()),
//                     ),
//             )
//     })
//     .bind("127.0.0.1:8088")?
//     .run()
//     .await
// }

// use actix_session::{CookieSession, Session};
// use actix_web::{web, App, Error, HttpResponse, HttpServer};
//
// async fn index(session: Session) -> Result<HttpResponse, Error> {
//     if let Some(count) = session.get::<i32>("counter")? {
//         session.set("counter", count + 1)?;
//     } else {
//         session.set("counter", 1)?;
//     }
//
//     Ok(HttpResponse::Ok().body(format!(
//         "Count is {:?}!",
//         session.get::<i32>("counter")?.unwrap()
//     )))
// }
//
// #[actix_rt::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .wrap(
//                 CookieSession::signed(&[0; 32])
//                     .secure(false)
//             )
//             .service(web::resource("/").to(index))
//     })
//     .bind("127.0.0.1:8088")?
//     .run()
//     .await
// }

use actix_web::middleware::errhandlers::{ErrorHandlerResponse, ErrorHandlers};
use actix_web::{dev, http, HttpResponse, Result};

fn render_500<B>(mut res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    res.response_mut().headers_mut().insert(
        http::header::CONTENT_TYPE,
        http::HeaderValue::from_static("Error"),
    );
    Ok(ErrorHandlerResponse::Response(res))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{web, App, HttpServer};

    HttpServer::new(|| {
        App::new()
            .wrap(
                ErrorHandlers::new()
                    .handler(http::StatusCode::INTERNAL_SERVER_ERROR, render_500),
            )
            .service(
                web::resource("/test")
                    .route(web::get().to(|| HttpResponse::Ok()))
                    .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
                )
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
