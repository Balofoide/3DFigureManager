use crate::{AppWindow,Impressoras};

use slint::Model;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::OpenOptions;
use std::io::Write;
use crate::utils::impressora_handle::JsonImpressora;

fn filament_coust(coust: f64) -> f64 {
    let filament_gram = coust / 1000.0; // converte o preço do filamento de kg para grama
    return filament_gram;
}

fn object_coust(object: f64, filament_price: f64) -> f64 {
    let object_price: f64 = object * filament_coust(filament_price);

    return object_price;
}

fn print_coust(time: i32, energy_price: f64, w: i64) -> f64 {
    let kwatts: f64 = w as f64 / 1000.0; // converte watts para kilowatts
    let energy_cousted = time as f64 * kwatts; // energia consumida = tempo * kilowatts
    let print = energy_cousted * energy_price; // custo de impressão = energia consumida * preço da energia
    return print;
}

fn sell_price(time: i32, w: i64, energy_price: f64, object: f64, filament_price: f64) -> f64 {
    let energy = print_coust(time, energy_price, w); // custo de energia
    let model = object_coust(object, filament_price); // custo do modelo
    let total = energy + model; // custo total
    return total;
}

pub fn calcular_venda(ui: &AppWindow) {
    let print_weight: f64 = ui.get_material().parse().unwrap_or(0.0);
    let print_time: i32 = ui.get_tempo().parse().unwrap_or(0);

    let filament_price: f64 = ui.get_filamento().parse().unwrap_or(0.0);

    // let watts: i64 = ui.get_watts().parse().unwrap_or(0);

    let watts = match get_combobox(ui) {
        Some(value) if !value.is_empty() => value.parse::<i64>().unwrap_or_else(|_| {
            eprintln!("Erro ao converter watts: {}", value);
            0
        }),
        _ => {
            println!("Nenhuma impressora selecionada ou valor vazio");
            0
        }
    };

    println!("Filamento:",);
    println!("Watts: {}", watts);
    let energy_kwh: f64 = ui.get_energia().parse().unwrap_or(0.0);

    let profit: i64 = ui.get_lucro().parse().unwrap_or(0);

    let total = sell_price(print_time, watts, energy_kwh, print_weight, filament_price)
        * (1 as f64 + profit as f64 / 100.0);

    ui.set_total(total as f32);
}

pub fn get_combobox(ui: &AppWindow) -> Option<String> {
    let selected_box = ui.get_combobox_selected();
    let mut impressoras = ui.get_impressoras();

    // Adicione debug para verificar os valores
    println!("Selected: {:?}", selected_box);
    println!("Impressoras: {:?}", impressoras);

    return impressoras
        .iter()
        .find(|i| {
            println!("Comparando: {} == {}", i.modelo, selected_box);
            i.modelo == selected_box
        })
        .map(|mut impressora| {
            println!("Impressora encontrada: {:?}", impressora);
            impressora.watts.to_string()
        });
}

pub fn total_vendas(ui: &AppWindow) -> f32 {
    return ui.get_clients_database().iter().map(|i| i.preco).sum();
}
pub fn atualizar_filamento(ui: &AppWindow, filamento: i32) {
    let selected_model = ui.get_combobox_selected();

    // 1. Obter a lista atual de impressoras
    let mut impressoras = ui
        .get_impressoras()
        .iter()
        .enumerate()
        .map(|(idx, _)| {
            let data = ui.get_impressoras().row_data(idx).unwrap();
            data
        })
        .collect::<Vec<Impressoras>>();

    // 2. Encontrar e modificar a impressora selecionada
    if let Some(impressora) = impressoras.iter_mut().find(|i| i.modelo == selected_model) {
        let novo_filamento = (impressora.filamento - filamento).max(0);
        
        // 3. Atualizar o JSON
        if let Err(e) = atualizar_filamento_json(&selected_model, novo_filamento) {
            eprintln!("Erro ao atualizar arquivo: {}", e);
            return;
        }
        
        impressora.filamento = novo_filamento;

        // 4. Atualizar UI
        let new_model = slint::VecModel::from(impressoras);
        ui.set_impressoras(slint::ModelRc::new(new_model));
    }
}

// Nova função para atualizar apenas o filamento no JSON
fn atualizar_filamento_json(modelo: &str, novo_filamento: i32) -> std::io::Result<()> {
    // Ler todas as linhas
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("impressoras.jsonl")?;
    
    let reader = BufReader::new(file);
    let mut impressoras = Vec::new();

    // Processar cada linha
    for line in reader.lines() {
        let mut json_imp: JsonImpressora = serde_json::from_str(&line?)?;
        
        // Atualizar apenas o registro correspondente
        if json_imp.modelo == modelo {
            json_imp.filamento = novo_filamento;
        }
        
        impressoras.push(json_imp);
    }

    // Reescrever todo o arquivo com os dados atualizados
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("impressoras.jsonl")?;

    for imp in impressoras {
        writeln!(file, "{}", serde_json::to_string(&imp)?)?;
    }

    Ok(())
}