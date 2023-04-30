use std::io::{self, prelude::*, BufReader};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;
use libirc::messages::{IrcMessage, NickMessage};

fn send_message(mut stream: &TcpStream, message: Box<dyn IrcMessage>) -> io::Result<()> {
    let assembled = message.assembly();
    stream.write_all(assembled.as_bytes())
}

fn handle_server(mut stream: TcpStream) -> io::Result<()> {
    let mut reader = BufReader::new(stream.try_clone()?);

    let nick_message = Box::new(NickMessage {
        nick: "mikolasan".to_string(),
    });
    send_message(&stream, nick_message)?;

    // // // Read and print the server's response to the NICK command
    // // let mut response = String::new();
    // // reader.read_line(&mut response)?;
    // // println!("{}", response.trim());

    // // Send a USER command to the server
    // let user = "mikolasan";
    // let real_name = "Nikolay Neupokoev 🦣";
    // let user_command = format!("USER {} 0 * :{}\r\n", user, real_name);
    // stream.write_all(user_command.as_bytes())?;

    // // Read and print the server's response to the USER command
    // response.clear();
    // reader.read_line(&mut response)?;
    // println!("{}", response.trim());

    // // Send a message to a channel
    // let channel = "#general";
    // let message = "Hello, world!";
    // let privmsg_command = format!("PRIVMSG {} :{}\r\n", channel, message);
    // stream.write_all(privmsg_command.as_bytes())?;

    let mut response = String::new();
    let mut input = String::new();
    loop {
        // input.clear();
        // io::stdin().read_line(&mut input)?;
        // let trimmed_input = input.trim();
        // if trimmed_input == "quit" {
        //     break
        // }

        // // Send the input to the server
        // writeln!(stream, "{}", trimmed_input)?;

        // Wait for a response from the server
        response.clear();
        reader.read_line(&mut response)?;
        println!("{}", response.trim());
    }

    Ok(())
}

fn main() -> io::Result<()> {
    loop {
        let connect_result = TcpStream::connect("127.0.0.1:6667");
        if let Ok(stream) = connect_result {
            println!("Connected to the server!");
            if let Err(error) = handle_server(stream) {
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
