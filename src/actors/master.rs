use std::collections::HashMap;

use crate::{
    actors::parser::Parser,
    messages::{master::MasterMessage, parse::ParseMessage},
    status::{Failure, Status},
};
use actix::{Actor, AsyncContext, Context, Handler};

/**
 * Only the master has access to the input/output channels
 * Only the master can communicate with the user
 */
pub struct Master {
    urls: HashMap<String, Status>,
}

impl Default for Master {
    fn default() -> Self {
        Master {
            urls: HashMap::new(),
        }
    }
}

impl Actor for Master {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Context<Self>) {
        println!("Master is alive");
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        println!("Master is stopped");
    }
}

impl Handler<MasterMessage> for Master {
    type Result = ();

    fn handle(&mut self, msg: MasterMessage, ctx: &mut Context<Self>) {
        match msg {
            MasterMessage::UserInput(user_input) => {
                println!("Received UserInputMessage");
                self.urls.insert(user_input.url.clone(), Status::Created);

                // start a parser
                let parser_addr = Parser {
                    master_addr: ctx.address(),
                }
                .start();

                // send the parser a parse message
                let msg = ParseMessage {
                    url: user_input.url,
                };

                parser_addr.do_send(msg);
            }
            MasterMessage::Parse(parse) => {
                match parse {
                    crate::messages::master::Parse::Success(_msg) => {
                        println!("Received ParseSuccessMessage");
                        // TODO: create a downloader and send him a download message
                    }
                    crate::messages::master::Parse::Failed(msg) => {
                        println!("Received ParseFailedMessage");
                        self.urls
                            .insert(msg.url, Status::Failure(Failure::ParseFailure));
                    }
                }
            }
        }
    }
}
