use actix::Message;
use reqwest::{Error as ReqwestError, Response, Url};

#[derive(Message)]
#[rtype(result = "()")]
pub struct DownloadMessage {
    pub url: String,
    pub domain: String,
    pub parsed_url: Url,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct DownloadSuccessMessage {
    pub url: String,
    pub domain: String,
    pub response: Response,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct DownloadFailedMessage {
    pub url: String,
    pub error: ReqwestError,
}
