use std::{net::TcpStream, ops::Sub, sync::Mutex};

use game::{
    pointers::{self, VoidPtr},
    GlobalScriptingContext,
};
use tracing::info;
use windows::{
    core::{HSTRING, PCWSTR},
    Win32::System::LibraryLoader::{GetModuleHandleA, GetModuleHandleW},
};

pub mod game;

#[ctor::ctor]
fn ctor() {
    println!("hi from library");

    let stream = TcpStream::connect("127.0.0.1:3000").unwrap();

    tracing_subscriber::fmt()
        .with_writer(Mutex::new(stream))
        .init();

    info!("library loaded");

    unsafe {
        detour_str_lower_utf();
        // detour_str_lower_utf_ilhook();
    }

    // use retour::GenericDetour;
    // let hook = unsafe {
    //     let imagebase = 0x140000000 as isize;
    //     let address = 0x1410058B7 as isize;
    //     let offset = address - imagebase;
    //     info!(offset=%offset);

    //     let pointer = match GetModuleHandleW(PCWSTR::null()) {
    //         Ok(p) => p,
    //         Err(e) => {
    //             tracing::error!(getmodulehandlew=%e);
    //             panic!();
    //         }
    //     }
    //     .0;
    //     info!(hmodule=%pointer);

    //     let pointer = pointer + offset;
    //     let original: unsafe fn() = std::mem::transmute(pointer as *const ());

    //     retour::RawDetour::new(
    //         pointer as *const (),
    //         register_script_functions_stub as *const (),
    //     )

    //     // GenericDetour::new(original, register_script_functions_stub)
    // };

    // unsafe {
    //     match hook {
    //         Ok(h) => {
    //             if let Err(e) = h.enable() {
    //                 tracing::error!(hookenableerror = %e);
    //             }
    //         }
    //         Err(e) => {
    //             tracing::error!(hookdetourerror = %e);
    //         }
    //     };
    // }
}

fn offset_address(addr: isize) -> isize {
    let ofs = addr - pointers::IMAGE_BASE;

    offset(ofs)
}

fn offset(offset: isize) -> isize {
    let pointer = unsafe {
        let s = HSTRING::from("witcher3.exe");
        match GetModuleHandleW(&s) {
            Ok(p) => p,
            Err(e) => {
                tracing::error!(getmodulehandlew=%e);
                panic!();
            }
        }
        .0
    };

    pointer + offset
}

unsafe fn detour_str_lower_utf() {
    use tracing::error;

    unsafe fn detour(a1: i64, a2: *mut i64, a3: i64) -> i64 {
        let original: unsafe fn(i64, *mut i64, i64) -> i64 =
            std::mem::transmute(pointers::SUB_STR_LOWER_UTF);

        info!("strLowerUTF()");

        original(a1, a2, a3)
    }

    let target = pointers::SUB_STR_LOWER_UTF_OFFSET;
    let detour_res = retour::RawDetour::new(offset(target) as VoidPtr, detour as VoidPtr);
    match detour_res {
        Ok(hook) => {
            if let Err(e) = hook.enable() {
                error!(detour_str_lower_utf_enable_error=%e);
            }
        }
        Err(e) => {
            error!(detour_str_lower_utf_error=%e);
        }
    };
}

unsafe fn disable_pause_on_offfocus() {
    use retour::GenericDetour;
    use retour::RawDetour;
}

unsafe fn vanilla_compilation_step() {
    let pointer = pointers::STRING_WS_REGISTER_FN as *const ();
    let somefn: fn() = std::mem::transmute(pointer);

    somefn();
}

unsafe fn register_script_functions_stub() {
    info!("register_script_functions_stub()");
    vanilla_compilation_step();
    game::cfunction::perform_script_function_registration("DBget", &mut sf);
}

fn sf(a1: VoidPtr, ctx: *mut GlobalScriptingContext, ret: *mut std::ffi::c_void) {
    info!("sh()");

    unsafe {
        let ctx = &mut *ctx;
        let stack = &mut *ctx.some_stack;

        *stack += 1;
    }

    db_get();
}

fn db_get() {
    info!("hello from witcherscript");
}
