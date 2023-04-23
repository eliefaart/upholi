use yew::prelude::*;

pub mod add_to_album_button;
pub mod confirm_button;
pub mod create_album_button;
pub mod delete_album_button;
pub mod delete_photos_button;
pub mod download_photo_button;
pub mod edit_album_button;
pub mod photo_exif_button;
pub mod remove_from_album_button;
pub mod set_album_cover_button;
pub mod share_album_button;

pub use add_to_album_button::*;
pub use confirm_button::*;
pub use create_album_button::*;
pub use delete_album_button::*;
pub use delete_photos_button::*;
pub use download_photo_button::*;
pub use edit_album_button::*;
pub use photo_exif_button::*;
pub use remove_from_album_button::*;
pub use set_album_cover_button::*;
pub use share_album_button::*;

#[derive(PartialEq)]
pub enum IconPosition {
    Left,
    Right,
}

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub label: AttrValue,
    pub on_click: Callback<()>,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_else(|| IconPosition::Left)]
    pub icon_position: IconPosition,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let icon_left = html! {
        if props.icon_position == IconPosition::Left {
            {props.children.clone()}
        }
    };
    let icon_right = html! {
        if props.icon_position == IconPosition::Right {
            {props.children.clone()}
        }
    };

    let mut classes = classes!();
    if !props.children.is_empty() {
        classes.push(match props.icon_position {
            IconPosition::Right => "icon-right",
            IconPosition::Left => "icon-left",
        });
        classes.push("with-icon");
    }

    let on_click = {
        let on_click = props.on_click.clone();

        move |event: MouseEvent| {
            event.prevent_default();
            on_click.emit(())
        }
    };

    html! {
        <button onclick={on_click} class={classes}>
            {icon_left}
            <span class="label">{&props.label}</span>
            {icon_right}
        </button>
    }
}
