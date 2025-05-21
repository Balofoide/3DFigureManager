use serde_json;
use slint::Model;

use std::io::Write;

use slint::ModelRc;
use slint::VecModel;
use std::fs::OpenOptions;
use std::io::BufRead;
use std::io::BufReader;
use uuid::Uuid;


use crate::{AppWindow, Estoque_Database};

#[derive(serde::Serialize, serde::Deserialize)]
struct JsonEstoque {
   id: String,
   material:String,
   quantidade:String,
   quantidade_total:i32,
   medida: String,
   preco: String,
}



pub fn atualizar_estoque(ui: &AppWindow) {
     
    
    let selected = ui.get_selectec_estoque();

    let temp_material = ui.get_temp_estoque_material();
    let temp_quantidade = ui.get_temp_estoque_quantidade();
    let temp_quantidade_total:i32 = ui.get_temp_estoque_quantidade_total().parse().unwrap_or(0);
    let temp_preco = ui.get_temp_estoque_preco();
    let temp_medida = ui.get_temp_estoque_medida();

    // 1. Obter a lista atual de clientes do UI
    let mut estoques = ui
        .get_estoque_database()
        .iter()
        .enumerate()
        .map(|(idx, _)| ui.get_estoque_database().row_data(idx).unwrap())
        .collect::<Vec<Estoque_Database>>();

    // 2. Encontrar e modificar o cliente selecionado
    if let Some(estoque) = estoques.iter_mut().find(|c| c.id == selected.id) {
        let mut updated = estoque.clone();

        if !temp_material.is_empty() {
            updated.material = temp_material.clone();
        }
         
        if temp_quantidade_total != 0{
            updated.quantidade_total = temp_quantidade_total.clone();

          
        }
        if !temp_preco.is_empty() {
            updated.preco = temp_preco.clone();
        }
        if !temp_medida.is_empty() {
            updated.medida = temp_medida.clone();
        }

        if !temp_quantidade.is_empty(){
            updated.quantidade = temp_quantidade.clone();
              
             
        }

        // if !quantidade.is_empty(){
        //     let quantidade_num:i32 = quantidade.parse().expect("erro ao converter quantidade para i32");
        //     if !quantidade.is_empty() &&   quantidade_num<= estoque.quantidade_total && quantidade_num >=0{
        //         updated.quantidade = quantidade.clone();
        //     }
        // }
        
 
        
        // 3. Atualizar o JSON no disco
        if let Err(e) = atualizar_estoque_json(&updated) {
            eprintln!("Erro ao atualizar arquivo: {}", e);
            return;
        }

        // 4. Atualizar o registro em memória e a UI
        *estoque = updated.clone();
        ui.set_selectec_estoque(updated);
        let new_model = VecModel::from(estoques);
        ui.set_estoque_database(ModelRc::new(new_model));
    }
}

pub fn atualizar_estoque_json(updated: &Estoque_Database) -> std::io::Result<()> {

    
    // Leitura das linhas existentes
    let file = OpenOptions::new().read(true).open("estoque.jsonl")?;
    let reader = BufReader::new(file);
    let mut registros = Vec::new();

    for line in reader.lines() {
        let mut rec: JsonEstoque = serde_json::from_str(&line?)?;
        if rec.id == updated.id.to_string() {
           
            rec.material = updated.material.to_string().clone();
            rec.preco = updated.preco.to_string().clone();
            rec.quantidade_total = updated.quantidade_total.clone();
            rec.medida = updated.medida.to_string().clone();
            rec.quantidade = updated.quantidade.to_string().clone();

            
            


        }
        registros.push(rec);
    }

    // Reescrita completa do arquivo
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("estoque.jsonl")?;
    for rec in registros {
        writeln!(file, "{}", serde_json::to_string(&rec)?)?;
    }
    Ok(())
}


pub fn add_estoque(ui: &AppWindow, novo_cliente: Estoque_Database){
    let current_model = ui.get_estoque_database();

    let vec_model = VecModel::default();

    if let Some(e_model) = current_model.as_any().downcast_ref::<VecModel<Estoque_Database>>() {
        for item in e_model.iter() {
            vec_model.push(item.clone());
        }
    }
    vec_model.push(novo_cliente);

    ui.set_estoque_database(ModelRc::new(vec_model));
}

pub fn save_estoque(id:String,material: String, quantidade:String, quantidade_total:i32, medida: String, preco:String) -> std::io::Result<()> {
  

    let client_data = JsonEstoque {
        id,
        material,
        quantidade,
        quantidade_total,
        medida,
        preco
    };

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("estoque.jsonl")?;

    let json = serde_json::to_string(&client_data)?;
    writeln!(file, "{}", json)?;

    Ok(())
}

pub fn register_estoque(ui: &AppWindow) {
    let material: String = ui.get_estoque_material().to_string();
    let quantidade: String = ui.get_estoque_quantidade().to_string();
    let quantidade_total: i32 = ui.get_estoque_quantidade_total().parse().unwrap_or(0);
    let medida: String = ui.get_estoque_medida().to_string();
    let preco: String = ui.get_estoque_preco_input().to_string();
    let id:String = Uuid::new_v4().to_string();

    save_estoque(id.clone(),material.clone(),quantidade.clone(),quantidade_total.clone(),medida.clone(),preco.clone())
        .expect("Erro ao salvar os dados do cliente");

    let estoque = Estoque_Database {
        id:id.into(),
        material: material.into(),
        quantidade: quantidade.into(),
        quantidade_total: quantidade_total.into(),
        medida:medida.into(),
        preco:preco.into(),
    };

    add_estoque(&ui, estoque);
}

pub fn load_estoque(ui: &AppWindow) -> std::io::Result<()> {
    let file = match OpenOptions::new().read(true).open("estoque.jsonl") {
        Ok(file) => file,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(e) => return Err(e),
    };

    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let json_client: JsonEstoque = serde_json::from_str(&line)?;

        let material = Estoque_Database {
            id:json_client.id.into(),
            material: json_client.material.into(),
            quantidade: json_client.quantidade.into(),
            quantidade_total: json_client.quantidade_total.into(),
            medida:json_client.medida.into(),
            preco:json_client.preco.into(),
 
        };

        add_estoque(ui, material);
    }

    Ok(())
}

pub fn total_estoque(ui:&AppWindow) -> i32{
    
    return ui.get_estoque_database().iter().map(|i| i.preco.parse::<i32>().expect("Erro ao converter para int")).sum();

 }

 
pub fn excluir_estoque(ui: &AppWindow) {
    let selected = ui.get_selectec_estoque();
    let target_id = selected.id.clone();

    // 1. Obter lista atual do UI, filtrando fora o cliente
    let estoques = ui
        .get_estoque_database()
        .iter()
        .enumerate()
        .map(|(idx, _)| ui.get_estoque_database().row_data(idx).unwrap())
        .filter(|c: &Estoque_Database| c.id != target_id)
        .collect::<Vec<Estoque_Database>>();

    // 2. Atualizar JSON no disco
    if let Err(e) = excluir_estoque_json(&target_id) {
        eprintln!("Erro ao excluir do arquivo: {}", e);
        return;
    }

    // 3. Atualizar UI
    ui.set_selectec_estoque(Estoque_Database{id:"".into(),material:"".into(),quantidade:"".into(),medida:"".into(),preco:"0".into(),quantidade_total:0.into()});
    let new_model = VecModel::from(estoques);
    ui.set_estoque_database(ModelRc::new(new_model));
}

fn excluir_estoque_json(id: &str) -> std::io::Result<()> {
    // Ler e filtrar registros
    let file = OpenOptions::new().read(true).open("estoque.jsonl")?;
    let reader = BufReader::new(file);
    let mut registros: Vec<JsonEstoque> = Vec::new();

    for line in reader.lines() {
        let rec: JsonEstoque = serde_json::from_str(&line?)?;
        if rec.id != id {
            registros.push(rec);
        }
    }

    // Reescrever arquivo sem o registro excluído
    let mut file = OpenOptions::new().write(true).truncate(true).open("estoque.jsonl")?;
    for rec in registros {
        writeln!(file, "{}", serde_json::to_string(&rec)?)?;
    }
    Ok(())
}


 