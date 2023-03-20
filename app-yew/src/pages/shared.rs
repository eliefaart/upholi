use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SharedPageProps {}

#[function_component(SharedPage)]
pub fn shared_page(props: &SharedPageProps) -> Html {
    html! {
        <>
            <h1>{"Shared"}</h1>
        </>
    }
}
