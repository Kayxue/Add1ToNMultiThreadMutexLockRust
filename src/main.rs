use parking_lot::Mutex;
use std::env;
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let x: i128 = args[1].parse().unwrap();
    let n: i128 = args[2].parse().unwrap();
    let mut handler: Vec<JoinHandle<()>> = Vec::new();

    let sum = Arc::new(Mutex::new(0i128));
    for i in 1..=x {
        let locker = Arc::clone(&sum);
        handler.push(thread::spawn(move || {
            println!("Thread {} started", i);
            let start_from = (i - 1) * (n / x) + 1;
            let end = if i == x { n } else { i * (n / x) };
            for i in start_from..=end {
                {
                    let mut sum = locker.lock();
                    *sum += i;
                }
            }
        }));
    }

    for handle in handler {
        handle.join().unwrap();
    }

    println!("Sum: {:?}", *sum.lock());
}
