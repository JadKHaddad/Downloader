use actix::Message;
use reqwest::{blocking::Response, Error as ReqwestError, Url};

#[derive(Message)]
#[rtype(result = "()")]
pub struct DownloadMessage {
    pub url: String,
    pub parsed_url: Url,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct DownloadSuccessMessage {
    pub url: String,
    pub response: Response,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct DownloadFailedMessage {
    pub url: String,
    pub error: ReqwestError,
}
