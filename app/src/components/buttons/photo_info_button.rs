use crate::components::{buttons::Button, IconInfo};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PhotoInfoButtonProps {
    pub photo_id: AttrValue,
}

#[function_component(PhotoInfoButton)]
pub fn photo_info_button(_props: &PhotoInfoButtonProps) -> Html {
    let on_click = move |_| {};

    html! {
        <Button label={"Info"} on_click={on_click}>
            <IconInfo/>
        </Button>
    }
}
