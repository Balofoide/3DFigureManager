use serde_json;
use slint::Model;

use std::io::Write;

use slint::ModelRc;
use slint::VecModel;
use std::fs::OpenOptions;
use std::io::BufRead;
use std::io::BufReader;
use uuid::Uuid;

 
use crate::{AppWindow, Database };

#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct JsonClient {
    id: String,
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

pub fn save_cliente(id:String,nome: String, endereco: String, entrega: String,preco:f32,modelo:String,observacao:String,status:String) -> std::io::Result<()> {
  
    
    let client_data = JsonClient {
        id,
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
    let id:String = Uuid::new_v4().to_string();

    save_cliente(id.clone(),nome_cliente.clone(), endereco.clone(), entrega.clone(),preco.clone(),modelo.clone(),observacao.clone(),status.clone())
        .expect("Erro ao salvar os dados do cliente");

    let client = Database {

        id: id.into(),
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
            id: json_client.id.into(),
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


pub fn atualizar_client(ui: &AppWindow) {
    let temp_nome = ui.get_temp_nome_client();
    let temp_entrega = ui.get_temp_entrega();
    
    let temp_observacao = ui.get_temp_observacao();
    let temp_endereco = ui.get_temp_endereco();
    let selected = ui.get_selected_client();

    // 1. Obter a lista atual de clientes do UI
    let mut clientes = ui
        .get_clients_database()
        .iter()
        .enumerate()
        .map(|(idx, _)| ui.get_clients_database().row_data(idx).unwrap())
        .collect::<Vec<Database>>();

    // 2. Encontrar e modificar o cliente selecionado
    if let Some(cliente) = clientes.iter_mut().find(|c| c.id == selected.id) {
        let mut updated = cliente.clone();
        if !temp_nome.is_empty() {
            updated.nome = temp_nome.clone();
        }
        if !temp_entrega.is_empty() {
            updated.entrega = temp_entrega.clone();
        }
         
        if !temp_observacao.is_empty() {
            updated.observacao = temp_observacao.clone();
        }
        if !temp_endereco.is_empty() {
            updated.endereco = temp_endereco.clone();
        }

        // 3. Atualizar o JSON no disco
        if let Err(e) = atualizar_client_json(&updated) {
            eprintln!("Erro ao atualizar arquivo: {}", e);
            return;
        }

        // 4. Atualizar o registro em memória e a UI
        *cliente = updated.clone();
        ui.set_selected_client(updated);
        let new_model = VecModel::from(clientes);
        ui.set_clients_database(ModelRc::new(new_model));
    }
}

fn atualizar_client_json(updated: &Database) -> std::io::Result<()> {
    // Leitura das linhas existentes
    let file = OpenOptions::new().read(true).open("clientes.jsonl")?;
    let reader = BufReader::new(file);
    let mut registros = Vec::new();

    for line in reader.lines() {
        let mut rec: JsonClient = serde_json::from_str(&line?)?;
        if rec.id == updated.id.to_string() {
            rec.nome = updated.nome.to_string().clone();
            rec.endereco = updated.endereco.to_string().clone();
            rec.entrega = updated.entrega.to_string().clone();
            
            rec.observacao = updated.observacao.to_string().clone();
            // mantém preco e status originais
        }
        registros.push(rec);
    }

    // Reescrita completa do arquivo
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("clientes.jsonl")?;
    for rec in registros {
        writeln!(file, "{}", serde_json::to_string(&rec)?)?;
    }
    Ok(())
}

pub fn excluir_client(ui: &AppWindow) {
    let selected = ui.get_selected_client();
    let target_id = selected.id.clone();

    // 1. Obter lista atual do UI, filtrando fora o cliente
    let clientes = ui
        .get_clients_database()
        .iter()
        .enumerate()
        .map(|(idx, _)| ui.get_clients_database().row_data(idx).unwrap())
        .filter(|c: &Database| c.id != target_id)
        .collect::<Vec<Database>>();

    // 2. Atualizar JSON no disco
    if let Err(e) = excluir_client_json(&target_id) {
        eprintln!("Erro ao excluir do arquivo: {}", e);
        return;
    }

    // 3. Atualizar UI
    ui.set_selected_client(Database{id:"".into(),endereco:"".into(),entrega:"".into(),modelo:"".into(),nome: "".into(),observacao: "".into(), preco:0.0.into(),status:"".into()});
    let new_model = VecModel::from(clientes);
    ui.set_clients_database(ModelRc::new(new_model));
}

fn excluir_client_json(id: &str) -> std::io::Result<()> {
    // Ler e filtrar registros
    let file = OpenOptions::new().read(true).open("clientes.jsonl")?;
    let reader = BufReader::new(file);
    let mut registros: Vec<JsonClient> = Vec::new();

    for line in reader.lines() {
        let rec: JsonClient = serde_json::from_str(&line?)?;
        if rec.id != id {
            registros.push(rec);
        }
    }

    // Reescrever arquivo sem o registro excluído
    let mut file = OpenOptions::new().write(true).truncate(true).open("clientes.jsonl")?;
    for rec in registros {
        writeln!(file, "{}", serde_json::to_string(&rec)?)?;
    }
    Ok(())
}


 