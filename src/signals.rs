use nix::sys::*;

extern "C" fn chld_handler(_: libc::c_int, _: *mut libc::siginfo_t, _: *mut libc::c_void){
    println!("\nHijo muerto");
}


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