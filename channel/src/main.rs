use std::sync::mpsc;
use std::thread;

fn main() {
    // println!("Hello, world!");
    let (sender,receiver) = mpsc::channel();

    thread::spawn(move || {
       let val = String::from("wodeshijie");
        sender.send(val.clone()).unwrap();
        sender.send(val.len()).unwrap();
    });
    let val = receiver.recv().unwrap();
    println!("{}",val);

}

