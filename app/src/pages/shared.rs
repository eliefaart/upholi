use crate::hooks::use_shares::use_shares;
use yew::prelude::*;

#[function_component(SharedPage)]
pub fn shared_page() -> Html {
    let shares = use_shares();

    let shares = (*shares)
        .clone()
        .into_iter()
        .map(|share| {
            html! {
                <div>{share.id}</div>
            }
        })
        .collect::<Html>();

    html! {
        <>
            <h1>{"Shared"}</h1>
            {shares}
        </>
    }
}
