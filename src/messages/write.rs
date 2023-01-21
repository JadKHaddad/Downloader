use actix::Message;
use reqwest::Response;

#[derive(Message)]
#[rtype(result = "()")]
pub struct WriteMessage {
    pub url: String,
    pub response: Response,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct WriteSuccessMessage {
    pub url: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct WriteFailedMessage {
    pub url: String,
    //TODO: add error
}
