use actix::{Message};

#[derive(Message)]
#[rtype(result = "()")]
pub struct ParseMessage {
    pub url: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ParseSuccessMessage {
    pub url: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ParseFailedMessage {
    pub url: String,
    //Error
}