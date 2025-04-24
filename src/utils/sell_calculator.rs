use crate::AppWindow;

 

fn filament_coust(coust: f64) -> f64 {
    let filament_gram = coust / 1000.0; // converte o preço do filamento de kg para grama
    return filament_gram;
}

fn object_coust(object: f64, filament_price: f64) -> f64 {
    let object_price: f64 = object * filament_coust(filament_price);

    return object_price;
}

fn print_coust(time: i32, energy_price: f64, w: i64) -> f64 {
    let kwatts: f64 = w as f64 / 1000.0; // converte watts para kilowatts
    let energy_cousted = time as f64 * kwatts; // energia consumida = tempo * kilowatts
    let print = energy_cousted * energy_price; // custo de impressão = energia consumida * preço da energia
    return print;
}

fn sell_price(time: i32, w: i64, energy_price: f64, object: f64, filament_price: f64) -> f64 {
    let energy = print_coust(time, energy_price, w); // custo de energia
    let model = object_coust(object, filament_price); // custo do modelo
    let total = energy + model; // custo total
    return total;
}

pub fn calcular_venda(ui: &AppWindow) {
   
        let print_weight: f64 = ui.get_material().parse().unwrap_or(0.0);
        let print_time: i32 = ui.get_tempo().parse().unwrap_or(0);

        let filament_price: f64 = ui.get_filamento().parse().unwrap_or(0.0);

        let watts: i64 = ui.get_watts().parse().unwrap_or(0);
        let energy_kwh: f64 = ui.get_energia().parse().unwrap_or(0.0);

        let profit: i64 = ui.get_lucro().parse().unwrap_or(0);

        let total = sell_price(print_time, watts, energy_kwh, print_weight, filament_price)
            * (1 as f64 + profit as f64 / 100.0);

        ui.set_total(total as f32);
       
    }
