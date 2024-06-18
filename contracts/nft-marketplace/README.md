---

# NFT Marketplace Smart Contract

This repository contains a smart contract for a Non-Fungible Token (NFT) marketplace on the Cosmos SDK using CosmWasm. The contract allows for fractional ownership and trading of NFTs within a decentralized application (dApp).

## Features

- **Fractional Ownership**: Allows NFTs to be owned by multiple users, each owning a fraction of the NFT.
- **Listing and Buying**: Users can list their NFTs for fractional ownership and buy shares of listed NFTs.
- **Cancellation**: Owners can cancel their listings and remove their NFTs from the marketplace.
- **Customizable Configurations**: Configurable owner addresses and other parameters via contract initialization.

## Components

The project consists of the following components:

- **Smart Contract (`nft-marketplace`)**: Contains the main logic for managing fractional ownership and transactions of NFTs.
- **Supporting Contracts (`deauthe`, `bidding-token`)**: Other contracts utilized within the ecosystem (optional).

## Getting Started

To build and deploy the smart contract, follow these steps:

### Prerequisites

- Rust (nightly) and Cargo: Install using [rustup](https://rustup.rs/).
- CosmWasm: Install using Cargo with `cargo install --version=0.14.0 cosmwasm-cli`.

### Build

1. Clone the repository:

   ```bash
   git clone <repository_url>
   cd nft-marketplace
   ```

2. Build the project:

   ```bash
   cargo build
   ```

### Deploy

To deploy the contract, you'll need to follow the CosmWasm deployment process. Ensure you have your chain configured and initialized appropriately.

### Usage

Provide instructions on how to use the smart contract. For example:

#### Listing an NFT for Fractional Ownership

To list an NFT for fractional ownership:

```rust
// Example Rust code snippet
use nft_marketplace::{MarketplaceContract, ContractError};
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, Addr, Uint128};

let contract = MarketplaceContract::default();
let mut deps: DepsMut = ...; // Initialize CosmWasm dependencies

let contract_address: Addr = ...; // Address of the NFT contract
let token_id: String = ...; // ID of the NFT
let shares: Uint128 = Uint128::from(100); // Number of shares
let price_per_share: Uint128 = Uint128::from(1000); // Price per share

let info: MessageInfo = ...; // Information of the sender

let response: Result<Response, ContractError> = contract.execute_list_fractional_nft(
    deps.as_mut(),
    Env::default(),
    info,
    contract_address,
    token_id,
    shares,
    price_per_share,
);

```

#### Buying Fractional Ownership

To buy shares of an NFT:

```rust
// Example Rust code snippet
use nft_marketplace::{MarketplaceContract, ContractError};
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, Addr, Uint128};

let contract = MarketplaceContract::default();
let mut deps: DepsMut = ...; // Initialize CosmWasm dependencies

let contract_address: Addr = ...; // Address of the NFT contract
let token_id: String = ...; // ID of the NFT
let shares: Uint128 = Uint128::from(50); // Number of shares to buy

let info: MessageInfo = ...; // Information of the sender

let response: Result<Response, ContractError> = contract.execute_buy_fractional_nft(
    deps.as_mut(),
    Env::default(),
    info,
    contract_address,
    token_id,
    shares,
);

```

### Contributing

Contributions are welcome! Please fork the repository and submit a pull request with your changes.
