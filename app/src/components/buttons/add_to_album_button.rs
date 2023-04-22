use crate::{
    components::{buttons::Button, dialog::ConfirmDialog, icons::IconAddToAlbum, PickAlbum},
    WASM_CLIENT,
};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AddToAlbumButtonProps {
    pub photo_ids: Vec<String>,
    pub on_added: Callback<()>,
}

#[function_component(AddToAlbumButton)]
pub fn add_to_album_button(props: &AddToAlbumButtonProps) -> Html {
    let dialog_state = use_state(|| false);
    let selected_album: UseStateHandle<Option<String>> = use_state(|| None);

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

    let add_to_album = {
        let photo_ids = props.photo_ids.clone();
        let on_added = props.on_added.clone();
        let dialog_state = dialog_state.clone();
        let selected_album = selected_album.clone();
        move |_| {
            let photo_ids = photo_ids.clone();
            let on_added = on_added.clone();
            let dialog_state = dialog_state.clone();

            if let Some(selected_album) = (*selected_album).clone() {
                wasm_bindgen_futures::spawn_local(async move {
                    WASM_CLIENT
                        .add_photos_to_album(&selected_album, &photo_ids)
                        .await
                        .unwrap();
                    dialog_state.set(false);
                    on_added.emit(())
                });
            }
        }
    };

    let dialog_visible = *dialog_state;

    html! {
        <>
            <Button label={"Add to album"} on_click={show_dialog}>
                <IconAddToAlbum/>
            </Button>
            <ConfirmDialog
                    visible={dialog_visible}
                    title="Choose album"
                    confirm_action={add_to_album}
                    cancel_action={hide_dialog}>
                <PickAlbum selected_album={selected_album.clone()}/>
            </ConfirmDialog>
        </>
    }
}
