use super::master::Master;
use crate::messages::{
    master::{MasterMessage, Write as MasterWrite},
    write::{WriteFailedMessage, WriteMessage, WriteSuccessMessage},
};
use actix::{fut, Actor, ActorFutureExt, Addr, Context, ContextFutureSpawner, Handler, WrapFuture};
use std::{fs::File, io::Write};

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
        incoming_msg
            .response
            .bytes()
            .into_actor(self)
            .then(|res, act, _ctx| {
                let msg = match res {
                    Ok(bytes) => {
                        match infer::get(&bytes) {
                            Some(kind) => {
                                let extension = kind.extension();
                                //TODO: get a filename, maybe from the url
                                let full_filename = format!("file.{}", extension);
                                let mut file = File::create(full_filename).unwrap();

                                match file.write_all(&bytes) {
                                    Ok(_) => {
                                        let write_success_msg = WriteSuccessMessage {
                                            url: incoming_msg.url,
                                        };
                                        MasterMessage::Write(MasterWrite::Success(
                                            write_success_msg,
                                        ))
                                    }
                                    Err(_e) => {
                                        //TODO: IO error
                                        let write_failed_msg = WriteFailedMessage {
                                            url: incoming_msg.url,
                                        };
                                        MasterMessage::Write(MasterWrite::Failed(write_failed_msg))
                                    }
                                }
                            }
                            None => {
                                //TODO: unknown file type
                                let write_failed_msg = WriteFailedMessage {
                                    url: incoming_msg.url,
                                };
                                MasterMessage::Write(MasterWrite::Failed(write_failed_msg))
                            }
                        }
                    }
                    Err(_e) => {
                        //TODO: can't read response body
                        let write_failed_msg = WriteFailedMessage {
                            url: incoming_msg.url,
                        };
                        MasterMessage::Write(MasterWrite::Failed(write_failed_msg))
                    }
                };
                act.master_addr.do_send(msg);
                fut::ready(())
            })
            .wait(_ctx);
    }
}
