# Microbonds Factory

The goal behind the Microbonds project is to allow projects to be created under governing municipalities, and have funds raised for their projects just like traditional bonds, but unlike the traditional way, have the bonds represented as a Non-Fungible Token. As there are many different parts to making this work under different legislations, the project has a few moving parts. We will cover them in detail in their individual code bases.

## Introduction

This contract is responsible for keeping track of municipalities, their projects and the projects' tokens.

The contract is also responsible for deploying these projects' tokens from a set of stored versions.

_Note: The token versions are not updatable, but you can store new versions._

## Table of Contents
1. [Municipalities](#municipalities)
2. [Projects](#projects)
3. [Tokens](#tokens)
4. [Token Versions](#token-versions)


## Municipalities

Let us go through the contract and what a typical use-case would look like for each functionality. You can see the overall flow of our use-case in the table of contents.


In our use-case, we want to have municipalities sign up to the platform and once they are approved, we want to be able to access which municipalities are approved on-chain. In order to do that we are able to create a municipality on-chain using the following function:

```rs
pub fn add_new_municipality(
    &mut self,
    municipality_id: String,
    memo: Option<String>,
)
```

Using this function, we can provide any unique identifier for the municipality we want to store, as well as an optional memo that will be broadcasted with the following event:

```rs
pub struct AddMunicipalityLog {
    pub municipality_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
}
```

## Projects

In our use-case, once a municipality is approved and they have been created on the contract, they will be able to create projects that will fall under their governance. 

Similar to above, we can use the following function:

```rs
pub fn add_new_project(
    &mut self,
    municipality_id: String,
    project_id: String,
    memo: Option<String>,
)
```

Using this function, we can provide any unique identifier for the project we want to store, as well as the governing municipality's id and an optional memo that will be broadcasted with the following event:

```rs
pub struct AddProjectLog {
    pub municipality_id: String,
    pub project_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
}
```

Once projects have been added under a municipality, we can query the projects for any municipality by calling the following view function:

```rs
pub fn view_projects_for_municipality(
    &self,
    municipality_id: String,
    from_index: Option<U128>,
    limit: Option<u64>,
) -> Vec<String>
```

By providing the municipality id, we can retrieve all the projects that are governed by the municipality. Since view calls are subject to a byte-limit, there is optional pagination included that can be used to retrieve the entire list of projects if they exceed the call limit.

## Tokens

Since the goal behind the creation of the contract is to issue bonds for each project, once the project has been created they can finally issue bonds in the form of a Non-Fungible Token.

We can do this by calling the following function:

```rs
#[payable]
pub fn add_new_token_for_project(
    &mut self,
    municipality_id: String,
    project_id: String,
    token_version: String,
    memo: Option<String>,
) -> Promise
```

By providing a municipality and project id, we can create a new token for the given project, governed under the given municipality. 

**_Both the municipality and project have to be valid for the creation to occur._**

As for the token version; the individual projects can decide which token version suits them best by providing the matching token version identifier as it is stored on the contract. All token versions will be unique, for instance royalty fees can be included on token 1, whereas with token 2 we can have some special event happen on mint. The token versions are completely customizable and it is up to the deployer to add more versions. More info on this [below](#token-versions).

**IMPORTANT**: A deposit must be attached, equal to the cost of storage for the token version that will be deployed with the call. This can also be explored [below](#token-versions).

Again, an optional memo can be attached which will be broadcasted with the following event:

```rs
pub struct AddProjectTokenLog {
    pub municipality_id: String,
    pub project_id: String,
    pub token_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
}
```

We can view all the tokens minted under a specific project by calling the following view function:

```rs
pub fn view_tokens_for_project(
    &self,
    project_id: String,
    from_index: Option<U128>,
    limit: Option<u64>,
) -> Vec<String>
```

Pagination is once again added, should the list exceed the byte-limit on view calls.

## Token Versions

We have decided to create a way for projects to choose from different token versions as some projects' needs are different from others. To add token versions, the following function can be called:

```rs
pub fn add_token_version(&mut self)
```

At first glance it seems like no input is passed to the function but with the near-sdk, arguments can also be passed as bytes using base64 encoding. This can be read using `env::input()`. This means we can technically pass information to the contract by passing some bytes through args. Something like this:

```ts
const code = fs.readFileSync('./test-contracts/nft.wasm', 'utf8');
const code_b64 = Buffer.from(code, 'base64');

await owner.call(
    factory_contract, // The contract that we are calling
    'add_token_version', // The method we are calling
    code_b64 // The args we are supplying - in this case the code encoded in base64.
);
```

For us to read it on contract-level we simply read the input like this:

```rs
let code = env::input().expect("No input given").to_vec();
```

With this implementation we can create new token versions as it becomes neccessary. The token versions have an auto-incrementing version number starting from zero (0). Meaning if you have one token stored, its id will be '0' and the next uploaded token version will be '1'.

We suggest storing the relevant metadata and descriptions for each token on the token itself.

To retrieve all the stored token versions, we can call the following view function:

```rs
pub fn get_token_versions(&self) -> Vec<String>
```

We can also retrieve the code for any stored version by providing the token_version id:

```rs
pub fn get_code_for_token_version(&self, token_version: &String) -> Vec<u8>
```

When creating tokens for a project, we'd like to know what the attached deposit must be for any given token version, so to get that before deployment the following function can be called:

```rs
pub fn get_deployment_cost(&self, token_version: String) -> U128
```


## How to build and run tests

You can use either yarn or npm for the following commands:

### Install packages

```
npm install
```

### Build contracts

```
npm build
```

### Run all tests
```
npm test
```

### Run unit tests only

```
npm run test:unit
```

### Run integration tests only in both TypeScript and Rust
```
npm run test:integration
```

### Run integration tests only in either TypeScript OR Rust
```
npm run test:integration:ts OR npm run test:integration:rs
```

Please check the package.json for all possible scripts.