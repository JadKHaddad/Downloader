use actix::Message;

use super::{
    parse::{ParseFailedMessage, ParseSuccessMessage},
    user_input::UserInput,
};

#[derive(Message)]
#[rtype(result = "()")]
pub enum MasterMessage {
    UserInput(UserInput),
    Parse(Parse),
}

pub enum Parse {
    Success(ParseSuccessMessage),
    Failed(ParseFailedMessage),
}
