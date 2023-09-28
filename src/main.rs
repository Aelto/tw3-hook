use std::{io::Write, net::TcpListener};

use dll_syringe::{process::OwnedProcess, Syringe};
use tracing::info;
use tracing_subscriber::filter::LevelFilter;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .init();

    let target_process = OwnedProcess::find_first_by_name("witcher3").unwrap();
    info!("found target process: {target_process:?}");

    let syringe = Syringe::for_process(target_process);
    info!("syringe created: {syringe:?}");

    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();

    let injected_payload = syringe.inject("./target/debug/twhook.dll");
    info!("payload injected: {injected_payload:?}");

    let (mut stream, address) = listener.accept().unwrap();
    info!("socket connected @ {address}");

    use std::io::Read;
    let mut buf = vec![0u8; 1024];
    let mut stdout = std::io::stdout();
    while let Ok(n) = stream.read(&mut buf[..]) {
        stdout.write_all(&buf[..n]).unwrap();
    }

    Ok(())
}
