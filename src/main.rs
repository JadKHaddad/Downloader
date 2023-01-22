use actix::{clock, prelude::*};
use downloader::{
    actors::master::Master,
    messages::{master::MasterMessage, user_input::UserInput},
};
use std::{io::Result as IoResult, time::Duration};

fn main() -> IoResult<()> {
    let urls = vec![
        "https://www.google.com",
        "https://www.youtube.com",
        "https://www.reddit.com",
        "https://upload.wikimedia.org/wikipedia/commons/1/15/Cat_August_2010-4.jpg",
    ];

    let sys = System::new();

    let master_addr = sys.block_on(async {
        let add = Master::default().start();
        let add_c = add.clone();

        actix_rt::spawn(async move {
            // Terminate the master and stop the system after 5 seconds
            clock::sleep(Duration::from_secs(5)).await;
            add_c
                .send(MasterMessage::Die)
                .await
                .expect("Failed to send Die message to Master");
            System::current().stop();
        });

        add
    });

    for url in urls {
        let user_input_msg = UserInput {
            url: url.to_string(),
        };
        let msg = MasterMessage::UserInput(user_input_msg);
        master_addr.do_send(msg);
    }

    sys.run()
}
