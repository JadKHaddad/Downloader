use std::collections::HashMap;

use crate::{messages::master::MasterMessage, Status, Failure};
use actix::{Actor, Context, Handler};

/**
 * Only the master has access to the input/output channels
 * Only the master can communicate with the user
 */
pub struct Master {
    urls: HashMap<String, Status>,
}

impl Master {
    pub fn new() -> Self {
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

    fn handle(&mut self, msg: MasterMessage, _ctx: &mut Context<Self>) {
        match msg {
            MasterMessage::Parse(parse) =>{
                match parse {
                    crate::messages::master::Parse::Success(_msg) => {
                        // TODO: create a downloader and send him a download message
                    }
                    crate::messages::master::Parse::Failed(msg) => {
                        println!("Received ParseFailedMessage");
                        self.urls.insert(msg.url, Status::Failure(Failure::ParseFailure));
                    }
                }
            }
        }
    }
}
