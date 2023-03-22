use yew::prelude::*;

#[function_component(NotFoundPage)]
pub fn not_found_page() -> Html {
    html! {
        <>
            <h1>{ "404" }</h1>
        </>
    }
}
