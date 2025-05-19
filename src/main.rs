// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
 
use std::error::Error;

mod utils;
 
// use utils::clock_handle::clock;
use utils::database_handle::{atualizar_client, excluir_client, load_clients, register_client, vendas_mes};
use utils::estoque_handle::{
    atualizar_estoque, excluir_estoque, load_estoque, register_estoque, total_estoque,
};
use utils::impressora_handle::{
    editar_impressora, excluir_impressora, load_impressoras, load_price, register_impressora, total_filamento
};
use utils::sell_calculator::{atualizar_filamento, calcular_venda, total_vendas};
use utils::settings_handle::{load_settings, load_tema, registrar_settings};

slint::include_modules!();


fn main() {
    if let Err(e) = run_app() {
        eprintln!("Application error: {}", e);
    }
}

fn run_app() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;
  
    initialize_ui(&ui)?;
    register_callbacks(&ui);

    ui.run()?;
    Ok(())
}

/// Inicializa os dados da interface carregando informações do banco de dados e calculando totais.
fn initialize_ui(ui: &AppWindow) -> Result<(), Box<dyn Error>> {
    load_clients(ui).expect("Erro ao carregar clients");
    load_impressoras(ui).expect("Erro ao carregar impressoras");
    load_estoque(ui).expect("Erro ao carregar o estoque");
    load_settings(ui).expect("Erro ao carregar Settings");
    load_price(ui).expect("erro ao carregar preco do filamento");
    vendas_mes(ui).expect("erro ao somar vendas do mês");
    // clock(ui);
    ui.set_vendas_total(total_vendas(ui));
    ui.set_filamento_total(total_filamento(ui));
    ui.set_preco_total_estoque(total_estoque(ui));
    load_tema(ui);

    Ok(())
}

/// Registra todos os callbacks da interface.
fn register_callbacks(ui: &AppWindow) {
    ui.on_calcular_preco({
        let ui_handle = ui.as_weak();
        move || {
            if let Some(ui) = ui_handle.upgrade() {
                calcular_venda(&ui);
                
            }
        }
    });

    ui.on_editar_cliente({
        let ui_handle = ui.as_weak();
        move || {
            if let Some(ui) = ui_handle.upgrade() {
                atualizar_client(&ui);
                ui.set_status("".into());
                ui.set_vendas_total(total_vendas(&ui));
            }
        }
    });

    ui.on_excluir_cliente({
        let ui_handle = ui.as_weak();
        move || {
            if let Some(ui) = ui_handle.upgrade() {
                excluir_client(&ui);
                ui.set_vendas_total(total_vendas(&ui));
            }
        }
    });

    ui.on_atualizar_filamento({
        let ui_handle = ui.as_weak();
        move || {
            if let Some(ui) = ui_handle.upgrade() {
                atualizar_filamento(&ui, ui.get_material().parse().unwrap_or(0));
                ui.set_filamento_total(0);
                ui.set_filamento_total(total_filamento(&ui));
            }
        }
    });

    ui.on_registrar_cliente({
        let ui_handle = ui.as_weak();
        move || {
            if let Some(ui) = ui_handle.upgrade() {
                register_client(&ui);
                ui.set_vendas_total(total_vendas(&ui));
                vendas_mes(&ui).expect("erro ao somar vendas do mês");
            }
        }
    });

    ui.on_registrar_impressora({
        let ui_handle = ui.as_weak();
        move || {
            if let Some(ui) = ui_handle.upgrade() {
                register_impressora(&ui);
            }
        }
    });

    ui.on_registrar_estoque({
        let ui_handle = ui.as_weak();
        move || {
            if let Some(ui) = ui_handle.upgrade() {
                register_estoque(&ui);
                ui.set_vendas_total(total_vendas(&ui));
                ui.set_preco_total_estoque(total_estoque(&ui));
            }
        }
    });

    ui.on_registrar_settings({
        let ui_handle = ui.as_weak();
        move || {
            if let Some(ui) = ui_handle.upgrade() {
                registrar_settings(&ui);
            }
        }
    });

    ui.on_atualizar_estoque({
        let ui_handle = ui.as_weak();
        move || {
            if let Some(ui) = ui_handle.upgrade() {
                atualizar_estoque(&ui);
                ui.set_preco_total_estoque(total_estoque(&ui));
            }
        }
    });

    ui.on_excluir_estoque({
        let ui_handle = ui.as_weak();
        move || {
            if let Some(ui) = ui_handle.upgrade() {
                excluir_estoque(&ui);
            }
        }
    });

    ui.on_excluir_impressora({
        let ui_handle = ui.as_weak();
        move || {
            if let Some(ui) = ui_handle.upgrade() {
                excluir_impressora(&ui);
                ui.set_filamento_total(total_filamento(&ui));
            }
        }
    });

    ui.on_editar_impressora({
        let ui_handle = ui.as_weak();
        move || {
            if let Some(ui) = ui_handle.upgrade() {
                editar_impressora(&ui);
                ui.set_filamento_total(total_filamento(&ui));
                
            }
        }
    });
    ui.on_atualizar_f_preco({
        let ui_handle = ui.as_weak();
        move || {
            if let Some(ui) = ui_handle.upgrade() {
                load_price(&ui).expect("erro ao carregar preco filamento");
            }
        }
    });

    ui.on_carregar_tema({
        let ui_handle = ui.as_weak();
        move || {
            if let Some(ui) = ui_handle.upgrade() {
               load_tema(&ui);
            }
        }
    });
}
