use crate::{
    components::{buttons::Button, dialog::ConfirmDialog, IconShare, ShareUrl},
    hooks::use_album_share,
    WASM_CLIENT,
};
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Clone, Default)]
pub struct ShareFormData {
    pub share: bool,
    pub password: String,
}

#[derive(Properties, PartialEq)]
pub struct ShareAlbumButtonProps {
    pub album_id: AttrValue,
    pub on_submitted: Callback<()>,
}

#[function_component(ShareAlbumButton)]
pub fn share_album_button(props: &ShareAlbumButtonProps) -> Html {
    let (share, refresh_share) = use_album_share(props.album_id.to_string());
    let form = use_state(ShareFormData::default);
    let dialog_state = use_state(|| false);
    let form_checkbox_share_ref = use_node_ref();
    let form_text_password_ref = use_node_ref();

    {
        // Re-sync form data with share when the share changes.
        let form = form.clone();
        use_effect_with_deps(
            move |share| {
                let mut form_data = (*form).clone();
                let (_, password) = match &**share {
                    Some(share) => (!share.password.is_empty(), share.password.to_string()),
                    None => (false, String::new()),
                };

                form_data.share = share.is_some();
                form_data.password = password;

                form.set(form_data);
            },
            share.clone(),
        )
    }

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
        let share = share.clone();
        let album_id = props.album_id.clone();
        let on_submitted = props.on_submitted.clone();

        let form_checkbox_share_ref = form_checkbox_share_ref.clone();
        let form_text_password_ref = form_text_password_ref.clone();

        move |_| {
            let form_checkbox_share_ref = form_checkbox_share_ref.cast::<HtmlInputElement>();
            let form_text_password_ref = form_text_password_ref.cast::<HtmlInputElement>();

            if let (Some(form_checkbox_share_ref), Some(form_text_password_ref)) =
                (form_checkbox_share_ref, form_text_password_ref)
            {
                let share = (*share).clone();
                let album_id = album_id.clone();
                let on_submitted = on_submitted.clone();
                let refresh_share = refresh_share.clone();

                let do_share = form_checkbox_share_ref.checked();
                let password = form_text_password_ref.value();

                wasm_bindgen_futures::spawn_local(async move {
                    if do_share {
                        WASM_CLIENT.upsert_share(&album_id, &password).await.unwrap();
                    } else if let Some(share) = share {
                        WASM_CLIENT.delete_share(&share.id).await.unwrap();
                    }

                    refresh_share.emit(());
                    on_submitted.emit(());
                });
            }
        }
    };

    let toggle_form_shared = {
        let form = form.clone();

        move |_| {
            let mut form_data = (*form).clone();
            form_data.share = !form_data.share;

            if !form_data.share {
                form_data.password = String::new();
            }

            form.set(form_data);
        }
    };

    let dialog_visible = *dialog_state;
    let is_shared = form.share;
    let password = form.password.clone();

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
                    <label class="checkbox-input">
                        <input type="checkbox"
                            ref={form_checkbox_share_ref}
                            checked={is_shared}
                            onchange={toggle_form_shared.clone()}/>
                        <span>{"Share via URL"}</span>
                    </label>
                    <label style={if !is_shared {"display: none;".to_string()} else {String::new()}}>
                        {"Password"}
                        <input type="text" ref={form_text_password_ref} value={password}/>
                    </label>
                    {html! {
                        if let Some(share) = &(*share) {
                            <label style={if !is_shared {"display: none;".to_string()} else {String::new()}}>
                                {"URL"}
                                <ShareUrl share_id={share.id.clone()}/>
                            </label>
                        }
                    }}
                </form>
            </ConfirmDialog>
        </>
    }
}
