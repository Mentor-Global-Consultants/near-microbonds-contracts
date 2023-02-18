use std::fmt;

use near_sdk::{serde::{Deserialize, Serialize}, serde_json};

/// Enum that represents the data type of the EventLog.
/// The enum can either be an NftMint or an NftTransfer.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "event", content = "data")]
#[serde(rename_all = "snake_case")]
#[serde(crate = "near_sdk::serde")]
#[non_exhaustive]
pub enum EventLogVariant {
    AddToken(Vec<AddTokenLog>),
    SendToken(Vec<SendTokenLog>),
    LinkAccount(Vec<LinkAccountLog>),
    ChangeAccount(Vec<ChangeAccountLog>),
}

/// Interface to capture data about an event
///
/// Arguments:
/// * `standard`: name of standard e.g. nep171
/// * `version`: e.g. 1.0.0
/// * `event`: associate event data
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct EventLog {
    pub version: String,

    // `flatten` to not have "event": {<EventLogVariant>} in the JSON, just have the contents of {<EventLogVariant>}.
    #[serde(flatten)]
    pub event: EventLogVariant,
}

impl fmt::Display for EventLog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "EVENT_JSON:{}",
            &serde_json::to_string(self).map_err(|_| fmt::Error)?
        ))
    }
}

/// An event log to capture adding a token
///
/// Arguments
/// * `owner_id`: id of the owner
/// * `token_account_id`: account id of the token
/// * `token_id`: token id of the token
/// * `memo`: optional message
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct AddTokenLog {
    pub owner_id: String,
    pub token_account_id: String,
    pub token_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
}

/// An event log to capture sending a token to its owner
///
/// Arguments
/// * `owner_id`: id of the owner
/// * `token_account_id`: account id of the token
/// * `token_id`: token id of the token
/// * `memo`: optional message
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct SendTokenLog {
    pub owner_id: String,
    pub token_account_id: String,
    pub token_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
}

/// An event log to capture adding an account to a user
/// 
/// Arguments:
/// * `user_id`: the user id of the user being added
/// * `account_id`: the account id of the user being added
/// * `memo` (optional): a memo to add to the event log
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct LinkAccountLog {
    pub user_id: String,
    pub account_id: String,
    pub memo: Option<String>,
}

/// An event log to capture changing an account for a user
/// 
/// Arguments:
/// * `user_id`: the user id of the user being added
/// * `old_account_id`: the old account id of the user being added
/// * `new_account_id`: the new account id of the user being added
/// * `memo` (optional): a memo to add to the event log
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct ChangeAccountLog {
    pub user_id: String,
    pub old_account_id: String,
    pub new_account_id: String,
    pub memo: Option<String>,
}