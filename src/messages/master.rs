use actix::Message;

use super::parse::{ParseFailedMessage, ParseSuccessMessage};

#[derive(Message)]
#[rtype(result = "()")]
pub enum MasterMessage {
    Parse(Parse),
}

pub enum Parse {
    Success(ParseSuccessMessage),
    Failed(ParseFailedMessage),
}
