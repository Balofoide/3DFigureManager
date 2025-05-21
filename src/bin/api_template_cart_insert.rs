use anyhow::{Context, Result};
use reqwest::header;
use serde::{Deserialize, Serialize};
use dotenv::dotenv;
use std::env;



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
}
#[derive(Debug, Deserialize)]
struct CartResponse {
    id: String,
    protocol: String,
    service_id: u32,
    agency_id: Option<u32>,
    contract: Option<String>,
    service_code: Option<String>,
    quote: f64,
    price: f64,
    coupon: Option<f64>,
    discount: f64,
    delivery_min: u32,
    delivery_max: u32,
    status: String,
    reminder: Option<String>,
    insurance_value: f64,
    weight: Option<f64>,
    width: Option<f64>,
    height: Option<f64>,
    length: Option<f64>,
    diameter: Option<f64>,
    format: String,
    billed_weight: f64,
    receipt: bool,
    own_hand: bool,
    collect: bool,
    collect_scheduled_at: Option<String>,
    reverse: u8,
    non_commercial: bool,
    authorization_code: Option<String>,
    tracking: Option<String>,
    self_tracking: Option<String>,
    delivery_receipt: Option<String>,
    additional_info: Option<String>,
    cte_key: Option<String>,
    paid_at: Option<String>,
    generated_at: Option<String>,
    posted_at: Option<String>,
    delivered_at: Option<String>,
    canceled_at: Option<String>,
    suspended_at: Option<String>,
    expired_at: Option<String>,
    created_at: String,
    updated_at: String,
    parse_pi_at: Option<String>,
    received_at: Option<String>,
    risk: bool,
}

async fn add_to_cart(request: CartRequest) -> Result<CartResponse> {
    dotenv().ok();
    let token = env::var("ACCESS_TOKEN")?;
    let base_url = env::var("BASE_URL")?;

    let client = reqwest::Client::new();

    let response = client
        .post(&format!("{}/api/v2/me/cart", base_url))
        .json(&request)
        .header(header::AUTHORIZATION, format!("Bearer {}", token))
        .header(header::USER_AGENT, "Minha Loja (suporte@minhaloja.com)")
        .header(header::CONTENT_TYPE, "application/json")
        .header(header::ACCEPT, "application/json")
        .send()
        .await?;

    let status = response.status();
    let response_text = response.text().await?;

    if !status.is_success() {
        anyhow::bail!("Erro na requisição ({}): {}", status, response_text);
    }

    // Debug adicional para resposta
    println!("Resposta bruta: {}", response_text);

    serde_json::from_str(&response_text)
        .context(format!("Falha ao decodificar resposta. Resposta bruta: {}", response_text))
}

// Atualize o main para usar a nova struct
#[tokio::main]
async fn main() -> Result<()> {
   let request = CartRequest {
        service: 2, // ID do serviço (ex: 2 = Sedex)
        agency: None, // Preencher se necessário
        
        // Endereço de origem
        from: Address {
            postal_code: "01001000".to_string(), // CEP sem hífen
            name: "Loja Exemplo Ltda".to_string(),
            phone: "1133334444".to_string(), // Telefone com DDD
            email: "loja@exemplo.com".to_string(),
            document: "12345678909".to_string(), // CPF/CNPJ
            address: "Rua Principal".to_string(),
            complement: "Sala 4".to_string(),
            number: "123".to_string(),
            district: "Centro".to_string(),
            city: "São Paulo".to_string(),
            state_abbr: "SP".to_string(), // UF com 2 caracteres
            country_id: "BR".to_string(),
        },
        
        // Endereço de destino
        to: Address {
            postal_code: "02002000".to_string(),
            name: "Cliente Exemplo".to_string(),
            phone: "1199998888".to_string(),
            email: "cliente@exemplo.com".to_string(),
            document: "98765432100".to_string(),
            address: "Avenida Secundária".to_string(),
            complement: "Ap 102".to_string(),
            number: "456".to_string(),
            district: "Jardim Paulista".to_string(),
            city: "São Paulo".to_string(),
            state_abbr: "SP".to_string(),
            country_id: "BR".to_string(),
        },
        
        // Volumes do pacote
        volumes: vec![
            Volume {
                height: 2.0,  // em cm
                width: 1.0,   // em cm
                length: 1.0,  // em cm
                weight: 0.2,   // em kg
            }
        ],
        products: vec![
            Products{
            name: "Venom Figure".to_string(),
            quantity:1.0,
            unitary_value:100.0,
            }
        ],
        
        // Opções do serviço
        options: CartOptions {
            insurance_value: 100.00, // Valor do seguro
            receipt: true,           // Aviso de recebimento
            own_hand: false,         // Entrega mão própria
        }
    };

   match add_to_cart(request).await {
        Ok(response) => {
            println!("✅ Pedido criado com sucesso!");
            println!("🆔 ID: {}", response.id);
            println!("📄 Protocolo: {}", response.protocol);
            println!("💲 Preço: R$ {:.2}", response.price);
            println!("📅 Prazo: {} a {} dias úteis", 
                response.delivery_min, 
                response.delivery_max
            );
        }
        Err(e) => eprintln!("❌ Erro: {}", e),
    }

    Ok(())
}