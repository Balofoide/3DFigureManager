use dotenv::dotenv;
use std::env;
use std::collections::HashMap;
use reqwest::blocking::Client;
use serde::Deserialize;

#[derive(Debug,Deserialize)]

struct Api_response {
    access_token: String,
    expires_in: u64,
    refresh_token:String,
    token_type:String,
}

fn get_token() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let client_id = env::var("CLIENT_ID")?;
    let client_secret = env::var("CLIENT_SECRET")?;
    let redirect_uri = env::var("REDIRECT_URI")?;
    let code = env::var("CODE")?;

    let mut form = HashMap::new();
    form.insert("grant_type", "authorization_code");
    form.insert("client_id", &client_id);
    form.insert("client_secret", &client_secret);
    form.insert("redirect_uri", &redirect_uri);
    form.insert("code", &code);

    let client = Client::new();
    let res = client
        .post("https://sandbox.melhorenvio.com.br/oauth/token")
        .form(&form)
        .send()?;

    if res.status().is_success() {
        let token: Api_response = res.json()?;
        println!("Access token: {}", token.access_token);
        println!("Refresh token: {}", token.refresh_token);
        println!("Token expires in: {} seconds", token.expires_in);
    } else {
        println!("Erro: status {}", res.status());
        println!("Resposta: {}", res.text()?);
    }

    Ok(())
}