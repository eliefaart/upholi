use yew::prelude::*;

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
            {&props.label}
            {icon_right}
        </button>
    }
}
