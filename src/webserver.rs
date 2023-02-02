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

#[post("/users")]
async fn insert_user(input: web::Form<String>) -> impl Responder {
// https://www.vultr.com/docs/building-rest-apis-in-rust-with-actix-web/
    println!("POST request accepted");
    let name: String = input.to_owned();
    let value = crate::database::insert_to_db(name);
    return HttpResponse::Ok()
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
