use crate::{
    components::{buttons::Button, dialog::ConfirmDialog, IconCreate},
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

                    wasm_bindgen_futures::spawn_local(async move {
                        let id = WASM_CLIENT.create_album(&album_title, vec![]).await.unwrap();
                        dialog_state.set(false);
                        on_created.emit(id)
                    });
                }
            }
        }
    };

    let dialog_visible = *dialog_state;

    html! {
        <>
            <Button label={"Create album"} on_click={show_dialog}>
                <IconCreate/>
            </Button>
            <ConfirmDialog
                    visible={dialog_visible}
                    title="Create album"
                    confirm_action={create_album}
                    cancel_action={hide_dialog}>
                    <label>{"Album title"}
                        <input ref={album_title_ref} type="text" />
                    </label>
            </ConfirmDialog>
        </>
    }
}
