use nix::sys::wait::*;
use nix::sys::signal::{self, Signal};


pub fn handling_signals(){
    
    // MUY parecido a C.
    let sig_action = signal::SigAction::new(signal::SigHandler::SigAction(chld_handler),
    signal::SaFlags::empty(),
    signal::SigSet::empty());
    
    unsafe { 
        match signal::sigaction(signal::SIGCHLD, &sig_action){
            Ok(_) => {} ,
            Err(error) => println!("{}",error.desc())
        }
    }
}

extern "C" fn chld_handler(_: libc::c_int, _: *mut libc::siginfo_t, _: *mut libc::c_void){
    //obtenemos el pid del proceso que termino
    let child_pid = waitpid(None,Some(WaitPidFlag::WNOHANG));
    
    if let Ok(result) = child_pid{
        let pid = result.pid().expect("Error al obtener el pid del hijo mierto");
        print!("\n[{}] done",pid);
    }
}

// extern "C" fn int_handler(_: libc::c_int, _: *mut libc::siginfo_t, _: *mut libc::c_void){
//     let pid = get_pid();
//     signal::kill(Pid::from_raw(pid), Signal::SIGTERM).unwrap();
//     print!("\n[{}] term",pid);
//     set_pid(0);

// }