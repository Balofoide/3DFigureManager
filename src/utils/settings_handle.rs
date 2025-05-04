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
}

pub fn set_settings(ui: &AppWindow) {
    let settings_energia = ui.get_settings_energia().to_string();
    let settings_lucro = ui.get_settings_lucro().to_string();

    ui.set_energia(settings_energia.into());
    ui.set_lucro(settings_lucro.into());
}

pub fn save_settings(energia: String, lucro: String) -> std::io::Result<()> {
    let client_data = JsonSettings { energia, lucro };

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("settings.jsonl")?;

    let json = serde_json::to_string(&client_data)?;
    writeln!(file, "{}", json)?;

    Ok(())
}

pub fn registrar_settings(ui: &AppWindow) {
    set_settings(ui);
   let _= save_settings(ui.get_energia().to_string(), ui.get_lucro().to_string());
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

        ui.set_settings_energia(energia_json.clone().into());
        ui.set_settings_lucro(lucro_json.clone().into());

        ui.set_energia(energia_json.clone().into());
        ui.set_lucro(lucro_json.clone().into());
    }

    Ok(())
}
