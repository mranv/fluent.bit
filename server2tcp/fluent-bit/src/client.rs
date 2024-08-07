use std::io::Write;
use std::net::TcpStream;
use serde_json::json;

fn main() {
    match TcpStream::connect("127.0.0.1:5170") {
        Ok(mut stream) => {
            println!("Connected to Fluent Bit");

            // Create a JSON message
            let message = json!({
                "tag": "test",
                "time": chrono::Utc::now().timestamp(),
                "record": {
                    "message": "Hello, Fluent Bit!",
                    "value": 42
                }
            });

            // Convert the JSON to a string
            let json_string = serde_json::to_string(&message).unwrap();

            // Send the JSON string to Fluent Bit
            stream.write_all(json_string.as_bytes()).unwrap();
            stream.write_all(b"\n").unwrap(); // Add a newline to separate messages

            println!("Sent: {}", json_string);
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}
