use actix_files::{Files, NamedFile};
use actix_web::*;
use std::path::PathBuf;
use tera::Tera;
// https://dev.to/0xbf/day11-write-web-app-with-actix-web-100dayofrust-1lkn

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/users")]
async fn users() -> Result<NamedFile> {
    // let path: PathBuf = "./http_rust/users.html".parse().unwrap();
    let tera = match Tera::new("./http_rust/users.html") { 
        Ok(t) => t,
        Err(_) => {
            println!("File not found");
            ::std::process::exit(1);
            
        }
    };

    return Ok(NamedFile::open(tera)?);
}

// https://actix.rs/docs/getting-started
#[actix_web::main]
pub async fn serve(names: Vec<String>) -> std::io::Result<()> {
    return HttpServer::new(|| {
        App::new()
            .service(home)
            .service(users)
            .service(Files::new("./http_rust/", ".").show_files_listing())
    })
    .bind(("127.0.0.1", 5656))?
    .run()
    .await;
}
