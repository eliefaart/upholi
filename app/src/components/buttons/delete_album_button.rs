use crate::{
    components::{icons::IconDelete, ConfirmButton},
    hooks::use_overlay,
    WASM_CLIENT,
};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct DeleteAlbumButtonProps {
    pub album_id: String,
    pub on_deleted: Callback<()>,
}

#[function_component(DeleteAlbumButton)]
pub fn delete_album_button(props: &DeleteAlbumButtonProps) -> Html {
    let (_, set_overlay) = use_overlay();
    let album_id = props.album_id.clone();
    let on_deleted = props.on_deleted.clone();

    let delete_album = move |_| {
        let set_overlay = set_overlay.clone();
        let album_id = album_id.clone();
        let on_deleted = on_deleted.clone();

        set_overlay.emit(true);

        wasm_bindgen_futures::spawn_local(async move {
            WASM_CLIENT.delete_album(&album_id).await.unwrap();
            set_overlay.emit(false);
            on_deleted.emit(())
        });
    };

    html! {
        <ConfirmButton label="Delete album"
            on_click={delete_album}
            confirm_dialog_title="Delete album?">
            <IconDelete/>
        </ConfirmButton>
    }
}
