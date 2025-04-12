use soroban_sdk::{Address, BytesN, Env, String, Vec};

use crate::error::ContractError;

pub trait AccountMasterTrait {
    fn initialize(e: Env, admin: Address, adapter_id: Address) -> Result<(), ContractError>;
    fn create_smart_wallet(
        e: Env,
        bls_pubkeys: Vec<BytesN<96>>,
        platform: String,
        social_username: String,
        web_pubkey: BytesN<77>,
    ) -> Result<Address, ContractError>;
    fn add_supported_platform(e: Env, platform: String);

    fn get_is_smart_wallet(e: Env, wallet_id: Address) -> bool;

    fn get_wallet_by_username(
        e: Env,
        platform: String,
        social_username: String,
    ) -> Result<Address, ContractError>;

    fn get_wallet_by_passkey(e: Env, passkey: BytesN<77>) -> Result<Address, ContractError>;

    fn upgrade(e: Env, new_wasm_hash: BytesN<32>);
}
