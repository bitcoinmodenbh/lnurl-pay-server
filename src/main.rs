use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use bech32::ToBase32;
use hex::encode as hex_encode;
use reqwest::{Client, Certificate};
use serde::Deserialize;
use serde_json::json;
use std::fs;

const DOMAIN: &str = ""; // Replace with your real domain
const LND_URL: &str = ""; // Lightning REST API address
const MACAROON_PATH: &str = ""; // Path to the macaroon file
const TLS_CERT_PATH: &str = ""; // Path to the LND TLS certificate

#[derive(Deserialize)]
struct CallbackQuery {
    amount: u64,
}

async fn lnurlp_info() -> impl Responder {
    let callback = format!("{}/.well-known/lnurlp/sats/callback", DOMAIN);
    let metadata = serde_json::to_string(&vec![vec!["text/plain", "Pay 1 to 1000 satoshis"]]).unwrap();
    HttpResponse::Ok().json(json!({
        "callback": callback,
        "maxSendable": 1_000_000, // Maximum 1000 satoshis (1,000,000 millisatoshis)
        "minSendable": 1_000,     // Minimum 1 satoshi (1,000 millisatoshis)
        "metadata": metadata,
        "tag": "payRequest"
    }))
}

async fn lnurlp_callback(query: web::Query<CallbackQuery>) -> impl Responder {
    let min_amount = 1_000;  // Minimum 1 satoshi
    let max_amount = 1_000_000;  // Maximum 1000 satoshis

    if query.amount < min_amount || query.amount > max_amount {
        return HttpResponse::BadRequest().body(format!(
            "The amount must be between {} and {} millisatoshis (1 to 1000 satoshis)",
            min_amount, max_amount
        ));
    }

    let macaroon_hex = hex_encode(fs::read(MACAROON_PATH).expect("Error reading macaroon"));
    let tls_cert = fs::read(TLS_CERT_PATH).expect("Error reading TLS certificate");
    let cert = Certificate::from_pem(&tls_cert).expect("Error parsing TLS certificate");

    let client = Client::builder()
        .add_root_certificate(cert)
        .build()
        .expect("Error building HTTP client");

    let res = client.post(format!("{}/v1/invoices", LND_URL))
        .header("Grpc-Metadata-macaroon", macaroon_hex)
        .json(&json!({
            "value": (query.amount / 1000).to_string(), // Convert millisatoshis to satoshis
            "memo": format!("Payment of {} satoshis", query.amount / 1000)
        }))
        .send()
        .await
        .expect("Error communicating with LND");

    let invoice: serde_json::Value = res.json().await.expect("Error parsing LND response");
    let pr = invoice["payment_request"].as_str().expect("Field payment_request not found");

    HttpResponse::Ok().json(json!({
        "pr": pr
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let url = format!("{}/.well-known/lnurlp/sats", DOMAIN);
    let lnurl = bech32::encode("lnurl", url.as_bytes().to_base32(), bech32::Variant::Bech32)
        .expect("Error encoding LNURL");
    let lightning_address = format!("sats@{}", DOMAIN.trim_start_matches("https://").trim_start_matches("http://"));
    println!("LNURLp: {}", lnurl);
    println!("Lightning address: {}", lightning_address);

    HttpServer::new(|| {
        App::new()
            .route("/.well-known/lnurlp/sats", web::get().to(lnurlp_info))
            .route("/.well-known/lnurlp/sats/callback", web::get().to(lnurlp_callback))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
