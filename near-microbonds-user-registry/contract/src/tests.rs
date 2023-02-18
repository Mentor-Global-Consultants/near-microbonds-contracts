/* unit tests */
#[cfg(test)]
use crate::Contract;
use near_sdk::json_types::{U128, U64};
use near_sdk::test_utils::{accounts, VMContextBuilder};
use near_sdk::testing_env;
use near_sdk::{env, AccountId};

fn get_context(predecessor: AccountId) -> VMContextBuilder {
    let mut builder = VMContextBuilder::new();
    builder.predecessor_account_id(predecessor);
    builder
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
    let contract = Contract::new(accounts(0));
    assert_eq!(contract.owner_id, accounts(0));
}

#[test]
fn test_add_user_to_municipality() {
    let context = get_context(accounts(0));
    testing_env!(context.build());
    let mut contract = Contract::new(accounts(0));
    contract.add_user_to_municipality("municipality1".to_string(), "user1".to_string());
    assert_eq!(contract.municipality_to_users.get(&"municipality1".to_string()).unwrap().contains(&"user1".to_string()), true);
}

#[test]
#[should_panic(expected = "Caller not owner")]
fn test_add_user_to_municipality_not_owner() {
    let context = get_context(accounts(1));
    testing_env!(context.build());
    let mut contract = Contract::new(accounts(0));
    contract.add_user_to_municipality("municipality1".to_string(), "user1".to_string());
}