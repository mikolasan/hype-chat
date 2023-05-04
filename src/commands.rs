use std::io::{Result, Write};
use std::net::TcpStream;

use crate::numerics::Numerics;

// House of message parsing

pub trait IrcCommand {
  fn parse(&self, stream: & TcpStream, parameters: &str) -> Result<()>;
  fn reply(&self, mut stream: & TcpStream, numeric: Numerics, parameters: &str) -> Result<()> {
    write!(stream, "{:0>3} <client> {parameters}\r\n", numeric as i32)
  }
}

pub struct NickCommand {
}

impl IrcCommand for NickCommand {
  fn parse(&self, stream: &TcpStream, parameters: &str) -> Result<()> {
    let nickname = parameters.trim();
    if nickname.is_empty() {
      self.reply(stream, Numerics::ERR_NONICKNAMEGIVEN, 
        ":No nickname given")?;
      return Ok(());
    }
    let network_name = "Hype";
    let user = "Nikolay";
    let host = "Home";
    let mut reply_parameters = format!(":Welcome to the {network_name} Network, {nickname}");
    if !user.is_empty() && !host.is_empty() {
      reply_parameters += format!("!{user}@{host}").as_str();
    }
    self.reply(stream, Numerics::RPL_WELCOME, reply_parameters.as_str())?;
    Ok(())
  }
}

pub struct JoinCommand {
}

impl IrcCommand for JoinCommand {
  fn parse(&self, _stream: &TcpStream, _parameters: &str) -> Result<()> {
      Ok(())
  }
}

pub trait CommandFactory {
  fn create_command(&self, command: &str) -> Option<Box<dyn IrcCommand>>;
}

pub struct DefaultCommandFactory;

impl CommandFactory for DefaultCommandFactory {
  fn create_command(&self, command: &str) -> Option<Box<dyn IrcCommand>> {
      match command.to_uppercase().as_str() {
          "NICK" => Some(Box::new(NickCommand {})),
          "JOIN" => Some(Box::new(JoinCommand {})),
          // Add other commands here...
          _ => None,
      }
  }
}