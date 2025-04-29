use serde_json;
use slint::Model;

use std::io::Write;

use slint::ModelRc;
use slint::VecModel;
use std::fs::OpenOptions;
use std::io::BufRead;
use std::io::BufReader;
 
use slint::SharedString;
use crate::{AppWindow,Impressoras};

#[derive(serde::Serialize, serde::Deserialize)]
struct JsonImpressora {
    modelo: String,
    watts: String,
}


pub fn add_impressora(ui: &AppWindow, novo_imp: Impressoras) {
    // 1. Extrai o modelo atual, clonando seus itens para um VecModel
    let current: ModelRc<Impressoras> = ui.get_impressoras();
    let vec_model = VecModel::default();
    if let Some(vm) = current.as_any().downcast_ref::<VecModel<Impressoras>>() {
        for imp in vm.iter() {
            vec_model.push(imp.clone());
        }
    }
    // 2. Adiciona a nova impressora
    vec_model.push(novo_imp);
    // 3. Constrói um ModelRc diretamente do VecModel
    ui.set_impressoras(ModelRc::new(vec_model));
}


pub fn convert_impressora(ui: &AppWindow) {
    // 1. Recupera o modelo de Impressoras
    let models: ModelRc<Impressoras> = ui.get_impressoras();
    let labels = VecModel::default();
    // 2. Para cada Impressoras, cria uma SharedString "modelo (watts)"
    if let Some(vm) = models.as_any().downcast_ref::<VecModel<Impressoras>>() {
        for imp in vm.iter() {
            let label = SharedString::from(format!("{}", imp.modelo));
            labels.push(label);
        }
    }
    // 3. Atualiza a propriedade impressoras_string com ModelRc::new
    ui.set_impressoras_string(ModelRc::new(labels));
}

pub fn register_impressora(ui: &AppWindow) {
    // 1. Lê os campos de input
    let modelo_txt = ui.get_input_modelo().to_string();
    let watts_txt  = ui.get_input_watts().to_string();

    // 2. Cria a struct Impressoras
    let impressora = Impressoras {
        modelo: SharedString::from(modelo_txt.clone()),
        watts:  SharedString::from(watts_txt.clone()),
    };

    // 3. Adiciona ao modelo e converte para strings
    add_impressora(ui, impressora);
    convert_impressora(ui);
    save_impressora(modelo_txt.clone(), watts_txt.clone()).expect("erro ao salvar impressoras");
}

pub fn save_impressora(modelo: String, watts: String) -> std::io::Result<()> {
  

    let impressora_data = JsonImpressora {
        modelo,
        watts,
    };

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("impressoras.jsonl")?;

    let json = serde_json::to_string(&impressora_data)?;
    writeln!(file, "{}", json)?;

    Ok(())
}

pub fn load_impressoras(ui: &AppWindow) -> std::io::Result<()> {
    let file = match OpenOptions::new().read(true).open("impressoras.jsonl") {
        Ok(file) => file,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(e) => return Err(e),
    };

    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let json_impressora: JsonImpressora = serde_json::from_str(&line)?;

        let impressora = Impressoras {
            modelo: json_impressora.modelo.into(),
            watts: json_impressora.watts.into(),
            
        };

        add_impressora(ui, impressora);
        
    }   
    convert_impressora(ui);

    Ok(())
}


