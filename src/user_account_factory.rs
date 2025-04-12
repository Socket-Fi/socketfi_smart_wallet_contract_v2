#![allow(unused)]
use crate::{
    error::ContractError,
    types::{AllowanceDetails, TokenDetails, TransferDataReceived, WebKeyDetails},
};
use soroban_sdk::{xdr::ToXdr, Address, Bytes, BytesN, Env, String};

soroban_sdk::contractimport!(
    file = "../socketfi_user_account_v2/target/wasm32-unknown-unknown/release/socketfi_user_smart_account_v2.wasm"
);

const WASM_IMPORT: &[u8] = include_bytes!("../.././socketfi_user_account_v2/target/wasm32-unknown-unknown/release/socketfi_user_smart_account_v2.wasm");

pub fn create_user_account(e: &Env, public_web: &BytesN<77>) -> Address {
    let user_account_wasm = e.deployer().upload_contract_wasm(WASM_IMPORT);

    let mut salt = Bytes::new(e);
    salt.append(&public_web.to_xdr(e));
    let salt = e.crypto().sha256(&salt);
    let user_account_address = e
        .deployer()
        .with_current_contract(salt)
        .deploy_v2(user_account_wasm, ());
    user_account_address
}
