use std::io::Error as IoError;
pub enum Error {
    UnknownFileType,
    CantReadResponse,
    CantCreateFile(IoError),
}
