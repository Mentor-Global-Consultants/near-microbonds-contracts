use crate::*;

impl Contract {
    pub(crate) fn assert_owner(&self) {
        assert!(
            self.owner_id == env::predecessor_account_id(),
            "Caller not owner"
        );
    }
}
