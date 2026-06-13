// storage.rs
use soroban_sdk::contracttype;

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Bounty(u32),
    Report(u32),

    NextBountyId,
    NextReportId,
}