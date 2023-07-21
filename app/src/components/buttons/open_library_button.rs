use crate::{
    components::DropUpload,
    hooks::{use_library_photos, use_photo_src},
    Route,
};
use yew::prelude::*;
use yew_router::prelude::use_navigator;

#[function_component(OpenLibraryButton)]
pub fn open_library_button() -> Html {
    let navigator = use_navigator().unwrap();
    let (library, _) = use_library_photos();

    let src = use_photo_src(
        &library.first().map(|p| p.id.clone()).unwrap_or_default(),
        upholi_lib::PhotoVariant::Thumbnail,
    );

    let open_library = {
        use_callback(
            |_, navigator| {
                navigator.push(&Route::Library);
            },
            navigator,
        )
    };

    // center center no-repeat center/cover

    html! {
        <DropUpload class="open-library-button">
            <div
                class="background"
                onclick={open_library}
                style={format!("background: linear-gradient(85deg, rgb(36, 176, 150), rgba(0, 0, 0, 0.2)), center/cover url({})", &(*src))}
                //style={format!("background-image: url({})", &(*src))}
                >
                <h2>{"Library"}</h2>
            </div>
        </DropUpload>
    }
}
