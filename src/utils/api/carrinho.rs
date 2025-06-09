use reqwest::blocking::Client;
use std::env;
use slint::VecModel;
use slint::Model;
use slint::ModelRc;
use crate::{AppWindow,Carrinho};


pub fn add_carrinho(ui: &AppWindow, novo_carrinho: Carrinho) {
    let current_model = ui.get_carrinho_database();

    let vec_model = VecModel::default();

    if let Some(e_model) = current_model.as_any().downcast_ref::<VecModel<Carrinho>>() {
        for item in e_model.iter() {
            vec_model.push(item.clone());
        }
    }
    vec_model.push(novo_carrinho);
     
    ui.set_carrinho_database(ModelRc::new(vec_model));
}

pub fn get_carrinho(ui: &AppWindow) {
    ui.set_carrinho_database(ModelRc::new(VecModel::default()));
    
    dotenv::dotenv().ok();
    let token = env::var("ACCESS_TOKEN").expect("Erro ao acessar token");
    let client = Client::new();

    let resposta = client
        .get("https://sandbox.melhorenvio.com.br/api/v2/me/cart")
        .header("Authorization", format!("Bearer {}", token))
        .header("Accept", "application/json")
        .send()
        .expect("Erro ao enviar requisição");

    if resposta.status().is_success() {
        let json: serde_json::Value = resposta.json().expect("Erro ao deserializar JSON");
        
        // Acessa o array dentro de "data"
        if let Some(items) = json.get("data").and_then(|d| d.as_array()) {
            for item in items {
                // Acessando campos aninhados com tratamento seguro
                let nome_cliente = item.get("to")
                    .and_then(|t| t.get("name"))
                    .and_then(|n| n.as_str())
                    .unwrap_or("")
                    .to_string();

                let id_pedido = item.get("id")
                    .and_then(|id| id.as_str())
                    .unwrap_or("")
                    .to_string();

                // Acessando service -> company -> name
                let transportadora = item.get("service")
                    .and_then(|s| s.get("company"))
                    .and_then(|c| c.get("name"))
                    .and_then(|n| n.as_str())
                    .unwrap_or("")
                    .to_string();

                let preco = item.get("price")
                .and_then(|p| p.as_f64())
                .unwrap_or(0.0);

                 
                // Criando struct com todos os campos
                let carrinho = Carrinho {
                    nome_cliente_carrinho: nome_cliente.clone().into(),
                    id_pedido: id_pedido.clone().into(),
                    transportadora: transportadora.clone().into(),
                    figure: "".to_string().into(), // Pode ser preenchido depois
                    preco:preco as f32,
                };

                println!("nome_cliente: {}", nome_cliente);
                println!("id: {}", id_pedido);
                println!("transportadora: {}", transportadora);
                println!("preco: {}", preco);

                add_carrinho(&ui, carrinho);
            }
        } else {
            println!("Nenhum item encontrado no carrinho");
        }
    } else {
        eprintln!("Erro {}: {:?}", resposta.status(), resposta.text());
    }
}