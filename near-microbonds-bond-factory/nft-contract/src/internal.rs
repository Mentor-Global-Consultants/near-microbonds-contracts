use crate::*;
use near_contract_standards::non_fungible_token::{hash_account_id};

//Assert that the user has attached at least 1 yoctoNEAR (for security reasons and to pay for storage)
pub(crate) fn assert_at_least_one_yocto() {
    assert!(
        env::attached_deposit() >= 1,
        "Requires attached deposit of at least 1 yoctoNEAR",
    )
}

pub(crate) fn royalty_to_payout(royalty_percentage: u32, amount_to_pay: Balance) -> U128 {
    U128(royalty_percentage as u128 * amount_to_pay / 10_000u128)
}

impl Contract {
    // add a token to the set of tokens an owner has
    pub(crate) fn internal_add_token_to_owner(
        &mut self,
        account_id: &AccountId,
        token_id: &TokenId,
    ) {
        // get the set of tokens for the given account
        let mut tokens_set = self.tokens_per_owner.get(account_id).unwrap_or_else(|| {
            UnorderedSet::new(
                StorageKey::TokenPerOwnerInner {
                    // we get a new unique prefix for the collection
                    account_id_hash: hash_account_id(&account_id),
                }
                .try_to_vec()
                .unwrap(),
            )
        });

        // we insert the token id into the set
        tokens_set.insert(token_id);

        // we insert that set for the given account id
        self.tokens_per_owner.insert(account_id, &tokens_set);
    }

    pub(crate) fn internal_remove_token_from_owner(
        &mut self,
        account_id: &AccountId,
        token_id: &TokenId
    ) {
        // we get the set of tokens that the owner has
        let mut tokens_set = self
                            .tokens_per_owner
                            .get(account_id)
                            // if there is no set of tokens for the owner, we panic
                            .expect("Token should be owner by the sender");
                            
        // we remove the token_id from the set of tokens
        tokens_set.remove(token_id);          

        // if the token set is now empty, we remove the owner from the tokens_per_owner collection
        if tokens_set.is_empty() {
            self.tokens_per_owner.remove(account_id);
        } else {
            // if the token set is not empty, we simply insert it back for the account ID
            self.tokens_per_owner.insert(account_id, &tokens_set);
        }
    }

    pub(crate) fn internal_transfer(
        &mut self,
        sender_id: &AccountId,
        receiver_id: &AccountId,
        token_id: &TokenId,
        // we introduce an approval ID so that people with that approval ID can transfer the token
        approval_id: Option<u64>,
        memo: Option<String>
    ) -> Token {
        // get the token object by passing in the token_id
        let token = self.tokens_by_id.get(token_id).expect("No token");

        // if the sender doesn't equal the owner, we panic
        if sender_id != &token.owner_id {
            // if the token's approved account IDs doesn't contain the sender, we panic
            if !token.approved_account_ids.contains_key(sender_id) {
                env::panic_str("Unauthorized");
            }
        }

        // If they included an approval_id, check if the sender's actual approval_id is the same as the one included
        if let Some(enforced_approval_id) = approval_id {
            // get the actual approval ID
            let actual_approval_id = token
                    .approved_account_ids
                    .get(sender_id)
                    // if the sender isn't in the map, we panic
                    .expect("Sender is not approved account");

            // make sure that the actual approval ID is the same as the one provided
            assert_eq!(
                actual_approval_id, &enforced_approval_id,
                "The actual approval_id {} is different from the gicen approval_id {}",
                actual_approval_id, enforced_approval_id
            );
        };

        // we make sure that the sender isn't sending the token to themselves
        assert_ne!(
            &token.owner_id,
            receiver_id,
            "The token owner and receiver should be different"
        );

        // we remove the token from its current owner's set
        self.internal_remove_token_from_owner(&token.owner_id, token_id);
        // we then add the token to the receiver_id's set
        self.internal_add_token_to_owner(receiver_id, token_id);

        // we create a new token struct
        let new_token = Token {
            owner_id: receiver_id.clone(),
            // reset the approval account IDs
            approved_account_ids: Default::default(),
            next_approval_id: token.next_approval_id,
            // we copy over the royalties from the previous token
            royalty: token.royalty.clone()
        };

        // insert that new token into the tokens_by_id, replacing the old entry
        self.tokens_by_id.insert(token_id, &new_token);

        // if there was some memo attached, we log it
        if let Some(memo) = memo.as_ref() {
            env::log_str(&format!("Memo: {}", memo).to_string());
        }

        // Default the authorized ID to be None for the logs.
        let mut authorized_id = None;
        // if the approval ID was provided, set the authorized ID equal to the sender
        if approval_id.is_some() {
            authorized_id = Some(sender_id.to_string());
        }

        // construct the transfer log as per the events standard
        let nft_transfer_log: EventLog = EventLog { 
            standard: NFT_STANDARD_NAME.to_string(), 
            version: NFT_METADATA_SPEC.to_string(), 
            event: EventLogVariant::NftTransfer(vec![NftTransferLog {
                // the optional authorized account ID to transfer the token on behalf of the old owner
                authorized_id,
                // the old owner's account ID
                old_owner_id: token.owner_id.to_string(),
                // the account ID of the new owner of the token
                new_owner_id: receiver_id.to_string(),
                // a vector containing the token IDs as strings
                token_ids: vec![token_id.to_string()],
                // an optional memo to include
                memo
            }]),
        };

        // log the serialized json
        env::log_str(&nft_transfer_log.to_string());

        // return the previous token object that was transferred
        token
    }
}