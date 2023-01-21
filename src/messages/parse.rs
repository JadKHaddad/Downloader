use actix::Message;
use reqwest::Url;
use url::ParseError;

#[derive(Message)]
#[rtype(result = "()")]
pub struct ParseMessage {
    pub url: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ParseSuccessMessage {
    pub url: String,
    pub parsed_url: Url,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ParseFailedMessage {
    pub url: String,
    pub error: ParseError,
}
