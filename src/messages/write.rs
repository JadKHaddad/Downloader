use std::{fs::File, io::Error as IoError};

use actix::Message;
use bytes::Bytes;

#[derive(Message)]
#[rtype(result = "()")]
pub struct WriteMessage {
    pub url: String,
    pub file: File,
    pub bytes: Bytes,
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
    pub error: IoError,
}
