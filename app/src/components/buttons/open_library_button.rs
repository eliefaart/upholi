use crate::Route;
use yew::prelude::*;
use yew_router::prelude::use_navigator;

#[function_component(OpenLibraryButton)]
pub fn open_library_button() -> Html {
    let navigator = use_navigator().unwrap();

    let open_library = {
        use_callback(
            |_, navigator| {
                navigator.push(&Route::Library);
            },
            navigator,
        )
    };

    html! {
        <div class="open-library-button" onclick={open_library}>
            <h1>{"Library"}</h1>
        </div>
    }
}
