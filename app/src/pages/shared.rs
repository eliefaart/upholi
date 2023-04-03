use crate::{components::layouts::PageLayout, hooks::use_shares::use_shares, WASM_CLIENT};
use yew::prelude::*;

#[function_component(SharedPage)]
pub fn shared_page() -> Html {
    let shares = use_shares();

    let shares = (*shares)
        .clone()
        .into_iter()
        .map(|share| {
            let delete_share = {
                let share_id = share.id.clone();

                move |_| {
                    let share_id = share_id.clone();

                    wasm_bindgen_futures::spawn_local(async move {
                        WASM_CLIENT.delete_share(&share_id).await.unwrap();
                    });
                }
            };
            html! {
                <div onclick={delete_share}>{share.id}</div>
            }
        })
        .collect::<Html>();

    html! {
        <PageLayout>
            <h1>{"Shared"}</h1>
            {shares}
        </PageLayout>
    }
}
