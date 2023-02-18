use near_sdk::collections::Vector;

use crate::*;

#[near_bindgen]
impl Contract {

    /// Get the tokens stored for given owner
    pub fn tokens_for_owner(
        &self,
        owner_id: String,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<String> {

        // Get the UnorderedSet of tokens that belong to an owner
        let tokens_for_owner_set = self.tokens_per_owner.get(&owner_id);

        // If tokens_for_owner_set is not empty, we store it in tokens, if empty we return None
        let tokens = if let Some(tokens_for_owner_set) = tokens_for_owner_set {
            tokens_for_owner_set
        } else {
            return vec![];
        };

        // Starting index
        let start = u128::from(from_index.unwrap_or(U128(0)));

        // iterate through each token using iterator
        tokens.iter()
            .skip(start as usize)
            .take(limit.unwrap_or(50) as usize)
            .collect()
    }
    

    /// Gets the account id for the given user id
    pub fn get_account_for_user(&self, user_id: String) -> Option<AccountId> {
        // Fetches the account id for the given user id
        self.user_to_account.get(&user_id)
    }
}
