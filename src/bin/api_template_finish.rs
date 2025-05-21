// src/generate_label.rs

use reqwest::blocking::Client;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde_json::json;
use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();
    let token = env::var("ACCESS_TOKEN").expect("Token não encontrado");

    let envio_id = "SEU_ID_AQUI"; // Substitua com o ID retornado da Parte 1
    let client = Client::new();

    // Etapa 1: pagar o envio
    let buy = client
        .post("https://sandbox.melhorenvio.com.br/api/v2/me/shipment/buy")
        .header(CONTENT_TYPE, "application/json")
        .header(AUTHORIZATION, format!("Bearer {}", token))
        .json(&json!({ "orders": [envio_id] }))
        .send()
        .expect("Erro ao pagar envio");

    if !buy.status().is_success() {
        println!("Erro ao pagar: {:?}", buy.text());
        return;
    }
    println!("Envio pago.");

    // Etapa 2: gerar etiqueta
    let generate = client
        .post("https://sandbox.melhorenvio.com.br/api/v2/me/shipment/generate")
        .header(CONTENT_TYPE, "application/json")
        .header(AUTHORIZATION, format!("Bearer {}", token))
        .json(&json!({ "orders": [envio_id] }))
        .send()
        .expect("Erro ao gerar etiqueta");

    if !generate.status().is_success() {
        println!("Erro ao gerar etiqueta: {:?}", generate.text());
        return;
    }

    println!("Etiqueta gerada com sucesso.");

    // Etapa 3: obter link da etiqueta
    let label_url = format!(
        "https://sandbox.melhorenvio.com.br/api/v2/me/shipment/print?orders[]={}",
        envio_id
    );

    println!("Imprimir etiqueta: {}", label_url);
}
