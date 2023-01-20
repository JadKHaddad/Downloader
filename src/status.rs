pub enum Status {
    Success,
    Failure(Failure),
}

pub enum Failure {
    ParseFailure,
}