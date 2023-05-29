use crate::{
    components::{icons::IconRemove, ConfirmButton},
    hooks::use_overlay,
    WASM_CLIENT,
};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct RemoveFromAlbumButtonProps {
    pub album_id: String,
    pub photo_ids: Vec<String>,
    pub on_removed: Callback<()>,
}

#[function_component(RemoveFromAlbumButton)]
pub fn remove_from_album_button(props: &RemoveFromAlbumButtonProps) -> Html {
    let (_, set_overlay) = use_overlay();

    let on_click = {
        let album_id = props.album_id.clone();
        let photo_ids = props.photo_ids.clone();
        let on_removed = props.on_removed.clone();

        move |_| {
            let album_id = album_id.clone();
            let photo_ids = photo_ids.clone();
            let on_removed = on_removed.clone();
            let set_overlay = set_overlay.clone();

            set_overlay.emit(true);

            wasm_bindgen_futures::spawn_local(async move {
                WASM_CLIENT
                    .remove_photos_from_album(&album_id, &photo_ids)
                    .await
                    .unwrap();
                set_overlay.emit(false);
                on_removed.emit(())
            });
        }
    };

    let n_selected_photos = props.photo_ids.len();

    html! {
        <ConfirmButton label="Remove from album"
            on_click={on_click}
            confirm_dialog_title="Remove photos from album?"
            confirm_dialog_body={format!("{n_selected_photos} photos will be removed.")}
            >
            <IconRemove/>
        </ConfirmButton>
    }
}
