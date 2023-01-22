use super::master::Master;
use crate::{
    errors::file::Error as FileError,
    messages::{
        file::{FileFailedMessage, FileMessage, FileSuccessMessage},
        master::{File as MasterFile, MasterMessage},
    },
};
use actix::{fut, Actor, ActorFutureExt, Addr, Context, ContextFutureSpawner, Handler, WrapFuture};
use std::{fs::File, path::Path};

pub struct Filer {
    pub master_addr: Addr<Master>,
}

impl Actor for Filer {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Context<Self>) {
        println!("Filer is alive");
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        println!("Filer is stopped");
    }
}

impl Handler<FileMessage> for Filer {
    type Result = ();

    fn handle(&mut self, incoming_msg: FileMessage, _ctx: &mut Context<Self>) {
        let domain = incoming_msg.domain;
        incoming_msg
            .response
            .bytes()
            .into_actor(self)
            .then(move |res, act, _ctx| {
                let msg = match res {
                    Ok(bytes) => {
                        match infer::get(&bytes) {
                            Some(kind) => {
                                let extension = kind.extension();
                                let mut full_filename = format!("{}.{}", domain, extension);
                                if Path::new(&full_filename).exists() {
                                    let mut counter = 0;
                                    full_filename = loop {
                                        if counter > 1000 {
                                            panic!("Can't find a filename! you have 1000 files with the nearly same name! I'm out of ideas!");
                                        }
                                        let filename = format!("{}_{}.{}", domain, counter, extension);
                                        if !Path::new(&filename).exists() {
                                            break filename;
                                        }
                                        counter += 1;
                                    };
                                }
                                match File::create(full_filename) {
                                    Ok(file) => {
                                        let file_success_msg = FileSuccessMessage {
                                            url: incoming_msg.url,
                                            file,
                                            bytes
                                        };
                                        MasterMessage::File(MasterFile::Success(file_success_msg))
                                    }
                                    Err(e) => {
                                        let file_failed_msg = FileFailedMessage {
                                            url: incoming_msg.url,
                                            error: FileError::CantCreateFile(e),
                                        };
                                        MasterMessage::File(MasterFile::Failed(file_failed_msg))
                                    }
                                }
                            }
                            None => {
                                let file_failed_msg = FileFailedMessage {
                                    url: incoming_msg.url,
                                    error: FileError::UnknownFileType,
                                };
                                MasterMessage::File(MasterFile::Failed(file_failed_msg))
                            }
                        }
                    }
                    Err(_) => {
                        let file_failed_msg = FileFailedMessage {
                            url: incoming_msg.url,
                            error: FileError::CantReadResponse,
                        };
                        MasterMessage::File(MasterFile::Failed(file_failed_msg))
                    }
                };
                act.master_addr.do_send(msg);
                fut::ready(())
            })
            .wait(_ctx);
    }
}
