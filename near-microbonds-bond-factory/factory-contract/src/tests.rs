/* unit tests */
#[cfg(test)]
use crate::Contract;
use crate::{JsonProject, JsonMunicipality};
use near_sdk::json_types::{U128, U64};
use near_sdk::test_utils::{accounts, VMContextBuilder};
use near_sdk::testing_env;
use near_sdk::{env, AccountId};

fn get_context(predecessor: AccountId) -> VMContextBuilder {
    let mut builder = VMContextBuilder::new();
    builder.predecessor_account_id(predecessor);
    builder
}

fn sample_project_data() -> JsonProject {
    JsonProject {
        project_id: "test_project_1_a".to_string(),
        tokens_account_ids: [accounts(3).to_string()].to_vec()
    }
}

fn sample_municipality_data() -> JsonMunicipality {
    JsonMunicipality {
        municipality_id: "test_municipality_1".to_string(),
        projects: ["test_project_1_a".to_string()].to_vec()
    }
}

#[test]
#[should_panic(expected = "The contract is not initialized")]
fn test_default() {
    let context = get_context(accounts(1));
    testing_env!(context.build());
    let _contract = Contract::default();
}

#[test]
fn test_initialization() {
    let mut context = get_context(accounts(1));
    testing_env!(context.build());
    
    let contract = Contract::new(accounts(1).into());
    testing_env!(context.is_view(true).build());

    let owner = contract.owner();
    assert_eq!(owner, accounts(1));
}

#[test]
fn test_add_municipality() {
    // Get context
    let context = get_context(accounts(0));
    testing_env!(context.build());

    // Get contract
    let mut contract = Contract::new(accounts(0).into());
    
    let municipality_data = sample_municipality_data();

    contract.add_new_municipality(municipality_data.municipality_id.clone(), Some("This is a test memo".to_string()));

    let projects_for_municipality = contract.view_projects_for_municipality(municipality_data.municipality_id.clone(), None, None);

    assert_eq!(projects_for_municipality.len(), 0, "Projects length expected to be zero");
}

#[test]
#[should_panic(expected = "Caller not owner")]
fn test_add_municipality_non_owner() {
    // Get context
    let mut context = get_context(accounts(0));
    testing_env!(context.predecessor_account_id(accounts(1)).build());

    // Get contract
    let mut contract = Contract::new(accounts(0).into());
    
    let municipality_data = sample_municipality_data();
    contract.add_new_municipality(municipality_data.municipality_id.clone(), Some("This is a test memo".to_string()));
}

#[test]
fn test_add_project() {
    // Get context
    let mut context = get_context(accounts(0));
    testing_env!(context.predecessor_account_id(accounts(0)).build());

    // Get contract
    let mut contract = Contract::new(accounts(0).into());
    
    let municipality_data = sample_municipality_data();
    let project_data = sample_project_data();

    contract.add_new_municipality(municipality_data.municipality_id.clone(), Some("This is a test memo".to_string()));
    contract.add_new_project(
        municipality_data.municipality_id.clone(),
        project_data.project_id.clone(),
        Some("This is a test memo".to_string())
    );

    let projects_for_municipality = contract.view_projects_for_municipality(municipality_data.municipality_id.clone(), None, None);
    let tokens_for_projects = contract.view_tokens_for_project(project_data.project_id.clone(), None, None);

    // println!("{:?}", tokens_for_projects); // print tokens for projects (debug)
    // println!("{:?}", projects_for_municipality); // print tokens for projects (debug)

    assert_eq!(projects_for_municipality[0], project_data.project_id.clone(), "Project ID does not match expected ID in projects_for_municipalities");

    assert_eq!(tokens_for_projects.len(), 0, "Token account ids expected to be of length zero");  
}

#[test]
#[should_panic(expected = "Caller not owner")]
fn test_add_project_non_owner() {
    // Get context
    let mut context = get_context(accounts(0));
    testing_env!(context.predecessor_account_id(accounts(1)).build());

    // Get contract
    let mut contract = Contract::new(accounts(0));
    
    let municipality_data = sample_municipality_data();
    let project_data = sample_project_data();

    // contract.add_new_municipality(municipality_data.municipality_id.clone(), Some("This is a test memo".to_string()));
    contract.add_new_project(
        municipality_data.municipality_id.clone(),
        project_data.project_id.clone(),
        Some("This is a test memo".to_string())
    );
}