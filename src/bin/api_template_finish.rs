// src/generate_label.rs

use reqwest::blocking::Client;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde_json::json;
use dotenv::dotenv;
use std::env;




fn main() {
    dotenv().ok();
    let token = env::var("ACCESS_TOKEN").expect("Token não encontrado");

    let envio_id = ""; // Substitua com o ID retornado da Parte 1
    let client = Client::new();

    // Etapa 1: pagar o envio
    let buy = client
        .post("https://sandbox.melhorenvio.com.br/api/v2/me/shipment/checkout")
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


     let print_etiqueta = client
        .post("https://sandbox.melhorenvio.com.br/api/v2/me/shipment/print")
        .header(CONTENT_TYPE, "application/json")
        .header(AUTHORIZATION, format!("Bearer {}", token))
        .json(&json!({ "orders": [envio_id] }))
        .send()
        .expect("Erro ao gerar etiqueta");

    if !generate.status().is_success() {
        println!("Erro ao gerar link da etiqueta: {:?}", print_etiqueta.text());
        return;
    }

  
    

    // Para obter a URL da etiqueta, você pode extrair o corpo da resposta:
    let response_json: serde_json::Value = print_etiqueta
        .json()
        .expect("Erro ao ler resposta da etiqueta");

    // Ajuste conforme o formato real da resposta da API
    if let Some(url) = response_json.get("url") {
        println!("URL: {}", url);
    } else {
        println!("URL da etiqueta não encontrada na resposta: {:?}", response_json);
    }
    
}
