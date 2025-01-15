use std::sync::Arc;

use tokio::{sync::Mutex, task};

#[tokio::main]
async fn main() {
    let data = Arc::new(Mutex::new(0));

    let data_clone = data.clone();
    let handle = task::spawn(async move {
        let mut data = data_clone.lock().await;
        *data += 1;
        println!("data: {}", *data);
    });

    handle.await.unwrap();

    let locked_data = data.lock().await;
    println!("locked_data: {}", *locked_data);
}
