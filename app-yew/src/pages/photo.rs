use yew::prelude::*;
use crate::components::photo::Photo;

#[derive(Properties, PartialEq)]
pub struct PhotoPageProps {
    pub id: String,
}

#[function_component(PhotoPage)]
pub fn photo_page(props: &PhotoPageProps) -> Html {
    let id = props.id.clone();

    html! {
        <>
            <Photo photo_id={id}/>
        </>
    }
}
