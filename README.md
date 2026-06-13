# Stellar Bug Bounty Platform

## Problem
Traditional bug bounty platforms often lack transparency in the review process and suffer from delayed payout distributions to security researchers.

## Solution
A decentralized application (dApp) that allows organizations to create bug bounty programs, securely lock reward pools within smart contracts, and automatically distribute payouts as soon as a vulnerability report is approved.

## Why Stellar
Soroban Smart Contracts on Stellar offer decentralized, transparent fund management with extremely low transaction fees and fast execution speeds, making bounty pool operations highly cost-effective.

## Target User
Web3 organizations/projects looking to surface security vulnerabilities and white-hat hackers seeking fair, automated bug bounty rewards.

## Live Demo
- Network: Stellar Testnet

- **Contract ID**: `CCCHE44FJ7EW23I4AHFREYAYZFT6DCHEDWMGSZPKTPOMFRJDSF6MO76G`
- **Transaction**: https://stellar.expert/explorer/testnet/tx/88b83ab0b938f32cf8e51b14b8a1cba23947ca79bc37f1e67bd873604da2b062 (Please replace this with your actual deposit/deployment Tx Hash if needed)

## How to Run
1. Clone: `git clone https://github.com/yourname/stellar-bug-bounty.git`
2. Build: `cd contracts/hello-world && stellar contract build`
3. Test: `cargo test`
4. Deploy: `stellar contract deploy --wasm ../../target/wasm32v1-none/release/hello_world.wasm --source GDFU35A5RHWY7FCJQ53AIYJDWFXDN3SW6KOP554PJ4E4NPNFWT7VOCKU --network testnet`
5. Frontend: `cd frontend && npx serve .`

## Tech Stack
- Smart Contract: Rust / Soroban SDK v25.3.1
- Frontend: HTML / JavaScript / @stellar/stellar-sdk
- Wallet: Freighter
- Network: Stellar Testnet

## Team
- Lâm Vũ Bằng | [bangcongcong1@gmail.com] | Hutech University 