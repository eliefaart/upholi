use crate::{
    components::{Button, IconDelete, ShareUrl},
    hooks::use_album,
    models::LibraryShare,
    WASM_CLIENT,
};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ShareInfoProps {
    pub share: LibraryShare,
}

#[function_component(ShareInfo)]
pub fn share_info(props: &ShareInfoProps) -> Html {
    let (album, _) = use_album(props.share.album_id.clone());

    let delete_share = {
        let share_id = props.share.id.clone();

        move |_| {
            let share_id = share_id.clone();

            wasm_bindgen_futures::spawn_local(async move {
                WASM_CLIENT.delete_share(&share_id).await.unwrap();
            });
        }
    };

    if let Some(album) = (*album).clone() {
        html! {
            <div class="share-info">
                <span>{&album.title}</span>
                <Button label="" on_click={delete_share}>
                    <IconDelete/>
                </Button>
                <ShareUrl share_id={props.share.id.clone()}/>
            </div>
        }
    } else {
        html! {}
    }
}
