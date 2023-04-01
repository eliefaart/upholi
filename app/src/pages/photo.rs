use crate::components::photo::PhotoPreview;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PhotoPageProps {
    pub id: String,
}

#[function_component(PhotoPage)]
pub fn photo_page(props: &PhotoPageProps) -> Html {
    let photo_id = props.id.clone();

    html! {
        <PhotoPreview photo_id={photo_id}/>
    }
}
