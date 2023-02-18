/* unit tests */
#[cfg(test)]
use crate::Contract;
use crate::{JsonTokens};
use near_sdk::json_types::{U128, U64};
use near_sdk::test_utils::{accounts, VMContextBuilder};
use near_sdk::testing_env;
use near_sdk::{env, AccountId};

/**
    THIS_CONTRACT: accounts(0);
    OWNER_ID: accounts(1);
    USER_REGISTRY_CONTRACT_ID: accounts(2);
    TOKEN_ACCOUNT_ID: accounts(3);
    TOKEN_ID: "1";
    TOKEN_ID_2: "2";
    TOKEN_OWNER: "token_owner";
*/

fn get_context(predecessor: AccountId) -> VMContextBuilder {
    let mut builder = VMContextBuilder::new();
    builder.predecessor_account_id(predecessor);
    builder
}

fn sample_json_tokens_data() -> JsonTokens {
    JsonTokens {
        owner_id: "token_owner_1".to_string(),
        tokens: vec![accounts(3).to_string() + &":1".to_string(), accounts(3).to_string() + &":2".to_string()],
    }
}

#[test]
#[should_panic(expected = "The contract is not initialized")]
fn test_default() {
    let context = get_context(accounts(0));
    testing_env!(context.build());
    let _contract = Contract::default();
}

#[test]
fn test_initialization() {
    let context = get_context(accounts(0));
    testing_env!(context.build());
    let _contract = Contract::new(accounts(1));
}

#[test]
fn test_tokens_for_owner() {
    let context = get_context(accounts(1));
    testing_env!(context.build());
    let mut contract = Contract::new(accounts(1));
    contract.add_new_token_for_owner(
        "token_owner_1".to_string(),
        accounts(3),
        "1".to_string(),
        None,
    );
    contract.add_new_token_for_owner(
        "token_owner_1".to_string(),
        accounts(3),
        "2".to_string(),
        None,
    );
    let tokens = contract.tokens_for_owner("token_owner_1".to_string(), None, None);
    
    // Check that the tokens length is the same as sample_json_tokens_data
    assert_eq!(tokens.len(), sample_json_tokens_data().tokens.len());
}

#[test]
fn test_tokens_for_owner_with_limit() {
    let context = get_context(accounts(1));
    testing_env!(context.build());
    let mut contract = Contract::new(accounts(1));
    contract.add_new_token_for_owner(
        "token_owner_1".to_string(),
        accounts(3),
        "1".to_string(),
        None,
    );
    contract.add_new_token_for_owner(
        "token_owner_1".to_string(),
        accounts(3),
        "2".to_string(),
        None,
    );
    let tokens = contract.tokens_for_owner("token_owner_1".to_string(), None, Some(U64(1).0));
    
    // Check that the tokens length is 1
    assert_eq!(tokens.len(), 1);

    // Check that the token is the same as the first token in sample_json_tokens_data
    assert_eq!(tokens[0], sample_json_tokens_data().tokens[0]);
}

#[test]
fn test_tokens_for_owner_with_limit_and_offset() {
    let context = get_context(accounts(1));
    testing_env!(context.build());
    let mut contract = Contract::new(accounts(1));
    contract.add_new_token_for_owner(
        "token_owner_1".to_string(),
        accounts(3),
        "1".to_string(),
        None,
    );
    contract.add_new_token_for_owner(
        "token_owner_1".to_string(),
        accounts(3),
        "2".to_string(),
        None,
    );
    let tokens = contract.tokens_for_owner("token_owner_1".to_string(), Some(U128(1)), None);
    
    // Check that the tokens length is 1
    assert_eq!(tokens.len(), 1);

    // Check that the token is the same as the second token in sample_json_tokens_data
    assert_eq!(tokens[0], sample_json_tokens_data().tokens[1]);
}

#[test]
#[should_panic(expected = "Caller not owner")]
fn test_add_new_token_for_owner() {
    let context = get_context(accounts(5));
    testing_env!(context.build());
    let mut contract = Contract::new(accounts(1));
    contract.add_new_token_for_owner(
        "token_owner_1".to_string(),
        accounts(3),
        "1".to_string(),
        None,
    );
}

#[test]
fn test_add_account_to_user() {
    let context = get_context(accounts(0));
    testing_env!(context.build());
    let mut contract = Contract::new(accounts(0));
    contract.link_account_to_user("user1".to_string(), accounts(1));
    assert_eq!(contract.user_to_account.get(&"user1".to_string()).unwrap(), accounts(1));
}

#[test]
fn test_change_account_for_user() {
    let context = get_context(accounts(0));
    testing_env!(context.build());
    let mut contract = Contract::new(accounts(0));
    contract.link_account_to_user("user1".to_string(), accounts(1));
    contract.link_account_to_user("user1".to_string(), accounts(2));
    assert_eq!(contract.user_to_account.get(&"user1".to_string()).unwrap(), accounts(2));
}

#[test]
#[should_panic(expected = "Caller not owner")]
fn test_add_account_to_user_not_owner() {
    let context = get_context(accounts(1));
    testing_env!(context.build());
    let mut contract = Contract::new(accounts(0));
    contract.link_account_to_user("user1".to_string(), accounts(1));
}