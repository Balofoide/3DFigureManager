use serde_json;
use slint::Model;

use std::io::Write;

use slint::ModelRc;
use slint::VecModel;
use std::fs::OpenOptions;
use std::io::BufRead;
use std::io::BufReader;


use crate::{AppWindow, Estoque_Database};

#[derive(serde::Serialize, serde::Deserialize)]
struct JsonEstoque {
   material:String,
   quantidade:String,
   quantidade_total:String,
   medida: String,
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

pub fn save_estoque(material: String, quantidade:String, quantidade_total:String, medida: String) -> std::io::Result<()> {
  

    let client_data = JsonEstoque {
        material,
        quantidade,
        quantidade_total,
        medida,
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
    let quantidade_total: String = ui.get_estoque_quantidade_total().to_string();
    let medida: String = ui.get_estoque_medida().to_string();

    save_estoque(material.clone(),quantidade.clone(),quantidade_total.clone(),medida.clone())
        .expect("Erro ao salvar os dados do cliente");

    let estoque = Estoque_Database {
        material: material.into(),
        quantidade: quantidade.into(),
        quantidade_total: quantidade_total.into(),
        medida:medida.into(),
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
            material: json_client.material.into(),
            quantidade: json_client.quantidade.into(),
            quantidade_total: json_client.quantidade_total.into(),
            medida:json_client.medida.into(),
 
        };

        add_estoque(ui, material);
    }

    Ok(())
}

