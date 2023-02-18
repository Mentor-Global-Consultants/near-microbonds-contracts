use near_sdk::{AccountId, collections::{LookupMap, UnorderedSet}, PanicOnDefault, near_bindgen, BorshStorageKey, env::{self}};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{U128};
use near_sdk::serde::{Deserialize, Serialize};

use crate::internal::*;
use crate::events::*;
use crate::views::*;
use crate::registry_core::*;

mod internal;
mod events;
mod views;
mod registry_core;


// Main contract structure to store all information
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    // Registry contract owner
    pub owner_id: AccountId,

    /// This tracks which users have been approved by which municipality
    /// MunicipalityId => [] of UserIds
    pub municipality_to_users: LookupMap<String, UnorderedSet<String>>,
}

#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKey {
    MunicipalityToUsers,
    MunicipalityToUsersInner,
    MuncipalityToUsersVector,
}

#[near_bindgen]
impl Contract {

    /**
     * Initialization function (can only be called once)
     */
    #[init]
    pub fn new(
        owner_id: AccountId
    ) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        let this = Self {
            // Set the owner_id field equal to the passed in owner_id
            owner_id,
            municipality_to_users: LookupMap::new(StorageKey::MunicipalityToUsers.try_to_vec().unwrap())
        };

        // Return the Contract object
        this
    }
}

#[cfg(test)]
mod tests;