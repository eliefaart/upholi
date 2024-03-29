use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct IconProps {
    svg_path_d: String,
}

#[function_component(Icon)]
pub fn icon(props: &IconProps) -> Html {
    html! {
        <svg viewBox="0 0 24 24">
            <path fill="currentColor" d={props.svg_path_d.clone()} />
        </svg>
    }
}

#[function_component(IconCreate)]
pub fn icon_create() -> Html {
    html! { <Icon svg_path_d={"M19,19V5H5V19H19M19,3A2,2 0 0,1 21,5V19A2,2 0 0,1 19,21H5A2,2 0 0,1 3,19V5C3,3.89 3.9,3 5,3H19M11,7H13V11H17V13H13V17H11V13H7V11H11V7Z".to_string()} /> }
}

#[function_component(IconClose)]
pub fn icon_close() -> Html {
    html! { <Icon svg_path_d={"M19,6.41L17.59,5L12,10.59L6.41,5L5,6.41L10.59,12L5,17.59L6.41,19L12,13.41L17.59,19L19,17.59L13.41,12L19,6.41Z".to_string()}/>}
}

#[function_component(IconCopy)]
pub fn icon_copy() -> Html {
    html! { <Icon svg_path_d={"M19,21H8V7H19M19,5H8A2,2 0 0,0 6,7V21A2,2 0 0,0 8,23H19A2,2 0 0,0 21,21V7A2,2 0 0,0 19,5M16,1H4A2,2 0 0,0 2,3V17H4V3H16V1Z".to_string()}/>}
}

#[function_component(IconDownload)]
pub fn icon_download() -> Html {
    html! { <Icon svg_path_d={"M5,20H19V18H5M19,9H15V3H9V9H5L12,16L19,9Z"}/>}
}

#[function_component(IconUpload)]
pub fn icon_upload() -> Html {
    html! { <Icon svg_path_d={"M9,16V10H5L12,3L19,10H15V16H9M5,20V18H19V20H5Z".to_string()}/>}
}

#[function_component(IconContextMenu)]
pub fn icon_contextmenu() -> Html {
    html! { <Icon svg_path_d={"M12,16A2,2 0 0,1 14,18A2,2 0 0,1 12,20A2,2 0 0,1 10,18A2,2 0 0,1 12,16M12,10A2,2 0 0,1 14,12A2,2 0 0,1 12,14A2,2 0 0,1 10,12A2,2 0 0,1 12,10M12,4A2,2 0 0,1 14,6A2,2 0 0,1 12,8A2,2 0 0,1 10,6A2,2 0 0,1 12,4Z".to_string()}/>}
}

#[function_component(IconLink)]
pub fn icon_link() -> Html {
    html! { <Icon svg_path_d={"M3.9,12C3.9,10.29 5.29,8.9 7,8.9H11V7H7A5,5 0 0,0 2,12A5,5 0 0,0 7,17H11V15.1H7C5.29,15.1 3.9,13.71 3.9,12M8,13H16V11H8V13M17,7H13V8.9H17C18.71,8.9 20.1,10.29 20.1,12C20.1,13.71 18.71,15.1 17,15.1H13V17H17A5,5 0 0,0 22,12A5,5 0 0,0 17,7Z".to_string()}/>}
}

#[function_component(IconDelete)]
pub fn icon_delete() -> Html {
    html! { <Icon svg_path_d={"M19,4H15.5L14.5,3H9.5L8.5,4H5V6H19M6,19A2,2 0 0,0 8,21H16A2,2 0 0,0 18,19V7H6V19Z".to_string()}/>}
}

#[function_component(IconRemove)]
pub fn icon_remove() -> Html {
    html! { <Icon svg_path_d={"M18 11H10V9H18M20 4V16H8V4H20M20 2H8C6.9 2 6 2.9 6 4V16C6 17.11 6.9 18 8 18H20C21.11 18 22 17.11 22 16V4C22 2.9 21.11 2 20 2M4 6H2V20C2 21.11 2.9 22 4 22H18V20H4V6Z".to_string()}/>}
}

#[function_component(IconAddToAlbum)]
pub fn icon_addtoalbum() -> Html {
    html! { <Icon svg_path_d={"M18 11H15V14H13V11H10V9H13V6H15V9H18M20 4V16H8V4H20M20 2H8C6.9 2 6 2.9 6 4V16C6 17.11 6.9 18 8 18H20C21.11 18 22 17.11 22 16V4C22 2.9 21.11 2 20 2M4 6H2V20C2 21.11 2.9 22 4 22H18V20H4V6Z".to_string()}/>}
}

#[function_component(IconImage)]
pub fn icon_image() -> Html {
    html! { <Icon svg_path_d={"M14,2L20,8V20A2,2 0 0,1 18,22H6A2,2 0 0,1 4,20V4A2,2 0 0,1 6,2H14M18,20V9H13V4H6V20H18M17,13V19H7L12,14L14,16M10,10.5A1.5,1.5 0 0,1 8.5,12A1.5,1.5 0 0,1 7,10.5A1.5,1.5 0 0,1 8.5,9A1.5,1.5 0 0,1 10,10.5Z".to_string()}/>}
}

#[function_component(IconPublic)]
pub fn icon_public() -> Html {
    html! { <Icon svg_path_d={"M17.9,17.39C17.64,16.59 16.89,16 16,16H15V13A1,1 0 0,0 14,12H8V10H10A1,1 0 0,0 11,9V7H13A2,2 0 0,0 15,5V4.59C17.93,5.77 20,8.64 20,12C20,14.08 19.2,15.97 17.9,17.39M11,19.93C7.05,19.44 4,16.08 4,12C4,11.38 4.08,10.78 4.21,10.21L9,15V16A2,2 0 0,0 11,18M12,2A10,10 0 0,0 2,12A10,10 0 0,0 12,22A10,10 0 0,0 22,12A10,10 0 0,0 12,2Z".to_string()}/>}
}

#[function_component(IconShare)]
pub fn icon_share() -> Html {
    html! { <Icon svg_path_d={"M18,16.08C17.24,16.08 16.56,16.38 16.04,16.85L8.91,12.7C8.96,12.47 9,12.24 9,12C9,11.76 8.96,11.53 8.91,11.3L15.96,7.19C16.5,7.69 17.21,8 18,8A3,3 0 0,0 21,5A3,3 0 0,0 18,2A3,3 0 0,0 15,5C15,5.24 15.04,5.47 15.09,5.7L8.04,9.81C7.5,9.31 6.79,9 6,9A3,3 0 0,0 3,12A3,3 0 0,0 6,15C6.79,15 7.5,14.69 8.04,14.19L15.16,18.34C15.11,18.55 15.08,18.77 15.08,19C15.08,20.61 16.39,21.91 18,21.91C19.61,21.91 20.92,20.61 20.92,19A2.92,2.92 0 0,0 18,16.08Z".to_string()}/>}
}

#[function_component(IconBack)]
pub fn icon_back() -> Html {
    html! { <Icon svg_path_d={"M20,11V13H8L13.5,18.5L12.08,19.92L4.16,12L12.08,4.08L13.5,5.5L8,11H20Z".to_string()}/>}
}

#[function_component(IconRefresh)]
pub fn icon_refresh() -> Html {
    html! { <Icon svg_path_d={"M17.65,6.35C16.2,4.9 14.21,4 12,4A8,8 0 0,0 4,12A8,8 0 0,0 12,20C15.73,20 18.84,17.45 19.73,14H17.65C16.83,16.33 14.61,18 12,18A6,6 0 0,1 6,12A6,6 0 0,1 12,6C13.66,6 15.14,6.69 16.22,7.78L13,11H20V4L17.65,6.35Z".to_string()}/>}
}

#[function_component(IconChevronUp)]
pub fn icon_chevronup() -> Html {
    html! { <Icon svg_path_d={"M7.41,15.41L12,10.83L16.59,15.41L18,14L12,8L6,14L7.41,15.41Z".to_string()}/>}
}

#[function_component(IconChevronDown)]
pub fn icon_chevrondown() -> Html {
    html! { <Icon svg_path_d={"M7.41,8.58L12,13.17L16.59,8.58L18,10L12,16L6,10L7.41,8.58Z".to_string()}/>}
}

#[function_component(IconChevronLeft)]
pub fn icon_chevronleft() -> Html {
    html! { <Icon svg_path_d={"M15.41,16.58L10.83,12L15.41,7.41L14,6L8,12L14,18L15.41,16.58Z".to_string()}/>}
}

#[function_component(IconChevronRight)]
pub fn icon_chevronright() -> Html {
    html! { <Icon svg_path_d={"M8.59,16.58L13.17,12L8.59,7.41L10,6L16,12L10,18L8.59,16.58Z".to_string()}/>}
}

#[function_component(IconMenu)]
pub fn icon_menu() -> Html {
    html! { <Icon svg_path_d={"M3,6H21V8H3V6M3,11H21V13H3V11M3,16H21V18H3V16Z"}/>}
}

#[function_component(IconHashTag)]
pub fn icon_hashtag() -> Html {
    html! { <Icon svg_path_d={"M5.41,21L6.12,17H2.12L2.47,15H6.47L7.53,9H3.53L3.88,7H7.88L8.59,3H10.59L9.88,7H15.88L16.59,3H18.59L17.88,7H21.88L21.53,9H17.53L16.47,15H20.47L20.12,17H16.12L15.41,21H13.41L14.12,17H8.12L7.41,21H5.41M9.53,9L8.47,15H14.47L15.53,9H9.53Z".to_string()}/>}
}

#[function_component(IconCheck)]
pub fn icon_check() -> Html {
    html! { <Icon svg_path_d={"M21,7L9,19L3.5,13.5L4.91,12.09L9,16.17L19.59,5.59L21,7Z".to_string()}/>}
}

#[function_component(IconInfo)]
pub fn icon_info() -> Html {
    html! { <Icon svg_path_d={"M11,9H13V7H11M12,20C7.59,20 4,16.41 4,12C4,7.59 7.59,4 12,4C16.41,4 20,7.59 20,12C20,16.41 16.41,20 12,20M12,2A10,10 0 0,0 2,12A10,10 0 0,0 12,22A10,10 0 0,0 22,12A10,10 0 0,0 12,2M11,17H13V11H11V17Z".to_string()}/>}
}
