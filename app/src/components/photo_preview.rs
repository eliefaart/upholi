use crate::hooks::{use_photo, use_photo_src::use_photo_src};
use std::ops::{AddAssign, Sub};
use upholi_lib::PhotoVariant;
use web_sys::HtmlElement;
use yew::prelude::*;

#[derive(Debug, PartialEq, Clone, Copy)]
struct PhotoViewState {
    zoom: f64,
    offset: XY,
    is_panning: bool,
    previous: Option<XY>,
}

impl Default for PhotoViewState {
    fn default() -> Self {
        Self {
            zoom: 1.0,
            offset: Default::default(),
            is_panning: Default::default(),
            previous: Default::default(),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Default)]
struct XY(i32, i32);

impl AddAssign for XY {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
        self.1 += other.1;
    }
}

impl Sub for XY {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self(self.0 - other.0, self.1 - other.1)
    }
}

#[derive(Properties, PartialEq)]
pub struct PhotoPreviewProps {
    pub photo_id: AttrValue,
}

#[function_component(PhotoPreview)]
pub fn photo_preview(props: &PhotoPreviewProps) -> Html {
    let container_node = use_node_ref();
    let photo_node = use_node_ref();
    let src = use_photo_src(&props.photo_id, PhotoVariant::Preview);
    let src = (*src).clone();
    let view_state = use_state(PhotoViewState::default);

    let toggle_zoom = use_callback(
        move |_, view_state| {
            if view_state.zoom != 1.0 {
                view_state.set(PhotoViewState::default());
            } else {
                let mut new_state = **view_state;
                new_state.zoom = 2.0;
                view_state.set(new_state);
            }
        },
        view_state.clone(),
    );

    let set_panning = use_callback(
        move |panning: bool, view_state| {
            if view_state.is_panning != panning {
                let mut new_state = **view_state;
                if !panning {
                    new_state.previous = None;
                }
                new_state.is_panning = panning;
                view_state.set(new_state);
            }
        },
        view_state.clone(),
    );

    let on_mouse_down = use_callback(
        |_: MouseEvent, set_panning| {
            set_panning.emit(true);
        },
        set_panning.clone(),
    );

    let on_mouse_up = use_callback(
        |_: MouseEvent, set_panning| {
            set_panning.emit(false);
        },
        set_panning.clone(),
    );

    let on_touch_start = use_callback(
        |event: TouchEvent, set_panning| {
            set_panning.emit(true);
            event.prevent_default();
            event.stop_immediate_propagation();
        },
        set_panning.clone(),
    );

    let on_touch_end = use_callback(
        |event: TouchEvent, set_panning| {
            set_panning.emit(false);
            event.prevent_default();
            event.stop_immediate_propagation();
        },
        set_panning,
    );

    let pan = {
        let photo_node = photo_node.clone();
        let container_node = container_node.clone();

        use_callback(
            move |client_xy: (i32, i32), view_state| {
                let (client_x, client_y) = client_xy;

                if view_state.is_panning {
                    let (container_width, container_height) = {
                        if let Some(element) = container_node.cast::<HtmlElement>() {
                            (element.client_width(), element.client_height())
                        } else {
                            (0, 0)
                        }
                    };
                    let (photo_width, photo_height) = {
                        if let Some(element) = photo_node.cast::<HtmlElement>() {
                            (
                                element.client_width() * view_state.zoom as i32,
                                element.client_height() * view_state.zoom as i32,
                            )
                        } else {
                            (0, 0)
                        }
                    };

                    let current = XY(client_x, client_y);
                    let mut offset = view_state.offset;

                    if let Some(prev) = view_state.previous {
                        let delta = current - prev;
                        offset += delta;

                        offset.0 = if container_width > photo_width {
                            0
                        } else {
                            let left_min = (container_width - photo_width) / 2;
                            let left_max = -left_min;
                            offset.0.clamp(left_min, left_max)
                        };

                        offset.1 = if container_height > photo_height {
                            0
                        } else {
                            let top_min = (container_height - photo_height) / 2;
                            let top_max = -top_min;
                            offset.1.clamp(top_min, top_max)
                        };
                    }

                    let mut new_state = **view_state;
                    new_state.offset = offset;
                    new_state.previous = Some(current);
                    view_state.set(new_state);
                }
            },
            view_state.clone(),
        )
    };

    let on_mouse_move = use_callback(
        move |event: MouseEvent, pan| {
            pan.emit((event.client_x(), event.client_y()));
        },
        pan.clone(),
    );

    let on_touch_move = use_callback(
        move |event: TouchEvent, pan| {
            let n_touches = event.touches().length();
            let mut touches = vec![];
            for i in 0..n_touches {
                if let Some(touch) = event.touches().item(i) {
                    touches.push(touch);
                }
            }

            let touch_x = touches.iter().map(|t| t.client_x()).sum::<i32>() / n_touches as i32;
            let touch_y = touches.iter().map(|t| t.client_y()).sum::<i32>() / n_touches as i32;

            pan.emit((touch_x, touch_y));
        },
        pan,
    );

    let style = format!(
        "scale: {}; top: {}px; left: {}px",
        view_state.zoom, view_state.offset.1, view_state.offset.0
    );
    html! {
        <div ref={container_node}
            class="photo"
            onmousedown={on_mouse_down}
            onmouseup={on_mouse_up.clone()}
            ontouchstart={on_touch_start}
            ontouchend={on_touch_end}

            onmouseleave={on_mouse_up}

            onmousemove={on_mouse_move}
            ontouchmove={on_touch_move}

            ondblclick={toggle_zoom}
        >
            <img ref={photo_node}
                src={src}
                draggable="false"
                style={style}/>
        </div>
    }
}
