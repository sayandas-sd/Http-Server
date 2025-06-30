use actix_web::{get, post, App, web, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signature, Signer},
    signer::keypair::keypair_from_seed,
};
use solana_system_program::system_instruction;
use base64::{engine::general_purpose, Engine};
use bs58;

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


#[derive(Serialize)]
struct KeypairData {
    pubkey: String,
    secret: String,
}

#[derive(Serialize)]
struct SignMessageResponse {
    success: bool,
    data: Option<SignMessageData>,
    error: Option<String>,
}

#[derive(Serialize)]
struct SignMessageData {
    signature: String,
    public_key: String,
    message: String,
}

#[derive(Deserialize)]
struct SignMessageRequest {
    message: String,
    secret: String,
}

#[derive(Deserialize)]
struct VerifyMessageRequest {
    message: String,
    signature: String,
    pubkey: String,
}

#[derive(Serialize)]
struct VerifyMessageResponse {
    success: bool,
    data: Option<VerifyMessageData>,
    error: Option<String>,
}

#[derive(Serialize)]
struct VerifyMessageData {
    valid: bool,
    message: String,
    pubkey: String,
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



#[post("/message/sign")]
async fn sign_message(req_body: web::Json<SignMessageRequest>) -> impl Responder {
   let request = req_body.into_inner();

    if request.message.is_empty() || request.secret.is_empty() {
        return web::Json(SignMessageResponse {
            success: false,
            data: None,
            error: Some("Missing required fields".to_string()),
        });
    }

    let secret_bytes = match bs58::decode(&request.secret).into_vec() {
        Ok(bytes) => {
            if bytes.len() != 64 {
                return web::Json(SignMessageResponse {
                    success: false,
                    data: None,
                    error: Some("Invalid secret key length".to_string()),
                });
            }
            bytes
        }
        Err(_) => {
            return web::Json(SignMessageResponse {
                success: false,
                data: None,
                error: Some("Invalid base58-encoded secret key".to_string()),
            });
        }
    };

    let keypair = match keypair_from_seed(&secret_bytes) {
        Ok(keypair) => keypair,
        Err(_) => {
            return web::Json(SignMessageResponse {
                success: false,
                data: None,
                error: Some("Failed to create keypair from secret key".to_string()),
            });
        }
    };

    let message_bytes = request.message.as_bytes();
    let signature = keypair.sign_message(message_bytes);
    let signature_b64 = base64::encode(signature.as_ref());

    let response = SignMessageResponse {
        success: true,
        data: Some(SignMessageData {
            signature: signature_b64,
            public_key: keypair.pubkey().to_string(),
            message: request.message,
        }),
        error: None,
    };

    web::Json(response)
}


#[post("/message/verify")]
async fn verify_message(req_body: web::Json<VerifyMessageRequest>) -> impl Responder {
    let request = req_body.into_inner();

    if request.message.is_empty() || request.signature.is_empty() || request.pubkey.is_empty() {
        return web::Json(VerifyMessageResponse {
            success: false,
            data: None,
            error: Some("Missing required fields".to_string()),
        });
    }

    let pubkey = match request.pubkey.parse::<Pubkey>() {
        Ok(pubkey) => pubkey,
        Err(_) => {
            return web::Json(VerifyMessageResponse {
                success: false,
                data: None,
                error: Some("Invalid public key".to_string()),
            });
        }
    };

    let signature_bytes = match general_purpose::STANDARD.decode(&request.signature) {
        Ok(bytes) => {
            if bytes.len() != 64 {
                return web::Json(VerifyMessageResponse {
                    success: false,
                    data: None,
                    error: Some("Invalid signature length".to_string()),
                });
            }
            bytes
        }
        Err(_) => {
            return web::Json(VerifyMessageResponse {
                success: false,
                data: None,
                error: Some("Invalid base64-encoded signature".to_string()),
            });
        }
    };

    let signature = match Signature::try_from(signature_bytes) {
        Ok(sig) => sig,
        Err(_) => {
            return web::Json(VerifyMessageResponse {
                success: false,
                data: None,
                error: Some("Invalid signature format".to_string()),
            });
        }
    };

    let message_bytes = request.message.as_bytes();
    let is_valid = signature.verify(pubkey.as_ref(), message_bytes);

    let response = VerifyMessageResponse {
        success: true,
        data: Some(VerifyMessageData {
            valid: is_valid,
            message: request.message,
            pubkey: request.pubkey,
        }),
        error: None,
    };

    web::Json(response)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let port = "0.0.0.0:8080";
    println!("Server is Running on http://{}", port);

    HttpServer::new(|| {
        App::new()
            .service(keypair_generate)
            .service(sign_message)
            .service(verify_message)
    })
    .bind(port)?
    .run()
    .await
}
