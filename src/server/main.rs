use std::io::{BufRead, BufReader, Read, Result};
use std::net::{TcpListener, TcpStream};
use std::thread;
use libirc::commands::{CommandFactory, DefaultCommandFactory};

fn handle_client(mut stream: &TcpStream) -> Result<()> {
    // Set up a buffer to read lines of text from the client
    let mut reader = 
        BufReader::new(stream);

    let factory = DefaultCommandFactory {};

    // Process commands from the client
    loop {
        let mut line = String::new();
        // or read by chunks
        let mut data = [0 as u8; 1024];
        match stream.read(&mut data) {
        //     Ok(size) => {
        //         let message = String::from_utf8_lossy(&data[0..size]).trim_end().to_string();
        //     },
        //     Err(e) => {
        //         println!("Error reading from socket: {:?}", e);
        //         break;
        //     }
        // }
        
        //match reader.read_line(&mut line) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    println!("Client closed the connection");
                    break;
                } else {
                    let line = String::from_utf8_lossy(&data[0..bytes_read]).trim_end().to_string();
                    println!("Received command: {}", line.trim());

                    let mut parts = line.splitn(2, ' ');
                    let command = parts.next().unwrap_or("");
                    let parameters = parts.next().unwrap_or("");
                    if let Some(command_object) = factory.create_command(command) {
                        command_object.parse(&stream, parameters)?;
                    } else {
                        println!("Unknown command: {}", command);
                    }
                }
            },
            Err(e) =>{
                println!("Error reading from socket: {:?}", e);
                break;
            }
        }
        
    }
    Ok(())
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6667").unwrap();
    println!("Listening on port 6667");

    for result in listener.incoming() {
        match result {
            Ok(mut stream) => {
                println!("New client connected");
                thread::spawn(move || {
                    if let Err(error) = handle_client(&mut stream) {
                        println!("Server error: {error}")
                    }
                });
            },
            Err(e) => {
                println!("Listen error: {e}")
            }
        }
    }
}
