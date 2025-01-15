use axum::{extract::State, response::IntoResponse, Json};
use pets::Pet;

use crate::PetStoreState;

pub async fn get_pets(State(pet_store): State<PetStoreState>) -> impl IntoResponse {
    let pets = pet_store.read().await.get_pets();
    Json(pets)
}

pub async fn add_pet(State(pet_store): State<PetStoreState>, Json(pet): Json<Pet>) -> impl IntoResponse {
    let pet_with_id = pet_store.write().await.add_pet(pet);
    Json(pet_with_id)
}