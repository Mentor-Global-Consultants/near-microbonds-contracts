use crate::*;

#[near_bindgen]
impl Contract {
    /// Adds an accountId to the user_to_account lookup map
    /// Arguments:
    /// * `user_id`: the user id of the user being added
    /// * `account_id`: the account id of the user being added
    pub fn link_account_to_user(
        &mut self, 
        user_id: String, 
        account_id: AccountId
    ) {
        // Assert that the caller is the owner
        self.assert_owner();

        // Fetches the account id for the given user id
        let account = self.user_to_account.get(&user_id);

        // If the account is None, insert the new account id and log the event with AddAccountLog
        // If the account is Some, assert that the account id is the same as the given account id
        // If  the account is Some and the account id is not the same as the given account id, log the event with ChangeAccountLog
        match account {
            None => {
                self.user_to_account.insert(&user_id, &account_id);
                env::log_str(&EventLog {
                    version: "1.0.0".to_string(),
                    event: EventLogVariant::LinkAccount(vec![LinkAccountLog {
                        user_id: user_id.clone(),
                        account_id: account_id.to_string(),
                        memo: None,
                    }]),
                }.to_string());
            },
            Some(existing_account_id) => {
                // If the account id is the same as the given account id, do nothing
                if existing_account_id == account_id {
                    return;
                } else {
                    // add the new account id and log the event with ChangeAccountLog
                    self.user_to_account.insert(&user_id, &account_id);
                    env::log_str(&EventLog {
                        version: "1.0.0".to_string(),
                        event: EventLogVariant::ChangeAccount(vec![ChangeAccountLog {
                            user_id: user_id.clone(),
                            old_account_id: existing_account_id.to_string(),
                            new_account_id: account_id.to_string(),
                            memo: None,
                        }]),
                    }.to_string());
                }
            }
        }
    }
}