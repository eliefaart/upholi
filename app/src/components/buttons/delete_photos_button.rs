use crate::{
    components::{icons::IconDelete, ConfirmButton},
    hooks::use_overlay,
    WASM_CLIENT,
};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct DeletePhotosButtonProps {
    pub selected_photos: Vec<String>,
    pub on_deleted: Callback<()>,
}

#[function_component(DeletePhotosButton)]
pub fn delete_photos_button(props: &DeletePhotosButtonProps) -> Html {
    let (_, set_overlay) = use_overlay();
    let selected_photos = props.selected_photos.clone();
    let on_deleted = props.on_deleted.clone();

    let delete_photos = move |_| {
        let selected_photos = selected_photos.clone();
        let on_deleted = on_deleted.clone();
        let set_overlay = set_overlay.clone();

        set_overlay.emit(true);

        wasm_bindgen_futures::spawn_local(async move {
            WASM_CLIENT.delete_photos(&selected_photos).await.unwrap();
            set_overlay.emit(false);
            on_deleted.emit(())
        });
    };

    let n_selected_photos = props.selected_photos.len();

    html! {
        <ConfirmButton label="Delete"
            on_click={delete_photos}
            confirm_dialog_title="Delete photos?"
            confirm_dialog_body={format!("{n_selected_photos} photos will be deleted.")}
            >
            <IconDelete/>
        </ConfirmButton>
    }
}
