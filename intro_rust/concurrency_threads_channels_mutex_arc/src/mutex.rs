use std::{
    sync::{Arc, Mutex},
    thread,
};

pub fn first_example() {
    println!("MUTEX - FIRST EXAMPLE");

    let mtx = Arc::new(Mutex::new(0));
    let mut thr = vec![];

    for _ in 0..10 {
        let mtx = Arc::clone(&mtx);
        let h = thread::spawn(move || {
            let mut num = mtx.lock().unwrap();
            *num += 1;
            // lock dropped, so automatic unlocked
        });
        thr.push(h);
    }

    for h in thr {
        h.join().unwrap()
    }

    println!("Result: {}", *mtx.lock().unwrap());

    println!();
}
