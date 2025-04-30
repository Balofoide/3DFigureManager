// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;


mod utils;

use utils::database_handle::load_clients;
use utils::estoque_handle::load_estoque;
use utils::impressora_handle::{load_impressoras, total_filamento};
use utils::sell_calculator::{atualizar_filamento, total_vendas};
 
use crate::utils::sell_calculator::calcular_venda;
use crate::utils::impressora_handle::register_impressora;
use crate::utils::database_handle::register_client;
use crate::utils::estoque_handle::register_estoque;
slint::include_modules!();

// pub use crate::slint_generatedAppWindow::{AppWindow, Database};

fn main() {
    
    callback().expect("Failed to execute callback");
}

fn callback() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;
    
   
    {
        let ui_handle = ui.as_weak();

        let ui = ui_handle.unwrap();
       load_clients(&ui).expect("Erro ao carregar clients");
       load_impressoras(&ui).expect("Erro ao caregar impressoras");
       load_estoque(&ui).expect("Erro ao carregar o estoque");
       ui.set_vendas_total(total_vendas(&ui));
       ui.set_filamento_total(total_filamento(&ui));
    }


    ui.on_calcular_preco({
        let ui_handle = ui.as_weak();

        move || {
            let ui = ui_handle.unwrap();
            calcular_venda(&ui);
 
        }

    });

    ui.on_atualizar_filamento({
        let ui_handle = ui.as_weak();

        move || {
            let ui = ui_handle.unwrap();
            atualizar_filamento(&ui, ui.get_material().parse().unwrap_or(0));
            ui.set_filamento_total(0);
            ui.set_filamento_total(total_filamento(&ui));
        }

    });
    


    ui.on_registrar_cliente({
        let ui_handle = ui.as_weak();

        move || {
            let ui = ui_handle.unwrap();
            register_client(&ui);
            ui.set_vendas_total(total_vendas(&ui));
        }
        

    });

    ui.on_registrar_impressora({

        let ui_handle = ui.as_weak();

        move || {
            let ui = ui_handle.unwrap();
           register_impressora(&ui);

        }

    });

    ui.on_registrar_estoque({
        let ui_handle = ui.as_weak();

        move || {
            let ui = ui_handle.unwrap();
            register_estoque(&ui);
            ui.set_vendas_total(total_vendas(&ui));
    }
});
     



    ui.run()?;

    Ok(())
}
 