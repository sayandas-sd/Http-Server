use std::str::FromStr;
use tiny_http::{Header, Server, Response, Method};
use solana_sdk::{signature::Keypair, signer::Signer, pubkey:: Pubkey};
use spl_token::{instruction::initialize_mint};
use serde::{Serialize, Deserialize};
use bs58;
use base64::{engine::general_purpose, Engine};
use spl_token::instruction::mint_to;
use serde_json::json;
use solana_sdk::signature::Signature;
use solana_sdk::system_instruction;


#[derive(Serialize)]
struct KeypairResponse {
    success: bool,
    data: KeypairData,
}

#[derive(Serialize)]
struct KeypairData {
    pubkey: String,
    secret: String,
}

#[derive(Deserialize)]
struct CreateTokenRequest {
    mintAuthority: String,
    mint: String,
    decimals: u8,
}

#[derive(Serialize)]
struct CreateTokenResponse {
    success: bool,
    data: TokenData,
}

#[derive(Serialize)]
struct TokenData {
    program_id: String,
    accounts: Vec<AccountMetaData>,
    instruction_data: String,
}

#[derive(Serialize)]
struct AccountMetaData {
    pubkey: String,
    is_signer: bool,
    is_writable: bool,
}

#[derive(Deserialize)]
struct MintTokenRequest {
    mint: String,
    destination: String,
    authority: String,
    amount: u64,
}

#[derive(Deserialize)]
struct SignMessageRequest {
    message: String,
    secret: String,
}

#[derive(Serialize)]
struct SignMessageResponse {
    success: bool,
    data: SignMessageData,
}

#[derive(Serialize)]
struct SignMessageData {
    signature: String,
    public_key: String,
    message: String,
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
    data: VerifyMessageData,
}

#[derive(Serialize)]
struct VerifyMessageData {
    valid: bool,
    message: String,
    pubkey: String,
}

#[derive(Deserialize)]
struct SendSolRequest {
    from: String,
    to: String,
    lamports: u64,
}

#[derive(Serialize)]
struct SendSolResponse {
    success: bool,
    data: SendSolData,
}

#[derive(Serialize)]
struct SendSolData {
    program_id: String,
    accounts: Vec<String>,
    instruction_data: String,
}




fn main() {
    let server = Server::http("0.0.0.0:8080").unwrap();
    println!("🚀 Server running at http://localhost:8080");

    for mut request in server.incoming_requests() {
        match (request.method(), request.url()) {
            (&Method::Post, "/keypair") => {
                let keypair = Keypair::new();
                let pubkey = keypair.pubkey().to_string();
                let secret = bs58::encode(keypair.to_bytes()).into_string();

                let response_data = KeypairResponse {
                    success: true,
                    data: KeypairData { pubkey, secret },
                };

                let json = serde_json::to_string(&response_data).unwrap();
                let response = Response::from_string(json)
                    .with_status_code(200)
                    .with_header(Header::from_bytes(b"Content-Type", b"application/json").unwrap());

                let _ = request.respond(response);
            }

            (&Method::Post, "/token/create") => {
                let mut content = String::new();
                let _ = request.as_reader().read_to_string(&mut content);

                let req_data: Result<CreateTokenRequest, _> = serde_json::from_str(&content);

                if let Ok(data) = req_data {
                    let mint_pubkey = Pubkey::from_str(&data.mint).unwrap();
                    let mint_authority = Pubkey::from_str(&data.mintAuthority).unwrap();

                    let ix = initialize_mint(
                        &spl_token::id(),
                        &mint_pubkey,
                        &mint_authority,
                        None,
                        data.decimals,
                    ).unwrap();

                    let accounts = ix.accounts.iter().map(|meta| {
                        AccountMetaData {
                            pubkey: meta.pubkey.to_string(),
                            is_signer: meta.is_signer,
                            is_writable: meta.is_writable,
                        }
                    }).collect::<Vec<_>>();

                    let response_data = CreateTokenResponse {
                        success: true,
                        data: TokenData {
                            program_id: ix.program_id.to_string(),
                            accounts,
                            instruction_data: general_purpose::STANDARD.encode(ix.data),
                        },
                    };

                    let json = serde_json::to_string(&response_data).unwrap();
                    let response = Response::from_string(json)
                        .with_status_code(200)
                        .with_header(Header::from_bytes(b"Content-Type", b"application/json").unwrap());

                    let _ = request.respond(response);
                } else {
                    let response = Response::from_string("{\"success\":false,\"error\":\"Invalid JSON\"}")
                        .with_status_code(400)
                        .with_header(Header::from_bytes(b"Content-Type", b"application/json").unwrap());

                    let _ = request.respond(response);
                }
            }

            (&Method::Post, "/token/mint") => {
                    let mut content = String::new();
                    let _ = request.as_reader().read_to_string(&mut content);

                    let req_data: Result<MintTokenRequest, _> = serde_json::from_str(&content);

                    if let Ok(data) = req_data {
                        let mint_pubkey = Pubkey::from_str(&data.mint).unwrap();
                        let dest_pubkey = Pubkey::from_str(&data.destination).unwrap();
                        let authority_pubkey = Pubkey::from_str(&data.authority).unwrap();

                        // Build mint_to instruction
                        let ix = mint_to(
                            &spl_token::id(),
                            &mint_pubkey,
                            &dest_pubkey,
                            &authority_pubkey,
                            &[],
                            data.amount,
                        ).unwrap();


                        let accounts = ix.accounts.iter().map(|meta| {
                            AccountMetaData {
                                pubkey: meta.pubkey.to_string(),
                                is_signer: meta.is_signer,
                                is_writable: meta.is_writable,
                            }
                        }).collect::<Vec<_>>();

                        let response_data = CreateTokenResponse {
                            success: true,
                            data: TokenData {
                                program_id: ix.program_id.to_string(),
                                accounts,
                                instruction_data: general_purpose::STANDARD.encode(ix.data),
                            },
                        };

                        let json = serde_json::to_string(&response_data).unwrap();
                        let response = Response::from_string(json)
                            .with_status_code(200)
                            .with_header(Header::from_bytes(b"Content-Type", b"application/json").unwrap());

                        let _ = request.respond(response);
                    } else {
                        let response = Response::from_string("{\"success\":false,\"error\":\"Invalid JSON\"}")
                            .with_status_code(400)
                            .with_header(Header::from_bytes(b"Content-Type", b"application/json").unwrap());

                        let _ = request.respond(response);
                    }
                }


            (&Method::Post, "/message/sign") => {
                    let mut content = String::new();
                    let _ = request.as_reader().read_to_string(&mut content);

                    let req_data: Result<SignMessageRequest, _> = serde_json::from_str(&content);

                    if let Ok(data) = req_data {
                        if data.message.is_empty() || data.secret.is_empty() {
                            let response = Response::from_string(
                                json!({ "success": false, "error": "Missing required fields" }).to_string()
                            )
                            .with_status_code(400)
                            .with_header(Header::from_bytes(b"Content-Type", b"application/json").unwrap());
                            let _ = request.respond(response);
                            continue;
                        }

                        // Decode secret key from base58
                        let secret_bytes = match bs58::decode(&data.secret).into_vec() {
                            Ok(bytes) => bytes,
                            Err(_) => {
                                let response = Response::from_string(
                                    json!({ "success": false, "error": "Invalid secret key encoding" }).to_string()
                                )
                                .with_status_code(400)
                                .with_header(Header::from_bytes(b"Content-Type", b"application/json").unwrap());
                                let _ = request.respond(response);
                                continue;
                            }
                        };

                        // Convert secret bytes to Keypair
                        let keypair = match Keypair::from_bytes(&secret_bytes) {
                            Ok(kp) => kp,
                            Err(_) => {
                                let response = Response::from_string(
                                    json!({ "success": false, "error": "Invalid secret key length" }).to_string()
                                )
                                .with_status_code(400)
                                .with_header(Header::from_bytes(b"Content-Type", b"application/json").unwrap());
                                let _ = request.respond(response);
                                continue;
                            }
                        };

                        // Sign the message
                        let signature = keypair.sign_message(data.message.as_bytes());

                        let response_data = SignMessageResponse {
                            success: true,
                            data: SignMessageData {
                                signature: base64::engine::general_purpose::STANDARD.encode(signature.as_ref()),
                                public_key: keypair.pubkey().to_string(),
                                message: data.message,
                            },
                        };

                        let json = serde_json::to_string(&response_data).unwrap();
                        let response = Response::from_string(json)
                            .with_status_code(200)
                            .with_header(Header::from_bytes(b"Content-Type", b"application/json").unwrap());

                        let _ = request.respond(response);
                    } else {
                        let response = Response::from_string(
                            json!({ "success": false, "error": "Invalid JSON" }).to_string()
                        )
                        .with_status_code(400)
                        .with_header(Header::from_bytes(b"Content-Type", b"application/json").unwrap());

                        let _ = request.respond(response);
                    }
                }

            (&Method::Post, "/message/verify") => {
                    let mut content = String::new();
                    let _ = request.as_reader().read_to_string(&mut content);

                    let req_data: Result<VerifyMessageRequest, _> = serde_json::from_str(&content);

                    if let Ok(data) = req_data {
                        // Decode signature from base64
                        let signature_bytes = match base64::engine::general_purpose::STANDARD.decode(&data.signature) {
                            Ok(bytes) => bytes,
                            Err(_) => {
                                let response = Response::from_string(
                                    json!({ "success": false, "error": "Invalid signature encoding" }).to_string()
                                )
                                .with_status_code(400)
                                .with_header(Header::from_bytes(b"Content-Type", b"application/json").unwrap());
                                let _ = request.respond(response);
                                continue;
                            }
                        };

                        // Parse public key
                        let pubkey = match Pubkey::from_str(&data.pubkey) {
                            Ok(pk) => pk,
                            Err(_) => {
                                let response = Response::from_string(
                                    json!({ "success": false, "error": "Invalid pubkey format" }).to_string()
                                )
                                .with_status_code(400)
                                .with_header(Header::from_bytes(b"Content-Type", b"application/json").unwrap());
                                let _ = request.respond(response);
                                continue;
                            }
                        };

                        // Parse signature bytes into Signature type
                        let signature = match Signature::try_from(signature_bytes.as_slice()) {
                            Ok(sig) => sig,
                            Err(_) => {
                                let response = Response::from_string(
                                    json!({ "success": false, "error": "Invalid signature length" }).to_string()
                                )
                                .with_status_code(400)
                                .with_header(Header::from_bytes(b"Content-Type", b"application/json").unwrap());
                                let _ = request.respond(response);
                                continue;
                            }
                        };

                        // Verify
                        let valid = signature.verify(pubkey.as_ref(), data.message.as_bytes());

                        let response_data = VerifyMessageResponse {
                            success: true,
                            data: VerifyMessageData {
                                valid,
                                message: data.message,
                                pubkey: pubkey.to_string(),
                            },
                        };

                        let json = serde_json::to_string(&response_data).unwrap();
                        let response = Response::from_string(json)
                            .with_status_code(200)
                            .with_header(Header::from_bytes(b"Content-Type", b"application/json").unwrap());

                        let _ = request.respond(response);
                    } else {
                        let response = Response::from_string(
                            json!({ "success": false, "error": "Invalid JSON" }).to_string()
                        )
                        .with_status_code(400)
                        .with_header(Header::from_bytes(b"Content-Type", b"application/json").unwrap());

                        let _ = request.respond(response);
                    }
                }

            (&Method::Post, "/send/sol") => {
                    let mut content = String::new();
                    let _ = request.as_reader().read_to_string(&mut content);

                    let req_data: Result<SendSolRequest, _> = serde_json::from_str(&content);

                    if let Ok(data) = req_data {
                        // Validate inputs
                        let from_pubkey = match Pubkey::from_str(&data.from) {
                            Ok(pk) => pk,
                            Err(_) => {
                                let response = Response::from_string(
                                    json!({ "success": false, "error": "Invalid sender address" }).to_string()
                                )
                                .with_status_code(400)
                                .with_header(Header::from_bytes(b"Content-Type", b"application/json").unwrap());
                                let _ = request.respond(response);
                                continue;
                            }
                        };

                        let to_pubkey = match Pubkey::from_str(&data.to) {
                            Ok(pk) => pk,
                            Err(_) => {
                                let response = Response::from_string(
                                    json!({ "success": false, "error": "Invalid recipient address" }).to_string()
                                )
                                .with_status_code(400)
                                .with_header(Header::from_bytes(b"Content-Type", b"application/json").unwrap());
                                let _ = request.respond(response);
                                continue;
                            }
                        };

                        if data.lamports == 0 {
                            let response = Response::from_string(
                                json!({ "success": false, "error": "Transfer amount must be greater than zero" }).to_string()
                            )
                            .with_status_code(400)
                            .with_header(Header::from_bytes(b"Content-Type", b"application/json").unwrap());
                            let _ = request.respond(response);
                            continue;
                        }

                        // Create transfer instruction
                        let ix = system_instruction::transfer(&from_pubkey, &to_pubkey, data.lamports);

                        let accounts = ix.accounts.iter().map(|meta| meta.pubkey.to_string()).collect::<Vec<_>>();

                        let response_data = SendSolResponse {
                            success: true,
                            data: SendSolData {
                                program_id: ix.program_id.to_string(),
                                accounts,
                                instruction_data: base64::engine::general_purpose::STANDARD.encode(ix.data),
                            },
                        };

                        let json = serde_json::to_string(&response_data).unwrap();
                        let response = Response::from_string(json)
                            .with_status_code(200)
                            .with_header(Header::from_bytes(b"Content-Type", b"application/json").unwrap());

                        let _ = request.respond(response);
                    } else {
                        let response = Response::from_string(
                            json!({ "success": false, "error": "Invalid JSON" }).to_string()
                        )
                        .with_status_code(400)
                        .with_header(Header::from_bytes(b"Content-Type", b"application/json").unwrap());

                        let _ = request.respond(response);
                    }
                }

            _ => {
                let response = Response::from_string("{\"success\":false,\"error\":\"Not Found\"}")
                    .with_status_code(404)
                    .with_header(Header::from_bytes(b"Content-Type", b"application/json").unwrap());

                let _ = request.respond(response);
            }
        }
    }
}
