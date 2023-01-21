use super::master::Master;
use crate::messages::{
    download::{DownloadFailedMessage, DownloadMessage, DownloadSuccessMessage},
    master::{Download, MasterMessage},
};
use actix::{fut, Actor, ActorFutureExt, Addr, Context, ContextFutureSpawner, Handler, WrapFuture};
use reqwest::get;

pub struct Downloader {
    pub master_addr: Addr<Master>,
}

impl Actor for Downloader {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Context<Self>) {
        println!("Downloader is alive");
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        println!("Downloader is stopped");
    }
}

impl Handler<DownloadMessage> for Downloader {
    type Result = ();

    fn handle(&mut self, incoming_msg: DownloadMessage, _ctx: &mut Context<Self>) {
        get(incoming_msg.parsed_url)
            .into_actor(self)
            .then(|res, act, _ctx| {
                let msg = match res {
                    Ok(response) => {
                        let download_success_msg = DownloadSuccessMessage {
                            url: incoming_msg.url,
                            response,
                        };
                        MasterMessage::Download(Download::Success(download_success_msg))
                    }
                    Err(e) => {
                        let download_failed_msg = DownloadFailedMessage {
                            url: incoming_msg.url,
                            error: e,
                        };
                        MasterMessage::Download(Download::Failed(download_failed_msg))
                    }
                };
                act.master_addr.do_send(msg);
                fut::ready(())
            })
            .wait(_ctx);
    }
}
