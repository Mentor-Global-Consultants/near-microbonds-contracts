# Microbonds User Registry

The goal behind the Microbonds project is to allow projects to be created under governing municipalities, and have funds raised for their projects just like traditional bonds, but unlike the traditional way, have the bonds represented as a Non-Fungible Token. As there are many different parts to making this work under different legislations, the project has a few moving parts. We will cover them in detail in their individual code bases.

## Introduction

This contract is responsible for keeping track of users that have registered under a municipality. 

## Table of Contents

1. [Adding Users](#adding-users)

## Adding Users

Let's go through the contract and what a typical use-case would look like for each functionality.

In our use-case, we want to have a user sign up to the platform and then register to a municipality. Some municipalities have different requirements for 
registration, so we will have to check that the user meets the requirements before registering them. If they are approved, we will add them with the following function:

```rs
pub fn add_user_to_municipality(
    &mut self, 
    municipality_id: String, 
    user_id: String
)
```
Using this function, we can provide any unique identifier for the municipality and the user that will be broadcasted with the following event:

```rs
pub struct AddUserLog {
    pub user_id: String,
    pub municipality_id: String,
    pub memo: Option<String>,
}
```

## Checking User Registration

We can now see if a user is registered to a municipality by using the following function:

```rs
pub fn is_user_in_municipality(
    &self, 
    municipality_id: String, 
    user_id: String
) -> bool
```

We can also get all the users who are registered under a municipality by using the following function:

```rs
pub fn get_users_for_municipality(
    &self, 
    municipality_id: String,
    from_index: Option<U128>,
    limit: Option<u64>,
) -> Vec<String>
```

# How to build and run tests

You can use either yarn or npm for the following commands:

### Install dependencies
```bash
npm install
```

### Build contracts 
```bash
npm run build
```

### Run all tests
```bash
npm run test
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