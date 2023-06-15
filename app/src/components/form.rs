use crate::components::Button;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct FormProps {
    pub title: AttrValue,
    #[prop_or_default]
    pub status: AttrValue,
    pub children: Children,
    pub on_submit: Callback<()>,
}

#[function_component(Form)]
pub fn form(props: &FormProps) -> Html {
    let on_submit = props.on_submit.clone();

    html! {
        <form class="form">
            <h1>{&props.title}</h1>
            {props.children.clone()}
            <div class="footer">
                <span class="status">{&props.status}</span>
                <Button label={"Submit"} on_click={move |_| on_submit.emit(())}/>
            </div>
        </form>
    }
}
