# 📦 Soroban Supply Chain Tracker

## Project Description
The **Soroban Supply Chain Tracker** is a decentralized application built on the Stellar network using the Soroban smart contract platform. It provides an immutable, transparent, and highly efficient way to record the lifecycle of physical goods as they move from production to the end consumer. By leveraging blockchain technology, this project ensures that all stakeholders have access to a single source of truth regarding an item's current state.

<img width="1920" height="1080" alt="image" src="https://github.com/user-attachments/assets/eec1eeb9-82d7-4143-a3ac-ea28d921fe81" />



## What it does
This smart contract acts as an on-chain ledger for physical items. When a product is manufactured, an authorized user can register it on the blockchain with a unique ID and a descriptive name. As the product moves through the logistics network, its status can be updated on-chain. Anyone with the item's ID can query the smart contract to instantly verify its current location or status in the supply chain pipeline. 

## Features
- **Asset Initialization:** Register new physical goods on the blockchain with a unique identifier and name.
- **State Management:** Seamlessly update the real-time status of goods across three primary lifecycle phases:
  - `Manufactured`
  - `InTransit`
  - `Delivered`
- **Transparent Querying:** Fetch immutable, up-to-date information about any tracked item using its unique ID.
- **Persistent Storage:** Securely stores item states using Soroban's persistent storage mechanisms to ensure data longevity. 

## Deployed Smart Contract Link
https://stellar.expert/explorer/testnet/contract/CANLGQ2SKX7EIS55U7IUFYQDP7CLT4HYNOOA5WHTFNFLUZNX75BXYHYD

---
*Built with Rust and Soroban on the Stellar Network.*
