use crate::{
    components::{photo::Photo, GalleryDetail, PhotoPlaceholder},
    models::AlbumPhoto,
};
use web_sys::Element;
use yew::prelude::*;
use yew_hooks::use_interval;

// Desired minimum height of a gallery photo.
const MIN_HEIGHT: f32 = 175.;
// Desired minimum height of a gallery photo.
const MAX_HEIGHT: f32 = 350.;
// Pixels between each photo, as per CSS.
const GAP_SIZE: f32 = 5.;

#[derive(PartialEq, Properties)]
pub struct GalleryProps {
    pub photos: Vec<AlbumPhoto>,
    pub selected_photos: UseStateHandle<Vec<String>>,
    pub on_selection_changed: Option<Callback<Vec<String>>>,
}

#[function_component(Gallery)]
pub fn gallery(props: &GalleryProps) -> Html {
    let node_ref = use_node_ref();
    let photo_opened = use_state(|| None);
    let available_width = use_state(|| None);
    let photo_ids_allowed_to_load: UseStateHandle<Vec<String>> =
        use_state(|| props.photos.iter().take(20).map(|p| p.id.to_string()).collect());

    let photo_ids: UseStateHandle<Vec<String>> = use_state(|| props.photos.iter().map(|p| p.id.to_string()).collect());

    {
        let state = photo_ids.clone();
        use_effect_with_deps(
            move |photos| {
                let photo_ids: Vec<String> = photos.iter().map(|p| p.id.to_string()).collect();
                state.set(photo_ids);
            },
            props.photos.clone(),
        );
    }

    {
        let photo_ids_allowed_to_load = photo_ids_allowed_to_load.clone();
        let all_photos_loaded = photo_ids.len() == photo_ids_allowed_to_load.len();
        let interval_ms = if !all_photos_loaded { 750 } else { 0 };

        // TODO: Re-determine this on scroll & resize events, throttled.

        use_interval(
            move || {
                let photo_ids: Vec<&String> = photo_ids
                    .iter()
                    .filter(|id| !photo_ids_allowed_to_load.contains(id))
                    .collect();
                let mut photos_have_been_in_view = (*photo_ids_allowed_to_load).clone();

                if !photo_ids.is_empty() {
                    let document = crate::get_document();
                    let document_element = document.document_element().expect("Document has no element");
                    for id in &*photo_ids {
                        if let Some(photo_element) = document.get_element_by_id(id) {
                            let visible = element_is_in_viewport(&photo_element, &document_element);
                            if visible {
                                photos_have_been_in_view.push(id.to_string());
                            }
                        }
                    }

                    if photos_have_been_in_view.len() > photo_ids_allowed_to_load.len() {
                        photo_ids_allowed_to_load.set(photos_have_been_in_view);
                    }
                }
            },
            interval_ms,
        );
    }

    let on_photo_clicked = {
        let selected_photos = props.selected_photos.clone();
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

    {
        let use_effect_selected_photos = props.selected_photos.clone();
        let use_effect_on_selection_changed = props.on_selection_changed.clone();
        use_effect_with_deps(
            move |_| {
                if let Some(on_selection_changed) = use_effect_on_selection_changed {
                    let selected_photos = (*use_effect_selected_photos).clone();
                    on_selection_changed.emit(selected_photos);
                }
            },
            props.selected_photos.clone(),
        );
    }

    if available_width.is_none() {
        let gallery_element = node_ref.cast::<Element>();
        if let Some(gallery_element) = gallery_element {
            let gallery_width = (gallery_element.client_width() - 1) as f32;
            available_width.set(Some(gallery_width));
        }
    }

    {
        let node_ref = node_ref.clone();
        let available_width = available_width.clone();

        // TODO: Re-determine this on resize event, throttled.

        use_interval(
            move || {
                let gallery_element = node_ref.cast::<Element>();
                if let Some(gallery_element) = gallery_element {
                    let gallery_width = (gallery_element.client_width() - 1) as f32;
                    let current_width = (*available_width).unwrap_or(0f32);
                    if current_width != gallery_width {
                        available_width.set(Some(gallery_width));
                    }
                }
            },
            500,
        );
    }

    let photos = use_memo(
        |(available_width, photos, selected_photos, allowed_to_load)| {
            let html = if let Some(available_width) = available_width {
                let photos = compute_sizes(photos, *available_width, MIN_HEIGHT, MAX_HEIGHT, GAP_SIZE);
                photos
                    .into_iter()
                    .map(|ResizedPhoto { photo, width, height }| {
                        let photo_id = photo.id.clone();
                        let may_load = allowed_to_load.contains(&photo_id);
                        let selected = selected_photos.contains(&photo.id);
                        let class = selected.then(|| "selected".to_string());

                        let photo_opened = photo_opened.clone();
                        let on_click_photo_id = photo.id.clone();
                        let on_click = Callback::from(move |_| {
                            photo_opened.set(Some(on_click_photo_id.clone()));
                        });

                        let on_context_menu_on_photo_clicked = on_photo_clicked.clone();
                        let on_context_menu_photo_id = photo.id.clone();
                        let on_context_menu = Callback::from(move |event: MouseEvent| {
                            event.prevent_default();
                            on_context_menu_on_photo_clicked.emit(on_context_menu_photo_id.clone());
                        });

                        if may_load {
                            html! {
                                <Photo
                                    class={class}
                                    photo_id={photo_id}
                                    width={width}
                                    height={height}
                                    on_click={on_click}
                                    on_context_menu={on_context_menu}/>
                            }
                        } else {
                            html! {
                                <PhotoPlaceholder
                                    class={class}
                                    photo_id={photo_id}
                                    width={width}
                                    height={height}
                                    on_click={on_click}
                                    on_context_menu={on_context_menu}/>
                            }
                        }
                    })
                    .collect::<Html>()
            } else {
                html! {}
            };
            html
        },
        (
            *available_width,
            props.photos.clone(),
            (*props.selected_photos).clone(),
            photo_ids_allowed_to_load,
        ),
    );

    html! {
        <div class="gallery" ref={node_ref}>
            {(*photos).clone()}
            <GalleryDetail
                photos={props.photos.clone()}
                photo_id={photo_opened}/>
        </div>
    }
}

#[derive(Debug, Clone)]
struct ResizedPhoto<'a> {
    photo: &'a AlbumPhoto,
    width: f32,
    height: f32,
}

fn compute_sizes(
    photos: &[AlbumPhoto],
    available_width: f32,
    min_height: f32,
    max_height: f32,
    gap_size: f32,
) -> Vec<ResizedPhoto> {
    let mut rows: Vec<Vec<ResizedPhoto>> = photos
        .iter()
        // Normalize all photos to minimum height
        .map(|photo| {
            let shrink_ratio = photo.height as f32 / min_height;
            let width = photo.width as f32 / shrink_ratio;
            let height = min_height;

            ResizedPhoto { photo, width, height }
        })
        // Split into rows
        .fold(vec![vec![]], |mut accumulator, photo| {
            let current_row = accumulator
                .last_mut()
                .expect("Make sure the accumulator starts with one row");

            let row_width: f32 = current_row.iter().map(|p| p.width + gap_size).sum();

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
        let row_width: f32 = row.iter().map(|p| p.width + gap_size).sum::<f32>() - gap_size;
        let bloat_ratio = available_width / row_width;

        for mut photo in row {
            photo.width *= bloat_ratio;
            photo.height *= bloat_ratio;
        }
    }

    // Last row may not contain enough photos to properly fill the row.
    // But since we let it fill its max width anyway, the photos may be stretched out a lot.
    // Resize them to meet the 'max_height' constraint.
    let last_row = rows.last_mut();
    if let Some(last_row) = last_row {
        for photo in last_row {
            if photo.height > max_height {
                photo.width /= photo.height / max_height;
                photo.height = max_height;
            }
        }
    }

    // Unwrap rows back into a list of (resized)photos
    rows.into_iter().flatten().collect()
}

fn element_is_in_viewport(element: &Element, document_element: &Element) -> bool {
    let document_height = document_element.client_height() as f64;
    let bounds = element.get_bounding_client_rect();
    let bounds_top = bounds.top();
    let bounds_bottom = bounds.bottom();

    let top_in_view = bounds_top >= 0. && bounds_top <= document_height;
    let bottom_in_view = bounds_bottom >= 0. && bounds_bottom <= document_height;

    top_in_view || bottom_in_view
}
