use dotenv::dotenv;
use std::env;
use reqwest::blocking::Client;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let token = env::var("ACCESS_TOKEN")?;

    let client = Client::new();
    let end_point = "https://sandbox.melhorenvio.com.br/api/v2/me/shipment/app-settings";
    let response = client
        .get(end_point)
        .header("Authorization", format!("Bearer {}", token))
        .send()?;

    println!("Status HTTP: {}", response.status());

    let body = response.text()?;
    println!("Resposta bruta:\n{}", body);

    Ok(())
}
