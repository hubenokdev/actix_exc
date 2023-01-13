// async fn index(
//     path: web::Path<(String, String)>,
//     json: web::Json<MyInfo>,
// ) -> impl Responder {
//     format!("{} {} {} {}", path.0, path.1, json.id, json.username)
// }

// use actix_web::{web, Result};
//
// async fn index(info: web::Path<(u32, String)>) -> Result<String> {
//     Ok(format!("Welcome {}, userid {}!", info.1, info.0))
// }
//
// #[actix_rt::main]
// async fn main() -> std::io::Result<()> {
//     use actix_web::{App, HttpServer};
//
//     HttpServer::new(|| {
//         App::new().route(
//             "users/{userid}/{friend}",
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
//     userid: u32,
//     friend: String,
// }
//
// async fn index(info: web::Path<Info>) -> Result<String> {
//     Ok(format!("Welcome {}, userid {}!", info.friend, info.userid))
// }
//
// #[actix_rt::main]
// async fn main() -> std::io::Result<()> {
//     use actix_web::{App, HttpServer};
//
//     HttpServer::new(|| {
//         App::new().route(
//             "users/{userid}/{friend}",
//             web::get().to(index),
//         )
//     })
//     .bind("127.0.0.1:8088")?
//     .run()
//     .await
// }

// use actix_web::{web, App, HttpServer, HttpRequest};
//
// async fn index(req: HttpRequest) -> Result<String> {
//     let name: String = req.match_info().get("friend").unwrap().parse().unwrap();
//     let userid: i32 = req.match_info().query("userid").parse().unwrap();
//
//     Ok(format!("Welcome {}, userid {}!", name, userid))
// }
//
// #[actix_rt::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new().route(
//             "/users/{userid}/{friend}",
//             web::get().to(index),
//         )
//     })
//     .bind("127.0.0.1:8080")?
//     .run()
//     .await
// }

// use actix_web::{web, App, HttpServer, HttpRequest};
// use serde::Deserialize;
//
// #[derive(Deserialize)]
// struct Info {
//     username: String,
// }
//
// async fn index(info: web::Query<Info>) -> String {
//     format!("Welcome {}!", info.username)
// }
//
// #[actix_rt::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new().route("/welcome", web::get().to(index))
//     })
//     .bind("127.0.0.1:8088")?
//     .run()
//     .await
// }

// curl 'localhost:8088/welcome?username=hi'                                                                                                                 ~@thelostwayfarer.local
// Welcome hi!

// use actix_web::{web, Result};
// use serde::Deserialize;
//
// #[derive(Deserialize)]
// struct Info {
//     username: String,
// }
//
// async fn index(info: web::Json<Info>) -> Result<String> {
//     Ok(format!("Welcome {}!", info.username))
// }

// use actix_web::{error, web, FromRequest, HttpResponse, Responder};
// use serde::Deserialize;
//
// #[derive(Deserialize)]
// struct Info {
//     username: String,
// }
//
// async fn index(info: web::Json<Info>) -> impl Responder {
//     format!("Welcome {}!", info.username)
// }
//
// #[actix_rt::main]
// async fn main() -> std::io::Result<()> {
//     use actix_web::{App, HttpServer};
//
//     HttpServer::new(|| {
//         App::new().service(
//             web::resource("/")
//                 .app_data(web::Json::<Info>::configure(|cfg| {
//                     cfg.limit(4096).error_handler(|err, _req| {
//                         error::InternalError::from_response(
//                             err,
//                             HttpResponse::Conflict().finish(),
//                         )
//                         .into()
//                     })
//                 }))
//                 .route(web::post().to(index)),
//         )
//     })
//     .bind("127.0.0.1:8088")?
//     .run()
//     .await
// }

// curl -X POST -H "Content-Type: application/json" -d '{"username": "hi"}' http://localhost:8088                                                            ~@thelostwayfarer.local
// Welcome hi!

// use actix_web::{web, Result};
// use serde::Deserialize;
//
// #[derive(Deserialize)]
// struct FormData {
//     username: String,
// }
//
// async fn index(form: web::Form<FormData>) -> Result<String> {
//     Ok(format!("Welcome {}!", form.username))
// }

use actix_web::{web, Responder};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

#[derive(Clone)]
struct AppState {
    count: Arc<AtomicUsize>,
}

async fn show_count(data: web::Data<AppState>) -> impl Responder {
    format!("count: {}", data.count.load(Ordering::Relaxed))
}

async fn add_one(data: web::Data<AppState>) -> impl Responder {
    data.count.fetch_add(1, Ordering::Relaxed);

    format!("count: {}", data.count.load(Ordering::Relaxed))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    let data = AppState {
        count: Arc::new(AtomicUsize::new(0)),
    };

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .route("/", web::to(show_count))
            .route("/add", web::to(add_one))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}

// compiles but returns "App data is not configured, to configure use App::data()"
