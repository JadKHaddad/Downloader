use actix::{Actor, Context, Handler};
use crate::messages::parse::ParseMessage;

pub struct Parser {
    // ...
}

impl Actor for Parser {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Context<Self>) {
        println!("Parser is alive");
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        println!("Parser is stopped");
    }
}

impl Handler<ParseMessage> for Parser {
    type Result = ();

    fn handle(&mut self, _msg: ParseMessage, _ctx: &mut Context<Self>) {
        println!("Received ParseMessage");
        // parse url
        // send ParseFailedMessage if failed
        // create a downloader and send him a download message
    }
}