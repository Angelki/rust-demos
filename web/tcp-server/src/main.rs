use std::io::{Read, Write};
use std::net::TcpListener;
use std::str;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("Running on port 3000...");

    for steam in listener.incoming() {
        match steam {
            Ok(mut stream) => {
                println!("Connection established!");
                let mut buffer = [0; 1024];
                stream.read(&mut buffer).unwrap();
                stream.write(&buffer).unwrap();
                println!("Msg from Client: {}", str::from_utf8(&buffer).unwrap());
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}
