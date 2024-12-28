use std::ops::Deref;

use crate::{
    bindgen::*,
    util::{
        DepositParams, GetBalanceParams, ShieldAccountProps, ShieldedAccountState,
        UnShieldAccountProps, UnShieldedAccountState, WithdrawParams,
    },
};
use serde_wasm_bindgen::to_value;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let shielded_accounts = use_state(|| vec![]);
    let unshielded_accounts = use_state(|| Vec::new());
    let deposit_amount = use_state(|| 0);

    {
        let unshielded_accounts = unshielded_accounts.clone();
        let deposit_amount = deposit_amount.clone();
        spawn_local(async move {
            let amount_val = invoke_without_args("get_default_amount").await.as_string();
            let amount = amount_val
                .clone()
                .unwrap_or("0".to_string())
                .parse::<u64>()
                .unwrap();

            deposit_amount.set(amount);

            let account_val = invoke_without_args("get_default_account").await.as_string();
            let account = account_val
                .clone()
                .unwrap_or("0".to_string())
                .parse::<u64>()
                .unwrap();

            let js_args = to_value(&GetBalanceParams { account }).unwrap();
            let balance = invoke("get_balance", js_args).await.as_string();
            let balance = balance.unwrap_or("0".to_string()).parse::<u64>().unwrap();
            let state =
                UnShieldedAccountState::new(account_val.unwrap_or("0".to_string()), balance);
            unshielded_accounts.set(vec![state]);
        });
    }

    let deposit_click = {
        let shielded_accounts = shielded_accounts.clone();
        Callback::from(move |new_shielded_addr| {
            let shielded_accounts = shielded_accounts.clone();
            spawn_local(async move {
                let mut accounts = shielded_accounts.to_vec();

                let account_id = accounts.len();

                let js_args = to_value(&DepositParams {
                    recipiant: account_id as u64,
                })
                .unwrap();
                let nullifier_str = invoke("deposit", js_args).await;

                accounts.push(ShieldedAccountState::new(
                    account_id,
                    new_shielded_addr,
                    false,
                    nullifier_str.as_string().unwrap(),
                ));
                shielded_accounts.set(accounts);
            });
        })
    };

    let withdraw_click = {
        let shielded_accounts = shielded_accounts.clone();
        Callback::from(move |(id, nullifier)| {
            let shielded_accounts = shielded_accounts.clone();
            let js_args = to_value(&WithdrawParams::from_hex_str(nullifier)).unwrap();
            spawn_local(async move {
                let withdrawn_res = invoke("withdraw", js_args).await;

                let mut accounts = shielded_accounts.to_vec();
                for account in accounts.iter_mut() {
                    if account.id == id {
                        account.withdraw_success = withdrawn_res.as_bool().unwrap();
                    }
                }
                shielded_accounts.set(accounts);
            });
        })
    };

    html! {
        <div class="container">
          <h1 class="accounts-title">{"Unshielded accounts"}</h1>
          <div class="accounts-list">
            {unshielded_accounts.iter().map(|UnShieldedAccountState { address, balance }| {
              html! {
                <div class="accounts-item">
                  <UnShieldedAccount address={address.clone()} balance={balance} deposit_clicked={deposit_click.clone()} />
                </div>
              }
            }).collect::<Html>()}
          </div>

          <h1 class="accounts-title">{"Shielded accounts"}</h1>
          <div class="accounts-list">
            {shielded_accounts.iter().map(|ShieldedAccountState {id, address, withdraw_success, nullifier }| {
              html! {
                <div class="accounts-item">
                    <ShieldedAccount
                        id={id}
                        address={address.clone()}
                        deposit_amount={*deposit_amount.deref()}
                        withdraw_success={withdraw_success}
                        withdraw_clicked={withdraw_click.clone()}
                        nullifier = {nullifier.clone()}
                    />
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

    // Handle address input change
    let on_address_change = {
        let shielded_address = shielded_address.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                shielded_address.set(input.value());
            }
        })
    };

    // Handle deposit button click
    let on_click = {
        let deposit_clicked = deposit_clicked.clone();
        let shielded_address = shielded_address.clone();
        Callback::from(move |_| {
            deposit_clicked.emit(shielded_address.to_string());
            shielded_address.set("".to_string());
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
        id,
        address,
        deposit_amount,
        withdraw_success,
        withdraw_clicked,
        nullifier,
    }: &ShieldAccountProps,
) -> Html {
    // Handle withdraw button click
    let on_click = {
        let withdraw_clicked = withdraw_clicked.clone();
        let id = id.clone();
        let nullifier = nullifier.clone();
        Callback::from(move |_| {
            withdraw_clicked.emit((id, nullifier.clone()));
        })
    };

    html! {
        <div>
            <div>
                {address.clone()}{" : "}{*deposit_amount}<strong>{" ETH"}</strong>
            </div>
            <div class = "withdraw-button">
                <button onclick={on_click} disabled={*withdraw_success} >
                    {format!("Withdraw{}", if *withdraw_success { " (Unshielded)" } else { "" })}
                </button>
            </div>
        </div>
    }
}
