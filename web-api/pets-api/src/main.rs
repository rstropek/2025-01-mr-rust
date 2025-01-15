use std::sync::Arc;

use axum::{routing::get, Router};
use handler::{add_pet, get_pets};
use pets::PetStore;
use tokio::{net::TcpListener, sync::RwLock};

type PetStoreState = Arc<RwLock<PetStore>>;

mod handler;

#[tokio::main]
async fn main() {
    let pet_store: PetStoreState = Arc::new(RwLock::new(PetStore::new()));

    let app = Router::new()
        .route("/pets", get(get_pets).post(add_pet))
        .with_state(pet_store);

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();	
    axum::serve(listener, app).await.unwrap();
}

