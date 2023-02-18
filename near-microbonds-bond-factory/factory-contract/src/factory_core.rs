use std::ops::Mul;

use crate::*;
use near_contract_standards::non_fungible_token::metadata::NFTContractMetadata;
use near_sdk::{Promise, env::attached_deposit, Gas, json_types::Base64VecU8, PromiseResult};

#[derive(Serialize)]
#[serde(crate = "near_sdk::serde")]
struct TokenInitArgs {
    owner_id: AccountId,
    metadata: NFTContractMetadata,
}

#[near_bindgen]
impl Contract {
    /**
     * Adds a new municipality account - caller has to be contract owner
     */
    pub fn add_new_municipality(
        &mut self,
        municipality_id: String,
        memo: Option<String>,
    ) {
        // Make sure called is owner
        self.assert_owner();

        // Insert the new municipality into the UnorderedSet and make sure it doesn't already exist
        assert!(
            self.municipalities.insert(&municipality_id),
            "Municipality already exists"
        );

        // Insert the new municipality and make sure it doesn't already exist
        assert!(
            self.municipality_to_projects.insert(
                &municipality_id, 
                &UnorderedSet::new(StorageKey::MunicipalityToProjectsInner.try_to_vec().unwrap())
            ).is_none(),
            "Municipality already exists"
        );

        // contruct the add municipality log
        let add_municipality_log: EventLog = EventLog { 
            version: "1.0.0".to_string(), 
            event: EventLogVariant::AddMunicipality(vec![AddMunicipalityLog {
                // Owner of the token
                municipality_id: municipality_id,

                // An optional memo to include
                memo: memo,
            }])
        };

        // log the serialized json
        env::log_str(&add_municipality_log.to_string());
    }

    /**
     * Adds a new project under an existing municipality - caller has to be contract owner
     */
    pub fn add_new_project(
        &mut self,
        municipality_id: String,
        project_id: String,
        memo: Option<String>,
    ) {
        // Make sure the called is the owner
        self.assert_owner();

        // Make sure the municipality exists
        assert!(
            self.municipality_to_projects.contains_key(&municipality_id),
            "Municipality does not exist"
        );

        // Add project to municipality_to_projects and make sure it doesn't already exist
        let mut projects = self.municipality_to_projects.get(&municipality_id).unwrap();
        assert!(
            projects.insert(&project_id),
            "Project already exists in mun"
        );

        // replace with the new set
        self.municipality_to_projects.insert(&municipality_id, &projects);

        // Add project to project_to_tokens and make sure it doesn't already exist
        assert!(
            self.project_to_tokens.insert(
                &project_id,
                &UnorderedSet::new(StorageKey::ProjectToTokensInner.try_to_vec().unwrap())
            ).is_none(),
            "Project already exists"
        );

        // contruct the add project log
        let add_project_log: EventLog = EventLog { 
            version: "1.0.0".to_string(), 
            event: EventLogVariant::AddProject(vec![AddProjectLog {
                // Municipality Id
                municipality_id: municipality_id,

                // Project Id
                project_id: project_id,

                // An optional memo to include
                memo: memo,
            }])
        };

        // log the serialized json
        env::log_str(&add_project_log.to_string());
    }

    /**
     * Adds a new token for a project (it has to be a registered project)
     */
    #[payable]
    pub fn add_new_token_for_project(
        &mut self,
        municipality_id: String,
        project_id: String,
        token_version: String,
        token_account_name: String,
        token_name: String,
        token_symbol: String,
        token_icon: Option<String>,
        token_base_uri: Option<String>,
        token_reference: Option<String>,
        token_reference_hash: Option<Base64VecU8>,
        memo: Option<String>,
    ) -> Promise {
        // Make sure the caller is owner
        self.assert_owner();

        // Make sure the municipality exists
        assert!(
            self.municipality_to_projects.contains_key(&municipality_id),
            "Municipality does not exist"
        );

        // Make sure the project exists
        assert!(
            !self.municipality_to_projects.get(&municipality_id).unwrap().is_empty(),
            "Project does not exist"
        );

        // Get the code for the token version if it exists
        let code = self.token_version_to_code.get(&token_version).expect("Token version does not exist").get().unwrap();
        
        let contract_bytes = code.len() as u128;
        let min_needed = env::STORAGE_PRICE_PER_BYTE.mul(contract_bytes);
        
        // Assert that the attached deposit is greater than the cost of deployment
        assert!(attached_deposit().ge(&min_needed), "Attach at least {min_needed} yoctoNEAR to deploy the contract");

        // Create new near account id
        let current_account_id = env::current_account_id();
        let new_token_account_id: AccountId = format!("{token_account_name}.{current_account_id}").parse().unwrap();
        assert!(
            env::is_valid_account_id(new_token_account_id.as_bytes()),
            "Subaccount ID is invalid"
        );

        // Get args
        let init_args = near_sdk::serde_json::to_vec(&TokenInitArgs {
            owner_id: env::signer_account_id(),
            metadata: NFTContractMetadata {
                spec: ("nft-".to_owned() + &token_version.clone()).to_string(),
                name: token_name, 
                symbol: token_symbol, 
                icon: token_icon, 
                base_uri: token_base_uri, 
                reference: token_reference,
                reference_hash: token_reference_hash,
            }
        }).unwrap();


        // Create the account
        Promise::new(new_token_account_id.clone())
            .create_account()
            .transfer(attached_deposit())
            .deploy_contract(code)
            .function_call("new".to_owned(), init_args, 0, Gas(10u64.pow(12)))
            .then(
                Self::ext(env::current_account_id())
                .resolve_deploy(municipality_id, project_id, new_token_account_id.to_string(), memo)
            )
    }

    #[private]
    pub fn resolve_deploy(
        &mut self,
        municipality_id: String,
        project_id: String,
        new_token_account_id: String,
        memo: Option<String>
    ) {
        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Failed => env::panic_str("Failed to deploy token contract"),
            PromiseResult::Successful(_) => {
                // Add token for project and make sure it doesn't already exist
                let mut tokens = self.project_to_tokens.get(&project_id).unwrap();
                assert!(
                    tokens.insert(
                        &AccountId::new_unchecked(new_token_account_id.clone())
                    ),
                    "Token already added for project"
                );

                // Replace the tokens with the new set
                self.project_to_tokens.insert(&project_id, &tokens);

                // contruct the add project log
                let add_token_log: EventLog = EventLog { 
                    version: "1.0.0".to_string(), 
                    event: EventLogVariant::AddProjectToken(vec![AddProjectTokenLog {
                        // Municipality Id
                        municipality_id: municipality_id,

                        // Project Id
                        project_id: project_id,

                        // Token Id
                        token_id: new_token_account_id.clone(),

                        // An optional memo to include
                        memo: memo,
                    }])
                };

                // log the serialized json
                env::log_str(&add_token_log.to_string());
            }
        }
    }
}
