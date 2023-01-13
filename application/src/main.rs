// use actix_web::{web, App, Responder, HttpServer};
//
// async fn index() -> Responder {
//     "Hello World!"
// }
//
// #[actix_rt::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new().service(
//             web::scope("/app").route("/index.html", web::get().to(index)),
//         )
//     })
//     .bind("127.0.0.1:8088")
//     .run()
//     .await
// }

use actix_web::{web, guard, App, HttpServer, HttpResponse};
use std::sync::Mutex;

// struct AppState {
//     app_name: String,
// }
//
// async fn index(data: web::Data<AppState>) -> String {
//     let app_name = &data.app_name;
//     format!("Hello {}!", app_name)
// }
//
// #[actix_rt::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .data(AppState {
//                 app_name: String::from("Actix-web"),
//             })
//             .route("/", web::get().to(index))
//     })
//     .bind("127.0.0.1:8088")?
//     .run()
//     .await
// }


// struct AppStateWithCounter {
//     counter: Mutex<i32>,
// }
//
// async fn _index(data: web::Data<AppStateWithCounter>) -> String {
//     let mut counter = data.counter.lock().unwrap();
//     *counter += 1;
//     format!("Request number: {}", counter)
// }
//
// #[actix_rt::main]
// async fn main() -> std::io::Result<()> {
//     let counter = web::Data::new(AppStateWithCounter {
//         counter: Mutex::new(0),
//     });
//
//     HttpServer::new(move || {
//         App::new()
//             .app_data(counter.clone())
//             .route("/", web::get().to(_index))
//     })
//     .bind("127.0.0.1:8088")?
//     .run()
//     .await
// }

// #[actix_rt::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .service(
//                 web::scope("/")
//                     .guard(guard::Header("Host", "www.rust-lang.org"))
//                     .route("", web::to(|| HttpResponse::Ok().body("www"))),
//             )
//             .service(
//                 web::scope("/")
//                     .guard(guard::Header("Host", "users.rust-lang.org"))
//                     .route("", web::to(|| HttpResponse::Ok().body("user"))),
//             )
//             .route("/", web::to(||HttpResponse::Ok()))
//     })
//     .bind("127.0.0.1:8088")?
//     .run()
//     .await
// }

fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/test")
            .route(web::get().to(|| HttpResponse::Ok().body("test")))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
    );
}

fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/app")
            .route(web::get().to(|| HttpResponse::Ok().body("app")))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
    );
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(config)
            .service(web::scope("/api").configure(scoped_config))
            .route("/", web::get().to(|| HttpResponse::Ok().body("/")))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
