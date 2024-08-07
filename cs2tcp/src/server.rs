use std::io::{Write, Error};
use std::net::TcpStream;
use serde_json::json;
use std::thread;
use std::time::Duration;

fn send_message(stream: &mut TcpStream, i: i32) -> Result<(), Error> {
    let message = json!({
        "tag": "test",
        "time": chrono::Utc::now().timestamp(),
        "record": {
            "message": format!("Message from server, number {}", i),
            "value": i * 10
        }
    });

    let json_string = serde_json::to_string(&message)? + "\n";
    stream.write_all(json_string.as_bytes())?;
    stream.flush()?;
    println!("Sent: {}", json_string.trim());
    Ok(())
}

fn main() {
    match TcpStream::connect("127.0.0.1:5170") {
        Ok(mut stream) => {
            println!("Connected to Fluent Bit input");

            for i in 0..5 {
                match send_message(&mut stream, i) {
                    Ok(_) => (),
                    Err(e) => println!("Error sending message {}: {}", i, e),
                }
                thread::sleep(Duration::from_secs(1));  // Increased sleep time
            }
            // Keep the connection open for a while to ensure all data is flushed
            thread::sleep(Duration::from_secs(10));
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}