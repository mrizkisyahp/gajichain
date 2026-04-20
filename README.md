# Stellar Inventory DApp 📦

**Stellar Inventory DApp** - Blockchain-Based Decentralized Inventory Management System

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Built on Stellar](https://img.shields.io/badge/Built%20on-Stellar-blue)](https://stellar.org)
[![Smart Contract: Soroban](https://img.shields.io/badge/Smart%20Contract-Soroban-purple)](https://soroban.stellar.org)
[![Network: Testnet](https://img.shields.io/badge/Network-Testnet-orange)]()

---

## Project Description

Stellar Inventory DApp is a decentralized smart contract solution built on the Stellar blockchain using Soroban SDK. It provides a secure, immutable platform for managing inventory data directly on the blockchain. The contract ensures that your data is stored transparently and is only manageable through predefined smart contract functions, eliminating reliance on centralized database providers.

The system allows users to add items, view inventory, update stock, and delete items, leveraging the efficiency and security of the Stellar network. Each item is uniquely identified and stored within the contract's instance storage, ensuring data persistence and reliability.

---

## Project Vision

Our vision is to revolutionize inventory management in the digital age by:

- **Decentralizing Data**: Moving inventory systems from centralized servers to a global, distributed blockchain
- **Ensuring Ownership**: Empowering users to have complete control and ownership over their inventory data
- **Guaranteeing Immutability**: Providing a tamper-proof record of inventory changes that cannot be altered by unauthorized parties
- **Enhancing Transparency**: Leveraging blockchain to make inventory operations verifiable and traceable
- **Building Trustless Systems**: Creating a platform where data integrity is guaranteed by code, not by company promises

We envision a future where inventory systems are transparent, secure, and fully controlled by their owners.

---

## Key Features

### 1. **Simple Item Creation**

- Add inventory items with a single function call
- Specify item name, stock quantity, and price
- Automated ID generation for unique identification
- Persistent storage on the Stellar blockchain

### 2. **Efficient Data Retrieval**

- Fetch all stored items in a single call
- Structured data representation for easy frontend integration
- Quick access to your entire inventory
- Real-time synchronization with the blockchain state

### 3. **Stock Management**

- Update stock levels for specific items
- Maintain accurate inventory tracking
- Immediate reflection of changes on-chain

### 4. **Secure Deletion**

- Remove specific items using their unique IDs
- Permanent removal from the contract storage
- Clean and efficient storage management
- Immediate update of the inventory list after deletion

### 5. **Transparency and Security**

- View all inventory operations on the blockchain
- Blockchain-based verification of all actions
- Immutable records of item creation and updates
- Protected against unauthorized modifications

### 6. **Stellar Network Integration**

- Leverages the high speed and low cost of Stellar
- Built using the modern Soroban Smart Contract SDK
- Scalable architecture for growing inventory data
- Interoperable with other Stellar-based services

---

## Contract Details

| Field | Value |
|---|---|
| **Contract Address** | `CASPJPI6YPLU3QS5BLAB2U6R36P24O6RR2JPD6SNM3JZGFXRVYWZWEXZ` |
| **Network** | Stellar Testnet |
| **Language** | Rust (Soroban SDK) |
| **Storage Type** | Instance Storage |

### Core Functions

| Function | Parameters | Description |
|---|---|---|
| `add_item()` | `name: String, stock: u64, price: i128` | Add a new inventory item |
| `get_items()` | — | Retrieve all stored items |
| `update_stock()` | `id: u64, new_stock: u64` | Update stock for a specific item |
| `delete_item()` | `id: u64` | Remove an item by its unique ID |

---

## Future Scope

### Short-Term Enhancements

1. **Item Categorization**: Add categories and tags for better organization
2. **Price Management**: Support updating item prices
3. **Search Functionality**: Implement search and filtering for inventory items
4. **Low Stock Alerts**: Notifications when stock reaches critical levels

### Medium-Term Development

5. **Multi-User Access**: Role-based permissions for managing inventory
   - Admin and staff roles
   - Controlled editing and viewing access
   - Activity tracking
6. **Notification System**: Alerts for inventory changes
7. **Transaction History**: Record all inventory updates
8. **Inter-Contract Integration**: Allow other contracts to interact with inventory data

### Long-Term Vision

9. **Cross-Chain Inventory Sync**: Extend inventory across multiple blockchains
10. **Decentralized UI Hosting**: Host frontend on IPFS or similar platforms
11. **AI-Based Forecasting**: Predict stock needs using AI
12. **Privacy Layers**: Advanced privacy features for sensitive data
13. **DAO Governance**: Community-driven feature development
14. **Identity Management**: Integration with decentralized identity (DID) systems

### Enterprise Features

15. **Warehouse Integration**: Support multi-location inventory systems
16. **Audit Logging**: Immutable logs for compliance and tracking
17. **Automated Reporting**: Generate reports from inventory data
18. **Multi-Language Support**: Improve accessibility globally

---

## Technical Requirements

- Soroban SDK
- Rust programming language
- Stellar blockchain network

---

## Getting Started

This project is deployed and interacted with using the **Soroban Online IDE** — no local installation required.

### 1. Open Soroban Online IDE

Go to [https://soroban.stellar.org/docs](https://soroban.stellar.org/docs) or use the Stellar playground environment to access the online IDE.

### 2. Load the Contract

Paste or import the contract source code (`lib.rs`) into the online IDE editor.

### 3. Compile the Contract

Use the IDE's built-in build tool to compile the Rust contract to WebAssembly (WASM).

### 4. Deploy to Testnet

Deploy the compiled WASM to the Stellar Testnet directly from the IDE. The deployed contract address is:

```
CASPJPI6YPLU3QS5BLAB2U6R36P24O6RR2JPD6SNM3JZGFXRVYWZWEXZ
```

### 5. Invoke Contract Functions

Use the IDE's invocation panel to call the contract functions:

```
# Add a new item
add_item(name="Laptop", stock=50, price=15000000)

# Get all items
get_items()

# Update stock for item with ID 1
update_stock(id=1, new_stock=45)

# Delete item with ID 1
delete_item(id=1)
```

### 6. Verify on Stellar Explorer

Track all transactions and contract state on the Stellar Testnet Explorer:  
[https://stellar.expert/explorer/testnet](https://stellar.expert/explorer/testnet)

Search for the contract address:
```
CASPJPI6YPLU3QS5BLAB2U6R36P24O6RR2JPD6SNM3JZGFXRVYWZWEXZ
```

---

**Stellar Inventory DApp** — Securing Your Inventory on the Blockchain
