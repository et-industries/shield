use crate::bindgen::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::util::AccountState;

const DEFAULT_BALANCE: u64 = 100;
const DEFAULT_DEPOSITED: u64 = 10;

#[function_component(App)]
pub fn app() -> Html {
    let unshielded_accounts: UseStateHandle<Vec<AccountState>> = use_state(|| {
        vec![AccountState::new(
            "0x1234....5678".to_string(),
            DEFAULT_BALANCE,
            0,
        )]
    });

    let shielded_accounts: UseStateHandle<Vec<AccountState>> = use_state(|| {
        vec![
            AccountState::new("0x1234....5678".to_string(), 0, DEFAULT_DEPOSITED),
            AccountState::new("0xabcd....efgh".to_string(), 0, DEFAULT_DEPOSITED),
        ]
    });

    // {
    //     let mut unshielded_accounts = unshielded_accounts.to_vec();
    //     use_effect(move || {
    //         spawn_local(async move {
    //             let res = invoke_without_args("get_default_account").await.as_string();
    //             unshielded_accounts.push(res.unwrap_or("0".to_string()));
    //         });
    //     });
    // }

    html! {
        <div class="container">
          <h1 class="accounts-title">{"Unshielded accounts"}</h1>
          <div class="accounts-list">
            {unshielded_accounts.iter().map(|AccountState { address, balance, deposited }| {
              html! {
                <div class="accounts-item">
                  <UnShieldedAccount address={address.clone()} balance={balance} deposited={deposited} />
                </div>
              }
            }).collect::<Html>()}
          </div>
          <h1 class="accounts-title">{"Shielded accounts"}</h1>
          <div class="accounts-list">
            {shielded_accounts.iter().map(|AccountState { address, deposited, balance }| {
              html! {
                <div class="accounts-item">
                  <ShieldedAccount address={address.clone()} balance={balance} deposited={deposited} />
                </div>
              }
            }).collect::<Html>()}
          </div>
        </div>
    }
}

#[function_component(UnShieldedAccount)]
pub fn unshielded_account(
    AccountState {
        address, balance, ..
    }: &AccountState,
) -> Html {
    // State to hold the shielded address
    let shielded_address = use_state(|| "0x12345....67890".to_string());

    // State to hold the deposit amount (dummy value for now)
    let deposit_amount = use_state(|| "0.00".to_string());

    // Handle address input change
    let on_address_change = {
        let address = shielded_address.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                address.set(input.value());
            }
        })
    };

    // Dummy actions for Deposit button
    let on_deposit = {
        let deposit_amount = deposit_amount.clone();
        Callback::from(move |_| {
            // Dummy deposit logic
            deposit_amount.set("10.00 ETH".to_string());
        })
    };

    html! {
        <div>
            <div>
                {(*address).clone()}{" : "}{(*balance).clone()}<strong>{" ETH"}</strong>
            </div>
            <div>
                <input
                    id="address"
                    type="text"
                    placeholder="Enter Shield address"
                    oninput={on_address_change}
                />
            </div>
            <div>
                <input
                    id="deposit_amount"
                    type="text"
                    placeholder="Enter Deposit amount"
                    // oninput={on_address_change}
                />
            </div>
            <div class = "deposit-button">
                <button onclick={on_deposit} >
                    {"Deposit"}
                </button>
            </div>
        </div>
    }
}

#[function_component(ShieldedAccount)]
pub fn shielded_account(
    AccountState {
        address, deposited, ..
    }: &AccountState,
) -> Html {
    // Dummy actions for Withdraw button
    let on_withdraw = {
        Callback::from(move |_| {
            // Dummy withdraw logic
            println!("withdraw clicked!");
        })
    };

    html! {
        <div>
            <div>
                {address.clone()}{" : "}{*deposited}<strong>{" ETH"}</strong>
            </div>
            <div class = "withdraw-button">
                <button onclick={on_withdraw} >
                    {"Withdraw"}
                </button>
            </div>
        </div>
    }
}
