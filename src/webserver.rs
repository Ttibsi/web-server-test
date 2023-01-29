use actix_files::{Files, NamedFile};
use actix_web::*;
use std::path::PathBuf;

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/users")]
async fn users() -> Result<NamedFile> {
    let path: PathBuf = "./http_rust/users.html".parse().unwrap();
    Ok(NamedFile::open(path)?)
}

// https://actix.rs/docs/getting-started
#[actix_web::main]
pub async fn serve(names: Vec<String>) -> std::io::Result<()> {
    return HttpServer::new(|| {
        App::new()
            .service(home)
            .service(users)
            .service(Files::new("/http_rust", "./http_rust"))
    })
    .bind(("127.0.0.1", 5656))?
    .run()
    .await;
}
