use std::str;
// use colored::Colorize;


mod utilities;
mod commands;
use crate::commands::*;
use crate::utilities::*;

// const START_MSG: &str  = "\nmyshell --> use at your own risk  

//                     |\\---/|
//                     | o_o |
//                      \\_^_/
// ";


fn main() {

    // println!("{}",START_MSG.red().bold());

    /* el metodo var devuelve un Option.
    guardamos en user la env si existe, sino panic!*/
    // let user= match env::var_os("USER"){
    //     Some(u) => u.into_string().unwrap(), //lo pasamos a un String
    //     None => panic!("Doesnt exist USER env variable."),
    // };
    
    
    
    loop {
        
        // imprimimos el prompt y obtenemos el comando a ejecutar
        let input = prompt();

        cmd_handler(&input);
        // match cmd_handler(&input) {
        //     Some(pid) => {}  ,
        //     None => {},
        // }
           
    }
}

