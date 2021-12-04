use std::env;
use std::process::Command;
use std::process::*;
use super::*;
use std::os::unix::io::AsRawFd;
use std::os::unix::io::FromRawFd;

// mod utilities;
use crate::utilities::*;


pub fn change_dir(mut args : Vec<&str>) {
    let mut curr_dir = env::current_dir().expect("Error al obtener el current dir");
    
    let path = args.pop().unwrap(); // obtenemos el path

    // caso especial en el que ingresamos ~
    if "~".eq(path){
        env::set_current_dir("/home/alejo").expect("Error al cambiar de dir");
    }
    else{
        env::set_current_dir(path).expect("Error al cambiar de dir");
    }
    // seteamos la env PWD y el OLDPWD
    env::set_var("OLDPWD", curr_dir);
    
    curr_dir = env::current_dir().expect("Error al obtener el current dir");
    env::set_var("PWD", curr_dir);
}

pub fn cmd_exec(program: &str ,args: Vec<&str>){
    let mut child = Command::new(program);
            
    if let Ok(mut c) = child.args(args).spawn() {
            match c.wait(){
                Ok(_resolve) =>{},
                Err(error) => panic!("{}",error),
            }
    }
}

pub fn pipe_exec(input: &str){
    let mut commands : Vec<&str> = input.split("|").collect();
    let cmd1 =  parse_cmd(commands.pop().unwrap());
    println!("comando 1: {:?}{}",cmd1.0,cmd1.1);
    let cmd2 =  parse_cmd(commands.pop().unwrap());
    println!("comando 2: {:?}{}",cmd2.0,cmd2.1);

   
    let child1 = Command::new(cmd2.1)
        .args(cmd2.0)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
 
    let _child2 = Command::new(cmd1.1)
        .args(cmd1.0)
        .stdin(child1.stdout.unwrap())
        .output()
        .unwrap();
}

pub fn cmd_handler(input: &str) -> bool{
    /* tomamos el input, lo parseamos y lo metemos en un vector */
    let mut temp = input.split_whitespace();
    let program = temp.next().unwrap(); 
    let args : Vec<&str> = temp.collect();

    //puedo implementar esto con enums
    
    match program {
        // si el comando es cd entramos aca
        "cd" => { change_dir(args); },
        "close" => { return false; },
        // en cualquier otro caso entramos aca
        &_ => { cmd_exec(program, args); },
    }
    true
}