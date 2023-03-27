use yew::prelude::*;

use crate::components::{button::Button, icons::IconClose};

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

    html! {
        if props.visible {
            <div>
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
        </div>
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct ConfirmDialogProps {
    pub visible: bool,
    pub title: String,
    pub confirm_action: Callback<()>,
    pub cancel_action: Callback<()>,
}

#[function_component(ConfirmDialog)]
pub fn confirm_dialog(props: &ConfirmDialogProps) -> Html {
    let confirm_action = props.confirm_action.clone();
    let cancel_action = props.cancel_action.clone();
    let on_request_close = props.cancel_action.clone();

    html! {
        if props.visible {
            <Dialog
                visible={props.visible}
                title={props.title.clone()}
                on_request_close={on_request_close}
                footer={html! {
                    <>
                        <Button label="Cancel" on_click={move |_| cancel_action.emit(())} />
                        <Button label="Ok" on_click={move |_| confirm_action.emit(())} />
                    </>
                }}>
                {html! {}}
            </Dialog>
        }
    }
}
