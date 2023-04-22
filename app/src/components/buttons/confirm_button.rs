use crate::components::{buttons::Button, dialog::ConfirmDialog};
use yew::prelude::*;

use super::IconPosition;

#[derive(Properties, PartialEq)]
pub struct ConfirmButtonProps {
    pub label: AttrValue,
    pub on_click: Callback<()>,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_else(|| IconPosition::Left)]
    pub icon_position: IconPosition,
    #[prop_or_default]
    pub confirm_dialog_title: AttrValue,
    #[prop_or_default]
    pub confirm_dialog_body: AttrValue,
}

#[function_component(ConfirmButton)]
pub fn confirm_button(props: &ConfirmButtonProps) -> Html {
    let dialog_visible = use_state(|| false);

    let show_dialog = {
        let dialog_visible = dialog_visible.clone();
        move |_| {
            dialog_visible.set(true);
        }
    };

    let hide_dialog = {
        let dialog_visible = dialog_visible.clone();
        move |_| {
            dialog_visible.set(false);
        }
    };

    let on_confirm = {
        let dialog_visible = dialog_visible.clone();
        let confirm_action = props.on_click.clone();
        move |mouse_event| {
            dialog_visible.set(false);
            confirm_action.emit(mouse_event);
        }
    };

    let dialog_visible = *dialog_visible;
    html! {
        <>
            <Button label={&props.label} on_click={show_dialog}>
                {props.children.clone()}
            </Button>
            <ConfirmDialog
                visible={dialog_visible}
                title={&props.confirm_dialog_title}
                confirm_action={on_confirm}
                cancel_action={hide_dialog}>
                {html! {
                    {&props.confirm_dialog_body}
                }}
            </ConfirmDialog>
        </>
    }
}
