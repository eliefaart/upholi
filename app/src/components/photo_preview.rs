use crate::hooks::use_photo_src::use_photo_src;
use std::ops::{AddAssign, Sub};
use upholi_lib::PhotoVariant;
use web_sys::HtmlElement;
use yew::prelude::*;

const ZOOM_FACTOR_MIN: f64 = 1.0;
const ZOOM_FACTOR_MAX: f64 = 3.0;

#[derive(Debug, PartialEq, Clone, Copy)]
struct PhotoViewState {
    zoom: f64,
    offset: XY,
    is_panning: bool,
    previous: Option<XY>,
    finger_distance: Option<f64>,
}

impl PhotoViewState {
    fn pan(&mut self, client_xy: (i32, i32), container_node: &NodeRef, photo_node: &NodeRef) {
        let (client_x, client_y) = client_xy;

        if self.is_panning {
            let current = XY(client_x, client_y);
            let mut offset = self.offset;

            if let Some(prev) = self.previous {
                let delta = current - prev;
                offset += delta;
            }

            self.offset = offset;
            self.previous = Some(current);
            self.clamp_photo_offset(container_node, photo_node);
        }
    }

    fn pinch(&mut self, finger_distance: f64, container_node: &NodeRef, photo_node: &NodeRef) {
        if let Some(prev_distance) = self.finger_distance {
            if let Some(photo_element) = photo_node.cast::<HtmlElement>() {
                let photo_width = photo_element.client_width() as f64;
                let delta = prev_distance - finger_distance;
                let zoom_step = (delta / photo_width) * 100.;
                let zoom_step = (self.zoom / 100.) * zoom_step;
                let new_zoom = self.zoom - zoom_step;
                self.set_zoom(new_zoom, container_node, photo_node);
            }
        }

        self.finger_distance = Some(finger_distance);
    }

    fn set_zoom(&mut self, target_zoom: f64, container_node: &NodeRef, photo_node: &NodeRef) {
        self.zoom = f64::clamp(target_zoom, ZOOM_FACTOR_MIN, ZOOM_FACTOR_MAX);
        self.clamp_photo_offset(container_node, photo_node);
    }

    fn clamp_photo_offset(&mut self, container_node: &NodeRef, photo_node: &NodeRef) {
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
                    (element.scroll_width() as f64 * self.zoom) as i32,
                    (element.scroll_height() as f64 * self.zoom) as i32,
                )
            } else {
                (0, 0)
            }
        };

        self.offset.0 = if container_width > photo_width {
            0
        } else {
            let left_min = (container_width - photo_width) / 2;
            let left_max = -left_min;
            self.offset.0.clamp(left_min, left_max)
        };

        self.offset.1 = if container_height > photo_height {
            0
        } else {
            let top_min = (container_height - photo_height) / 2;
            let top_max = -top_min;
            self.offset.1.clamp(top_min, top_max)
        };
    }
}

impl Default for PhotoViewState {
    fn default() -> Self {
        Self {
            zoom: 1.0,
            offset: Default::default(),
            is_panning: false,
            previous: None,
            finger_distance: None,
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

    let reset_zoom = use_callback(
        move |_, view_state| {
            if view_state.zoom != 1.0 {
                view_state.set(PhotoViewState::default());
            }
        },
        view_state.clone(),
    );

    let set_panning = use_callback(
        move |panning: bool, view_state| {
            if view_state.is_panning != panning {
                let mut updated_state = **view_state;
                if !panning {
                    updated_state.previous = None;
                    updated_state.finger_distance = None;
                }
                updated_state.is_panning = panning;
                view_state.set(updated_state);
            }
        },
        view_state.clone(),
    );

    let on_wheel = {
        let photo_node = photo_node.clone();
        let container_node = container_node.clone();

        use_callback(
            move |event: WheelEvent, view_state| {
                let mut updated_state = **view_state;

                let zooming_in = event.delta_y() < 0.;
                let zoom_step_percentage = if zooming_in { 15. } else { -15. };
                let zoom = updated_state.zoom + ((updated_state.zoom / 100.) * zoom_step_percentage);
                updated_state.set_zoom(zoom, &container_node, &photo_node);

                view_state.set(updated_state);
            },
            view_state.clone(),
        )
    };

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
        |_: TouchEvent, set_panning| {
            set_panning.emit(true);
        },
        set_panning.clone(),
    );

    let on_touch_end = use_callback(
        |_: TouchEvent, set_panning| {
            set_panning.emit(false);
        },
        set_panning,
    );

    let on_mouse_move = {
        let photo_node = photo_node.clone();
        let container_node = container_node.clone();

        use_callback(
            move |event: MouseEvent, view_state| {
                let mut updated_state = **view_state;
                updated_state.pan((event.client_x(), event.client_y()), &container_node, &photo_node);
                view_state.set(updated_state);
            },
            view_state.clone(),
        )
    };

    let on_touch_move = {
        let photo_node = photo_node.clone();
        let container_node = container_node.clone();

        use_callback(
            move |event: TouchEvent, view_state| {
                let n_touches = event.touches().length();
                let mut touches = vec![];
                for i in 0..n_touches {
                    if let Some(touch) = event.touches().item(i) {
                        touches.push(touch);
                    }
                }

                let mut updated_state = **view_state;

                {
                    // Handle panning
                    // Touch position is average X and Y of all touches

                    let touch_x = touches.iter().map(|t| t.client_x()).sum::<i32>() / n_touches as i32;
                    let touch_y = touches.iter().map(|t| t.client_y()).sum::<i32>() / n_touches as i32;

                    updated_state.pan((touch_x, touch_y), &container_node, &photo_node);
                }

                {
                    // Handle pinch zooming
                    // Only take the first two touches into account for now

                    if n_touches >= 2 {
                        let touch_1 = touches.get(0).unwrap();
                        let touch_2 = touches.get(1).unwrap();
                        let t1_x = touch_1.client_x() as f64;
                        let t1_y = touch_1.client_y() as f64;
                        let t2_x = touch_2.client_x() as f64;
                        let t2_y = touch_2.client_y() as f64;

                        let distance = ((t2_x - t1_x).powf(2.) + (t2_y - t1_y).powf(2.)).sqrt();
                        updated_state.pinch(distance, &container_node, &photo_node);
                    }
                }

                view_state.set(updated_state);
            },
            view_state.clone(),
        )
    };

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
            onwheel={on_wheel}
            onmouseleave={on_mouse_up}

            onmousemove={on_mouse_move}
            ontouchmove={on_touch_move}

            ondblclick={reset_zoom}
        >
            <img ref={photo_node}
                src={src}
                draggable="false"
                style={style}/>
        </div>
    }
}
