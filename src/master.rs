use actix::{Actor, Context, Handler};
use crate::messages::master::MasterMessage;

pub struct Master {
    // ...
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
            MasterMessage::InputMessage(_msg) => {
                println!("Received InputMessage");
            }
            MasterMessage::ParseFailedMessage(_msg) => {
                println!("Received ParseFailedMessage");
            }
        }
    }
}