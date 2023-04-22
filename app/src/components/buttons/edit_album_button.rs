use crate::{
    components::{buttons::Button, dialog::ConfirmDialog, IconHashTag},
    hooks::use_album,
    WASM_CLIENT,
};
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct EditAlbumButtonProps {
    pub album_id: AttrValue,
    pub on_submitted: Callback<()>,
}

#[function_component(EditAlbumButton)]
pub fn edit_album_button(props: &EditAlbumButtonProps) -> Html {
    let (album, refresh_album) = use_album(props.album_id.to_string());
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
        let album_id = props.album_id.clone();
        let on_submitted = props.on_submitted.clone();
        let dialog_state = dialog_state.clone();
        let album_title_ref = album_title_ref.clone();
        let refresh_album = refresh_album.clone();

        move |_| {
            let album_title_input = album_title_ref.cast::<HtmlInputElement>();

            if let Some(album_title_input) = album_title_input {
                let album_title = album_title_input.value();
                if !album_title.is_empty() {
                    let album_id = album_id.clone();
                    let on_submitted = on_submitted.clone();
                    let dialog_state = dialog_state.clone();
                    let refresh_album = refresh_album.clone();

                    wasm_bindgen_futures::spawn_local(async move {
                        let id = WASM_CLIENT
                            .update_album_title_tags(&album_id, &album_title, vec![])
                            .await
                            .unwrap();
                        dialog_state.set(false);
                        refresh_album.emit(());
                        on_submitted.emit(id);
                    });
                }
            }
        }
    };

    let album_title = match (*album).clone() {
        Some(album) => album.title,
        None => String::new(),
    };
    let dialog_visible = *dialog_state;

    html! {
        <>
            <Button label={"Edit album"} on_click={show_dialog}>
                <IconHashTag/>
            </Button>
            <ConfirmDialog
                visible={dialog_visible}
                title="Edit album"
                confirm_action={create_album}
                cancel_action={hide_dialog}>
                <label>{"Title"}
                    <input ref={album_title_ref} type="text" value={album_title}/>
                </label>
            </ConfirmDialog>
        </>
    }
}
