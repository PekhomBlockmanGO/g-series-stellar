#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, String};

// Define the possible states for an item in the supply chain
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ItemStatus {
    Manufactured,
    InTransit,
    Delivered,
}

// Define the structure of a supply chain item
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Item {
    pub name: String,
    pub status: ItemStatus,
}

#[contract]
pub struct SupplyChainTracker;

#[contractimpl]
impl SupplyChainTracker {
    
    /// Initializes a new item in the supply chain.
    /// Defaults the status to `Manufactured`.
    pub fn create_item(env: Env, id: u32, name: String) {
        let item = Item {
            name,
            status: ItemStatus::Manufactured,
        };
        // Store the item persistently using its ID as the key
        env.storage().persistent().set(&id, &item);
    }

    /// Updates the status of an existing item.
    pub fn update_status(env: Env, id: u32, new_status: ItemStatus) {
        // Fetch the existing item
        if let Some(mut item) = env.storage().persistent().get::<_, Item>(&id) {
            item.status = new_status; // Update the status
            env.storage().persistent().set(&id, &item); // Save the updated item
        } else {
            panic!("Item not found in the supply chain!");
        }
    }

    /// Retrieves the details of a specific item by its ID.
    pub fn get_item(env: Env, id: u32) -> Option<Item> {
        env.storage().persistent().get(&id)
    }
}