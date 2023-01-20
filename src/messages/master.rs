use actix::Message;

use crate::InputMessage;

use super::parse::ParseFailedMessage;

#[derive(Message)]
#[rtype(result = "()")]
pub enum MasterMessage {
    InputMessage(InputMessage),
    ParseFailedMessage(ParseFailedMessage),
}