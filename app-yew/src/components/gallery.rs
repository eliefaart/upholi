use crate::{components::photo::Photo, Route};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct GalleryProps {
    pub photo_ids: Vec<String>,
}

#[function_component(Gallery)]
pub fn gallery(props: &GalleryProps) -> Html {
    let navigator = use_navigator().unwrap();
    let selected_photos = use_state(|| Vec::<String>::new());

    let on_photo_clicked = {
        let selected_photos = selected_photos.clone();
        Callback::from(move |url: String| {
            let mut temp = selected_photos.to_vec();
            if temp.contains(&url) {
                temp.retain(|i| i != &url)
            } else {
                temp.push(url);
            }

            selected_photos.set(temp);
        })
    };

    let photos = props
        .photo_ids
        .clone()
        .into_iter()
        .map(|photo_id| {
            let selected = selected_photos.contains(&photo_id);
            let class = selected.then(|| format!("selected"));

            let on_click_navigator = navigator.clone();
            let on_click_photo_id = photo_id.clone();
            let on_click = Callback::from(move |_| {
                on_click_navigator.push(&Route::Photo {
                    id: on_click_photo_id.clone(),
                })
            });

            let on_context_menu_on_photo_clicked = on_photo_clicked.clone();
            let on_context_menu_photo_id = photo_id.clone();
            let on_context_menu = Callback::from(move |event: MouseEvent| {
                event.prevent_default();
                on_context_menu_on_photo_clicked.emit(on_context_menu_photo_id.clone());
            });

            html! {
                <div onclick={on_click} oncontextmenu={on_context_menu}>
                    <Photo class={class} photo_id={photo_id}/>
                </div>
            }
        })
        .collect::<Html>();

    html! {
        <div class="gallery">
            {photos}
        </div>
    }
}
