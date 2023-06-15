use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ShareUrlProps {
    pub share_id: AttrValue,
}

#[function_component(ShareUrl)]
pub fn share_url(props: &ShareUrlProps) -> Html {
    html! {
        <input class="share-url" type="text"
            value={format!("{}/s/{}", crate::ORIGIN.as_str(), props.share_id)}
            readonly={true}/>
    }
}
