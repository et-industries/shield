use yew::{Callback, Properties};
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct AccountState {
    pub address: String,
    pub balance: u64,
    pub deposited: u64,
}

impl AccountState {
    pub fn new(address: String, balance: u64, deposited: u64) -> Self {
        Self {
            address,
            balance,
            deposited,
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
    pub address: String,
    pub deposited: u64,
    pub withdraw_clicked: Callback<String>,
}

#[derive(Serialize)]
pub struct DepositParams {
    recipiant: u64,
}
