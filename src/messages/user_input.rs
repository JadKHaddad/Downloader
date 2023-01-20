use actix::Message;

#[derive(Message)]
#[rtype(result = "()")]
pub struct UserInput {
    pub url: String,
}
