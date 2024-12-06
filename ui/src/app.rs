use crate::bindgen::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let unshielded_accounts = use_state(|| Vec::new());
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
                            <p>{ &*x }</p>
                        </div>
                    }
                }).collect::<Html>()}
            </div>
            <div class="shielded-accounts">
                <h2 class="accounts-title">{"Shielded accounts"}</h2>
            </div>
        </main>
    }
}
