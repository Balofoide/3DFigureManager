use reqwest::blocking::Client;
// use reqwest::StatusCode;
use std::env;
use slint::VecModel;
use slint::Model;
use slint::ModelRc;
use crate::{AppWindow,Etiqueta};

fn add_etiqueta(ui: &AppWindow, nova_etiqueta: Etiqueta) {
    let current_model = ui.get_etiquetas_database();

    let vec_model = VecModel::default();

    if let Some(e_model) = current_model.as_any().downcast_ref::<VecModel<Etiqueta>>() {
        for item in e_model.iter() {
            vec_model.push(item.clone());
        }
    }
    vec_model.push(nova_etiqueta);

    ui.set_etiquetas_database(ModelRc::new(vec_model));
}

pub fn get_etiqueta(ui: &AppWindow) {
    ui.set_etiquetas_database(ModelRc::new(VecModel::default()));

    dotenv::dotenv().ok();
    let token = env::var("ACCESS_TOKEN").expect("Erro ao acessar token");
    let client = Client::new();

    let resposta = client
        .get("https://sandbox.melhorenvio.com.br/api/v2/me/orders")
        .header("Authorization", format!("Bearer {}", token))
        .header("Accept", "application/json")
        .send()
        .expect("Erro ao enviar requisição");

    if resposta.status().is_success() {
        let json: serde_json::Value = resposta.json().expect("Erro ao deserializar JSON");
        
        // Acessa o array dentro de "data"
        if let Some(items) = json.get("data").and_then(|d| d.as_array()) {
            for item in items {
                let etiqueta_id = item.get("id").and_then(|id| id.as_str()).unwrap_or("").to_string();
                let status = item.get("status").and_then(|s| s.as_str()).unwrap_or("").to_string();
                let cliente = item.get("to").and_then(|t| t.get("name")).and_then(|n| n.as_str()).unwrap_or("").to_string();
                // let produto =  item.get("products").and_then(|t| t.get("name")).and_then(|n| n.as_str()).unwrap_or("").to_string();
                let preco = item.get("price").and_then(|p| p.as_f64()).unwrap_or(0.0);
                // let servico = item.get("services").and_then(|s| s.get("company")).and_then(|n| n.get("name")).and_then(|n| n.as_str()).unwrap_or("").to_string();
                        let produto = item.get("products")
                    .and_then(|p| p.as_array())
                    .and_then(|arr| arr.get(0))
                    .and_then(|prod| prod.get("name"))
                    .and_then(|n| n.as_str())
                    .unwrap_or("")
                    .to_string();
 

                   let servico = json.get("service")
                    .and_then(|p| p.as_array())
                    .and_then(|arr| arr.get(0))
                    .and_then(|prod| prod.get("name"))
                    .and_then(|n| n.as_str())
                    .unwrap_or("")
                    .to_string();
 

                    // Criando struct com todos os campos
                    let etiqueta = Etiqueta {
                        id: etiqueta_id.clone().into(),
                        status: status.clone().into(),
                        cliente: cliente.clone().into(),
                        produto: produto.clone().into(),
                        preco: preco as f32,
                        servico: servico.clone().into(),
                    };
                    // Exibindo os dados da etiqueta
                    println!("Etiqueta ID: {}", etiqueta.id);   
                    println!("id: {}", etiqueta.id);
                    println!("status: {}", etiqueta.status);
                    println!("cliente: {}", etiqueta.cliente);
                    println!("produto: {}", etiqueta.produto);
                    println!("preco: {}", etiqueta.preco);
                    println!("servico: {}", etiqueta.servico);

                    if status != "pending"{
                        println!("Skipped");
                    }
                    else{add_etiqueta(&ui, etiqueta);}
            }
        } else {
            println!("Nenhum item encontrado no carrinho");
        }
    } else {
        eprintln!("Erro {}: {:?}", resposta.status(), resposta.text());
    }
}