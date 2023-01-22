use crate::errors::file::Error as FileError;
use reqwest::Error as ReqwestError;
use std::io::Error as IoError;

pub enum Status {
    Created,
    Success,
    Failure(Failure),
}

pub enum Failure {
    ParseFailure,
    DownloadFailure(ReqwestError),
    FileFailure(FileError),
    WriteFailure(IoError),
}
