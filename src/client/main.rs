use std::io;
use std::thread;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::io::{AsyncBufReadExt, AsyncBufRead, AsyncWrite, AsyncWriteExt, BufReader};
use tokio_util::sync::CancellationToken;

use libirc::messages::{EmptyMessage, IrcMessage, NickMessage, PrivateMessage, UserMessage};

async fn process_server_responses<Reader: AsyncBufRead + std::marker::Unpin>(mut reader: Reader, token: &CancellationToken) {
    loop {
        let mut line = String::new();
        match reader.read_line(&mut line).await {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    println!("Server closed the connection");
                    break;
                } else {
                    println!("Received: {}", line);
                }
            },
            Err(e) => {
                println!("Error reading from socket: {:?}", e);
                token.cancel();
                break;
            }
        }
    }
}

async fn process_user_input<Writer: AsyncWrite + std::marker::Unpin>(mut writer: Writer, token: &CancellationToken) {
    let mut input = String::new();
    // Get tokio's version of stdin, which implements AsyncRead
    let stdin = tokio::io::stdin();
    let mut stdin_reader = BufReader::new(stdin);
    tokio::select! {
        _ = token.cancelled() => {
            println!("Stop processing user's input");
        }
        _ = async {
            loop {
                stdin_reader.read_line(&mut input).await.unwrap();
                let assembled;
                {
                    let message = parse_user_input(&input);
                    assembled = message.assembly();
                }
                println!("Sending message '{}'", assembled.trim_end());
                writer.write_all(assembled.as_bytes()).await.unwrap();
                input.clear();
            }
        } => {}
    }
}

fn parse_user_input(input: &String) -> Box<dyn IrcMessage> {
    match input.trim() {
        "NICK" => Box::new(NickMessage {
            nick: "mikolasan".to_string(),
        }),
        "USER" => Box::new(UserMessage {
            nick: "mikolasan".to_string(),
            real_name: "Nikolay Neupokoev".to_string(),
        }),
        "PRIVMSG" => Box::new(PrivateMessage {
            target: "admin".to_string(),
            message: "well hello".to_string(),
        }),
        &_ => {
            println!("Unknown command :(");
            Box::new(EmptyMessage {})
        }
    }
}

async fn handle_server(stream: TcpStream) -> io::Result<()> {
    let (read, write) = tokio::io::split(stream);
    
    let reader = BufReader::new(read);
    let token = CancellationToken::new();
    let cloned_token = token.clone();
    
    let read_handle = tokio::spawn(async move {
        process_server_responses(reader, &token).await;
    });
    let write_handle = tokio::spawn(async move {
        process_user_input(write, &cloned_token).await;
    });

    read_handle.await?;
    write_handle.await?;

    Ok(())  
}

#[tokio::main]
async fn main() -> io::Result<()> {
    loop {
        if let Ok(stream) = TcpStream::connect("127.0.0.1:6667").await {
            println!("Connected to the server!");

            if let Err(error) = handle_server(stream).await {
                println!("Client error: {error}");
            }
        } else {
            println!("Couldn't connect to server...");
        }
        for i in (1..=5).rev() {
            println!("Wait to reconnect ({})...", i);
            thread::sleep(Duration::from_secs(1));
        }
    }
}
