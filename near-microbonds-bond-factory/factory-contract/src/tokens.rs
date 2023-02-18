use near_sdk::{log, Promise};
use crate::*;

#[near_bindgen]
impl Contract {
    // Add a new token version to the contract
    pub fn add_token_version(&mut self) {
        // Make sure the caller is the owner
        self.assert_owner();

        let code = env::input().expect("No input given");

        assert!(code.len() > 0, "No code given");

        // Token version
        let token_version: u64 = self.token_versions.len();

        // Read the code from the input. It is the rest of the input
        let code: LazyOption<Vec<u8>> = LazyOption::new(
            StorageKey::TokenVersionToCodeInner.try_to_vec().unwrap(), 
            Some(&code)
        );

        // Add the token version to the contract
        self.token_version_to_code.insert(
            &token_version.to_string(), 
            &code
        );

        // Add the token to the list of token versions
        self.token_versions.insert(&token_version.to_string());
    }
}