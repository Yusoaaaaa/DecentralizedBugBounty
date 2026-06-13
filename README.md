# Stellar Bug Bounty Platform

## Problem
Các nền tảng săn lỗi nhận thưởng truyền thống thường thiếu minh bạch trong khâu kiểm duyệt và chậm trễ trong quá trình giải ngân tiền thưởng cho các nhà nghiên cứu bảo mật.

## Solution
Một ứng dụng phi tập trung (dApp) cho phép các tổ chức tạo chương trình Bug Bounty, lưu trữ tiền thưởng an toàn trên hợp đồng thông minh và tự động giải ngân ngay khi báo cáo lỗi được phê duyệt.

## Why Stellar
Tính năng Hợp đồng thông minh Soroban kết hợp với chi phí giao dịch cực thấp và tốc độ xử lý nhanh của mạng lưới Stellar giúp việc quản lý quỹ thưởng minh bạch, phân quyền và tối ưu chi phí.

## Target User
Các tổ chức/dự án Web3 cần tìm kiếm lỗ hổng bảo mật và các nhà nghiên cứu bảo mật (White-hat Hackers) muốn săn lỗi nhận thưởng công bằng.

## Live Demo
- Network: Stellar Testnet

- **Contract ID**: `CCCHE44FJ7EW23I4AHFREYAYZFT6DCHEDWMGSZPKTPOMFRJDSF6MO76G`
- **Transaction**: https://stellar.expert/explorer/testnet/tx/88b83ab0b938f32cf8e51b14b8a1cba23947ca79bc37f1e67bd873604da2b062 (Vui lòng thay thế bằng mã Tx Hash nạp tiền/deploy thực tế của bạn)

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
- [Lâm Vũ Bằng] | [bangcongcong1@gmail.com] | [Hutech + sinh viên năm 3]