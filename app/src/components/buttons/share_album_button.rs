use crate::{
    components::{buttons::Button, dialog::ConfirmDialog, IconShare},
    hooks::use_album_share,
    WASM_CLIENT,
};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ShareAlbumButtonProps {
    pub album_id: AttrValue,
    pub on_submitted: Callback<()>,
}

#[function_component(ShareAlbumButton)]
pub fn share_album_button(props: &ShareAlbumButtonProps) -> Html {
    let (share, refresh_share) = use_album_share(props.album_id.to_string());
    let dialog_state = use_state(|| false);

    weblog::console_log!(format!("{:?}", share));

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

    let update_album_share_settings = {
        let album_id = props.album_id.clone();
        let on_submitted = props.on_submitted.clone();
        let dialog_state = dialog_state.clone();
        let refresh_share = refresh_share.clone();

        move |_| {
            let album_id = album_id.clone();
            let on_submitted = on_submitted.clone();
            let dialog_state = dialog_state.clone();
            let refresh_share = refresh_share.clone();

            wasm_bindgen_futures::spawn_local(async move {
                WASM_CLIENT.upsert_share(&album_id, "").await.unwrap();
                dialog_state.set(false);
                refresh_share.emit(());
                on_submitted.emit(());
            });
        }
    };

    let dialog_visible = *dialog_state;

    html! {
        <>
            <Button label={"Share"} on_click={show_dialog}>
                <IconShare/>
            </Button>
            <ConfirmDialog
                visible={dialog_visible}
                title="Share album"
                confirm_action={update_album_share_settings}
                cancel_action={hide_dialog}>
                <form>
                    <label>
                        <input type="checkbox"/>
                        {"Share via URL"}
                    </label>
                    <label>
                        <input type="checkbox"/>
                        {"Require password"}
                    </label>
                    <label>
                        <input type="text"/>
                        {"Require password"}
                    </label>
                </form>
            </ConfirmDialog>
        </>
    }
}
