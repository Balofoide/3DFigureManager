use anyhow::{Context, Result};
use reqwest::header;
use serde::{Deserialize, Serialize};
use dotenv::dotenv;
use std::env;
use crate::{AppWindow};


#[derive(Debug, Serialize)]
struct Products {
    name:String,
    quantity:f64,
    unitary_value:f64,
}
 
#[derive(Debug, Serialize)]
struct CartRequest {
    service: u32,
    agency: Option<u32>,
    from: Address,
    to: Address,    
    products: Vec<Products>,
    volumes: Vec<Volume>,
    options: CartOptions,
}

#[derive(Debug, Serialize)]
struct Address {
    postal_code: String,
    name: String,
    phone: String,
    email: String,
    document: String,
    address: String,
    complement: String,
    number: String,
    district: String,
    city: String,
    state_abbr: String,
    country_id: String,
}

#[derive(Debug, Serialize)]
struct Volume {
    height: f64,
    width: f64,
    length: f64,
    weight: f64,
}

#[derive(Debug, Serialize)]
struct CartOptions {
    insurance_value: f64,
    receipt: bool,
    own_hand: bool,
    reverse:bool,
}
#[derive(Debug, Deserialize)]
struct CartResponse {
    id: String,
    // protocol: String,
    // service_id: u32,
    // agency_id: Option<u32>,
    // contract: Option<String>,
    // service_code: Option<String>,
    // quote: f64,
    // price: f64,
    // coupon: Option<f64>,
    // discount: f64,
    // delivery_min: u32,
    // delivery_max: u32,
    // status: String,
    // reminder: Option<String>,
    // insurance_value: f64,
    // weight: Option<f64>,
    // width: Option<f64>,
    // height: Option<f64>,
    // length: Option<f64>,
    // diameter: Option<f64>,
    // format: String,
    // billed_weight: f64,
    // receipt: bool,
    // own_hand: bool,
    // collect: bool,
    // collect_scheduled_at: Option<String>,
    // reverse: u8,
    // non_commercial: bool,
    // authorization_code: Option<String>,
    // tracking: Option<String>,
    // self_tracking: Option<String>,
    // delivery_receipt: Option<String>,
    // additional_info: Option<String>,
    // cte_key: Option<String>,
    // paid_at: Option<String>,
    // generated_at: Option<String>,
    // posted_at: Option<String>,
    // delivered_at: Option<String>,
    // canceled_at: Option<String>,
    // suspended_at: Option<String>,
    // expired_at: Option<String>,
    // created_at: String,
    // updated_at: String,
    // parse_pi_at: Option<String>,
    // received_at: Option<String>,
    // risk: bool,
}

// async fn add_to_cart(request: CartRequest) -> Result<CartResponse> {
//     dotenv().ok();
//     let token = env::var("ACCESS_TOKEN")?;
//     let base_url = env::var("BASE_URL")?;

//     let client = reqwest::Client::new();

//     let response = client
//         .post(&format!("{}/api/v2/me/cart", base_url))
//         .json(&request)
//         .header(header::AUTHORIZATION, format!("Bearer {}", token))
//         .header(header::USER_AGENT, "Minha Loja (suporte@minhaloja.com)")
//         .header(header::CONTENT_TYPE, "application/json")
//         .header(header::ACCEPT, "application/json")
//         .send()
//         .await?;

//     let status = response.status();
//     let response_text = response.text().await?;

//     if !status.is_success() {
//         anyhow::bail!("Erro na requisição ({}): {}", status, response_text);
//     }

//     // Debug adicional para resposta
//     println!("Resposta bruta: {}", response_text);

//     serde_json::from_str(&response_text)
//         .context(format!("Falha ao decodificar resposta. Resposta bruta: {}", response_text))
// }
//
// Versão síncrona da função add_to_cart usando reqwest blocking.
//
fn add_to_cart(request: CartRequest) -> Result<CartResponse> {
    dotenv().ok();
    let token = env::var("ACCESS_TOKEN")?;
    let base_url = env::var("BASE_URL")?;

    let client = reqwest::blocking::Client::new();

    let response = client
        .post(&format!("{}/api/v2/me/cart", base_url))
        .json(&request)
        .header(header::AUTHORIZATION, format!("Bearer {}", token))
        .header(header::USER_AGENT, "Minha Loja (suporte@minhaloja.com)")
        .header(header::CONTENT_TYPE, "application/json")
        .header(header::ACCEPT, "application/json")
        .send()?;

    let status = response.status();
    let response_text = response.text()?;

    if !status.is_success() {
        anyhow::bail!("Erro na requisição ({}): {}", status, response_text);
    }

    println!("Resposta bruta: {}", response_text);

    serde_json::from_str(&response_text)
        .context(format!("Falha ao decodificar resposta. Resposta bruta: {}", response_text))
}

pub fn inserir_carrinho(ui: &AppWindow) {


    let destinatario_nome = ui.get_destinatario_nome().to_string();
    let destinatario_endereco = ui.get_destinatario_endereco().to_string();
    let destinatario_cep = ui.get_destinatario_cep().to_string();
    let destinatario_cpf = ui.get_destinatario_cpf().to_string();
    let destinatario_numero = ui.get_destinatario_numero().to_string();
    let destinatario_estado = ui.get_destinatario_estado().to_string();
    let destinatario_cidade = ui.get_destinatario_cidade().to_string();

    let remetente_nome = ui.get_remetente_nome().to_string();
    let remetente_endereco = ui.get_remetente_endereco().to_string();
    let remetente_cep = ui.get_remetente_cep().to_string();
    let remetente_cpf = ui.get_remetente_cpf().to_string();
    let remetente_numero = ui.get_remetente_numero().to_string();
    let remetente_estado = ui.get_remetente_estado().to_string();
    let remetente_cidade = ui.get_remetente_cidade().to_string();

    let pacote_width:f64 = ui.get_pacote_width().parse().unwrap_or(0.0);
    let pacote_height:f64 = ui.get_pacote_height().parse().unwrap_or(0.0);
    let pacote_largura:f64 = ui.get_pacote_largura().parse().unwrap_or(0.0);
    let pacote_peso:f64 = ui.get_pacote_peso().parse().unwrap_or(0.0);
    let pacote_nome_produto = ui.get_pacote_nome_produto().to_string();
    let pacote_quantidade:f64 = ui.get_pacote_quantidade().parse().unwrap_or(0.0);
    let pacote_preco:f64 = ui.get_pacote_preco().parse().unwrap_or(0.0);
    let pacote_seguro:f64 = ui.get_pacote_seguro().parse().unwrap_or(0.0);

    let aviso_recebimento:bool = ui.get_aviso_recebimento();
    let own_hand:bool = ui.get_own_hand();
    let entrega_reversa:bool = ui.get_entrega_reversa();
    let servico_entrega:u32 = ui.get_servico_entrega().parse().unwrap_or(0); // Exemplo: 2 para Sedex

    let remetente_telefone = ui.get_remetente_telefone().to_string(); // Telefone com DDD
    let destinatario_telefone = ui.get_destinatario_telefone().to_string(); // Telefone com DDD


   
   let request = CartRequest {
        service: servico_entrega,
        agency: None, // Preencher se necessário
        
        // Endereço de origem
        from: Address {
            postal_code: remetente_cep, // CEP sem hífen
            name: remetente_nome,
            phone: remetente_telefone,
            email: "".to_string(),
            document: remetente_cpf,
            address: remetente_endereco,
            complement: "".to_string(),
            number: remetente_numero,
            district: "".to_string(),
            city: remetente_cidade,
            state_abbr: remetente_estado,
            country_id: "BR".to_string(),
        },
        
        // Endereço de destino
        to: Address {
            postal_code: destinatario_cep,
            name: destinatario_nome,
            phone: destinatario_telefone,
            email: "".to_string(),
            document: destinatario_cpf,
            address: destinatario_endereco,
            complement: "".to_string(),
            number: destinatario_numero,
            district: "".to_string(),
            city: destinatario_cidade,
            state_abbr: destinatario_estado,
            country_id: "BR".to_string(),
        },
        
        // Volumes do pacote
        volumes: vec![
            Volume {
                height: pacote_height,  // em cm
                width: pacote_width,   // em cm
                length: pacote_largura,  // em cm
                weight: pacote_peso,   // em kg
            }
        ],
        products: vec![
            Products{
            name:  pacote_nome_produto,
            quantity: pacote_quantidade,
            unitary_value: pacote_preco,
            }
        ],
        
        // Opções do serviço
        options: CartOptions {
            insurance_value: pacote_seguro, // Valor do seguro
            receipt: aviso_recebimento,           // Aviso de recebimento
            own_hand: own_hand,     
            reverse: entrega_reversa,    // Entrega mão própria
        }
    };

   match add_to_cart(request){
        Ok(response) => {
            println!("✅ Pedido criado com sucesso!");
            println!("🆔 ID: {}", response.id);
        }
        Err(e) => eprintln!("❌ Erro: {}", e),
    }

 
}