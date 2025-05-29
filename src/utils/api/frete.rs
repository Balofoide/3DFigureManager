use reqwest::blocking::Client;
use serde::Serialize;
use std::env;
use slint::VecModel;
use slint::Model;
use slint::ModelRc;
use crate::{AppWindow,Calculo_Entrega};


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




pub fn add_transportadora(ui: &AppWindow, nova_transportadora: Calculo_Entrega) {
    let current_model = ui.get_calculo_entrega_database();

    let vec_model = VecModel::default();

    if let Some(e_model) = current_model.as_any().downcast_ref::<VecModel<Calculo_Entrega>>() {
        for item in e_model.iter() {
            vec_model.push(item.clone());
        }
    }
    vec_model.push(nova_transportadora);

    ui.set_calculo_entrega_database(ModelRc::new(vec_model));
}

pub fn resetar_transportadora(ui: &AppWindow){
    
    let vec_model = VecModel::default();
    ui.set_calculo_entrega_database(ModelRc::new(vec_model));
}

pub fn calcular_frete(ui: &AppWindow){
    dotenv::dotenv().ok();
    let cep_origem: String = ui.get_cep_origem().to_string();
    let cep_destino: String = ui.get_cep_destino().to_string();

    let token = env::var("ACCESS_TOKEN").expect("Erro ao acessar token");

    let client = Client::new();

   let frete = ShipmentRequest {
        from: Location {
            postal_code:  cep_origem, // Remetente
        },
        to: Location {
            postal_code: cep_destino, // Destinatário
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

    let resposta = client
    .post("https://sandbox.melhorenvio.com.br/api/v2/me/shipment/calculate")
    .header("Authorization", format!("Bearer {}", token))
    .header("Accept", "application/json")
    .header("Content-Type", "application/json")
    .json(&frete)
    .send()
    .expect("Erro ao enviar requisição");

    if resposta.status().is_success() {
        let json: serde_json::Value = resposta.json().expect("erro ao deserializar json");
        println!("📦 Resultados de frete disponíveis:\n");

        if let Some(array) = json.as_array() {
            for item in array {
                let nome_transportadora = item["name"].as_str().unwrap_or("-");
                let preco_de_entrega = item["price"].as_str().unwrap_or("-");
                let tempo_de_entrega = item["delivery_time"].as_i64().unwrap_or(0);
                let id_transportadora = item["id"].as_i64().unwrap_or(0);
                println!("{}- {}: R$ {} ({} dias úteis)", id_transportadora,nome_transportadora, preco_de_entrega, tempo_de_entrega);

                 let transportadora = Calculo_Entrega {
                    id: id_transportadora as i32,
                    nome_transportadora: nome_transportadora.into(),
                    tempo_de_entrega: tempo_de_entrega as i32,
                    preco_entrega: preco_de_entrega.into(),
                };
                if preco_de_entrega != "-"{
                add_transportadora(&ui, transportadora);
                }
                }
            }
        
    } else {
        eprintln!("Erro {}: {:?}", resposta.status(), resposta.text());
    }
}

