# dApp-Staking Chain Extension Workshop
This repository contains two ink! example contract that was shown on the Workshop of Polkadot North America Hackathon

### Example 1: Use custom environment & ink! macros
Folder: `dapps-staking-chain-extension-contract` \
This example shows how to use dApp-Staking chain-extension by defining the function inside a custom environment and using `#[ink::chain_extension]` macro
It relies on [ink! chain-extension documentation](https://ink.substrate.io/macros-attributes/chain-extension#details-handle_status) & [ink! chain-extension contract example](https://github.com/paritytech/ink/tree/master/examples/rand-extension)

### Example 2: Use chain-extension builder
Folders: `dapps-staking-crate` & `staking-contract` \
This examples use the chain-extension builder directly in order to avoid using custom environment and to separate the chain-extension crate & the smart-contract. \
It shows how chain-extension will be implemented in **Astar**. \
It was inspired by the OpenBrush implementation of [Pallet-Asset chain-extension](https://github.com/Supercolony-net/openbrush-contracts/blob/feature/psp22-extension-pallet-assets/contracts/token/psp22/utils/pallet_assets.rs#L35) [link refers to a PR so it migth change]
