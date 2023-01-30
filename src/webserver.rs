use actix_files::Files;
use actix_web::*;
use sailfish::TemplateOnce;

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[derive(TemplateOnce)]
#[template(path = "users.stpl")]
struct UsersTemplate {
    messages: Vec<String>,
}

#[get("/users")]
async fn users() -> impl Responder {
    let ctx = UsersTemplate {
        messages: crate::database::read_from_db(),
    };

    return HttpResponse::Ok().body(ctx.render_once().unwrap());
}

// https://actix.rs/docs/getting-started
#[actix_web::main]
pub async fn serve() -> std::io::Result<()> {
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
