use crate::{
    components::{layouts::PageLayout, ShareInfo},
    hooks::use_shares::use_shares,
};
use yew::prelude::*;

#[function_component(SharedPage)]
pub fn shared_page() -> Html {
    let (shares, refresh) = use_shares();

    let shares = (*shares)
        .clone()
        .into_iter()
        .map(|share| {
            html! {
                <ShareInfo
                    share={share}
                    on_deleted={refresh.clone()}
                />
            }
        })
        .collect::<Html>();

    html! {
        <PageLayout title="Shared">
            <div class="shared">
                {shares}
            </div>
        </PageLayout>
    }
}
