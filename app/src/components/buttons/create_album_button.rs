use crate::{
    components::{dialog::ConfirmDialog, Button, IconCreate},
    hooks::use_overlay,
    WASM_CLIENT,
};
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CreateAlbumButtonProps {
    pub on_created: Callback<String>,
}

#[function_component(CreateAlbumButton)]
pub fn create_album_button(props: &CreateAlbumButtonProps) -> Html {
    let dialog_state = use_state(|| false);
    let (_, set_overlay) = use_overlay();
    let album_title_ref = use_node_ref();

    let show_dialog = {
        let dialog_state = dialog_state.clone();
        move |_| {
            dialog_state.set(true);
        }
    };

    let hide_dialog = {
        let dialog_state = dialog_state.clone();
        move |_| {
            dialog_state.set(false);
        }
    };

    let create_album = {
        let on_created = props.on_created.clone();
        let dialog_state = dialog_state.clone();
        let album_title_ref = album_title_ref.clone();

        move |_| {
            let album_title_input = album_title_ref.cast::<HtmlInputElement>();

            if let Some(album_title_input) = album_title_input {
                let album_title = album_title_input.value();
                if !album_title.is_empty() {
                    let on_created = on_created.clone();
                    let dialog_state = dialog_state.clone();
                    let set_overlay = set_overlay.clone();

                    set_overlay.emit(true);

                    wasm_bindgen_futures::spawn_local(async move {
                        let id = WASM_CLIENT.create_album(&album_title, vec![]).await.unwrap();
                        dialog_state.set(false);
                        set_overlay.emit(false);
                        on_created.emit(id)
                    });
                }
            }
        }
    };

    let dialog_visible = *dialog_state;

    html! {
        <>
            <Button label={"New album"} on_click={show_dialog}>
                <IconCreate/>
            </Button>
            // <div class="create-album-button" onclick={show_dialog}>
            //     <IconCreate/>
            //     <h2>{"Album"}</h2>
            // </div>
            <ConfirmDialog
                visible={dialog_visible}
                title="Create album"
                confirm_action={create_album}
                cancel_action={hide_dialog}>
                <label>{"Title"}
                    <input ref={album_title_ref} type="text"/>
                </label>
            </ConfirmDialog>
        </>
    }
}
