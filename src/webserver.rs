use actix_files::Files;
use actix_web::http::StatusCode;
use actix_web::web::*;
use actix_web::*;
use sailfish::TemplateOnce;
use serde::Deserialize;

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

#[derive(Debug, Deserialize)]
pub struct FormData {
    user: String,
}

async fn insert_user(params: web::Form<FormData>) -> impl Responder {
    let value = crate::database::insert_to_db(params.user.clone());
    return Redirect::to("/users").using_status_code(StatusCode::FOUND);
}

#[actix_web::main]
pub async fn serve() -> std::io::Result<()> {
    return HttpServer::new(|| {
        App::new()
            .service(home)
            .service(users)
            .service(web::resource("/users").route(web::post().to(insert_user)))
            .service(Files::new("/http_rust", "./http_rust"))
    })
    .bind(("127.0.0.1", 5656))?
    .run()
    .await;
}
