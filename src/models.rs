use soroban_sdk::{symbol_short, Address, Env, Map};
use crate::ContractError;

const RELAYER_LEDGER_TRACKING_KEY: soroban_sdk::Symbol = symbol_short!("RLGR_SEQ");

pub const MIN_LEDGER_GAP: u32 = 3;

pub fn get_last_ledger_for_relayer(env: &Env, relayer: &Address) -> u32 {
    let tracking: Map<Address, u32> = env
        .storage()
        .persistent()
        .get(&RELAYER_LEDGER_TRACKING_KEY)
        .unwrap_or_else(|| Map::new(env));
    tracking.get(relayer.clone()).unwrap_or(0)
}

pub fn verify_ledger_gap(env: &Env, relayer: &Address) -> Result<(), ContractError> {
    let last_ledger = get_last_ledger_for_relayer(env, relayer);
    let current_ledger = env.ledger().sequence();

    if current_ledger.saturating_sub(last_ledger) < MIN_LEDGER_GAP {
        return Err(ContractError::LedgerGapNotSatisfied);
    }

    Ok(())
}

pub fn record_relayer_ledger(env: &Env, relayer: &Address, ledger: u32) {
    let mut tracking: Map<Address, u32> = env
        .storage()
        .persistent()
        .get(&RELAYER_LEDGER_TRACKING_KEY)
        .unwrap_or_else(|| Map::new(env));
    tracking.set(relayer.clone(), ledger);
    env.storage().persistent().set(&RELAYER_LEDGER_TRACKING_KEY, &tracking);
}

pub fn record_ledger_gap(env: &Env, relayer: &Address) {
    let current_ledger = env.ledger().sequence();
    record_relayer_ledger(env, relayer, current_ledger);
}