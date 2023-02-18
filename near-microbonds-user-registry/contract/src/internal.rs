use crate::*;

impl Contract {
    // Private function to assert if the called is the owner
    pub(crate) fn assert_owner(&self) {
        assert_eq!(
            env::predecessor_account_id(),
            self.owner_id,
            "Caller not owner"
        );
    }
}