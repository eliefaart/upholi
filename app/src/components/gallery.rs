use crate::{components::photo::Photo, get_document, models::AlbumPhoto, Route};
use yew::prelude::*;
use yew_router::prelude::*;

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

    // TODO: Don't bother with this on first render.
    // TODO: Update on resize
    // TODO: Use use_node_ref(), safer, easier
    let gallery_element = get_document().get_element_by_id("ASDAS");
    let available_width = if let Some(gallery_element) = gallery_element {
        (gallery_element.client_width() - 1) as f32
    } else {
        1f32
    };
    let photos = compute_sizes(props.photos.clone(), available_width, MIN_HEIGHT);

    // use_memo
    let photos = photos
        .into_iter()
        .map(|ResizedPhoto { photo, width, height }| {
            let selected = selected_photos.contains(&photo.id);
            let class = selected.then(|| format!("selected"));

            let on_click_navigator = navigator.clone();
            let on_click_photo_id = photo.id.clone();
            let on_click = Callback::from(move |_| {
                on_click_navigator.push(&Route::Photo {
                    id: on_click_photo_id.clone(),
                });
            });

            let on_context_menu_on_photo_clicked = on_photo_clicked.clone();
            let on_context_menu_photo_id = photo.id.clone();
            let on_context_menu = Callback::from(move |event: MouseEvent| {
                event.prevent_default();
                on_context_menu_on_photo_clicked.emit(on_context_menu_photo_id.clone());
            });

            html! {
                <Photo
                    class={class}
                    photo_id={photo.id}
                    width={width}
                    height={height}
                    on_click={on_click}
                    on_context_menu={on_context_menu}/>
            }
        })
        .collect::<Html>();

    html! {
        <div id="ASDAS" class="gallery">
            {photos}
        </div>
    }
}

#[derive(Debug, Clone)]
struct ResizedPhoto {
    photo: AlbumPhoto,
    width: f32,
    height: f32,
}

fn compute_sizes(photos: Vec<AlbumPhoto>, available_width: f32, min_height: f32) -> Vec<ResizedPhoto> {
    let mut rows: Vec<Vec<ResizedPhoto>> = photos
        .into_iter()
        // Normalize all photos to minimum height
        .map(|photo| {
            let shrink_ratio = photo.height as f32 / min_height;
            let width = photo.width as f32 / shrink_ratio;
            let height = min_height;

            ResizedPhoto { photo, width, height }
        })
        // Split into rows
        .fold(vec![vec![]], |mut accumulator, photo| {
            let current_row = accumulator.last_mut().expect("Make sure the accumulator starts with one row");

            let row_width: f32 = current_row.iter().map(|p| p.width).sum();

            if row_width + photo.width > available_width {
                // new row
                accumulator.push(vec![photo]);
            } else {
                // Add to row
                current_row.push(photo);
            }

            accumulator
        });

    // Increase each photo's size to fully fill the row's width
    for row in &mut rows {
        let row_width: f32 = row.iter().map(|p| p.width).sum();
        let bloat_ratio = available_width / row_width;

        for mut photo in row {
            photo.width = photo.width * bloat_ratio;
            photo.height = photo.height * bloat_ratio;
        }
    }

    // Unwrap rows back into a list of (resized)photos
    rows.into_iter().flat_map(|row| row).collect()
}
