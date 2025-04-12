use soroban_sdk::{contract, contractimpl, Address, BytesN, Env, String, Vec};

use crate::{
    access::{
        authenticate_admin, has_admin, read_dapp_adapter_id, write_admin, write_dapp_adapter_id,
    },
    contract_trait::AccountMasterTrait,
    error::ContractError,
    platforms::{
        read_is_smart_wallet, read_passkey_wallet_map, read_username_wallet_map,
        write_is_smart_wallet, write_passkey_wallet_map, write_supported_platform,
        write_username_wallet_map,
    },
    user_account_factory::{self, create_user_account},
};

#[contract]
pub struct AccountMaster;

#[contractimpl]
impl AccountMasterTrait for AccountMaster {
    ///Initialize Contract
    fn initialize(e: Env, admin: Address, adapter_id: Address) -> Result<(), ContractError> {
        let is_initialized = has_admin(&e);
        if is_initialized {
            return Err(ContractError::AlreadyInitialized);
        }
        write_admin(&e, &admin);
        write_dapp_adapter_id(&e, &adapter_id);

        Ok(())
    }
    ///Create smart wallet
    fn create_smart_wallet(
        e: Env,
        bls_pubkeys: Vec<BytesN<96>>,
        platform: String,
        social_username: String,
        web_pubkey: BytesN<77>,
    ) -> Result<Address, ContractError> {
        let smart_account_address = create_user_account(&e, &web_pubkey.clone());

        let dapp_router_contract_id = read_dapp_adapter_id(&e).unwrap();

        user_account_factory::Client::new(&e, &smart_account_address).init(
            &bls_pubkeys,
            &platform,
            &social_username,
            &web_pubkey,
            &e.current_contract_address(),
            &dapp_router_contract_id,
        );
        write_username_wallet_map(&e, platform, social_username, smart_account_address.clone())?;
        write_passkey_wallet_map(&e, web_pubkey, smart_account_address.clone())?;
        write_is_smart_wallet(&e, smart_account_address.clone());

        Ok(smart_account_address)
    }

    fn add_supported_platform(e: Env, platform: String) {
        authenticate_admin(&e);
        write_supported_platform(&e, platform);
    }

    fn get_is_smart_wallet(e: Env, wallet_id: Address) -> bool {
        read_is_smart_wallet(&e, wallet_id)
    }

    fn get_wallet_by_username(
        e: Env,
        platform: String,
        social_username: String,
    ) -> Result<Address, ContractError> {
        read_username_wallet_map(&e, platform, social_username)
    }

    fn get_wallet_by_passkey(e: Env, passkey: BytesN<77>) -> Result<Address, ContractError> {
        read_passkey_wallet_map(&e, passkey)
    }

    fn upgrade(e: Env, new_wasm_hash: BytesN<32>) {
        authenticate_admin(&e);
        e.deployer().update_current_contract_wasm(new_wasm_hash);
    }
}
