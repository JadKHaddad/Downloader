use actix::Message;

use super::parse::ParseFailedMessage;
use super::parse::ParseSuccessMessage;

#[derive(Message)]
#[rtype(result = "()")]
pub enum MasterMessage {
    Parse(Parse),
}

pub enum Parse {
    Success(ParseSuccessMessage),
    Failed(ParseFailedMessage),
}