use yew::Properties;

#[derive(Properties, PartialEq, Debug)]
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
