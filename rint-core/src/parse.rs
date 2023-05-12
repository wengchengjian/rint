use crate::{
    cmd::{unknown::Unknown, Command},
    protocol::Message,
    Result,
};

pub struct Parser {
    message: Message,
}

impl Parser {
    pub fn new(message: Message) -> Result<Parser> {
        return Ok(Parser { message });
    }

    pub fn parse(&self) -> Result<Command> {
        let cmd = serde_json::from_slice::<Command>(self.message.get_body());
        match cmd {
            Ok(cmd) => Ok(cmd),
            Err(_) => Ok(Command::Unknown(Unknown::new(
                self.message.get_body().clone(),
            ))),
        }
    }
}
