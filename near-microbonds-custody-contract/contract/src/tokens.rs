use near_sdk::{Promise};

use crate::*;

#[near_bindgen]
impl Contract {

    /**
     * Adds a new token to an owner
     */
    pub fn add_new_token_for_owner(
        &mut self,
        owner_id: String,
        token_account_id: AccountId,
        token_id: String,
        memo: Option<String>
    ) {
        // Make sure the caller is the contract owner
        self.assert_owner();

        // Create a new string which stores token_account_id:token_id
        let token_info = token_account_id.to_string().clone() + DELIMITER + token_id.as_str();

        // Check if the owner already exists
        if !self.tokens_per_owner.contains_key(&owner_id) { // owner has already been initialized
            self.tokens_per_owner.insert(
                &owner_id,
                &UnorderedSet::new(StorageKey::TokensPerOwnerInner.try_to_vec().unwrap())
            );
        }

        // Get the unordered set of token strings
        let mut tokens = self.tokens_per_owner.get(&owner_id).unwrap();

        // Add the new token_info string to the set and make sure it doesn't already exist
        assert!(
            tokens.insert(&token_info),
            "Token info already exists"
        );

        // Save the new tokens set to the owner
        self.tokens_per_owner.insert(&owner_id, &tokens);

        // construct add token log
        let add_token_log: EventLog = EventLog { 
            version: "1.0.0".to_string(), 
            event: EventLogVariant::AddToken(vec![AddTokenLog { 
                owner_id, 
                token_account_id: token_account_id.to_string(), 
                token_id, 
                memo
            }])
        };

        env::log_str(&add_token_log.to_string());
        
    }

    /**
     * Sends an existing token to the owner
     */
    pub fn send_token_to_owner(
        &self,
        owner_id: String,
        token_account_id: AccountId,
        token_id: String,
        transfer_memo: Option<String>,
        resolve_memo: Option<String>,
    ) -> Promise {

        // Get the linked account for user, if it doesn't exist assert
        let account_id = self.user_to_account.get(&owner_id).expect("No account linked to user");
        
        // Assert whether the accountId is the same as the caller
        assert!(
            account_id == env::signer_account_id(),
            "Caller is not the owner of the account"
        );

        // Get the unordered set of token strings
        let tokens = self.tokens_per_owner.get(&owner_id).unwrap();

        // The provided token info formatted as stored
        let token_info = token_account_id.to_string().clone() + DELIMITER + token_id.as_str();

        // Make sure the owner owns the provided token
        assert!(
            tokens.contains(&token_info),
            "Owner does not own provided token"
        );

        // Make the cross-contract call to the token contract to transfer the token to the owner
        token_contract::ext(token_account_id.clone())
        // attach 1 yoctoNEAR with status GAS equal to the GAS for nft transfer.
        .with_attached_deposit(1)
        .with_static_gas(GAS_FOR_NFT_TRANSFER)
        .nft_transfer(
            env::signer_account_id(), 
            token_id.clone(), 
            None, 
            transfer_memo
        )
        .then(
            Self::ext(env::current_account_id())
            .with_static_gas(GAS_FOR_RESOLVE_TRANSFER)
            .resolve_transfer(owner_id, token_account_id.clone(), token_id.clone(), resolve_memo)
        )
    }

    /**
     * Removes the token from the registry
     */
    #[private]
    pub fn resolve_transfer(
        &mut self,
        owner_id: String,
        token_account_id: AccountId,
        token_id: String,
        memo: Option<String>
    ) {        
        // Get the unordered set of token strings
        let mut tokens = self.tokens_per_owner.get(&owner_id).unwrap();

        // The provided token info formatted as stored
        let token_info = token_account_id.to_string().clone() + DELIMITER + token_id.as_str();

        assert!(
            tokens.remove(&token_info),
            "The token was not found in the owners token set"
        );

        // Store the new set
        self.tokens_per_owner.insert(&owner_id, &tokens);

        // construct add token log
        let send_token_log: EventLog = EventLog { 
            version: "1.0.0".to_string(), 
            event: EventLogVariant::SendToken(vec![SendTokenLog { 
                owner_id, 
                token_account_id: token_account_id.to_string(), 
                token_id, 
                memo
            }])
        };

        env::log_str(&send_token_log.to_string());
    }
}