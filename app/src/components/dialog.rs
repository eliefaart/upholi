use crate::components::{buttons::Button, icons::IconClose};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct DialogProps {
    pub visible: bool,
    pub title: String,
    #[prop_or_default]
    pub on_request_close: Callback<()>,
    pub children: Children,
    #[prop_or_default]
    pub footer: Children,
}

#[function_component(Dialog)]
pub fn dialog(props: &DialogProps) -> Html {
    let on_request_close = props.on_request_close.clone();

    let modal_host = web_sys::window()
        .expect("No window")
        .document()
        .expect("No document")
        .get_element_by_id("modal-host")
        .expect("No modal host element");

    let dialog = html! {
        if props.visible {
            <div class="dialog-overlay">
                <div class="dialog">
                    <div class="header">
                        <span>{&props.title}</span>
                        <Button label="" on_click={move |_| on_request_close.emit(()) }>
                            <IconClose/>
                        </Button>
                    </div>

                    <div class="body">
                        {props.children.clone()}
                    </div>

                    <div class="footer">
                        {props.footer.clone()}
                    </div>
                </div>
            </div>
        }
    };

    create_portal(dialog, modal_host.into())
}

#[derive(Properties, PartialEq)]
pub struct ConfirmDialogProps {
    pub visible: bool,
    pub title: String,
    pub confirm_action: Callback<()>,
    pub cancel_action: Callback<()>,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(ConfirmDialog)]
pub fn confirm_dialog(props: &ConfirmDialogProps) -> Html {
    let confirm_action = props.confirm_action.clone();
    let on_request_close = props.cancel_action.clone();

    html! {
        if props.visible {
            <Dialog
                visible={props.visible}
                title={props.title.clone()}
                on_request_close={on_request_close}
                footer={html! {
                    <Button label="Ok" on_click={move |_| confirm_action.emit(())} />
                }}>
                {props.children.clone()}
            </Dialog>
        }
    }
}
