use std::{net::TcpStream, sync::Mutex};

use tracing::info;

pub mod game;

#[ctor::ctor]
fn ctor() {
    println!("hi from library");

    let stream = TcpStream::connect("127.0.0.1:3000").unwrap();

    tracing_subscriber::fmt()
        .with_writer(Mutex::new(stream))
        .init();

    info!("library loaded");

    use retour::GenericDetour;
    // let hook = unsafe {
    //     let pointer = 0x1410058B7 as *const ();
    //     let original: fn() = std::mem::transmute(pointer);
    //     GenericDetour::new(original, register_script_functions).expect("msg")
    // };

    unsafe {
        // hook.enable().unwrap();
        let pointer = 0x14102D6F0 as *const ();
        let somefn: fn() = std::mem::transmute(pointer);

        somefn();
        game::cfunction::perform_script_function_registration("DBget", &mut db_get);
    }
}

unsafe fn register_script_functions() {
    let pointer = 0x14102D6F0 as *const ();
    let somefn: fn() = std::mem::transmute(pointer);

    somefn();
    game::cfunction::perform_script_function_registration("DBget", &mut db_get);
}

fn db_get() {
    info!("hello from witcherscript");
}
