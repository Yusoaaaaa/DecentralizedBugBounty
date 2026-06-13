use soroban_sdk::{contracttype, Address, String};

#[derive(Clone)]
#[contracttype]
pub struct Bounty {
    pub id: u32,
    pub owner: Address,
    pub title: String,
    pub reward: i128,
    pub active: bool,
}

#[derive(Clone)]
#[contracttype]
pub struct Report {
    pub id: u32,
    pub bounty_id: u32,
    pub researcher: Address,
    pub report_hash: String,
    pub accepted: bool,
}