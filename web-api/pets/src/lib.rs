use std::{collections::HashMap, sync::atomic::AtomicUsize};

use serde::{Deserialize, Serialize};

/// A pet
/// 
/// This structure represents a **pet**.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pet {
    pub name: String,
    pub number_of_legs: u8,
}

/// A pet with an id
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PetWithId {
    pub id: usize,

    #[serde(flatten)]
    pub pet: Pet,
}

pub struct PetStore {
    pets: HashMap<usize, PetWithId>,
    id_generator: AtomicUsize,
}

impl PetStore {
    pub fn new() -> Self {
        Self {
            pets: HashMap::new(),
            id_generator: AtomicUsize::new(0),
        }
    }

    /// Adds a new pet to the store
    /// 
    /// ### Arguments
    /// 
    /// * `pet` - The pet to add to the store
    /// 
    /// ### Returns
    /// 
    /// Returns a `PetWithId` containing the added pet and its assigned unique ID
    /// 
    /// ### Example
    /// 
    /// ```
    /// use pets::{Pet, PetStore};
    /// 
    /// let mut store = PetStore::new();
    /// let pet = Pet {
    ///     name: "Fluffy".to_string(),
    ///     number_of_legs: 4,
    /// };
    /// 
    /// let pet_with_id = store.add_pet(pet);
    /// # assert_eq!(pet_with_id.id, 0); // First pet gets ID 0
    /// ```
    pub fn add_pet(&mut self, pet: Pet) -> PetWithId {
        let id = self.id_generator.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let pet_with_id = PetWithId { id, pet };
        self.pets.insert(id, pet_with_id.clone());
        pet_with_id
    }

    pub fn remove_pet(&mut self, id: usize) {
        self.pets.remove(&id);
    }

    pub fn get_pets(&self) -> Vec<PetWithId> {
        self.pets.values().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_get_remove() {
        let mut pet_store = PetStore::new();
        let pet = Pet { name: "Fluffy".to_string(), number_of_legs: 4 };
        let pet_with_id = pet_store.add_pet(pet);
        assert_eq!(pet_store.get_pets().len(), 1);
        pet_store.remove_pet(pet_with_id.id);
        assert_eq!(pet_store.get_pets().len(), 0);
    }
}
