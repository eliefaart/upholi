use yew::prelude::*;

pub mod add_to_album_button;
pub mod create_album_button;
pub mod delete_album_button;
pub mod delete_photos_button;
pub mod remove_from_album_button;
pub mod set_album_cover_button;

pub use add_to_album_button::*;
pub use create_album_button::*;
pub use delete_album_button::*;
pub use delete_photos_button::*;
pub use remove_from_album_button::*;
pub use set_album_cover_button::*;

#[derive(PartialEq)]
pub enum IconPosition {
    Left,
    Right,
}

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub label: String,
    pub on_click: Callback<MouseEvent>,
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

    let class = match props.icon_position {
        IconPosition::Right => "icon-right",
        IconPosition::Left => "icon-left",
    };

    html! {
        <button onclick={&props.on_click} class={class}>
            {icon_left}
            <span class="label">{&props.label}</span>
            {icon_right}
        </button>
    }
}
