use crate::*;

#[near_bindgen]
impl Contract {
    /// Adds a new user to the municipality_to_users lookup map
    pub fn add_user_to_municipality(
        &mut self, 
        municipality_id: String, 
        user_id: String
    ) {
        // Assert that the caller is the owner
        self.assert_owner();

        // Get the UnorderedSet of users for the given municipality
        let mut users = self.municipality_to_users.get(&municipality_id).unwrap_or_else(|| {
            // If the UnorderedSet doesn't exist, create a new one
            UnorderedSet::new(StorageKey::MunicipalityToUsersInner.try_to_vec().unwrap())
        });

        // Add the user to the UnorderedSet if it doesn't already exist
        // Uses assert to check
        assert!(
            users.insert(&user_id),
            "User already exists in the given municipality"
        );

        // Save the UnorderedSet back to the lookup map
        self.municipality_to_users.insert(&municipality_id, &users);

        // Log the event
        env::log_str(&EventLog {
            version: "1.0.0".to_string(),
            event: EventLogVariant::AddUser(vec![AddUserLog {
                user_id,
                municipality_id,
                memo: None,
            }]),
        }.to_string());
    }
}