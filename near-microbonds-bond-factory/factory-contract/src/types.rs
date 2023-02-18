use crate::*;

/**
 * Both only used for testing currently
 */
#[derive(BorshDeserialize, BorshSerialize)]
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct JsonProject {
    pub project_id: String, // The owner of the project
    pub tokens_account_ids: Vec<String>, // The ids of the NFT tokens
}

#[derive(BorshDeserialize, BorshSerialize)]
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct JsonMunicipality {
    pub municipality_id: String, // The owner of the Municipality
    pub projects: Vec<String> // Vec containing ProjectIds
}