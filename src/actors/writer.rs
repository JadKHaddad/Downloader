use super::master::Master;
use crate::messages::{
    master::{MasterMessage, Write as MasterWrite},
    write::{WriteFailedMessage, WriteMessage, WriteSuccessMessage},
};
use actix::{Actor, Addr, Context, Handler};
use std::io::Write;

pub struct Writer {
    pub master_addr: Addr<Master>,
}

impl Actor for Writer {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Context<Self>) {
        println!("Writer is alive");
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        println!("Writer is stopped");
    }
}

impl Handler<WriteMessage> for Writer {
    type Result = ();

    fn handle(&mut self, incoming_msg: WriteMessage, _ctx: &mut Context<Self>) {
        let mut file = incoming_msg.file;
        let msg = match file.write_all(&incoming_msg.bytes) {
            Ok(_) => {
                let write_success_msg = WriteSuccessMessage {
                    url: incoming_msg.url,
                };
                MasterMessage::Write(MasterWrite::Success(write_success_msg))
            }
            Err(e) => {
                let write_failed_msg = WriteFailedMessage {
                    url: incoming_msg.url,
                    error: e,
                };
                MasterMessage::Write(MasterWrite::Failed(write_failed_msg))
            }
        };
        self.master_addr.do_send(msg);
    }
}
