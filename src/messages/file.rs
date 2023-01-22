use crate::errors::file::Error;
use actix::Message;
use bytes::Bytes;
use reqwest::Response;
use std::fs::File;

#[derive(Message)]
#[rtype(result = "()")]
pub struct FileMessage {
    pub url: String,
    pub domain: String,
    pub response: Response,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct FileSuccessMessage {
    pub url: String,
    pub file: File,
    pub bytes: Bytes,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct FileFailedMessage {
    pub url: String,
    pub error: Error,
}
