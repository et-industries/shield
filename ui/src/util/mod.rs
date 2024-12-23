use serde::Serialize;
use shield_circuit::Hash;
use yew::{Callback, Properties};

#[derive(Debug, Clone)]
pub struct UnShieldedAccountState {
    pub address: String,
    pub balance: u64,
}

impl UnShieldedAccountState {
    pub fn new(address: String, balance: u64) -> Self {
        Self { address, balance }
    }
}

#[derive(Debug, Clone)]
pub struct ShieldedAccountState {
    pub id: usize,
    pub address: String,
    pub deposit_amount: u64,
    pub withdraw_success: bool,
    pub nullifier: String,
}

impl ShieldedAccountState {
    pub fn new(
        id: usize,
        address: String,
        deposit_amount: u64,
        withdraw_success: bool,
        nullifier: String,
    ) -> Self {
        Self {
            id,
            address,
            deposit_amount,
            withdraw_success,
            nullifier,
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct UnShieldAccountProps {
    pub address: String,
    pub balance: u64,
    pub deposit_clicked: Callback<(String, u64)>,
}

#[derive(Properties, PartialEq)]
pub struct ShieldAccountProps {
    pub id: usize,
    pub address: String,
    pub deposit_amount: u64,
    pub withdraw_success: bool,
    pub nullifier: String,
    pub withdraw_clicked: Callback<(usize, String)>,
}

#[derive(Serialize)]
pub struct DepositParams {
    pub(crate) recipiant: u64,
}

#[derive(Serialize)]
pub struct WithdrawParams {
    pub(crate) nullifier: Hash,
}

impl WithdrawParams {
    pub fn from_hex_str(nullifier: String) -> Self {
        Self {
            nullifier: Hash::from_hex(nullifier),
        }
    }
}
