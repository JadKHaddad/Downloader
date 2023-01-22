use actix::prelude::*;
use downloader::{
    actors::master::Master,
    messages::{master::MasterMessage, user_input::UserInput},
};
use std::io::Result as IoResult;

fn main() -> IoResult<()> {
    let urls = vec![
        "https://www.google.com",
        "https://www.youtube.com",
        "https://www.reddit.com",
        "https://upload.wikimedia.org/wikipedia/commons/1/15/Cat_August_2010-4.jpg",
    ];

    let sys = System::new();

    let master_addr = sys.block_on(async { Master::default().start() });

    for url in urls {
        let user_input_msg = UserInput {
            url: url.to_string(),
        };
        let msg = MasterMessage::UserInput(user_input_msg);
        master_addr.do_send(msg);
    }

    sys.run()
    // TODO: Stop the system when all downloads are complete
}
