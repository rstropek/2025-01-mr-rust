use std::{sync::{mpsc, Arc, Mutex}, thread, time::Duration};

// MPSC - Multiple Producer Single Consumer
//        Mulitple Producers Multiple Consumers

fn main() {
    let (tx, rx) = mpsc::channel();
    let rx = Arc::new(Mutex::new(rx));	

    for id in 0..2 {
        let tx = tx.clone();
        thread::spawn(move || {
            println!("Producer {} started", id);
            
            for i in 0..3 {
                tx.send(i).unwrap();
                thread::sleep(Duration::from_millis(250));
            }
        });
    }

    drop(tx);
    
    let mut handles = Vec::new();
    for _ in 0..2 {
        let rx = rx.clone();
        handles.push(thread::spawn(move || {
            while let Ok(val) = rx.lock().unwrap().recv() {
                println!("Received: {}", val);
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
