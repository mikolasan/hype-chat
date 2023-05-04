
// House of message assembly

pub trait IrcMessage {
  fn assembly(&self) -> String;
}

pub struct EmptyMessage {}

impl IrcMessage for EmptyMessage {
  fn assembly(&self) -> String {
    "".to_string()
  }  
}

pub struct NickMessage {
  pub nick: String,
}

impl IrcMessage for NickMessage {
  fn assembly(&self) -> String {
    format!("NICK {}\r\n", self.nick)
  }
}

pub struct UserMessage {
  pub nick: String,
  pub real_name: String,
}

impl IrcMessage for UserMessage {
  fn assembly(&self) -> String {
    format!("USER {} 0 * :{}\r\n", self.nick, self.real_name)
  }
}

pub struct PrivateMessage {
  pub target: String,
  pub message: String,
}

impl IrcMessage for PrivateMessage {
  fn assembly(&self) -> String {
    format!("PRIVMSG {} {}\r\n", self.target, self.message)
  }
}