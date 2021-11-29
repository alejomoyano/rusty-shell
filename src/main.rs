use std::io::*;
use std::process::Command;
use std::env;
use std::str;
use colored::Colorize;

use crate::commands::change_dir;

const START_MSG: &str  = "\nmyshell --> use at your own risk  |\\---/|
                                  | o_o |
                                   \\_^_/
";

mod commands;

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
        let result = stdin().read_line(&mut input);
        match result{
            Ok(_r) => { }, //solo me importa handlear el error
            Err(e) => panic!("Error al leer una linea!\n {}",e),
        }
        
        /* tomamos el input, lo parseamos y lo metemos en un vector */
        let mut temp = input.split_whitespace();
        let program = temp.next().unwrap(); 
        let mut args : Vec<&str> = temp.clone().collect();

        match program {
            // si el comando es cd entramos aca
            "cd" => { change_dir(args); },
        // en cualquier otro caso entramos aca
            &_ => {
                let mut child = Command::new(program);
        
                if let Ok(mut c) = child.args(args).spawn() {
                        match c.wait(){
                            Ok(_resolve) =>{},
                            Err(error) => panic!("{}",error),
                        }
                }
                else{
                    println!("Error al ejecutar el comando!");
                }
            },
        }

        // println!("{}{:?}",program,cmd);

      



    }
}
