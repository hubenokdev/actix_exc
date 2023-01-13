// use actix_web::HttpResponse;
//
// async fn index() -> HttpResponse {
//     HttpResponse::Ok()
//         .content_type("plain/text")
//         .header("X-Hdr", "sample")
//         .body("data")
// }

// use actix_web::{middleware, HttpResponse};
//
// async fn index_br() -> HttpResponse {
//     HttpResponse::Ok().body("data")
// }
//
// #[actix_rt::main]
// async fn main() -> std::io::Result<()> {
//     use actix_web::{web, App, HttpServer};
//
//     HttpServer::new(|| {
//         App::new()
//             .wrap(middleware::Compress::default())
//             .route("/", web::get().to(index_br))
//     })
//     .bind("127.0.0.1:8088")?
//     .run()
//     .await
// }

// use actix_web::{http::ContentEncoding, dev::BodyEncoding, HttpResponse};
//
// async fn index_br() -> HttpResponse {
//     HttpResponse::Ok()
//         .encoding(ContentEncoding::Br)
//         .body("data")
// }
//
// #[actix_rt::main]
// async fn main() -> std::io::Result<()> {
//     use actix_web::{middleware, web, App, HttpServer};
//
//     HttpServer::new(|| {
//         App::new()
//             .wrap(middleware::Compress::default())
//             .route("/", web::get().to(index_br))
//     })
//     .bind("127.0.0.1:8088")?
//     .run()
//     .await
// }
//
// > curl -i localhost:8088                                                                                                                                    ~@thelostwayfarer.local
// HTTP/1.1 200 OK
// transfer-encoding: chunked
// content-encoding: br
// date: Wed, 11 Mar 2020 02:47:17 GMT
//
// ??data%


// use actix_web::{http::ContentEncoding, dev::BodyEncoding, HttpResponse};
//
// async fn index_br() -> HttpResponse {
//     HttpResponse::Ok().body("data")
// }
//
// #[actix_rt::main]
// async fn main() -> std::io::Result<()> {
//     use actix_web::{middleware, web, App, HttpServer};
//
//     HttpServer::new(|| {
//         App::new()
//             .wrap(middleware::Compress::new(ContentEncoding::Br))
//             .route("/", web::get().to(index_br))
//     })
//     .bind("127.0.0.1:8088")?
//     .run()
//     .await
// }

// use actix_web::{http::ContentEncoding, dev::BodyEncoding, HttpResponse};
//
// async fn index() -> HttpResponse {
//     HttpResponse::Ok()
//         .encoding(ContentEncoding::Identity)
//         .body("data")
// }
//
// #[actix_rt::main]
// async fn main() -> std::io::Result<()> {
//     use actix_web::{middleware, web, App, HttpServer};
//
//     HttpServer::new(|| {
//         App::new()
//             .wrap(middleware::Compress::default())
//             .route("/", web::get().to(index))
//     })
//     .bind("127.0.0.1:8088")?
//     .run()
//     .await
// }

use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct MyObj {
    name: String,
}

async fn index(obj: web::Path<MyObj>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(MyObj {
        name: obj.name.to_string(),
    }))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    HttpServer::new(|| App::new().route(r"/a/{name}", web::get().to(index)))
        .bind("127.0.0.1:8088")?
        .run()
        .await
}

// > curl -i localhost:8088/a/hi                                                                                                                               ~@thelostwayfarer.local
// HTTP/1.1 200 OK
// content-length: 13
// content-type: application/json
// date: Wed, 11 Mar 2020 03:14:51 GMT
//
// {"name":"hi"}%
