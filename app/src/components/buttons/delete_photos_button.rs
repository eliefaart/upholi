use crate::{
    components::{dialog::ConfirmDialog, icons::IconDelete, buttons::Button},
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
    let dialog_visible_state = use_state(|| false);

    let show_dialog_state = dialog_visible_state.clone();
    let show_dialog = move |_| {
        show_dialog_state.set(true);
    };

    let hide_dialog_state = dialog_visible_state.clone();
    let hide_dialog = move |_| {
        hide_dialog_state.set(false);
    };

    let selected_photos = props.selected_photos.clone();
    let on_deleted = props.on_deleted.clone();
    let on_deleted_dialog_state = dialog_visible_state.clone();
    let delete_photos = move |_| {
        let selected_photos = selected_photos.clone();
        let on_deleted = on_deleted.clone();
        let on_deleted_dialog_state = on_deleted_dialog_state.clone();

        wasm_bindgen_futures::spawn_local(async move {
            WASM_CLIENT.delete_photos(&selected_photos).await.unwrap();
            on_deleted_dialog_state.set(false);
            on_deleted.emit(())
        });
    };

    let dialog_visible = *dialog_visible_state;
    let n_selected_photos = props.selected_photos.len();

    html! {
        <Button label={"Delete"} on_click={show_dialog}>
            <IconDelete/>
            <ConfirmDialog
                visible={dialog_visible}
                title="Delete photos?"
                confirm_action={delete_photos}
                cancel_action={hide_dialog}>
                {html! {
                    <span>{&format!("{n_selected_photos} photos will be deleted.")}</span>
                }}
            </ConfirmDialog>
        </Button>
    }
}
