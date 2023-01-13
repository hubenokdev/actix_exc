// use actix_files::NamedFile;
// use actix_web::{HttpRequest, Result};
// use std::path::PathBuf;
//
// async fn index(req: HttpRequest) -> Result<NamedFile> {
//     let path: PathBuf = req.match_info().query("filename").parse().unwrap();
//     Ok(NamedFile::open(path)?)
// }
//
// #[actix_rt::main]
// async fn main() -> std::io::Result<()> {
//     use actix_web::{web, App, HttpServer};
//
//     HttpServer::new(|| App::new().route("/{filename:.*}", web::get().to(index)))
//         .bind("127.0.0.1:8088")?
//         .run()
//         .await
// }

// use actix_files as fs;
// use actix_web::{App, HttpServer};
//
// #[actix_rt::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new().service(fs::Files::new("/static", ".").show_files_listing())
//     })
//     .bind("127.0.0.1:8088")?
//     .run()
//     .await
// }

// use actix_files as fs;
// use actix_web::http::header::{ContentDisposition, DispositionType};
// use actix_web::{web, App, Error, HttpRequest, HttpServer};
//
// async fn index(req: HttpRequest) -> Result<fs::NamedFile, Error> {
//     let path: std::path::PathBuf = req.match_info().query("filename").parse().unwrap();
//     let file = fs::NamedFile::open(path)?;
//     Ok(file
//         .use_last_modified(true)
//         .set_content_disposition(ContentDisposition {
//             disposition: DispositionType::Attachment,
//             parameters: vec![],
//         }))
// }
//
// #[actix_rt::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| App::new().route("/{filename:.*}", web::get().to(index)))
//         .bind("127.0.0.1:8088")?
//         .run()
//         .await
// }

use actix_files as fs;
use actix_web::{App, HttpServer};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            fs::Files::new("/static", ".")
                .show_files_listing()
                .use_last_modified(true),
        )
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
