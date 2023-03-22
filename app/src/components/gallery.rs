use crate::{components::photo::Photo, models::AlbumPhoto, Route};
use yew::prelude::*;
use yew_router::prelude::*;

const MIN_WIDTH: f32 = 250.;
const MIN_HEIGHT: f32 = 175.;

#[derive(Properties, PartialEq)]
pub struct GalleryProps {
    pub photos: Vec<AlbumPhoto>,
    pub on_selection_changed: Option<Callback<Vec<String>>>,
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

    let use_effect_selected_photos = selected_photos.clone();
    let use_effect_on_selection_changed = props.on_selection_changed.clone();
    use_effect_with_deps(
        move |_| {
            if let Some(on_selection_changed) = use_effect_on_selection_changed {
                let selected_photos = (*use_effect_selected_photos).clone();
                on_selection_changed.emit(selected_photos);
            }
        },
        selected_photos.clone(),
    );

    let photos = props
        .photos
        .clone()
        .into_iter()
        .map(|photo| {
            let selected = selected_photos.contains(&photo.id);
            let class = selected.then(|| format!("selected"));

            let on_click_navigator = navigator.clone();
            let on_click_photo_id = photo.id.clone();
            let on_click = Callback::from(move |_| {
                on_click_navigator.push(&Route::Photo {
                    id: on_click_photo_id.clone(),
                })
            });

            let on_context_menu_on_photo_clicked = on_photo_clicked.clone();
            let on_context_menu_photo_id = photo.id.clone();
            let on_context_menu = Callback::from(move |event: MouseEvent| {
                event.prevent_default();
                on_context_menu_on_photo_clicked.emit(on_context_menu_photo_id.clone());
            });

            let shrink_ratio = f32::min(photo.width as f32 / MIN_WIDTH, photo.height as f32 / MIN_HEIGHT);
            let width = photo.width as f32 / shrink_ratio;
            let height = photo.height as f32 / shrink_ratio;

            html! {
                <div onclick={on_click} oncontextmenu={on_context_menu}>
                    <Photo class={class} photo_id={photo.id} width={width} height={height}/>
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
