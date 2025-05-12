use serde_json;
use slint::Model;

use std::io::Write;

use slint::ModelRc;
use slint::VecModel;
use std::fs::OpenOptions;
use std::io::BufRead;
use std::io::BufReader;
use uuid::Uuid;

 
use slint::SharedString;
use crate::{AppWindow,Impressoras};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct JsonImpressora {
    pub id: String,
    pub modelo: String,
    pub watts: String,
    pub filamento:i32,
    pub filamento_total:i32,
    pub filamento_preco:i32,
    pub filamento_tipo:String,
    pub nozzle:String,
    pub diametro:String,
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
    let filamento_txt: i32 = ui.get_filamento_printer().parse().unwrap_or(0);
    let id:String = Uuid::new_v4().to_string();
    let filamento_total = filamento_txt;
    let filamento_preco:i32 = ui.get_input_filamento_preco().parse().unwrap_or(0);
    let filamento_tipo = ui.get_input_tipo_filamento().to_string();
    let nozzle = ui.get_input_nozzle().to_string();
    let diametro = ui.get_input_area_impressao().to_string();
    // 2. Cria a struct Impressoras
    let impressora = Impressoras {
        id :  id.clone().into(),
        modelo: SharedString::from(modelo_txt.clone()),
        watts:  SharedString::from(watts_txt.clone()),
        filamento: filamento_txt.clone(),
        filamento_total:filamento_total.clone(),
        filamento_preco:filamento_preco.clone(),
        tipo_filamento: SharedString::from(filamento_tipo.clone()),
        nozzle: SharedString::from(nozzle.clone()),
        diametro:SharedString::from(diametro.clone()),
    };

    // 3. Adiciona ao modelo e converte para strings
    add_impressora(ui, impressora);
    convert_impressora(ui);
    save_impressora(modelo_txt.clone(), watts_txt.clone(),filamento_txt.clone(),id.clone(),filamento_total.clone(),filamento_preco.clone(),filamento_tipo.clone(),nozzle.clone(),diametro.clone()).expect("erro ao salvar impressoras");
}

pub fn save_impressora(modelo: String, watts: String,filamento:i32,id:String,filamento_total:i32,filamento_preco:i32,filamento_tipo:String,nozzle:String,diametro:String) -> std::io::Result<()> {
  

    let impressora_data = JsonImpressora {
        id,
        modelo,
        watts,
        filamento,
        filamento_total,
        filamento_preco,
        filamento_tipo,
        nozzle,
        diametro,
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
            id: json_impressora.id.into(),
            modelo: json_impressora.modelo.into(),
            watts: json_impressora.watts.into(),
            filamento:json_impressora.filamento.into(),
            filamento_total:json_impressora.filamento_total.into(),
            filamento_preco:json_impressora.filamento_preco.into(),
            tipo_filamento:json_impressora.filamento_tipo.into(),
            nozzle:json_impressora.nozzle.into(),
            diametro:json_impressora.diametro.into(),
            
        };

        add_impressora(ui, impressora);
        
    }   
    convert_impressora(ui);

    Ok(())
}


pub fn total_filamento(ui:&AppWindow) -> i32{
        
    return ui.get_impressoras().iter().map(|i| i.filamento).sum();

 }

 
pub fn excluir_impressora(ui: &AppWindow) {
    let selected = ui.get_selected_impressora();
    let target_id = selected.id.clone();

    // 1. Obter lista atual do UI, filtrando fora o cliente
    let impressoras = ui
        .get_impressoras()
        .iter()
        .enumerate()
        .map(|(idx, _)| ui.get_impressoras().row_data(idx).unwrap())
        .filter(|c: &Impressoras| c.id != target_id)
        .collect::<Vec<Impressoras>>();

    // 2. Atualizar JSON no disco
    if let Err(e) = excluir_impressora_json(&target_id) {
        eprintln!("Erro ao excluir do arquivo: {}", e);
        return;
    }

    // 3. Atualizar UI
    ui.set_selected_impressora(Impressoras{id:"".into(),modelo:"".into(),filamento:0.into(),watts:"".into(),filamento_total:0.into(),filamento_preco:0.into(),tipo_filamento:"".into(),nozzle:"".into(),diametro:"".into()});
    let new_model = VecModel::from(impressoras);
    ui.set_impressoras(ModelRc::new(new_model));
}

fn excluir_impressora_json(id: &str) -> std::io::Result<()> {
    // Ler e filtrar registros
    let file = OpenOptions::new().read(true).open("impressoras.jsonl")?;
    let reader = BufReader::new(file);
    let mut registros: Vec<JsonImpressora> = Vec::new();

    for line in reader.lines() {
        let rec: JsonImpressora = serde_json::from_str(&line?)?;
        if rec.id != id {
            registros.push(rec);
        }
    }

    // Reescrever arquivo sem o registro excluído
    let mut file = OpenOptions::new().write(true).truncate(true).open("impressoras.jsonl")?;
    for rec in registros {
        writeln!(file, "{}", serde_json::to_string(&rec)?)?;
    }
    Ok(())
}





pub fn editar_impressora(ui: &AppWindow) {
  
    
    let selected = ui.get_selected_impressora();
    let temp_modelo = ui.get_temp_modelo();

    let temp_watts = ui.get_temp_watts();
    let temp_filamento = ui.get_temp_filamento();
    let temp_filamento_total:i32 = ui.get_temp_filamento_total().parse().unwrap_or(0);
    let temp_filamento_preco:i32 = ui.get_temp_filamento_preco().parse().unwrap_or(0);
    let temp_filamento_tipo = ui.get_temp_tipo_filamento();
    let temp_nozzle = ui.get_temp_nozzle();
    let temp_diametro = ui.get_temp_diametro();
    // 1. Obter a lista atual de clientes do UI
    let mut impressoras = ui
        .get_impressoras()
        .iter()
        .enumerate()
        .map(|(idx, _)| ui.get_impressoras().row_data(idx).unwrap())
        .collect::<Vec<Impressoras>>();

    // 2. Encontrar e modificar o cliente selecionado
    if let Some(impressora) = impressoras.iter_mut().find(|c| c.id == selected.id) {
        let mut updated = impressora.clone();

        if temp_filamento_preco != 0 {
            updated.filamento_preco = temp_filamento_preco.clone();
        }

        if !temp_modelo.is_empty() {
            updated.modelo = temp_modelo.clone();
        }
         
        if !temp_nozzle.is_empty(){
            updated.nozzle = temp_nozzle.clone();
        }

        if !temp_filamento_tipo.is_empty(){
            updated.tipo_filamento = temp_filamento.clone();
        }

        if !temp_diametro.is_empty(){
            updated.diametro = temp_diametro.clone();
        }

        if !temp_watts.is_empty() {
            updated.watts = temp_watts.clone();
        }
        if !temp_filamento.is_empty() {
            updated.filamento = temp_filamento.parse::<i32>().unwrap_or(0).clone();
        }

        if temp_filamento_total != 0{
            updated.filamento_total = temp_filamento_total.clone();
        }
        
        
        // 3. Atualizar o JSON no disco
        if let Err(e) = atualizar_impressora_json(&updated) {
            eprintln!("Erro ao atualizar arquivo: {}", e);
            return;
        }

        // 4. Atualizar o registro em memória e a UI
        *impressora = updated.clone();
        ui.set_selected_impressora(updated);
        let new_model = VecModel::from(impressoras);
        ui.set_impressoras(ModelRc::new(new_model));
    }
}

pub fn atualizar_impressora_json(updated: &Impressoras) -> std::io::Result<()> {

    
    // Leitura das linhas existentes
    let file = OpenOptions::new().read(true).open("impressoras.jsonl")?;
    let reader = BufReader::new(file);
    let mut registros = Vec::new();

    for line in reader.lines() {
        let mut rec: JsonImpressora = serde_json::from_str(&line?)?;
        if rec.id == updated.id.to_string() {
           
            rec.modelo = updated.modelo.to_string().clone();
            rec.watts = updated.watts.to_string().clone();
            rec.filamento = updated.filamento.clone();
            rec.filamento_total = updated.filamento_total.clone();
            rec.filamento_preco = updated.filamento_preco.clone();
            rec.filamento_tipo = updated.tipo_filamento.to_string().clone();
            rec.diametro = updated.diametro.to_string().clone();
            rec.nozzle = updated.nozzle.to_string().clone();
            


        }
        registros.push(rec);
    }

    // Reescrita completa do arquivo
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("impressoras.jsonl")?;
    for rec in registros {
        writeln!(file, "{}", serde_json::to_string(&rec)?)?;
    }
    Ok(())
}
pub fn load_price(ui: &AppWindow) -> std::io::Result<()> {
    let file = match OpenOptions::new().read(true).open("impressoras.jsonl") {
        Ok(file) => file,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(e) => return Err(e),
    };
    let target = ui.get_combobox_selected();

    let reader = BufReader::new(file);

    for line in reader.lines() {

        
        let line = line?;
        let json_client: JsonImpressora = serde_json::from_str(&line)?;

        let preco_json = json_client.filamento_preco.to_string();
        if json_client.modelo == target.to_string(){
            ui.set_filamento(preco_json.into());
        }
      

 
    }

    Ok(())
}
