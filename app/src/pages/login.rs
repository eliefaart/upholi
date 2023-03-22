use crate::WASM_CLIENT;
use yew::prelude::*;

#[function_component(LoginPage)]
pub fn login_page() -> Html {
    let on_click = Callback::from(move |_| {
        wasm_bindgen_futures::spawn_local(async move {
            WASM_CLIENT.login("eric", "a").await.unwrap();
        });
    });
    html! {
        <>
            <h1>{ "Login" }</h1>
            <input type="text"/>
            <input type="password"/>
            <button onclick={on_click}>{"Log in"}</button>
        </>
    }
}
