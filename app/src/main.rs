use api_client::ApiClient;
use once_cell::sync::Lazy;
use pages::{AlbumPage, AlbumsPage, HomePage, LoginPage, NotFoundPage, PhotoPage, RegisterPage, SharePage, SharedPage};
use wasm_bindgen::UnwrapThrowExt;
use wasm_client::WasmClient;
use web_sys::Document;
use yew::prelude::*;
use yew_router::{BrowserRouter, Routable, Switch};

mod api_client;
mod components;
mod encryption;
mod exif;
mod hashing;
mod hooks;
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

pub fn get_document() -> Document {
    web_sys::window()
        .expect("Could not find global 'window'.")
        .document()
        .expect("No document")
}

#[derive(Debug, Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/albums")]
    Albums,
    #[at("/album/:id")]
    Album { id: String },
    #[at("/photo/:id")]
    Photo { id: String },
    #[at("/shared")]
    Shared,
    #[at("/share/:id")]
    Share { id: String },
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <HomePage/> },
        Route::Albums => html! { <AlbumsPage/> },
        Route::Album { id } => html! { <AlbumPage id={id}/> },
        Route::Photo { id } => html! { <PhotoPage id={id} /> },
        Route::Shared => html! { <SharedPage/> },
        Route::Share { id } => html! {<SharePage id={id}/>},
        Route::Login => html! { <LoginPage/> },
        Route::Register => html! { <RegisterPage/> },
        Route::NotFound => html! { <NotFoundPage/> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
            <div id="modal-host"/>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
