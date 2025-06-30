use actix_web::{get, post, App, web, HttpResponse, HttpServer, Responder};
use serde::Serialize;
use  solana_sdk :: { signature :: Keypair ,  signer :: Signer };
use solana_sdk::bs58;

#[derive(Serialize)]
struct keypairData {
    pubkey: String,
    secret: String
}


#[derive(Serialize)]
struct KeypairRes {
    success: bool,
    data: keypairData
}


#[post("/keypair")]
async fn keypair_generate(req_body: String) -> impl Responder {


    let keypair = Keypair::new();
    let address  =  keypair.pubkey().to_string();
    let secret = bs58::encode(keypair.to_bytes()).into_string();

    let res = KeypairRes {
        success: true,
        data: keypairData {
            pubkey: address,
            secret,
        }
    };

    web::Json(res)
}




#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let port = "0.0.0.0:8080";
    println!("Server is Running on http://{}", port);

    HttpServer::new(|| {
        App::new()
            .service(keypair_generate)
    })
    .bind(port)?
    .run()
    .await
}