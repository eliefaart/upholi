use crate::{
    components::{buttons::Button, dialog::ConfirmDialog, icons::IconDelete},
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
    let dialog_visible_state = use_state(|| false);

    let show_dialog_state = dialog_visible_state.clone();
    let show_dialog = move |_| {
        show_dialog_state.set(true);
    };

    let hide_dialog_state = dialog_visible_state.clone();
    let hide_dialog = move |_| {
        hide_dialog_state.set(false);
    };

    let album_id = props.album_id.clone();
    let on_deleted = props.on_deleted.clone();
    let on_deleted_dialog_state = dialog_visible_state.clone();
    let delete_album = move |_| {
        let album_id = album_id.clone();
        let on_deleted = on_deleted.clone();
        let on_deleted_dialog_state = on_deleted_dialog_state.clone();

        wasm_bindgen_futures::spawn_local(async move {
            WASM_CLIENT.delete_album(&album_id).await.unwrap();
            on_deleted_dialog_state.set(false);
            on_deleted.emit(())
        });
    };

    let dialog_visible = *dialog_visible_state;

    html! {
        <>
            <Button label={"Delete album"} on_click={show_dialog}>
                <IconDelete/>
            </Button>
            <ConfirmDialog
                visible={dialog_visible}
                title="Delete album?"
                confirm_action={delete_album}
                cancel_action={hide_dialog}/>
        </>
    }
}
