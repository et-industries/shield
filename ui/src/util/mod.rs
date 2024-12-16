use serde::de;

#[derive(Debug)]
pub struct AccountState {
    address: String,
    balance: u64,
    deposited: u64,
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
