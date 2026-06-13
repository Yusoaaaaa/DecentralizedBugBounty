#![cfg(test)]

extern crate std;

use soroban_sdk::{
    testutils::Address as _,
    Address,
    Env,
    String,
};

// Import Client đã được tự động generate bởi macro #[contractimpl]
use crate::{BugBountyContract, BugBountyContractClient};

#[test]
fn test_create_bounty() {
    let env = Env::default();
    
    // Giả lập cấp quyền xác thực cho tất cả các Address (để vượt qua require_auth)
    env.mock_all_auths();

    let owner = Address::generate(&env);

    // Đăng ký hợp đồng vào môi trường test
    let contract_id = env.register_contract(None, BugBountyContract);
    
    // Khởi tạo Client để gọi hàm
    let client = BugBountyContractClient::new(&env, &contract_id);

    // Chuẩn bị dữ liệu String cho tham số title
    let title = String::from_str(&env, "Test Bounty");
    let reward: i128 = 1000;

    // Gọi hàm qua client (không cần truyền Env vào nữa)
    let id = client.create_bounty(&owner, &title, &reward);

    assert_eq!(id, 1);
}