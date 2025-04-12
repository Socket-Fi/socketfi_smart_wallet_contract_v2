use soroban_sdk::{Address, BytesN, Env, String};

use crate::{data::DataKey, error::ContractError, formatter::to_lower_bytes};

pub fn read_is_smart_wallet(e: &Env, wallet_id: Address) -> bool {
    let key = DataKey::IsSmartWallet(wallet_id);
    e.storage().instance().get(&key).unwrap_or(false)
}

pub fn write_is_smart_wallet(e: &Env, wallet_id: Address) {
    let key = DataKey::IsSmartWallet(wallet_id.clone());
    e.storage().instance().set(&key, &wallet_id);
}

pub fn is_supported_platform(e: &Env, platform: String) -> bool {
    let key = DataKey::SupportedPlatform(to_lower_bytes(e, platform));
    e.storage().instance().has(&key)
}

pub fn write_supported_platform(e: &Env, platform: String) {
    let key = DataKey::SupportedPlatform(to_lower_bytes(e, platform.clone()));
    e.storage().instance().set(&key, &platform);
}

pub fn is_registered_username(e: &Env, platform: String, username: String) -> bool {
    let key =
        DataKey::UsernameSmartWalletMap(to_lower_bytes(e, platform), to_lower_bytes(e, username));
    e.storage().instance().has(&key)
}

pub fn write_username_wallet_map(
    e: &Env,
    platform: String,
    username: String,
    wallet_address: Address,
) -> Result<(), ContractError> {
    if !is_supported_platform(e, platform.clone()) {
        return Err(ContractError::PlatformNotSupported);
    }
    if is_registered_username(e, platform.clone(), username.clone()) {
        return Err(ContractError::UsernameAlreadyRegistered);
    }
    let key =
        DataKey::UsernameSmartWalletMap(to_lower_bytes(e, platform), to_lower_bytes(e, username));
    e.storage().instance().set(&key, &wallet_address);
    Ok(())
}

pub fn read_username_wallet_map(
    e: &Env,
    platform: String,
    username: String,
) -> Result<Address, ContractError> {
    if !is_supported_platform(e, platform.clone()) {
        return Err(ContractError::PlatformNotSupported);
    }
    if !is_registered_username(e, platform.clone(), username.clone()) {
        return Err(ContractError::UsernameNotRegistered);
    }
    let key =
        DataKey::UsernameSmartWalletMap(to_lower_bytes(e, platform), to_lower_bytes(e, username));

    Ok(e.storage()
        .instance()
        .get::<DataKey, Address>(&key)
        .unwrap())
}

pub fn is_linked_passkey(e: &Env, passkey: BytesN<77>) -> bool {
    let key = DataKey::PasskeySmartWalletMap(passkey);
    e.storage().instance().has(&key)
}

pub fn write_passkey_wallet_map(
    e: &Env,
    passkey: BytesN<77>,
    wallet_address: Address,
) -> Result<(), ContractError> {
    if is_linked_passkey(e, passkey.clone()) {
        return Err(ContractError::PasskeyAlreadyLinked);
    }
    let key = DataKey::PasskeySmartWalletMap(passkey);
    e.storage().instance().set(&key, &wallet_address);
    Ok(())
}

pub fn read_passkey_wallet_map(e: &Env, passkey: BytesN<77>) -> Result<Address, ContractError> {
    if !is_linked_passkey(e, passkey.clone()) {
        return Err(ContractError::PasskeyNotLinked);
    }
    let key = DataKey::PasskeySmartWalletMap(passkey);

    Ok(e.storage()
        .instance()
        .get::<DataKey, Address>(&key)
        .unwrap())
}
