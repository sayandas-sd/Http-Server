use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};


#[get("/")]
async fn get_data() -> impl Responder {

    let serverdir = std::fs::read_to_string("index.html");

    match serverdir {
        Ok(html) => HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(html),
        Err(e) => {
            eprintln!("erroor :{}", e);
            HttpResponse::InternalServerError().body("something is wrong")
        }

    }
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

    let port = "0.0.0.0:8080";
    println!("Server is Running on http://{}", port);

    HttpServer::new(|| {
        App::new()
            .service(get_data)
            .service(post_data)
            .route("/health", web::get().to(manual_hello))
    })
    .bind(port)?
    .run()
    .await
}