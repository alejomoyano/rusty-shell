use std::io::*;
use std::env;
use std::str;
use colored::Colorize;

mod utilities;
mod commands;
use crate::commands::*;


const START_MSG: &str  = "\nmyshell --> use at your own risk  

                    |\\---/|
                    | o_o |
                     \\_^_/
";

// enum BuildinCommands {
//     Cd,
//     Close,
// }

fn main() {

    /* el metodo var devuelve un Option.
    guardamos en user la env si existe, sino panic!*/
    // let user= match env::var_os("USER"){
    //     Some(u) => u.into_string().unwrap(), //lo pasamos a un String
    //     None => panic!("Doesnt exist USER env variable."),
    // };
    
    
    println!("{}",START_MSG.red().bold());
    
    loop {
    
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

        // buscamos si el comando requiere un pipe
        let pipe: bool = match input.find('|'){
            Some(resolve) => {
                if resolve > 0 {
                    true
                }
                else{
                    false
                }
            },
            None =>{ false },  
        };
        if pipe {
            pipe_exec(&input);
        }
        else if !cmd_handler(&input){
            break;
        }
    }
}

