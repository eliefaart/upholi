use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct DialogProps {
    pub visible: bool,
    pub title: String,
    pub children: Children,
}

#[function_component(Dialog)]
pub fn dialog(props: &DialogProps) -> Html {
    html! {
        if props.visible {
            <div class={"dialog-overlay"}>
                <div class={"dialog"}>
                    <header>{&props.title}</header>
                    {props.children.clone()}
                </div>
            </div>
        }
    }
}
