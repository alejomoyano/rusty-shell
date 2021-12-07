use std::env;
use std::process::{Command, Stdio};
use super::*;
use std::os::unix::io::FromRawFd;
use nix::unistd::pipe;
use std::io::ErrorKind;


// mod utilities;
// use crate::utilities::*;


pub struct Comando <'a>{
    program: &'a str,
    args: Vec<&'a str>,
    bg: bool,
    pid: u32
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

    fn default() -> Self {
        let empty = vec![];
        Comando {
            program: "none",
            args: empty,
            bg: false,
            pid: 0
        }
    }
}   

        

static mut CHILD_PID: u32 = 0 ;

/* Metodo que implementa el comando cd
   para cambiar de directorio de trabajo */
pub fn change_dir(mut args : Vec<&str>) {
    let mut curr_dir = env::current_dir()
        .expect("Error al obtener el current dir");
    
    let path = args.pop().unwrap(); // obtenemos el path

    let result; // despues handleamos esta variable
    // no se si es la mejor manera de hanlear dos errores que son iguales
    match path{
        // caso especial en el que ingresamos ~
        "~" => {
            result =  env::set_current_dir("/home/alejo"); 
        },
        // demas casos de paths
        _ => { 
            result = env::set_current_dir(path);
        } 
    }

    // no termino de entender como funciona ese if let
    // puede servir para cuando queremos handlear solo el error
    if let Err(error) = result {
        match error.kind() {
            ErrorKind::NotFound => println!("Directorio no encontrado!"),
            other_error => {
                panic!("{:?}",other_error);
            }
        }
    }
    
    // seteamos la env PWD y el OLDPWD
    env::set_var("OLDPWD", curr_dir);
    curr_dir = env::current_dir().expect("Error al obtener el current dir");
    env::set_var("PWD", curr_dir);
}

/* Metodo para ejecutar un comando */
pub fn cmd_exec(mut comando: Comando){

    if !comando.program.eq("none"){
        let mut child = Command::new(comando.program)
            .args(comando.args)
            .spawn()
            .expect("Problema al ejecutar el comando");

        //guardamos el pid en la struct
        comando.pid = child.id();

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
                Err(_error) => println!("Error al esperar al child"),
            }
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

    // si no ingresan comando y dan enter da un None,
    // por ende hay que handlear, sino paniquea.
    let program = match temp.next() {
        Some(result) => result,
        // si no matchea nada entonces devolvemos un commando en default (vacio)
        None => {
            let comando = Comando::default();
            return comando;
        }
    };
    
    let args : Vec<&str> = temp.collect();

    let mut comando = Comando {
        program: program, 
        args: args, 
        bg: false, 
        pid: 0
    };

    // para checkear si el comando debe ejecutarse en bg
    comando.check();
    comando
}