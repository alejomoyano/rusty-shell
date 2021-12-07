use std::io::*;
use std::env;
use colored::Colorize;

// use crate::commands::*;

pub fn prompt() -> String {
    
    /* el metodo var devuelve un Option.
    guardamos en user la env si existe, sino panic!*/
    // let user= match env::var_os("USER"){
    //     Some(u) => u.into_string().unwrap(), //lo pasamos a un String
    //     None => panic!("Doesnt exist USER env variable."),
    // };
    
    let path= match env::var_os("PWD"){
        Some(p) => p.into_string().unwrap(), //lo pasamos a un String
        None => panic!("Doesnt exist PWD env variable."),
    };
    
    print!("\n{}> ", path.cyan().bold());
    stdout().flush().expect("Error al hacer flush");
    
    let mut input = String::new();
    
    match stdin().read_line(&mut input){
        Ok(_resolve) => { }, //solo me importa handlear el error
        Err(error) => panic!("Error al leer una linea!\n {}",error),
    }
    
    input
}