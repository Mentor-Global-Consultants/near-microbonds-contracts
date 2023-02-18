use crate::*;

// Currently only used for testing
#[derive(BorshDeserialize, BorshSerialize)]
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct JsonTokens {
    pub owner_id: String,
    pub tokens: Vec<String>, // List of `tokenAccountId:tokenId`
}