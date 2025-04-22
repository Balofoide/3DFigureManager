// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{error::Error, f64};



slint::include_modules!();

fn main() {
   
    calculator();
    
}



fn calculator() -> Result<(), Box<dyn Error>>{
    let ui = AppWindow::new()?;
    

    ui.on_request_increase_value({
    
    let ui_handle = ui.as_weak();
   
    move || {
       
        let ui = ui_handle.unwrap();
        let print_weight: f64 = ui.get_material().parse().unwrap_or(0.0);
        let print_time: i32 = ui.get_tempo().parse().unwrap_or(0);

        let filament_price: f64 =  ui.get_filamento().parse().unwrap_or(0.0);

        let watts : i64 = ui.get_watts().parse().unwrap_or(0);
        let energy_kwh: f64 = ui.get_energia().parse().unwrap_or(0.0);

        let profit : i64 = ui.get_lucro().parse().unwrap_or(0);
        
        



        let total = total_coust(print_time, watts, energy_kwh, print_weight, filament_price) * (1 as f64 + profit as f64 / 100.0);

        ui.set_total(total as f32);
        
        
        // ui.set_total(total_coust(20,350,1.83,150.0,107.0) as f32);
        
    }
    });


    ui.run()?;

    Ok(())
    
   
}



fn filament_coust(coust:f64) -> f64 {
    let filament_gram = coust / 1000.0;  // converte o preço do filamento de kg para grama
    return filament_gram;
}

fn object_coust(object:f64, filament_price:f64 ) -> f64 {
    
    let object_price : f64 = object * filament_coust(filament_price);

    return object_price;
}

fn print_coust(time:i32, energy_price: f64, w : i64) -> f64 {
    let kwatts: f64 = w as f64 / 1000.0;  // converte watts para kilowatts
    let energy_cousted = time as f64 * kwatts;  // energia consumida = tempo * kilowatts
    let print = energy_cousted * energy_price;  // custo de impressão = energia consumida * preço da energia
    return print;
}

fn total_coust(time:i32, w:i64, energy_price: f64, object:f64, filament_price:f64) -> f64{
    let energy = print_coust(time, energy_price, w);  // custo de energia
    let model = object_coust(object, filament_price);  // custo do modelo
    let total = energy + model;  // custo total
    return total;
}

 


 