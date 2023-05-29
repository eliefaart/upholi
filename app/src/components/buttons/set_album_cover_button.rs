use crate::{
    components::{buttons::Button, icons::IconImage},
    hooks::use_overlay,
    WASM_CLIENT,
};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SetAlbumCoverButtonProps {
    pub album_id: String,
    pub photo_id: String,
    pub on_set: Callback<()>,
}

#[function_component(SetAlbumCoverButton)]
pub fn set_album_cover_button(props: &SetAlbumCoverButtonProps) -> Html {
    let (_, set_overlay) = use_overlay();

    let set_cover = {
        let album_id = props.album_id.clone();
        let photo_id = props.photo_id.clone();
        let on_set = props.on_set.clone();

        move |_| {
            let album_id = album_id.clone();
            let photo_id = photo_id.clone();
            let on_set = on_set.clone();
            let set_overlay = set_overlay.clone();

            set_overlay.emit(true);

            wasm_bindgen_futures::spawn_local(async move {
                WASM_CLIENT.update_album_cover(&album_id, &photo_id).await.unwrap();
                set_overlay.emit(false);
                on_set.emit(())
            });
        }
    };

    html! {
        <Button label={"Set album cover"} on_click={set_cover}>
            <IconImage/>
        </Button>
    }
}
