use std::io::{BufRead, BufReader};
use std::net::TcpListener;
use serde_json::Value;
use std::time::Instant;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:5171").expect("Failed to bind to address");
    println!("Client listening on port 5171 for messages from Fluent Bit");

    let start_time = Instant::now();
    let mut total_count = 0;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection established!");
                let mut reader = BufReader::new(stream);
                let mut line = String::new();

                loop {
                    match reader.read_line(&mut line) {
                        Ok(0) => {
                            println!("Connection closed.");
                            break;
                        },
                        Ok(_) => {
                            match serde_json::from_str::<Value>(&line) {
                                Ok(json) => {
                                    total_count += 1;
                                    let elapsed = start_time.elapsed();
                                    println!("Received message {} at {:?}: {}", total_count, elapsed, json);
                                },
                                Err(e) => println!("Error parsing JSON: {}. Raw line: {}", e, line),
                            }
                            line.clear();
                        },
                        Err(e) => {
                            println!("Error reading from stream: {}", e);
                            break;
                        }
                    }
                }
            },
            Err(e) => println!("Error: {}", e),
        }
    }

    println!("Total messages received: {}", total_count);
}