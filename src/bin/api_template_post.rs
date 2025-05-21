// src/create_shipment.rs

use reqwest::blocking::Client;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde_json::json;
use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();
    let token = env::var("ACCESS_TOKEN").expect("Token não encontrado");

    let client = Client::new();

    let response = client
        .post("https://sandbox.melhorenvio.com.br/api/v2/me/shipment/calculate")
        .header(CONTENT_TYPE, "application/json")
        .header(AUTHORIZATION, format!("Bearer {}", token))
        .json(&json!( {
                "from": {
                    "postal_code": "01002001"
                },
                "to": {
                    "postal_code": "90570020"
                },
                "package": {
                    "height": 4,
                    "width": 12,
                    "length": 17,
                    "weight": 0.3
                },
                "options": {
                    "insurance_value": 1180.87,
                    "receipt": false,
                    "own_hand": false
                },
                "services": "1,2,3,4,7,11"
        }))
        .send()
        .expect("Erro ao criar envio");

    if response.status().is_success() {
        let body: serde_json::Value = response.json().expect("Erro ao parsear JSON");
        let shipment_id = body["id"].as_str().unwrap_or_default();
        println!("Envio criado com sucesso! ID: {}", shipment_id);
    } else {
        println!("Erro: {:?}", response.text().unwrap_or_default());
    }
}
