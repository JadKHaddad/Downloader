use actix::Message;

#[derive(Message)]
#[rtype(result = "()")]
pub struct InputMessage {
    pub urls: Vec<String>,
}