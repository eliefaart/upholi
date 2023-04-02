use crate::{
    components::{buttons::Button, icons::IconRemove},
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
    let album_id = props.album_id.clone();
    let photo_ids = props.photo_ids.clone();
    let on_removed = props.on_removed.clone();
    let set_cover = move |_| {
        let album_id = album_id.clone();
        let photo_ids = photo_ids.clone();
        let on_removed = on_removed.clone();
        wasm_bindgen_futures::spawn_local(async move {
            WASM_CLIENT.remove_photos_from_album(&album_id, &photo_ids).await.unwrap();
            on_removed.emit(())
        });
    };

    html! {
        <Button label={"Remove from album"} on_click={set_cover}>
            <IconRemove/>
        </Button>
    }
}
