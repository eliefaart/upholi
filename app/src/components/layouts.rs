use crate::{components::dialog::Dialog, Route};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PageLayoutProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub header_actions: Children,
}

#[function_component(PageLayout)]
pub fn page_layout(props: &PageLayoutProps) -> Html {
    html! {
        <>
        <header>
            <nav>
                <Link<Route> to={Route::Home}>{ "Library" }</Link<Route>>
                <Link<Route> to={Route::Albums}>{ "Albums" }</Link<Route>>
                <Link<Route> to={Route::Shared}>{ "Shared" }</Link<Route>>
            </nav>
            <div class="space"/>
            <div class="actions">
                {props.header_actions.clone()}
            </div>
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
