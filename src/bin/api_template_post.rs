use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize)]
struct ShipmentRequest {
    from: Location,
    to: Location,
    products: Vec<Product>,
    options: Options,
    services: Vec<String>,
}

#[derive(Serialize)]
struct Location {
    postal_code: String,
}

#[derive(Serialize)]
struct Product {
    width: f32,
    height: f32,
    length: f32,
    weight: f32,
    quantity: u32,
}

#[derive(Serialize)]
struct Options {
    insurance_value: f32,
    receipt: bool,
    own_hand: bool,
    reverse: bool,
    non_commercial: bool,
}

#[derive(Deserialize, Debug)]
struct FreightResult {
    name: String,
    price: String,
    delivery_time: u32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let token = env::var("ACCESS_TOKEN")?;

    let client = Client::new();

    let shipment = ShipmentRequest {
        from: Location {
            postal_code: "71805108".to_string(), // Remetente
        },
        to: Location {
            postal_code: "71805108".to_string(), // Destinatário
        },
        products: vec![Product {
            width: 11.0,
            height: 17.0,
            length: 20.0,
            weight: 0.3,
            quantity: 1,
        }],
        options: Options {
            insurance_value: 100.0,
            receipt: false,
            own_hand: false,
            reverse: false,
            non_commercial: true,
        },
        services: vec![], // vazio = retornar todos disponíveis
    };

    let response = client
        .post("https://sandbox.melhorenvio.com.br/api/v2/me/shipment/calculate")
        .header("Authorization", format!("Bearer {}", token))
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .json(&shipment)
        .send()?;

    if response.status().is_success() {
        let json: serde_json::Value = response.json()?;
        println!("📦 Resultados de frete disponíveis:\n");

        if let Some(array) = json.as_array() {
            for item in array {
                let name = item["name"].as_str().unwrap_or("-");
                let price = item["price"].as_str().unwrap_or("-");
                let prazo = item["delivery_time"].as_u64().unwrap_or(0);
                let id = item["id"].as_i64().unwrap_or(0);
                println!("{}- {}: R$ {} ({} dias úteis)", id,name, price, prazo);
            }
        }
    } else {
        eprintln!("Erro {}: {:?}", response.status(), response.text()?);
    }

    Ok(())
} 