use std::{panic, sync::{atomic::AtomicBool, Arc}, thread};

fn main() {
    let panic_happened = Arc::new(AtomicBool::new(false));

    let panic_happened_clone = panic_happened.clone();
    std::panic::set_hook(Box::new(move |info| {
        panic_happened_clone.store(true, std::sync::atomic::Ordering::Relaxed);
        eprintln!("Panic occurred: {}", info);
    }));

    let result = thread::spawn(|| {
        panic_with_message("Something went wrong");
    }).join();

    match result {
        Ok(()) => println!("Thread finished successfully"),
        Err(_) => println!("Thread panicked with error"),
    }

    let result = panic::catch_unwind(|| {
        panic_with_message("Something went wrong");
    });

    match result {
        Ok(()) => println!("Thread finished successfully"),
        Err(_) => println!("Thread panicked with error"),
    }
}

fn panic_with_message(message: &str) {
    panic!("{}", message);
}
