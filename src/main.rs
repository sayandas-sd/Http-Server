use actix_web::{get, post, App, web, HttpResponse, HttpServer, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct ApiRes {
    id: u8,
    message: String,
    status: String,
}


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

#[get("/jsondata")]
async fn json_data() -> impl Responder {
    let res = ApiRes {
        id: 1,
        message: "Json response".to_string(),
        status: "Success reponse: 200".to_string(),
    };

    web::Json(res)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let port = "0.0.0.0:8080";
    println!("Server is Running on http://{}", port);

    HttpServer::new(|| {
        App::new()
            .service(get_data)
            .service(post_data)
            .service(json_data)
    })
    .bind(port)?
    .run()
    .await
}