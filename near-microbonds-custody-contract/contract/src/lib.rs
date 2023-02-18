use near_sdk::{AccountId, collections::{LookupMap, UnorderedSet}, PanicOnDefault, near_bindgen, BorshStorageKey, env::{self}, Gas};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{U128};
use near_sdk::serde::{Deserialize, Serialize};

use crate::external::*;
use crate::types::*;
use crate::events::*;
use crate::user_account::*;

mod views;
mod internal;
mod external;
mod types;
mod events;
mod tokens;
mod user_account;

const GAS_FOR_NFT_TRANSFER: Gas = Gas(15_000_000_000_000);
const GAS_FOR_RESOLVE_TRANSFER: Gas = Gas(15_000_000_000_000);
const GAS_FOR_REGISTRY_RESOLVE: Gas = Gas(15_000_000_000_000);

static DELIMITER: &str = ":";

/**
 * Description: 
 * This contract is responsible for managing the custody of NFTs on behalf of users.
 * The contract is designed to be used by a web app that allows users to bind their NEAR wallet to the contract.
 * Once bound, the user can transfer their stored NFTs to their linked wallet.
 */

// Main contract structure to store all information
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    // Custody contract owner
    pub owner_id: AccountId,

    // Stores the set of accountId:tokenId for each owner
    pub tokens_per_owner: LookupMap<String, UnorderedSet<String>>,

    /// This tracks if a user has bound an external near wallet to the registry
    /// UserString => AccountId
    pub user_to_account: LookupMap<String, AccountId>
}

#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKey {
    TokensPerOwner,
    TokensPerOwnerInner,
    TokensAsVector,
    UserToAccount,
}

#[near_bindgen]
impl Contract {

    /**
     * Initialization function (can only be called once)
     */
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        let this = Self {
            // Set the owner_id field equal to the passed in owner_id
            owner_id,
            tokens_per_owner: LookupMap::new(StorageKey::TokensPerOwner.try_to_vec().unwrap()),
            user_to_account: LookupMap::new(StorageKey::UserToAccount.try_to_vec().unwrap())
        };

        // Return the Contract object
        this
    }
}

#[cfg(test)]
mod tests;