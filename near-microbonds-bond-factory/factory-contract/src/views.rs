use near_sdk::near_bindgen;

use crate::*;

#[near_bindgen]
impl Contract {

    // Get the token information for a specific project
    pub fn view_tokens_for_project(
        &self,
        project_id: String,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<String> {
        // Get the set of tokens that belong to the project_id
        let tokens_for_project_set = self.project_to_tokens.get(&project_id);

        // If there is some set of tokens, we'll set the tokens variable equal to that set, otherwise return empty vector
        let tokens = if let Some(tokens_for_project_set) = tokens_for_project_set {
            tokens_for_project_set
        } else {
            return vec![];
        };

        // Starting index
        let start = u128::from(from_index.unwrap_or(U128(0)));

        // iterate through token using iterator
        tokens.iter()
            .skip(start as usize)
            .take(limit.unwrap_or(50) as usize)
            .map(|token| token.to_string())
            .collect()
    }

    // Get the projects that belong to a municipality
    pub fn view_projects_for_municipality(
        &self,
        municipality_id: String,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<String> {
        // Get the projects that belong to the municipality
        let projects_for_municipality_set = self.municipality_to_projects.get(&municipality_id); // todo unwrap

        // If there is some set of projects, we'll set the projects variable equal to that set, otherwise return empty vector
        let projects = if let Some(projects_for_municipality_set) = projects_for_municipality_set {
            projects_for_municipality_set
        } else {
            return vec![];
        };

        // Starting index
        let start = u128::from(from_index.unwrap_or(U128(0)));

        // iterate through projects using iterator
        projects.iter()
            .skip(start as usize)
            .take(limit.unwrap_or(50) as usize)
            .collect()
    }

    // Get the municipalities stored in the contract
    pub fn view_municipalities(
        &self,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<String> {
        // Starting index
        let start = u128::from(from_index.unwrap_or(U128(0)));

        // iterate through municipalities using iterator
        self.municipalities.iter()
            .skip(start as usize)
            .take(limit.unwrap_or(50) as usize)
            .collect()
    }

    pub fn owner(
        &self
    ) -> AccountId {
        self.owner_id.clone()
    }

    /// Get the code for a token version
    /// Could fail if the token code is larger than the allowed view call size
    pub fn get_code_for_token_version(&self, token_version: &String) -> Vec<u8> {
        // Make sure the token version exists
        assert!(self.token_version_to_code.contains_key(&token_version), "Token version does not exist");

        // Get the code for the token version
        self.token_version_to_code.get(&token_version).unwrap().get().unwrap()
    }

    // Get the list of token versions
    pub fn get_token_versions(&self) -> Vec<String> {
        self.token_versions.to_vec()
    }

    // Get the storage cost for the deployment of a token version
    pub fn get_deployment_cost(&self, token_version: String) -> U128 {
        // Get the token version
        let token_version = self.token_version_to_code.get(&token_version).unwrap();

        // Calculate the cost
        let contract_bytes = token_version.get().unwrap().len() as u128;
        let cost = contract_bytes * env::STORAGE_PRICE_PER_BYTE;

        // Return the cost
        U128(cost)
    }
}