use std::io::{BufRead, BufReader, Result, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) -> Result<()> {
    // Set up a buffer to read lines of text from the client
    let mut reader = BufReader::new(stream.try_clone().unwrap());

    // Send a welcome message to the client
    write!(stream, "Welcome to my IRC server!\r\n")?;

    // Process commands from the client
    loop {
        let mut line = String::new();
        reader.read_line(&mut line)?;
        println!("Received command: {}", line.trim());

        // Parse the command
        let mut parts = line.trim().splitn(2, ' ');
        let command = parts.next().unwrap_or("");
        let args = parts.next().unwrap_or("");

        // Handle the command
        match command {
            "NICK" => {
                let nickname = args.trim();
                write!(stream, "Your nickname is now {}\r\n", nickname)?;
            }
            _ => {
                write!(stream, "Unknown command: {}\r\n", command)?;
            }
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6667").unwrap();
    println!("Listening on port 6667");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread::spawn(move || {
            if let Err(error) = handle_client(stream) {
                println!("Server error: {error}")
            }
        });
    }
}
