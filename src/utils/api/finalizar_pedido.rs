 
use reqwest::blocking::Client;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde_json::json;
use dotenv::dotenv;
use std::env;

 use crate::{AppWindow};


 pub fn finalizar_pedido(ui: &AppWindow) {
    dotenv().ok();

    let token = env::var("ACCESS_TOKEN").expect("Token não encontrado");

    let envio_id = ui.get_id_pedido().to_string(); // Substitua com o ID retornado da Parte 1
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
}