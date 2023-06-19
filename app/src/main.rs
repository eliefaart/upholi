use crate::components::{FileUploader, Overlay};
use api_client::ApiClient;
use bounce::BounceRoot;
use once_cell::sync::Lazy;
use pages::{AlbumPage, AlbumsPage, HomePage, LoginPage, NotFoundPage, RegisterPage, SharePage, SharedPage};
use serde::{Deserialize, Serialize};
use wasm_bindgen::{prelude::wasm_bindgen, UnwrapThrowExt};
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

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = offerAsFileDownload)]
    fn offer_as_file_download(filename: &str, src: &str);
}

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
    #[at("/shared")]
    Shared,
    #[at("/s/:id")]
    Share { id: String },
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,
    #[not_found]
    #[at("/404")]
    NotFound,
}

/// Query string parameters
#[derive(Serialize, Deserialize)]
pub struct RouteQuery {
    /// Photo opened in the GalleryDetail component.
    pub photo_id: String,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <HomePage/> },
        Route::Albums => html! { <AlbumsPage/> },
        Route::Album { id } => html! { <AlbumPage id={id}/> },
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
        <BounceRoot>
            <BrowserRouter>
                <Switch<Route> render={switch} />
                <FileUploader/>
                <Overlay/>
                <div id="modal-host"/>
            </BrowserRouter>
        </BounceRoot>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
