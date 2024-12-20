use crate::{
    bindgen::*,
    util::{AccountState, ShieldAccountProps, UnShieldAccountProps},
};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use serde_wasm_bindgen::to_value;

const DEFAULT_BALANCE: u64 = 100;
const DEFAULT_DEPOSITED: u64 = 10;

#[function_component(App)]
pub fn app() -> Html {
    let unshielded_accounts = use_state(|| {
        vec![AccountState::new(
            "0x1234....5678".to_string(),
            DEFAULT_BALANCE,
            0,
        )]
    });

    let shielded_accounts = use_state(|| {
        vec![
            AccountState::new("0x1234....5678".to_string(), 0, DEFAULT_DEPOSITED),
            AccountState::new("0xabcd....efgh".to_string(), 0, DEFAULT_DEPOSITED),
        ]
    });

    let nullifier = use_state(|| "".to_string());

    // {
    //     let mut unshielded_accounts = unshielded_accounts.to_vec();
    //     use_effect(move || {
    //         spawn_local(async move {
    //             let res = invoke_without_args("get_default_account").await.as_string();
    //             unshielded_accounts.push(res.unwrap_or("0".to_string()));
    //         });
    //     });
    // }

    let deposit_click = {
        let shielded_accounts = shielded_accounts.clone();
        let nullifier = nullifier.clone();
        Callback::from(move |(new_shielded_addr, deposit_amount)| {
            let mut accounts = shielded_accounts.to_vec();
            accounts.push(AccountState::new(new_shielded_addr, 0, deposit_amount));
            shielded_accounts.set(accounts);

            // call deposit function of backend
            let nullifier = nullifier.clone();
            let js_args = to_value(&DepositParams { recipiant: 456 }).unwrap();
            spawn_local(async move {
                let nullifier_str = unsafe { invoke("deposit", js_args) }.await;
                nullifier.set(nullifier_str.as_string().unwrap());
            });
        })
    };

    let withdraw_click = {
        let shielded_accounts = shielded_accounts.clone();
        Callback::from(move |shielded_addr: String| {
            let mut accounts = shielded_accounts.to_vec();
            accounts.retain(|a| a.address != shielded_addr);
            shielded_accounts.set(accounts);

            // TODO: call withdraw function of backend
        })
    };

    html! {
        <div class="container">
          <h1 class="accounts-title">{"Unshielded accounts"}</h1>
          <div class="accounts-list">
            {unshielded_accounts.iter().map(|AccountState { address, balance, .. }| {
              html! {
                <div class="accounts-item">
                  <UnShieldedAccount address={address.clone()} balance={balance} deposit_clicked={deposit_click.clone()} />
                </div>
              }
            }).collect::<Html>()}
          </div>

          <div class="nullifier-container">
            <label for="nullifier" class="nullifier-label">{"Nullifier"}</label>
            <p id="nullifier" class="nullifier-text" readonly=true>{nullifier.to_string()}</p>
          </div>

          <h1 class="accounts-title">{"Shielded accounts"}</h1>
          <div class="accounts-list">
            {shielded_accounts.iter().map(|AccountState { address, deposited, .. }| {
              html! {
                <div class="accounts-item">
                  <ShieldedAccount address={address.clone()} deposited={deposited} withdraw_clicked={withdraw_click.clone()} />
                </div>
              }
            }).collect::<Html>()}
          </div>
        </div>
    }
}

#[function_component(UnShieldedAccount)]
pub fn unshielded_account(
    UnShieldAccountProps {
        address,
        balance,
        deposit_clicked,
    }: &UnShieldAccountProps,
) -> Html {
    // State to hold the shielded address
    let shielded_address = use_state(|| "".to_string());

    // State to hold the deposit amount (dummy value for now)
    let deposit_amount = use_state(|| 0u64);

    // Handle address input change
    let on_address_change = {
        let shielded_address = shielded_address.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                shielded_address.set(input.value());
            }
        })
    };

    // Handle deposit amount input change
    let on_deposit_amount_change = {
        let deposit_amount = deposit_amount.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                deposit_amount.set(input.value().parse().unwrap_or_default());
            }
        })
    };

    // Handle deposit button click
    let on_click = {
        let deposit_clicked = deposit_clicked.clone();
        let shielded_address = shielded_address.clone();
        let deposit_amount = deposit_amount.clone();
        Callback::from(move |_| {
            deposit_clicked.emit((shielded_address.to_string(), *deposit_amount));
            shielded_address.set("".to_string());
            deposit_amount.set(0);
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
                    value={shielded_address.to_string()}
                />
            </div>
            <div>
                <input
                    id="deposit_amount"
                    type="text"
                    placeholder="Enter Deposit amount"
                    oninput={on_deposit_amount_change}
                    value={if *deposit_amount == 0 { "".to_string() } else { deposit_amount.to_string() }}
                />
            </div>
            <div class = "deposit-button">
                <button onclick={on_click} >
                    {"Deposit"}
                </button>
            </div>
        </div>
    }
}

#[function_component(ShieldedAccount)]
pub fn shielded_account(
    ShieldAccountProps {
        address,
        deposited,
        withdraw_clicked,
    }: &ShieldAccountProps,
) -> Html {
    // Handle withdraw button click
    let on_click = {
        let withdraw_clicked = withdraw_clicked.clone();
        let address = address.clone();
        Callback::from(move |_| {
            withdraw_clicked.emit(address.clone());
        })
    };

    html! {
        <div>
            <div>
                {address.clone()}{" : "}{*deposited}<strong>{" ETH"}</strong>
            </div>
            <div class = "withdraw-button">
                <button onclick={on_click} >
                    {"Withdraw"}
                </button>
            </div>
        </div>
    }
}
