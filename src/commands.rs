use std::env;

pub fn change_dir(mut args : Vec<&str>) {
    let mut curr_dir = env::current_dir().expect("Error al obtener el current dir");
    
    let path = args.pop().unwrap(); // obtenemos el path
    env::set_current_dir(path).expect("Error al cambiar de dir");
    
    // seteamos la env PWD y el OLDPWD
    env::set_var("OLDPWD", curr_dir);
    
    curr_dir = env::current_dir().expect("Error al obtener el current dir");
    env::set_var("PWD", curr_dir);
}