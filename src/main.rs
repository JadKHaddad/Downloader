use std::time::Duration;

use actix::{clock, prelude::*};
use downloader::{
    actors::master::Master,
    messages::{master::MasterMessage, user_input::UserInput},
};

#[actix_rt::main]
async fn main() {
    let urls = vec![
        "https://www.google.com",
        "https://www.youtube.com",
        "https://www.reddit.com",
        "https://upload.wikimedia.org/wikipedia/commons/1/15/Cat_August_2010-4.jpg",
    ];

    let master_addr = Master::default().start();

    for url in urls {
        let user_input_msg = UserInput {
            url: url.to_string(),
        };
        let msg = MasterMessage::UserInput(user_input_msg);
        master_addr.do_send(msg);
    }

    clock::sleep(Duration::from_secs(20)).await;
    System::current().stop();
}
