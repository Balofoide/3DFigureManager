use std::time::{Duration, SystemTime};
use std::thread::sleep;

use crate::{AppWindow, Database };

pub fn clock(ui:&AppWindow){
    let relogio = chrono::Local::now().format("%H:%M:%S").to_string();
    
   
    ui.set_time(relogio.into());
     
 

}