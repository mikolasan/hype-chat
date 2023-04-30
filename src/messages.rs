
// House of message assembly

pub trait IrcMessage {
  fn assembly(&self) -> String;
}

pub struct NickMessage {
  pub nick: String,
}

impl IrcMessage for NickMessage {
  fn assembly(&self) -> String {
    format!("NICK {}\r\n", self.nick)
  }
}