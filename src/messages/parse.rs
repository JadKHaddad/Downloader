use actix::{Addr, Message};

#[derive(Message)]
#[rtype(result = "()")]
pub struct ParseMessage {
    pub url: String,
    pub master: Addr<crate::master::Master>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ParseFailedMessage {
    pub url: String,
    //Error
}