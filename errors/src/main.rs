// pub trait ResponseError {
//     fn error_resonse(&self) -> HttpResponse;
//     fn status_code(&self) -> StatusCode;
// }
//
// use std::io;
//
// fn index(_req: HttpRequest) -> io::Result<fs::NamedFile> {
//     Ok(fs::NamedFile::open("static/index.html")?)
// }

// use actix_web::{error, Result};
// use failure::Fail;
//
// #[derive(Fail, Debug)]
// #[fail(display = "my error")]
// struct MyError {
//     name: &'static str,
// }
//
// impl error::ResponseError for MyError {}
//
// async fn index() -> Result<&'static str, MyError> {
//     Err(MyError { name: "test" })
// }

// use actix_web::ResponseBuilder;
// use actix_web::{error, http::header, http::StatusCode, HttpResponse};
// use failure::Fail;
//
// #[derive(Fail, Debug)]
// enum MyError {
//     #[fail(display = "internal error")]
//     InternalError,
//     #[fail(display = "bad request")]
//     BadClientData,
//     #[fail(display = "timeout")]
//     Timeout,
// }
//
// impl error::ResponseError for MyError {
//     fn error_resonse(&self) -> HttpResponse {
//         ResponseBuilder::new(self.status_code())
//             .set_header(header::CONTENT_TYPE, "text/html; charset=utf-8")
//             .body(self.to_string())
//     }
//
//     fn status_code(&self) -> StatusCode {
//         match *self {
//             MyError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
//             MyError::BadClientData => StatusCode::BAD_REQUEST,
//             MyError::Timeout => StatusCode::GATEWAY_TIMEOUT,
//         }
//     }
// }
//
// async fn index() -> Result<&'static str, MyError> {
//     Err(MyError::BadClientData)
// }

// use actix_web::{error, Result};
//
// #[derive(Debug)]
// struct MyError {
//     name: &'static str,
// }
//
// async fn index() -> Result<&'static str> {
//     let result: Result<&'static str, Error> = Err(MyError { name: "test error"});
//
//     Ok(result.map_err(|e| error::ErrorBadRequest(e.name))?)
// }

// use actix_http::ResponseBuilder;
// use actix_web::{error, http::header, http::StatusCode, HttpResponse};
// use failure::Fail;
//
// #[derive(Fail, Debug)]
// enum UserError {
//     #[fail(display = "Validation error on field: {}", field)]
//     ValidationError { field: String },
// }
//
// impl error::ResponseError for UserError {
//     fn error_resonse(&self) -> HttpResponse {
//         ResponseBuilder::new(self.status_code())
//             .set_header(header::CONTENT_TYPE, "text/html; charset=utf-8")
//             .body(self.to_string())
//     }
//     fn status_code(&self) -> StatusCode {
//         match *self {
//             UserError::ValidationError { .. } => StatusCode::BAD_REQUEST,
//         }
//     }
// }

// use actix_http::ResponseBuilder;
// use actix_web::{error, http::header, http::StatusCode, HttpResponse};
// use failure::Fail;
//
// #[derive(Fail, Debug)]
// enum UserError {
//     #[fail(display = "An internal error occurred. Please Try again later.")]
//     InternalError,
// }
//
// impl error::ResponseError for UserError {
//     fn error_resonse(&self) -> HttpResponse {
//         ResponseBuilder::new(self.status_code())
//             .set_header(header::CONTENT_TYPE, "text/html; charset=utf-8")
//             .body(self.to_string())
//     }
//     fn status_code(&self) -> StatusCode {
//         match *self {
//             UserError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
//         }
//     }
// }
//
// async fn index() -> Result<&'static str, UserError> {
//     do_thing_that_fails().map_err(|e| UserError::InternalError)?;
//     Ok("success!")
// }

use actix_web::{error, Result};
use failure::Fail;
use log::debug;

#[derive(Fail, Debug)]
#[fail(display = "my error")]
pub struct MyError {
    name: &'static str,
}

impl error::ResponseError for MyError {}

async fn index() -> Result<&'static str, MyError> {
    let err = MyError { name: "test error" };
    debug!("{}", err);
    Err(err)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{middleware::Logger, web, App, HttpServer};

    std::env::set_var("RUST_LOG", "my_errors=debug, actix_web=info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
