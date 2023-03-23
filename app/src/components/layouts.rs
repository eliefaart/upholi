use crate::{components::dialog::Dialog, Route};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PageLayoutProps {
    #[prop_or_default]
    pub children: Children,
    pub header_actions_left: Option<Children>,
    pub header_actions_right: Option<Children>,
}

#[function_component(PageLayout)]
pub fn page_layout(props: &PageLayoutProps) -> Html {
    let header = html! {
        <nav>
            <Link<Route> to={Route::Home}>{ "Library" }</Link<Route>>
            <Link<Route> to={Route::Albums}>{ "Albums" }</Link<Route>>
            <Link<Route> to={Route::Shared}>{ "Shared" }</Link<Route>>
        </nav>
    };

    let header_left = html! {
        if let Some(header_actions_left) = props.header_actions_left.clone() {
            <div class="actions">
                {header_actions_left}
            </div>
        } else {
            {header}
        }
    };

    let header_right = html! {
        if let Some(header_actions_right) = props.header_actions_right.clone() {
            <div class="actions">
                {header_actions_right}
            </div>
        }
    };

    html! {
        <>
        <header>
            {header_left}
            <div class="space"/>
            {header_right}
        </header>

        <main>
            {props.children.clone()}
        </main>

        <Dialog visible={false} title="Dialog title">
            <span>{"Dialog body"}</span>
        </Dialog>
        </>
    }
}
