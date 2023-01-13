// use actix_web::{guard, web, App, HttpResponse};
//
// fn index() -> HttpResponse {
//     HttpResponse::Ok().body("Hello")
// }
//
// pub fn main() {
//     App::new()
//         .service(web::resource("/prefix").to(index))
//         .service(
//             web::resource("/user/{name}")
//                 .name("user_detail")
//                 .guard(guard::Header("content-type", "application/json"))
//                 .route(web::get().to(|| HttpResponse::Ok()))
//                 .route(web::put().to(|| HttpResponse::Ok()))
//         );
// }

// App::new().service(
//     web::resource("/path").route(
//         web::route()
//             .guard(guard::Get())
//             .guard(guard::Headder("content-type", "text/plain"))
//             .to(|| HttpResponse::Ok()),
//     ),
// )

// use actix_web::{guard, web, App, HttpResponse, HttpServer};
//
// async fn show_users() -> HttpResponse {
//     HttpResponse::Ok().body("Show users")
// }
//
// async fn user_detail(path: web::Path<(u32,)>) -> HttpResponse {
//     HttpResponse::Ok().body(format!("User detail: {}", path.0))
// }
//
// #[actix_rt::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new().service(
//             web::scope("/users")
//                 .route("/show", web::get().to(show_users))
//                 .route("show/{id}", web::get().to(user_detail)),
//         )
//     })
//     .bind("127.0.0.1:8088")?
//     .run()
//     .await
// }

// use actix_web::{HttpRequest, HttpResponse, Result};
//
// async fn index(req: HttpRequest) -> Result<String> {
//     let v1: u8 = req.match_info().get("v1").unwrap().parse().unwrap();
//     let v2: u8 = req.match_info().query("v2").parse().unwrap();
//     let (v3, v4): (u8, u8) = req.match_info().load().unwrap();
//     Ok(format!("Values {} {} {} {}", v1, v2, v3, v4))
// }
//
// #[actix_rt::main]
// async fn main() -> std::io::Result<()> {
//     use actix_web::{web, App, HttpServer};
//
//     HttpServer::new(|| {
//         App::new()
//             .route("/a/{v1}/{v2}/", web::get().to(index))
//             .route("", web::get().to(|| HttpResponse::Ok()))
//     })
//     .bind("127.0.0.1:8088")?
//     .run()
//     .await
// }


// use actix_web::{HttpRequest, Result};
// use std::path::PathBuf;
//
// async fn index(req: HttpRequest) -> Result<String> {
//     let path: PathBuf = req.match_info().query("tail").parse().unwrap();
//     Ok(format!("Path {:?}", path))
// }
//
// #[actix_rt::main]
// async fn main() -> std::io::Result<()> {
//     use actix_web::{web, App, HttpServer};
//
//     HttpServer::new(|| App::new().route(r"/a/{tail:.*}", web::get().to(index)))
//         .bind("127.0.0.1:8088")?
//         .run()
//         .await
// }

// use actix_web::{web, Result};
//
// async fn index(info: web::Path<(String, u32)>) -> Result<String> {
//     Ok(format!("Welcome {}! id: {}", info.0, info.1))
// }
//
// #[actix_rt::main]
// async fn main() -> std::io::Result<()> {
//     use actix_web::{App, HttpServer};
//
//     HttpServer::new(|| {
//         App::new().route(
//             "/{username}/{id}/index.html",
//             web::get().to(index),
//         )
//     })
//     .bind("127.0.0.1:8088")?
//     .run()
//     .await
// }

// use actix_web::{web, Result};
// use serde::Deserialize;
//
// #[derive(Deserialize)]
// struct Info {
//     username: String,
// }
//
// async fn index(info: web::Path<Info>) -> Result<String> {
//     Ok(format!("Welcome {}!", info.username))
// }
//
// #[actix_rt::main]
// async fn main() -> std::io::Result<()> {
//     use actix_web::{App, HttpServer};
//
//     HttpServer::new(|| {
//         App::new().route(
//             "/{username}/index.html",
//             web::get().to(index),
//         )
//     })
//     .bind("127.0.0.1:8088")?
//     .run()
//     .await
// }

// use actix_web::{guard, http::header, HttpRequest, HttpResponse, Result};
//
// async fn index(req: HttpRequest) -> Result<HttpResponse> {
//     let url = req.url_for("foo", &["1", "2", "3"])?;
//
//     Ok(HttpResponse::Found()
//         .header(header::LOCATION, url.as_str())
//         .finish())
// }
//
// #[actix_rt::main]
// async fn main() -> std::io::Result<()> {
//     use actix_web::{web, App, HttpServer};
//
//     HttpServer::new(|| {
//         App::new()
//             .service(
//                 web::resource("test/{a}/{b}/{c}")
//                     .name("foo")
//                     .guard(guard::Get())
//                     .to(|| HttpResponse::Ok()),
//             )
//             .route("/test/", web::get().to(index))
//     })
//     .bind("127.0.0.1:8088")?
//     .run()
//     .await
// }
//
// > curl -v localhost:8088/test/                                                                                                                              ~@thelostwayfarer.local
// *   Trying ::1...
// * TCP_NODELAY set
// * Connection failed
// * connect to ::1 port 8088 failed: Connection refused
// *   Trying 127.0.0.1...
// * TCP_NODELAY set
// * Connected to localhost (127.0.0.1) port 8088 (#0)
// > GET /test/ HTTP/1.1
// > Host: localhost:8088
// > User-Agent: curl/7.54.0
// > Accept: */*
// >
// < HTTP/1.1 302 Found
// < content-length: 0
// < location: http://localhost:8088/test/1/2/3
// < date: Tue, 10 Mar 2020 21:25:55 GMT
// <
// * Connection #0 to host localhost left intact

// use actix_web::{HttpRequest, Responder};
//
// async fn index(req: HttpRequest) -> impl Responder {
//     let url = req.url_for("youtube", &["oHg5SJYRHA0"]).unwrap();
//     assert_eq!(url.as_str(), "https://youtube.com/watch/oHg5SJYRHA0");
//
//     url.into_string()
// }
//
// #[actix_rt::main]
// async fn main() -> std::io::Result<()> {
//     use actix_web::{web, App, HttpServer};
//
//     HttpServer::new(|| {
//         App::new()
//             .route("/", web::get().to(index))
//             .external_resource("youtube", "https://youtube.com/watch/{video_id}")
//             // .route("/", actix_web::get().to(index))
//     })
//     .bind("127.0.0.1:8088")?
//     .run()
//     .await
// }

// use actix_web::{middleware, HttpResponse};
//
// async fn index() -> HttpResponse {
//     HttpResponse::Ok().body("Hello")
// }
//
// #[actix_rt::main]
// async fn main() -> std::io::Result<()> {
//     use actix_web::{web, App, HttpServer};
//
//     HttpServer::new(|| {
//         App::new()
//             .wrap(middleware::NormalizePath)
//             .route("/resource/", web::to(index))
//     })
//     .bind("127.0.0.1:8088")?
//     .run()
//     .await
// }

// use actix_web::{http::Method, middleware, web, App, HttpServer, HttpResponse};
//
// async fn index() -> HttpResponse {
//     HttpResponse::Ok().body("Hello")
// }
//
// #[actix_rt::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .wrap(middleware::NormalizePath)
//             .route("/resource/", web::get().to(index))
//             .default_service(web::route().method(Method::GET))
//     })
//     .bind("127.0.0.1:8088")?
//     .run()
//     .await
// }

// use actix_web::{http::Method, middleware, web, App, HttpServer, HttpResponse};
//
// async fn show_users() -> HttpResponse {
//     HttpResponse::Ok().body("Show users")
// }
//
// async fn user_detail(path: web::Path<(u32,)>) -> HttpResponse {
//     HttpResponse::Ok().body(format!("User Detail: {}", path.0))
// }
//
// #[actix_rt::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new().service(
//             web::scope("/users")
//                 .route("/show", web::get().to(show_users))
//                 .route("/show/{id}", web::get().to(user_detail)),
//         )
//     })
//     .bind("127.0.0.1:8088")?
//     .run()
//     .await
// }

// use actix_web::{dev::RequestHead, guard::Guard, http, HttpResponse};
//
// struct ContentTypeHeader;
//
// impl Guard for ContentTypeHeader {
//     fn check(&self, req: &RequestHead) -> bool {
//         req.headers().contains_key(http::header::CONTENT_TYPE)
//     }
// }
//
// #[actix_rt::main]
// async fn main() -> std::io::Result<()> {
//     use actix_web::{web, App, HttpServer};
//
//     HttpServer::new(|| {
//         App::new().route(
//             "/",
//             web::route()
//                 .guard(ContentTypeHeader)
//                 .to(|| HttpResponse::Ok())
//         )
//     })
//     .bind("127.0.0.1:8088")?
//     .run()
//     .await
// }

use actix_web::{guard, web, App, HttpResponse, HttpServer};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().route(
            "/",
            web::route()
                .guard(guard::Not(guard::Get()))
                .to(|| HttpResponse::MethodNotAllowed())
        )
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}

// > curl -v -X PUT localhost:8088/                                                                                                                            ~@thelostwayfarer.local
// *   Trying ::1...
// * TCP_NODELAY set
// * Connection failed
// * connect to ::1 port 8088 failed: Connection refused
// *   Trying 127.0.0.1...
// * TCP_NODELAY set
// * Connected to localhost (127.0.0.1) port 8088 (#0)
// > PUT / HTTP/1.1
// > Host: localhost:8088
// > User-Agent: curl/7.54.0
// > Accept: */*
// >
// < HTTP/1.1 405 Method Not Allowed
// < content-length: 0
// < date: Tue, 10 Mar 2020 22:41:20 GMT
// <
// * Connection #0 to host localhost left intact
