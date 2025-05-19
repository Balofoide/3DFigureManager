use serde_json;
 

use std::io::Write;

use std::fs::OpenOptions;
use std::io::BufRead;
use std::io::BufReader;

use crate::AppWindow;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct JsonSettings {
    lucro: String,
    energia: String,
    tema: String,
}

pub fn set_settings(ui: &AppWindow) {
    let settings_energia = ui.get_settings_energia().to_string();
    let settings_lucro = ui.get_settings_lucro().to_string();
 
    ui.set_energia(settings_energia.into());
    ui.set_lucro(settings_lucro.into());
}

pub fn save_settings(energia: String, lucro: String,tema: String) -> std::io::Result<()> {
    let client_data = JsonSettings { energia, lucro, tema};

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("settings.jsonl")?;

    let json = serde_json::to_string(&client_data)?;
    writeln!(file, "{}", json)?;

    Ok(())
}

pub fn registrar_settings(ui: &AppWindow) {
    set_settings(ui);
   let _= save_settings(ui.get_energia().to_string(), ui.get_lucro().to_string(),ui.get_tema().to_string());
}

pub fn load_settings(ui: &AppWindow) -> std::io::Result<()> {
    let file = match OpenOptions::new().read(true).open("settings.jsonl") {
        Ok(file) => file,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(e) => return Err(e),
    };

    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let json_client: JsonSettings = serde_json::from_str(&line)?;

        let energia_json = json_client.energia;
        let lucro_json = json_client.lucro;
        let tema = json_client.tema;

        ui.set_settings_energia(energia_json.clone().into());
        ui.set_settings_lucro(lucro_json.clone().into());
        ui.set_tema(tema.clone().into());

        ui.set_energia(energia_json.clone().into());
        ui.set_lucro(lucro_json.clone().into());
    }

    Ok(())
}


pub fn load_tema(ui: &AppWindow) {

    if ui.get_tema().to_string() == "Light"{
        
        // Cores Principais (baseadas na paleta)
        ui.set_primary(slint::Color::from_argb_u8(0xFF, 0x40, 0x1F, 0x71));        /* Roxo escuro vibrante */
        ui.set_primaryLight(slint::Color::from_argb_u8(0xFF, 0x82, 0x4D, 0x74));   /* Roxo médio suavizado */
        ui.set_primaryDark(slint::Color::from_argb_u8(0xFF, 0x2A, 0x13, 0x49));    /* Tom mais escuro derivado da primary */

        // Cores de Apoio (tons terciários da paleta)
        ui.set_secondary(slint::Color::from_argb_u8(0xFF, 0xBE, 0x7B, 0x72));      /* Salmão rosado */
        ui.set_secondaryLight(slint::Color::from_argb_u8(0xFF, 0xFD, 0xAF, 0x7B)); /* Salmão claro energético */

        // Cores de Texto e Superfície
        ui.set_onPrimary(slint::Color::from_argb_u8(0xFF, 0xFF, 0xFF, 0xFF));      /* Texto branco puro */
        ui.set_onSecondary(slint::Color::from_argb_u8(0xFF, 0x2A, 0x2A, 0x2A));    /* Texto escuro suave */
        ui.set_t_background(slint::Color::from_argb_u8(0xFF, 0xFD, 0xF5, 0xF0));    /* Fundo bege-claro (tom pastel do FDAF7B) */
        ui.set_surface(slint::Color::from_argb_u8(0xFF, 0xF0, 0xE4, 0xE1));        /* Superfície rosada neutra */
        ui.set_onBackground(slint::Color::from_argb_u8(0xFF, 0x40, 0x1F, 0x71));   /* Texto usando a cor primary */
        ui.set_onSurface(slint::Color::from_argb_u8(0xFF, 0x4A, 0x2B, 0x5C));
        ui.set_onSurfaceInput(slint::Color::from_argb_u8(0x57, 0x4A, 0x2B, 0x5C)); /* Texto roxo-escuro suavizado */
        
        // Cores de Estado (mantidas para contraste)
        ui.set_error(slint::Color::from_argb_u8(0xFF, 0xD3, 0x2F, 0x2F));
        ui.set_onError(slint::Color::from_argb_u8(0xFF, 0xFF, 0xFF, 0xFF));      
    }

    else if ui.get_tema().to_string() == "Dark"{
        ui.set_primary(slint::Color::from_argb_u8(0xFF, 0xAB, 0x44, 0x59));        /* Vinho intenso (tom base) */
        ui.set_primaryLight(slint::Color::from_argb_u8(0xFF, 0xF2, 0x9F, 0x58));   /* Laranja dourado (realce vibrante) */
        ui.set_primaryDark(slint::Color::from_argb_u8(0xFF, 0x44, 0x17, 0x52));    /* Roxo profundo */

        // Cores de Apoio
        ui.set_secondary(slint::Color::from_argb_u8(0xFF, 0x1B, 0x18, 0x33));      /* Azul-noite escuro */
        ui.set_secondaryLight(slint::Color::from_argb_u8(0xFF, 0x2A, 0x27, 0x4D)); /* Azul médio derivado */

        // Cores de Texto e Superfície
        ui.set_onPrimary(slint::Color::from_argb_u8(0xFF, 0xFF, 0xF4, 0xEB));      /* Branco cremoso */
        ui.set_onSecondary(slint::Color::from_argb_u8(0xFF, 0xEA, 0xD8, 0xC8));    /* Bege claro */
        ui.set_t_background(slint::Color::from_argb_u8(0xFF, 0x12, 0x0E, 0x1A));   /* Fundo azul quase preto */
        ui.set_surface(slint::Color::from_argb_u8(0xFF, 0x1F, 0x1A, 0x2F));        /* Superfície roxa escura */
        ui.set_onBackground(slint::Color::from_argb_u8(0xFF, 0xD1, 0xC8, 0xE1));   /* Lilás claro */
        ui.set_onSurface(slint::Color::from_argb_u8(0xFF, 0xB8, 0xAE, 0xCF));      /* Lavanda média */

        // Cores de Estado
        ui.set_error(slint::Color::from_argb_u8(0xFF, 0xC4, 0x45, 0x45));          /* Vermelho terroso */
        ui.set_onError(slint::Color::from_argb_u8(0xFF, 0xFF, 0xE9, 0xE1));        /* Rosa claro suave */
        
    }


}
