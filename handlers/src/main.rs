// async fn index(_req: HttpRequest) -> &'static str {
//     "Hello world!"
// }
//
// async fn index(_req: HttpRequest) -> String {
//     "Hello World!".to_owned()
// }
//
// async fn index(_req: HttpRequest) -> impl Responder {
//     Bytes::from_static(b"Hello World!")
// }
//
// async fn index(req: HttpRequest) -> Box<Future<Item=HttpResponse, Error=Error>> {
//     .....
// }

// use actix_web::{Error, HttpRequest, HttpResponse, Responder}
// use serde::Serialize;
// use futures::future::{ready, Ready};
//
// #[derive(Serialize)]
// struct MyObj {
//     name: &'static str,
// }
//
// impl Responder for MyObj {
//     type Error = Error;
//     type Future = Ready<Result<HttpResponse, Error>>;
//
//     fn respond_to(self, _req: &HttpRequest) -> Self::Future {
//         let body = serde_json::to_string(&self).unwrap();
//
//         ready(Ok(HttpResponse::Ok().content_type("application/json").body(body)))
//     }
// }
//
// async fn index() -> impl Responder {
//     MyObj { name: "user"}
// }

use actix_web::{web, App, HttpServer, Error, HttpResponse};
use bytes::Bytes;
use futures::stream::once;
use futures::future::ok;

async fn index() -> HttpResponse {
    let body = once(ok::<_, Error>(Bytes::from_static(b"test")));

    HttpResponse::Ok().content_type("application/json").streaming(body)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/async", web::to(index)))
        .bind("127.0.0.1:8088")?
        .run()
        .await
}

// use actix_web::{Either, Error, HttpResponse};
//
// type RegisterResult = Either<HttpResponse, Result<&'static str, Error>>;
//
// fn index() -> RegisterResult {
//     if is_a_variant() {
//         Either::A(HttpResponse::BadRequest().body("Bad data"))
//     } else {
//         Either::B(Ok("Hello!"))
//     }
// }
