use api_client::ApiClient;
use once_cell::sync::Lazy;
use pages::login::LoginPage;
use pages::shared::SharedPage;
use pages::{albums::AlbumsPage, home::HomePage, not_found::NotFoundPage, photo::PhotoPage};
use wasm_bindgen::UnwrapThrowExt;
use wasm_client::WasmClient;
use yew::prelude::*;
use yew_router::prelude::*;
use yew_router::{BrowserRouter, Routable, Switch};

use crate::components::dialog::Dialog;

mod api_client;
mod components;
mod encryption;
mod exif;
mod hashing;
mod images;
mod keys;
mod models;
mod multipart;
mod pages;
mod repository;
mod wasm_client;

static ORIGIN: Lazy<String> = Lazy::new(|| {
    let window = web_sys::window().expect_throw("Could not find global 'window'.");
    let location = window.location();
    location.origin().expect_throw("could not determine 'origin'.")
});
static API_CLIENT: Lazy<ApiClient> = Lazy::new(|| ApiClient::new(&format!("{}/api", ORIGIN.as_str())));
static WASM_CLIENT: Lazy<WasmClient> = Lazy::new(|| WasmClient::new(&API_CLIENT));

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/albums")]
    Albums,
    #[at("/photo/:id")]
    Photo { id: String },
    #[at("/shared")]
    Shared,
    #[at("/login")]
    Login,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <HomePage/> },
        Route::Albums => html! { <AlbumsPage/> },
        Route::Photo { id } => html! { <PhotoPage id={id} /> },
        Route::Shared => html! { <SharedPage/> },
        Route::Login => html! { <LoginPage/> },
        Route::NotFound => html! { <NotFoundPage/> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <header>
                <nav>
                    <Link<Route> to={Route::Home}>{ "Library" }</Link<Route>>
                    <Link<Route> to={Route::Albums}>{ "Albums" }</Link<Route>>
                    <Link<Route> to={Route::Shared}>{ "Shared" }</Link<Route>>
                </nav>
            </header>
            <main>
                <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
            </main>

            <Dialog visible={false} title="Dialog title">
                <span>{"Dialog body"}</span>
            </Dialog>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
