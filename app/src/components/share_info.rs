use crate::{
    components::{ConfirmButton, IconDelete, ShareUrl},
    hooks::use_album,
    models::LibraryShare,
    WASM_CLIENT,
};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ShareInfoProps {
    pub share: LibraryShare,
    pub on_deleted: Callback<()>,
}

#[function_component(ShareInfo)]
pub fn share_info(props: &ShareInfoProps) -> Html {
    let (album, _) = use_album(props.share.album_id.clone());

    let delete_share = {
        let share_id = props.share.id.clone();
        let on_deleted = props.on_deleted.clone();

        move |_| {
            let share_id = share_id.clone();
            let on_deleted = on_deleted.clone();

            wasm_bindgen_futures::spawn_local(async move {
                WASM_CLIENT.delete_share(&share_id).await.unwrap();
                on_deleted.emit(());
            });
        }
    };

    if let Some(album) = (*album).clone() {
        html! {
            <div class="share-info">
                <span>{&album.title}</span>
                <ConfirmButton label=""
                    on_click={delete_share}
                    confirm_dialog_title="Delete share?">
                    <IconDelete/>
                </ConfirmButton>
                <ShareUrl share_id={props.share.id.clone()}/>
            </div>
        }
    } else {
        html! {}
    }
}
