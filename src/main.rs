use std::str;
// use colored::Colorize;


mod utilities;
mod commands;
mod signals;
use crate::commands::*;
use crate::utilities::*;
use crate::signals::*;

// const START_MSG: &str  = "\nmyshell --> use at your own risk  

//                     |\\---/|
//                     | o_o |
//                      \\_^_/
// ";


fn main() {

    // println!("{}",START_MSG.red().bold());

    
    loop {
        
        handling_signals();
        // imprimimos el prompt y obtenemos el comando a ejecutar
        let input = prompt();
        
        cmd_handler(&input)
           
    }
}

