use std::process::Command;
use dialoguer::Input;
use std::env;
use std::str;

fn main() {
    /* el metodo var devuelve un Option.
    guardamos en user la env si existe, sino panic!*/
    let user= match env::var_os("USER"){
        Some(u) => u.into_string().unwrap(), //lo pasamos a un String
        None => panic!("Doesnt exist USER env variable."),
    };

    let path= match env::var_os("PWD"){
        Some(p) => p.into_string().unwrap(), //lo pasamos a un String
        None => panic!("Doesnt exist PWD env variable."),
    };
    
    println!("\nmyshell --> use at your own risk\n");

    loop {
        let input: String = Input::new()
            .with_prompt(&user)
            .interact_text()
            .unwrap();
            
        /* tomamos el imput, lo parseamos y lo metemos en un vector */
        let mut iter = input.split_whitespace();
        let mut command: Vec<&str> = iter.collect();

        if "quit".eq(&input) {
            break;
        }

        println!("{:?}",command);
        let size = command.len();
        println!("command size: {}",size);

        /* si el comando  no tiene argumentos */
        if command.len() < 2{

            let child = Command::new(command[0])
                .spawn();

        }
        else{ /* si tiene argumentos */
            command.remove(0);

            let child = Command::new(command[0])
                .args(command)
                .spawn();
            }
    }
    // ahora necesitamos parsear el comando y ejecutarlo


}
