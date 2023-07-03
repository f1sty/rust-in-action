use std::thread;
use crossbeam::channel::unbounded;

use crate::ConnectivityCheck::*;

#[derive(Debug)]
enum ConnectivityCheck {
    Ping,
    Pong,
    Pang,
}

fn main() {
    let n_messages = 3;
    let (requests_tx, requests_rx) = unbounded();
    let (responses_tx, responses_rx) = unbounded();

    thread::spawn(move || loop {
        match requests_rx.recv().unwrap() {
            Pong => eprintln!("unexpected pong response"),
            Ping => responses_tx.send(Pong).unwrap(),
            Pang => return,
        }
    });

    (0..n_messages).for_each(|_| requests_tx.send(Ping).unwrap());

    requests_tx.send(Pang).unwrap();

    (0..n_messages).for_each(|_| println!("{:?}", responses_rx.recv().unwrap()));
}
