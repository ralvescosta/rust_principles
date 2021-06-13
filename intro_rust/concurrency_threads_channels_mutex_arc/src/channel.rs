use core::time::Duration;
use std::{sync::mpsc, thread};

pub fn simple_example() {
    println!("CHANNEL - SIMPLE EXAMPLE");

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || tx.send(42).unwrap());

    println!("got {}", rx.recv().unwrap());

    println!();
}

pub fn multiple_producer_one_consumer() {
    println!("CHANNEL - MULTIPLE PRODUCER ONE CONSUMER");
    const NUM_TIMERS: usize = 24;

    fn timer(d: usize, tx: mpsc::Sender<usize>) {
        thread::spawn(move || {
            println!("{}: setting timer...", d);
            thread::sleep(Duration::from_secs(d as u64));
            println!("{}: sent!", d);
            tx.send(d).unwrap();
        });
    }

    let (tx, rx) = mpsc::channel();

    for i in 0..NUM_TIMERS {
        timer(i, tx.clone());
    }

    for v in rx.iter().take(NUM_TIMERS) {
        println!("{}: received!", v);
    }

    println!("");
}
