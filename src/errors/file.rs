use std::io::Error as IoError;

pub enum Error {
    UnknownFileType,
    CantReadResponse,
    CantCreateFileName,
    CantCreateFile(IoError),
}
