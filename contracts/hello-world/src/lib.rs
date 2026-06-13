#![no_std]

mod storage;
mod types;

use soroban_sdk::{
    contract,
    contractimpl,
    Address,
    Env,
    String,
};

use storage::DataKey;
use types::{Bounty, Report};

#[contract]
pub struct BugBountyContract;

#[contractimpl]
impl BugBountyContract {

    pub fn create_bounty(
        env: Env,
        owner: Address,
        title: String,
        reward: i128,
    ) -> u32 {

        owner.require_auth();

        let id: u32 = env
            .storage()
            .persistent()
            .get(&DataKey::NextBountyId)
            .unwrap_or(1);

        let bounty = Bounty {
            id,
            owner,
            title,
            reward,
            active: true,
        };

        env.storage()
            .persistent()
            .set(&DataKey::Bounty(id), &bounty);

        env.storage()
            .persistent()
            .set(&DataKey::NextBountyId, &(id + 1));

        id
    }

    pub fn submit_report(
        env: Env,
        researcher: Address,
        bounty_id: u32,
        report_hash: String,
    ) -> u32 {

        researcher.require_auth();

        let report_id: u32 = env
            .storage()
            .persistent()
            .get(&DataKey::NextReportId)
            .unwrap_or(1);

        let report = Report {
            id: report_id,
            bounty_id,
            researcher,
            report_hash,
            accepted: false,
        };

        env.storage()
            .persistent()
            .set(&DataKey::Report(report_id), &report);

        env.storage()
            .persistent()
            .set(&DataKey::NextReportId, &(report_id + 1));

        report_id
    }

    pub fn approve_report(
        env: Env,
        owner: Address,
        report_id: u32,
    ) {

        owner.require_auth();

        let mut report: Report = env
            .storage()
            .persistent()
            .get(&DataKey::Report(report_id))
            .unwrap();

        let bounty: Bounty = env
            .storage()
            .persistent()
            .get(&DataKey::Bounty(report.bounty_id))
            .unwrap();

        if bounty.owner != owner {
            panic!("not owner");
        }

        report.accepted = true;

        env.storage()
            .persistent()
            .set(&DataKey::Report(report_id), &report);
    }

    pub fn get_bounty(
        env: Env,
        id: u32,
    ) -> Bounty {

        env.storage()
            .persistent()
            .get(&DataKey::Bounty(id))
            .unwrap()
    }

    pub fn get_report(
        env: Env,
        id: u32,
    ) -> Report {

        env.storage()
            .persistent()
            .get(&DataKey::Report(id))
            .unwrap()
    }
}