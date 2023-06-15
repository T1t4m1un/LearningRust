use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

fn increment(counter: Arc<AtomicUsize>) {
    for _ in 0..1_000_000 {
        counter.fetch_add(1, Ordering::Relaxed); // No data race here
    }
}

fn main() {
    let counter = Arc::new(AtomicUsize::new(0));

    let t1 = thread::spawn({
        let counter = Arc::clone(&counter);
        move || {
            increment(counter);
        }
    });

    let t2 = thread::spawn({
        let counter = Arc::clone(&counter);
        move || {
            increment(counter);
        }
    });

    t1.join().unwrap();
    t2.join().unwrap();

    println!("Counter: {}", counter.load(Ordering::Relaxed));
}
