use near_sdk::ext_contract;

use crate::*;

pub type TokenId = String;

/// external contract calls

#[ext_contract(token_contract)]
trait TokenContract {
    fn nft_transfer(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        // we introduce an approval ID so that people with that approval ID can transfer the token
        approval_id: Option<u64>,
        memo: Option<String>,
    );
}