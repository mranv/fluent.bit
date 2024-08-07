use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    match TcpStream::connect("127.0.0.1:5170") {
        Ok(mut stream) => {
            println!("Connected to server");

            let msg = "Hello, server!";
            stream.write_all(msg.as_bytes()).unwrap();
            println!("Sent: {}", msg);

            let mut buffer = [0; 512];
            match stream.read(&mut buffer) {
                Ok(size) => {
                    let received = String::from_utf8_lossy(&buffer[..size]);
                    println!("Received: {}", received);
                }
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}
