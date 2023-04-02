use crate::{components::album_thumb::AlbumThumb, hooks::use_albums};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PickAlbumProps {
    pub selected_album: UseStateHandle<Option<String>>,
}

#[function_component(PickAlbum)]
pub fn pick_album(props: &PickAlbumProps) -> Html {
    let (albums, _) = use_albums();

    let albums_html = (*albums)
        .iter()
        .map(|album| {
            let on_click = {
                let selected_album = props.selected_album.clone();
                let album_id = album.id.clone();
                Callback::from(move |_| selected_album.set(Some(album_id.clone())))
            };

            let is_selected = match (*props.selected_album).clone() {
                Some(selected_album_id) => selected_album_id == album.id,
                None => false,
            };

            html! {
                <div
                    class={classes!("pick-album-entry", { if is_selected {"selected"} else { "" } } )}
                    onclick={on_click}>
                    <AlbumThumb album={album.clone()}/>
                </div>
            }
        })
        .collect::<Html>();

    html! {
        <div class="pick-album">
            {albums_html}
        </div>
    }
}
