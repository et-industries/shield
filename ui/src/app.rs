use crate::bindgen::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let unshielded_accounts =
        use_state(|| vec!["0x1234....5678".to_string(), "0xabcd....efgh".to_string()]);
    let shielded_accounts =
        use_state(|| vec!["0x1234....5678".to_string(), "0xabcd....efgh".to_string()]);
    {
        let mut unshielded_accounts = unshielded_accounts.to_vec();
        use_effect(move || {
            spawn_local(async move {
                let res = invoke_without_args("get_default_account").await.as_string();
                unshielded_accounts.push(res.unwrap_or("0".to_string()));
            });
        });
    }

    html! {
        <main class="container">
            <div class="unshielded-accounts">
                <h2 class="accounts-title">{"Unshielded accounts"}</h2>
                {unshielded_accounts.iter().map(|x| {
                    html! {
                        <div class="accounts-item">
                            <UnShieldedAccount />
                        </div>
                    }
                }).collect::<Html>()}
            </div>
            <div class="shielded-accounts">
                <h2 class="accounts-title">{"Shielded accounts"}</h2>
                {shielded_accounts.iter().map(|x| {
                    html! {
                        <div class="accounts-item">
                            <ShieldedAccount />
                        </div>
                    }
                }).collect::<Html>()}
            </div>
        </main>
    }
}

#[function_component(UnShieldedAccount)]
pub fn unshielded_account() -> Html {
    // State to hold the shielded address
    let unshielded_address = use_state(|| "0x12345....67890".to_string());

    // State to hold the shielded address
    let shielded_address = use_state(|| "0x12345....67890".to_string());

    // State to hold the balance (dummy value for now)
    let balance = use_state(|| "100.00".to_string());

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
        let balance = balance.clone();
        Callback::from(move |_| {
            // Dummy deposit logic
            balance.set("0.00 ETH".to_string());
        })
    };

    html! {
        <div>
            <div>
                {(*unshielded_address).clone()}{" : "}{(*balance).clone()}<strong>{" ETH"}</strong>
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
            <div style="margin-top: 10px;">
                <button
                    onclick={on_deposit}
                    style="padding: 8px 12px; margin-right: 10px; background-color: green; color: white; border: none; border-radius: 4px; cursor: pointer;">
                    {"Deposit"}
                </button>
            </div>
        </div>
    }
}

#[function_component(ShieldedAccount)]
pub fn shielded_account() -> Html {
    // State to hold the shielded address
    let shielded_address = use_state(|| "0x12345....67890".to_string());

    // State to hold the deposit amount (dummy value for now)
    let deposited_amount = use_state(|| "10.00".to_string());

    // Dummy actions for Withdraw button
    let on_withdraw = {
        let deposited_amount = deposited_amount.clone();
        Callback::from(move |_| {
            // Dummy withdraw logic
            deposited_amount.set("0.00".to_string());
        })
    };

    html! {
        <div>
            <div>
                {(*shielded_address).clone()}{" : "}{(*deposited_amount).clone()}<strong>{" ETH"}</strong>
            </div>
            <div style="margin-top: 10px;">
                <button
                    onclick={on_withdraw}
                    style="padding: 8px 12px; background-color: red; color: white; border: none; border-radius: 4px; cursor: pointer;">
                    {"Withdraw"}
                </button>
            </div>
        </div>
    }
}
