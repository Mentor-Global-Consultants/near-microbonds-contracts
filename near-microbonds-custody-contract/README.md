# Microbonds Custody Contract

The goal behind the Microbonds project is to allow projects to be created under governing municipalities, and have funds raised for their projects just like traditional bonds, but unlike the traditional way, have the bonds represented as a Non-Fungible Token. As there are many different parts to making this work under different legislations, the project has a few moving parts. We will cover them in detail in their individual code bases.

## Introduction

This contract is responsible for storing all tokens held by users who have not bound their own wallets.

So in essence tokens will be minted here and a registry will be kept of these tokens.

Users can then bind a near account to their account on the user registry contract and withdraw their 
tokens to the bound account.

## Table of Contents

1. [Tokens](#tokens)
2. [User Accounts](#user-accounts)

## Tokens

Let's go through the contract and what a typical use-case would look like for each functionality. You can see the overall contract structure in the
table of contents.

In our use-case, we want to allow tokens to be stored for users who have not linked their own wallets. This is done using the following function:

```rs
pub fn add_new_token_for_owner(
    &mut self,
    owner_id: String,
    token_account_id: AccountId,
    token_id: String,
    memo: Option<String>
)
```

Using this function, we have stored a new token for the user and it will emit the following event:

```rs
pub struct AddTokenLog {
    pub owner_id: String,
    pub token_account_id: String,
    pub token_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
}
```

We can also view the tokens that any user has in the custody contract with the following function:

```rs
pub fn tokens_for_owner(
    &self,
    owner_id: String,
    from_index: Option<U128>,
    limit: Option<u64>,
) -> Vec<String>
```

If a user has linked a wallet to the contract, we can send the token to the user's wallet using the following function:

```rs
pub fn send_token_to_owner(
    &self,
    owner_id: String,
    token_account_id: AccountId,
    token_id: String,
    transfer_memo: Option<String>,
    resolve_memo: Option<String>,
) -> Promise
```

This function will emit the following event:

```rs
pub struct SendTokenLog {
    pub owner_id: String,
    pub token_account_id: String,
    pub token_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
}
```

## User Accounts

In our use-case, we want to allow users to bind their own wallets to their account on the user registry contract. This is done using the following function:

```rs
pub fn link_account_to_user(
    &mut self, 
    user_id: String, 
    account_id: AccountId
)
```

Depending on whether the user has already linked a wallet or not, this function will emit one of the following events:

```rs
pub struct LinkAccountLog {
    pub user_id: String,
    pub account_id: String,
    pub memo: Option<String>,
}

// OR

pub struct ChangeAccountLog {
    pub user_id: String,
    pub old_account_id: String,
    pub new_account_id: String,
    pub memo: Option<String>,
}
```

We can also view the connected account for a user at any point in time using the following function:

```rs
pub fn get_account_for_user(
    &self, 
    user_id: String
) -> Option<AccountId>
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