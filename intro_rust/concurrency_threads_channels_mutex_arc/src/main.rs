mod channel;
mod mutex;
mod thread;
fn main() {
    thread::spawn_threads();

    channel::simple_example();
    channel::multiple_producer_one_consumer();

    mutex::first_example();
}
