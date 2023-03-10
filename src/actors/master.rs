use std::collections::HashMap;

use crate::{
    actors::{downloader::Downloader, filer::Filer, parser::Parser, writer::Writer},
    messages::{
        download::DownloadMessage, file::FileMessage, master::MasterMessage, parse::ParseMessage,
        write::WriteMessage,
    },
    status::{Failure, Status},
};
use actix::{Actor, ActorContext /*WrapFuture*/, AsyncContext, Context, Handler};

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
                self.on_user_input(user_input, ctx);
            }
            MasterMessage::Parse(parse) => {
                self.on_parse(parse, ctx);
            }
            MasterMessage::Download(download) => {
                self.on_download(download, ctx);
            }
            MasterMessage::File(file) => {
                self.on_file(file, ctx);
            }
            MasterMessage::Write(write) => {
                self.on_write(write);
            }
            MasterMessage::Die => {
                ctx.terminate();
            }
        }
        // Use some async code and wait for it to finish
        // let fut = async move {
        //     // Some asynchronous code
        // }.into_actor(self);
        // ctx.wait(fut);
    }
}

impl Master {
    fn on_user_input(
        &mut self,
        user_input: crate::messages::user_input::UserInput,
        ctx: &mut Context<Master>,
    ) {
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

    fn on_parse(&mut self, parse: crate::messages::master::Parse, ctx: &mut Context<Master>) {
        match parse {
            crate::messages::master::Parse::Success(parse_success_msg) => {
                // start a downloader
                let downloader_addr = Downloader {
                    master_addr: ctx.address(),
                }
                .start();

                // send the downloader a download message
                let msg = DownloadMessage {
                    url: parse_success_msg.url,
                    domain: parse_success_msg.domain,
                    parsed_url: parse_success_msg.parsed_url,
                };

                downloader_addr.do_send(msg);
            }
            crate::messages::master::Parse::Failed(msg) => {
                self.urls
                    .insert(msg.url, Status::Failure(Failure::ParseFailure));
            }
        }
    }

    fn on_download(
        &mut self,
        download: crate::messages::master::Download,
        ctx: &mut Context<Master>,
    ) {
        match download {
            crate::messages::master::Download::Success(download_success_msg) => {
                // start a filer
                let filer_addr = Filer {
                    master_addr: ctx.address(),
                }
                .start();

                // send the filer a file message
                let msg = FileMessage {
                    url: download_success_msg.url,
                    domain: download_success_msg.domain,
                    response: download_success_msg.response,
                };

                filer_addr.do_send(msg);
            }
            crate::messages::master::Download::Failed(download_fail_msg) => {
                self.urls.insert(
                    download_fail_msg.url,
                    Status::Failure(Failure::DownloadFailure(download_fail_msg.error)),
                );
            }
        }
    }

    fn on_file(&mut self, file: crate::messages::master::File, ctx: &mut Context<Master>) {
        match file {
            crate::messages::master::File::Success(file_success_msg) => {
                // start a writer
                let writer_addr = Writer {
                    master_addr: ctx.address(),
                }
                .start();

                // send the writer a write message
                let msg = WriteMessage {
                    url: file_success_msg.url,
                    file: file_success_msg.file,
                    bytes: file_success_msg.bytes,
                };

                writer_addr.do_send(msg);
            }
            crate::messages::master::File::Failed(file_fail_msg) => {
                self.urls.insert(
                    file_fail_msg.url,
                    Status::Failure(Failure::FileFailure(file_fail_msg.error)),
                );
            }
        }
    }

    fn on_write(&mut self, write: crate::messages::master::Write) {
        match write {
            crate::messages::master::Write::Success(write_success_msg) => {
                self.urls.insert(write_success_msg.url, Status::Success);
            }
            crate::messages::master::Write::Failed(write_fail_msg) => {
                self.urls.insert(
                    write_fail_msg.url,
                    Status::Failure(Failure::WriteFailure(write_fail_msg.error)),
                );
            }
        }
    }
}
