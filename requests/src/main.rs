use actix_web::{web, App, HttpServer, Result};
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    username: String,
}

async fn index(info: web::Json<Info>) -> Result<String> {
    Ok(format!("Welcome {}!", info.username))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::post().to(index)))
        .bind("127.0.0.1:8088")?
        .run()
        .await
}

// > curl -v -X POST -H "Content-Type: application/json" -d '{"username": "hi"}' localhost:8088/                                                               ~@thelostwayfarer.local
// Note: Unnecessary use of -X or --request, POST is already inferred.
// *   Trying ::1...
// * TCP_NODELAY set
// * Connection failed
// * connect to ::1 port 8088 failed: Connection refused
// *   Trying 127.0.0.1...
// * TCP_NODELAY set
// * Connected to localhost (127.0.0.1) port 8088 (#0)
// > POST / HTTP/1.1
// > Host: localhost:8088
// > User-Agent: curl/7.54.0
// > Accept: */*
// > Content-Type: application/json
// > Content-Length: 18
// >
// * upload completely sent off: 18 out of 18 bytes
// < HTTP/1.1 200 OK
// < content-length: 11
// < content-type: text/plain; charset=utf-8
// < date: Wed, 11 Mar 2020 01:53:33 GMT
// <
// * Connection #0 to host localhost left intact
// Welcome hi!%

// use actix_web::{error, web, App, Error, HttpResponse};
// use bytes::BytesMut;
// use futures::StreamExt;
// use serde::{Deserialize, Serialize};
// use serde_json;
//
// #[derive(Serialize, Deserialize)]
// struct MyObj {
//     name: String,
//     number: i32,
// }
//
// const MAX_SIZE: usize = 262_144
//
// async fn index_manual(mut payload: web::payload) -> Result<HttpResponse, Error> {
//     let mut body = BytesMut::new();
//     while let Some(chunk) = payload.next().await {
//         let chunk = chunk?;
//         if (body.len() + chunk.len()) > MAX_SIZE {
//             return Err(error::ErrorBadRequest("overflow"));
//         }
//         body.extend_from_slice(&chunk);
//     }
//     let obj = serde_json::from_slice::<MyObj>(&body)?;
//     Ok(HttpResponse::Ok().json(obj))
// }

// use actix_web::{web, HttpResponse};
// use serde::Deserialize;
//
// #[derive(Deserialize)]
// struct FormData {
//     username: String,
// }
//
// async fn index(form: web::Form<FormData>) -> HttpResponse {
//     HttpResponse::Ok().body(format!("username: {}", form.username))
// }

// use actix_web::{web, Error, HttpResponse};
// use futures::StreamExt;
//
// async fn index(mut body: web::Payload) -> Result<HttpResponse, Error> {
//     let mut bytes = web::BytesMut::new();
//     while let Some(item) = body.next().await {
//         let item = item?;
//         println!("Chunk: {:?}", &item);
//         bytes.extend_from_slice(&item);
//     }
//
//     Ok(HttpResponse::Ok().finish())
// }
