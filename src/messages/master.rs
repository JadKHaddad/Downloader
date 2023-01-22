use actix::Message;

use super::{
    download::{DownloadFailedMessage, DownloadSuccessMessage},
    file::{FileFailedMessage, FileSuccessMessage},
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
    File(File),
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

pub enum File {
    Success(FileSuccessMessage),
    Failed(FileFailedMessage),
}

pub enum Write {
    Success(WriteSuccessMessage),
    Failed(WriteFailedMessage),
}
