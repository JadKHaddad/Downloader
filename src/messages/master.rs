use actix::Message;

use super::{
    download::{DownloadFailedMessage, DownloadSuccessMessage},
    parse::{ParseFailedMessage, ParseSuccessMessage},
    user_input::UserInput,
    write::{WriteFailedMessage, WriteSuccessMessage},
};

#[derive(Message)]
#[rtype(result = "()")]
pub enum MasterMessage {
    UserInput(UserInput),
    Parse(Parse),
    Download(Download),
    Write(Write),
}

pub enum Parse {
    Success(ParseSuccessMessage),
    Failed(ParseFailedMessage),
}

pub enum Download {
    Success(DownloadSuccessMessage),
    Failed(DownloadFailedMessage),
}

pub enum Write {
    Success(WriteSuccessMessage),
    Failed(WriteFailedMessage),
}
