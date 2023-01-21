use std::collections::HashMap;

use crate::{
    actors::{downloader::Downloader, parser::Parser, writer::Writer},
    messages::{
        download::DownloadMessage, master::MasterMessage, parse::ParseMessage, write::WriteMessage,
    },
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
                    crate::messages::master::Parse::Success(parse_success_msg) => {
                        println!("Received ParseSuccessMessage");

                        // start a downloader
                        let downloader_addr = Downloader {
                            master_addr: ctx.address(),
                        }
                        .start();

                        // send the downloader a download message
                        let msg = DownloadMessage {
                            url: parse_success_msg.url,
                            parsed_url: parse_success_msg.parsed_url,
                        };

                        downloader_addr.do_send(msg);
                    }
                    crate::messages::master::Parse::Failed(msg) => {
                        println!("Received ParseFailedMessage");
                        self.urls
                            .insert(msg.url, Status::Failure(Failure::ParseFailure));
                    }
                }
            }
            MasterMessage::Download(download) => {
                match download {
                    crate::messages::master::Download::Success(download_success_msg) => {
                        println!("Received DownloadSuccessMessage");

                        // start a writer
                        let writer_addr = Writer {
                            master_addr: ctx.address(),
                        }
                        .start();

                        // send the writer a write message
                        let msg = WriteMessage {
                            url: download_success_msg.url,
                            response: download_success_msg.response,
                        };

                        writer_addr.do_send(msg);
                    }
                    crate::messages::master::Download::Failed(msg) => {
                        println!("Received DownloadFailedMessage");
                        self.urls
                            .insert(msg.url, Status::Failure(Failure::DownloadFailure));
                    }
                }
            }
            MasterMessage::Write(write) => match write {
                crate::messages::master::Write::Success(write_success_msg) => {
                    println!("Received WriteSuccessMessage");
                    self.urls.insert(write_success_msg.url, Status::Success);
                }
                crate::messages::master::Write::Failed(msg) => {
                    println!("Received WriteFailedMessage");
                    self.urls
                        .insert(msg.url, Status::Failure(Failure::WriteFailure));
                }
            },
        }
    }
}
