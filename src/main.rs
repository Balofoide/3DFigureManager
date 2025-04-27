// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;


mod utils;

use utils::database_handle::load_clients;
use utils::impressora_handle::load_impressoras;
 
use crate::utils::sell_calculator::calcular_venda;
use crate::utils::impressora_handle::register_impressora;
use crate::utils::database_handle::register_client;

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
    }


    ui.on_calcular_preco({
        let ui_handle = ui.as_weak();

        move || {
            let ui = ui_handle.unwrap();
            calcular_venda(&ui);
        }

    });


    ui.on_registrar_cliente({
        let ui_handle = ui.as_weak();

        move || {
            let ui = ui_handle.unwrap();
            register_client(&ui);
            
        }

    });

    ui.on_registrar_impressora({

        let ui_handle = ui.as_weak();

        move || {
            let ui = ui_handle.unwrap();
           register_impressora(&ui);

        }

    });
     



    ui.run()?;

    Ok(())
}
 