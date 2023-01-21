pub enum Status {
    Created,
    Success,
    Failure(Failure),
}

pub enum Failure {
    ParseFailure,
    DownloadFailure,
}
