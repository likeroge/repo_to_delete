use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread::{self},
};

pub fn study_fn_main(ap: (Sender<String>, Receiver<String>)) {
    println!("Multithreading!!!");
    let (tx, rx) = mpsc::channel::<String>();

    thread::spawn(move || {
        let data = String::from("Hellowwwww");

        match tx.send(data) {
            Ok(_) => println!("Sended to channel"),
            Err(_) => todo!(),
        };
    });

    match rx.recv() {
        Ok(data_from_channel) => println!("{data_from_channel}"),
        Err(_) => todo!(),
    }
}
