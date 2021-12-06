use std::env;
use std::process::{Command, Stdio};
use super::*;
use std::os::unix::io::FromRawFd;
use nix::unistd::pipe;


// mod utilities;
use crate::utilities::*;

pub struct Comando <'a>{
    program: &'a str,
    args: Vec<&'a str>,
    bg: bool
}

impl Comando <'_> {
    fn check(&mut self){

        let mut index = self.args.len();  

        if  index > 0 {
            index = index-1;

        // ver si al final tiene un '&'
            if self.args.get(index).eq(&Some(&"&")){ //medio confuso pero compacto
                self.args.remove(index);
                self.bg = true;
            }              
        }
    }
}   

        

static mut CHILD_PID: u32 = 0 ;

/* Metodo que implementa el comando cd
   para cambiar de directorio de trabajo */
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

/* Metodo para ejecutar un comando */
pub fn cmd_exec(comando: Comando){

    let mut child = Command::new(comando.program)
        .args(comando.args)
        .spawn()
        .expect("Problema al ejecutar el comando");

    // si es en bg entonces no lo esperamos
    if comando.bg {
        unsafe { CHILD_PID = 0; }; //ver como hacer esto mejor
    }
    else{
        // para guardar el pid del proceso hijo en primer plano
        unsafe { CHILD_PID = child.id(); }; //ver como hacer esto mejor
        // si no es en bg entonces lo esperamos
        match child.wait() {
            Ok(_r) => {},
            Err(_e) => println!("Error al esperar al child"),
        }
    }   

}

pub fn pipe_exec(input: &str){
    let mut commands : Vec<&str> = input.split("|").collect();

    // parseamos los dos comandos
    let cmd1 =  parse_cmd(commands.pop().unwrap());
    let cmd2 =  parse_cmd(commands.pop().unwrap());

    // generamos el pipe
    let fd = pipe().unwrap();

    let (pipe_in,pipe_out) = unsafe {
        (Stdio::from_raw_fd(fd.0),
        Stdio::from_raw_fd(fd.1))
    };

    // ejecutamos los dos child y los pipeamos
    let _child1 = Command::new(cmd2.program)
        .args(cmd2.args)
        .stdout(pipe_out)
        .spawn()
        .unwrap();
 
    let mut child2 = Command::new(cmd1.program)
        .args(cmd1.args)
        .stdin(pipe_in)
        .spawn()
        .unwrap();

    // esperamos que termine
    child2.wait().unwrap();
}

pub fn cmd_handler(input: &str){

    // buscamos si el comando requiere un pipe
    match input.find('|'){
        Some(_resolve) => pipe_exec(&input),
        None => { 
            /* tomamos el input, lo parseamos y lo metemos en un vector */
            let comando = parse_cmd(input);
            
            match comando.program {
                // si el comando es cd entramos aca
                "cd" => { 
                    change_dir(comando.args);
                    // return None;
                },
                // "close" => { return false; },
                // en cualquier otro caso entramos aca
                &_ => { 
                    cmd_exec(comando);
                    // unsafe { return Some(CHILD_PID); }
                },
            }
        },  
    }


}

pub fn parse_cmd(command: &str) -> Comando {
    
    let mut temp = command.split_whitespace();
    let program = temp.next().unwrap(); 
    let args : Vec<&str> = temp.collect();

    let mut comando = Comando {program: program, args: args, bg: false};
    comando.check();
    comando
}