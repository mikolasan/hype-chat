use std::io;
use std::thread;
use std::time::Duration;
use tokio::io::AsyncWrite;
use tokio::net::TcpStream;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter};
use tokio_util::sync::CancellationToken;

use libirc::messages::{EmptyMessage, IrcMessage, NickMessage, PrivateMessage, UserMessage};

async fn send_message<S: AsyncWrite + std::marker::Unpin>(writer: &mut BufWriter<S>, message: Box<dyn IrcMessage>) -> io::Result<()> {
    let assembled = message.assembly();
    writer.write_all(assembled.as_bytes()).await
}

async fn handle_server(stream: TcpStream) -> io::Result<()> {
    let (read, mut write) = tokio::io::split(stream);
    let mut reader = BufReader::new(read);
    // let mut writer = BufWriter::new(write);

    // Step 1: Create a new CancellationToken
    let token = CancellationToken::new();
    // Step 2: Clone the token for use in another task
    let cloned_token = token.clone();

    let read_handle = tokio::spawn(async move {
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
    });

    // Construct a local task set that can run `!Send` futures.
    let local = tokio::task::LocalSet::new();
    // Run the local task set.
    local.run_until(async move {
        // Send messages to the server
        let mut input = String::new();
        // Get tokio's version of stdin, which implements AsyncRead
        let stdin = tokio::io::stdin();
        // Create a buffered wrapper, which implements BufRead
        let mut stdin_reader = BufReader::new(stdin);
        tokio::select! {
            // Step 3: Using cloned token to listen to cancellation requests
            _ = cloned_token.cancelled() => {
                println!("The token was cancelled, task can shut down");
            }
            _ = async {
                loop {
                    stdin_reader.read_line(&mut input).await.unwrap();
                    let message: Box<dyn IrcMessage> = match input.trim() {
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
                    };
                    input.clear();

                    // `spawn_local` ensures that the future is spawned on the local task set.
                    // tokio::task::spawn_local(async move {
                    //     println!("{}", unsend_data);
                    //     // ...
                    // }).await.unwrap();
                    // send_message(&mut writer, message).await?;
                    let assembled = message.assembly();
                    println!("Sending message '{}'", assembled.trim_end());
                    write.write_all(assembled.as_bytes()).await.unwrap();
                }
            } => {}
        }
    }).await;

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
