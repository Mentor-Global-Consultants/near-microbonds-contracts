use near_sdk::collections::Vector;

use crate::*;

#[near_bindgen]
impl Contract {

    /// Gets the list of users for the given municipality
    pub fn get_users_for_municipality(
        &self, 
        municipality_id: String,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<String> {
        // Get the UnorderedSet of users for the given municipality, if it doesn't exist return an empty Vec
        let users_for_municipality_set = self.municipality_to_users.get(&municipality_id);

        let users = if let Some(users_for_municipality_set) = users_for_municipality_set {
            users_for_municipality_set
        } else {
            // Return an empty Vec
            return vec![];
        };

        let start = u128::from(from_index.unwrap_or(U128(0)));

        users.iter()
            .skip(start as usize)
            .take(limit.unwrap_or(50) as usize)
            .collect()
    }

    /// Returns if the user is in the given municipality
    pub fn is_user_in_municipality(&self, municipality_id: String, user_id: String) -> bool {
        
        // Get the UnorderedSet of users for the given municipality
        let users = self.municipality_to_users.get(&municipality_id);

        // Assert that the UnorderedSet exists
        if !users.is_some() {
            return false;
        }

        // Get the UnorderedSet from the Option
        let users = users.unwrap();

        // Check if the user is in the UnorderedSet
        users.contains(&user_id)
    }
}