use soroban_sdk::{Address, Env};

use crate::data::DataKey;

pub fn has_admin(e: &Env) -> bool {
    let key = DataKey::Admin;
    e.storage().instance().has(&key)
}

pub fn read_admin(e: &Env) -> Option<Address> {
    let key = DataKey::Admin;
    e.storage().instance().get(&key).expect("Admin not found!")
}

pub fn write_admin(e: &Env, admin: &Address) {
    let key = DataKey::Admin;
    e.storage().instance().set(&key, admin);
}

pub fn authenticate_admin(e: &Env) {
    let admin = read_admin(e).unwrap();
    admin.require_auth();
}

pub fn read_dapp_adapter_id(e: &Env) -> Option<Address> {
    let key = DataKey::DappAdapterId;
    e.storage()
        .instance()
        .get(&key)
        .expect("dApp adapter not found!")
}

pub fn write_dapp_adapter_id(e: &Env, adapter_id: &Address) {
    let key = DataKey::DappAdapterId;
    e.storage().instance().set(&key, adapter_id);
}
