use serde_json;
use slint::Model;

use std::io::Write;

use slint::ModelRc;
use slint::VecModel;
use std::fs::OpenOptions;
use std::io::BufRead;
use std::io::BufReader;

 
use crate::{AppWindow, Database };

#[derive(serde::Serialize, serde::Deserialize)]
struct JsonClient {
    nome: String,
    endereco: String,
    entrega: String,
    preco: f32,
    modelo: String,
    observacao: String,
    status: String,
}


pub fn add_client(ui: &AppWindow, novo_cliente: Database) {
    let current_model = ui.get_clients_database();

    let vec_model = VecModel::default();

    if let Some(e_model) = current_model.as_any().downcast_ref::<VecModel<Database>>() {
        for item in e_model.iter() {
            vec_model.push(item.clone());
        }
    }
    vec_model.push(novo_cliente);

    ui.set_clients_database(ModelRc::new(vec_model));
}

pub fn save_cliente(nome: String, endereco: String, entrega: String,preco:f32,modelo:String,observacao:String,status:String) -> std::io::Result<()> {
  

    let client_data = JsonClient {
        nome,
        endereco,
        entrega,
        preco,
        modelo,
        observacao,
        status,
    };

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("clientes.jsonl")?;

    let json = serde_json::to_string(&client_data)?;
    writeln!(file, "{}", json)?;

    Ok(())
}

pub fn register_client(ui: &AppWindow) {
    let nome_cliente: String = ui.get_nome_cliente().to_string();
    let endereco: String = ui.get_endereco().to_string();
    let entrega: String = ui.get_entrega().to_string();
    let preco: f32 = ui.get_total();
    let modelo: String = ui.get_modelo().to_string();
    let observacao: String = ui.get_observacao().to_string();
    let status:String = ui.get_status().to_string();

    save_cliente(nome_cliente.clone(), endereco.clone(), entrega.clone(),preco.clone(),modelo.clone(),observacao.clone(),status.clone())
        .expect("Erro ao salvar os dados do cliente");

    let client = Database {
        nome: nome_cliente.into(),
        endereco: endereco.into(),
        entrega: entrega.into(),
        preco: preco.into(),
        modelo: modelo.into(),
        observacao: observacao.into(),
        status: status.into(),
    };

    add_client(&ui, client);
    
}

pub fn load_clients(ui: &AppWindow) -> std::io::Result<()> {
    let file = match OpenOptions::new().read(true).open("clientes.jsonl") {
        Ok(file) => file,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(e) => return Err(e),
    };

    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let json_client: JsonClient = serde_json::from_str(&line)?;

        let client = Database {
            nome: json_client.nome.into(),
            endereco: json_client.endereco.into(),
            entrega: json_client.entrega.into(),
            preco: json_client.preco.into(),
            modelo: json_client.modelo.into(),
            observacao: json_client.observacao.into(),
            status: json_client.status.into(),
        };

        add_client(ui, client);
    }

    Ok(())
}

