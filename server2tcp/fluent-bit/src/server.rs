use std::io::Write;
use std::net::TcpStream;
use serde_json::json;
use std::thread;
use std::time::Duration;

fn main() {
    match TcpStream::connect("127.0.0.1:5170") {
        Ok(mut stream) => {
            println!("Connected to Fluent Bit");

            for i in 0..5 {
                let message = json!({
                    "tag": "test",
                    "time": chrono::Utc::now().timestamp(),
                    "record": {
                        "message": format!("Message number {}", i),
                        "value": i * 10
                    }
                });

                let json_string = serde_json::to_string(&message).unwrap();

                stream.write_all(json_string.as_bytes()).unwrap();
                stream.write_all(b"\n").unwrap();

                println!("Sent: {}", json_string);

                thread::sleep(Duration::from_secs(1));
            }
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}
