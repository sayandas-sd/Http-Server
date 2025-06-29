use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};


#[get("/")]
async fn get_data() -> impl Responder {
    HttpResponse::Ok().body("hey, welcome to our page")
}


#[post("/")]
async fn post_data(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}


async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("server is working")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_data)
            .service(post_data)
            .route("/health", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}