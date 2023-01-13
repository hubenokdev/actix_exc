// use actix_web::{web, App, HttpResponse, HttpServer};
//
// #[actix_rt::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new().route("/", web::get().to(|| HttpResponse::Ok()))
//     })
//     .bind("127.0.0.1:8088")?
//     .run()
//     .await
// }

// use actix_rt::System;
// use actix_web::{web, App, HttpResponse, HttpServer};
// use std::sync::mpsc;
// use std::thread;
//
// #[actix_rt::main]
// async fn main() {
//     let (tx, rx) = mpsc::channel();
//
//     thread::spawn(move || {
//         let sys = System::new("http-server");
//
//         let srv = HttpServer::new(|| {
//             App::new().route("/", web::get().to(|| HttpResponse::Ok()))
//         })
//         .bind("127.0.0.1:8088")?
//         .shutdown_timeout(60)
//         .run();
//
//         let _ = tx.send(srv);
//         sys.run()
//     });
//
//     let srv = rx.recv().unwrap();
//
//     srv.pause().await;
//     srv.resume().await;
//     srv.stop(true).await;
// }

// use actix_web::{web, App, HttpResponse, HttpServer};
//
// #[actix_rt::main]
// async fn main() {
//     HttpServer::new(|| {
//         App::new().route("/", web::get().to(|| HttpResponse::Ok()))
//     })
//     .workers(4)
// }

// use actix_web::{web, App, HttpRequest, HttpServer, Responder};
// use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
//
// async fn index(_req: HttpRequest) -> impl Responder {
//     "Welcome!"
// }
//
// #[actix_rt::main]
// async fn main() -> std::io::Result<()> {
//     let mut builder =
//         SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
//     builder
//         .set_private_key_file("key.pem", SslFiletype::PEM)
//         .unwrap();
//     builder.set_certificate_chain_file("cert.pem").unwrap();
//
//     HttpServer::new(|| App::new().route("/", web::get().to(index)))
//         .bind_openssl("127.0.0.1:8088", builder)?
//         .run()
//         .await
// }

use actix_web::{web, App, HttpResponse, HttpServer};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let one = HttpServer::new(|| {
        App::new().route("/", web::get().to(|| HttpResponse::Ok()))
    })
    .keep_alive(75);

    let _three = HttpServer::new(|| {
        App::new().route("/", web::get().to(|| HttpResponse::Ok()))
    })
    .keep_alive(None);

    one.bind("127.0.0.1:8088")?.run().await
}
