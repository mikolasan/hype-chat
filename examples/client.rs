use std::io::{self, prelude::*, BufReader};
use std::net::TcpStream;

fn handle_server(mut stream: TcpStream) -> io::Result<()> {
    let mut reader = BufReader::new(stream.try_clone()?);

    // Send a NICK command to the server
    let nick = "my_nickname";
    let nick_command = format!("NICK {}\r\n", nick);
    stream.write_all(nick_command.as_bytes())?;

    // Read and print the server's response to the NICK command
    let mut response = String::new();
    reader.read_line(&mut response)?;
    println!("{}", response.trim());

    // Send a USER command to the server
    let user = "my_username";
    let real_name = "My Real Name";
    let user_command = format!("USER {} 0 * :{}\r\n", user, real_name);
    stream.write_all(user_command.as_bytes())?;

    // Read and print the server's response to the USER command
    response.clear();
    reader.read_line(&mut response)?;
    println!("{}", response.trim());

    // Send a message to a channel
    let channel = "#general";
    let message = "Hello, world!";
    let privmsg_command = format!("PRIVMSG {} :{}\r\n", channel, message);
    stream.write_all(privmsg_command.as_bytes())?;

    let mut input = String::new();
    loop {
        input.clear();
        io::stdin().read_line(&mut input)?;
        let trimmed_input = input.trim();
        if trimmed_input == "quit" {
            break
        }

        // Send the input to the server
        writeln!(stream, "{}", trimmed_input)?;

        // Wait for a response from the server
        response.clear();
        reader.read_line(&mut response)?;
        println!("{}", response.trim());
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let connect_result = TcpStream::connect("127.0.0.1:6667");
    if let Ok(stream) = connect_result {
        println!("Connected to the server!");
        if let Err(error) = handle_server(stream) {
            println!("Client error: {error}");
        }
    } else {
        println!("Couldn't connect to server...");
    }

    Ok(())
}
