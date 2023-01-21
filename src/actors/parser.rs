use crate::messages::{
    master::{MasterMessage, Parse},
    parse::{ParseFailedMessage, ParseMessage, ParseSuccessMessage},
};
use actix::{Actor, Addr, Context, Handler};
use reqwest::Url;

use super::master::Master;

pub struct Parser {
    pub master_addr: Addr<Master>,
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

    fn handle(&mut self, incoming_msg: ParseMessage, _ctx: &mut Context<Self>) {
        let msg: MasterMessage;
        match Url::parse(&incoming_msg.url) {
            Ok(url) => {
                let parse_success_msg = ParseSuccessMessage {
                    url: incoming_msg.url,
                    parsed_url: url,
                };
                msg = MasterMessage::Parse(Parse::Success(parse_success_msg));
            }
            Err(e) => {
                let parse_failed_msg = ParseFailedMessage {
                    url: incoming_msg.url,
                    error: e,
                };
                msg = MasterMessage::Parse(Parse::Failed(parse_failed_msg));
            }
        }
        self.master_addr.do_send(msg);
    }
}
