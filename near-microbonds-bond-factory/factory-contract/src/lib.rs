use near_sdk::{AccountId, collections::{LookupMap, UnorderedSet, LazyOption}, PanicOnDefault, near_bindgen, BorshStorageKey, env::{self}, CryptoHash};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{U128};
use near_sdk::serde::{Deserialize, Serialize};

use crate::views::*;
use crate::internal::*;
use crate::types::*;
use crate::events::*;
use crate::factory_core::*;
use crate::tokens::*;

mod views;
mod internal;
mod types;
mod events;
mod factory_core;
mod tokens;

/**
 * Description:
 * This contract is responsible for tracking all registered municipalities and the projects registered under them. Projects can then register tokens under them.
 * Each municipality can have multiple projects registered under it and each project can have multiple tokens registered under it.
 * When a project is created, we create a new token account for that project and deploy the code to that account.
 */

// Main contract structure to store all information
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    // Registry contract owner
    pub owner_id: AccountId,

    // MunicipalitiesObjectIds
    pub municipalities: UnorderedSet<String>,

    // MunicipalityObjectId => ProjectObjectId
    pub municipality_to_projects: LookupMap<String, UnorderedSet<String>>,

    // ProjectId => Set of accountIds for each token
    pub project_to_tokens: LookupMap<String, UnorderedSet<AccountId>>,

    // TokenVersions
    pub token_versions: UnorderedSet<String>,

    // TokenVersion => hash
    pub token_version_to_code: LookupMap<String, LazyOption<Vec<u8>>>,
}

#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKey {
    Municipalities,
    MunicipalityToProjects,
    MunicipalityToProjectsInner,
    ProjectToTokens,
    ProjectToTokensInner,
    TokensAsVector,
    ProjectsAsVector,

    TokenVersions,
    TokenVersionToCode,
    TokenVersionToCodeInner
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
            municipalities: UnorderedSet::new(StorageKey::Municipalities.try_to_vec().unwrap()),
            municipality_to_projects: LookupMap::new(StorageKey::MunicipalityToProjects.try_to_vec().unwrap()),
            project_to_tokens: LookupMap::new(StorageKey::ProjectToTokens.try_to_vec().unwrap()),
            token_versions: UnorderedSet::new(StorageKey::TokenVersions.try_to_vec().unwrap()),
            token_version_to_code: LookupMap::new(StorageKey::TokenVersionToCode.try_to_vec().unwrap())
        };

        // Return the Contract object
        this
    }
}

#[cfg(test)]
mod tests;