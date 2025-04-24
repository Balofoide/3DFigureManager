use serde_json;
use slint::Model;
use std::vec;
use std::io::Write;

use slint::ModelRc;
use slint::VecModel;
use std::fs::OpenOptions;

use crate::{AppWindow, Database};
 




pub fn add_client(ui: &AppWindow, novo_cliente: Database) {
    // Recupera o modelo atual
    let current_model = ui.get_clients_database();

    // Cria um novo VecModel
    let vec_model = VecModel::default();

    // Se o modelo atual não estiver vazio, copia os dados existentes
    if let Some(e_model) = current_model.as_any().downcast_ref::<VecModel<Database>>() {
        for item in e_model.iter() {
            vec_model.push(item.clone());
        }
    }

    // Adiciona o novo cliente ao modelo
    vec_model.push(novo_cliente);

    // Atualiza o modelo na UI
    ui.set_clients_database(ModelRc::new(vec_model));
}

pub fn save_cliente(nome: String, endereco: String, entrega: String) -> std::io::Result<()> {
    let dados: Vec<String> = vec![nome, endereco, entrega];

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("clientes.jsonl")?;

    let json = serde_json::to_string(&dados)?;
    writeln!(file, "{}", json)?;

    Ok(())
}


pub fn register_client(ui:&AppWindow){

    let nome_cliente: String = ui.get_nome_cliente().to_string();
    let endereco: String = ui.get_endereco().to_string();
    let entrega: String = ui.get_entrega().to_string();

    save_cliente(nome_cliente.clone(), endereco.clone(), entrega.clone()).expect("Erro ao salvar os dados do cliente");
    

    // Create a vector of Teste structs
    let client = Database {
        nome: nome_cliente.into(),
        endereco: endereco.into(),
        entrega: entrega.into(),
    };

    add_client(&ui, client);
    

}