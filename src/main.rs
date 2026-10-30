use std::fs::File;
use log::error;
use std::io::Read;
use std::net::TcpListener;

fn handle_zorro(file_name: &str) {
    match File::create(file_name) {
        Ok(_) => println!("Successfully created file {}", file_name),
        Err(e) => error!("Failed creating file {}: {}", file_name, e),
    }
}

fn handle_request(bytes: &[u8]) {
    let Ok(req) = std::str::from_utf8(bytes) else {
        error!("The received message is invalid");
        return;
    };

    let mut split = req.split_ascii_whitespace();

    if split.next() == Some("ZORRO") {
        if let Some(file) = split.next() {
            println!("Creating a file as requested");
            handle_zorro(file);
        }
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4567")?;

    println!("Listening on port 4567...");
    println!("Working directory: {:?}", std::env::current_dir()?);

    let (mut stream, _) = listener.accept()?;

    let mut buffer = [0u8; 1024];

    loop {
        let size = stream.read(&mut buffer)?;

        if size == 0 {
            break;
        }

        handle_request(&buffer[..size]);

        println!("Received: {:?}", &buffer[..size]);
        println!("{}\n", String::from_utf8_lossy(&buffer[..size]));
    }

    Ok(())
}
