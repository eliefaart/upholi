use crate::exif::Exif;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PhotoExifProps {
    pub exif: Exif,
}

#[function_component(PhotoExif)]
pub fn photo_exif(props: &PhotoExifProps) -> Html {
    let model = {
        let manufactorer = props.exif.manufactorer.clone();
        let model = props.exif.model.clone();

        html! {
            <PhotoExifProperty title="Camera" values={vec![manufactorer, model]}/>
        }
    };

    let exposure = {
        let aperture = props.exif.aperture.clone();
        let exposure_time = props.exif.exposure_time.clone();
        let iso = props.exif.iso.map(|iso| format!("ISO-{iso}"));

        html! {
            <PhotoExifProperty title="Exposure" values={vec![aperture, exposure_time, iso]}/>
        }
    };

    let focal_length = {
        let focal_length = props.exif.focal_length.map(|fl| format!("{fl}mm"));
        let focal_length_35mm_equiv = props
            .exif
            .focal_length_35mm_equiv
            .map(|fl| format!("({fl}mm in 35mm equivalent)"));

        html! {
            <PhotoExifProperty title="Focal length" values={vec![focal_length, focal_length_35mm_equiv]}/>
        }
    };

    let taken_on = {
        let date_taken = props.exif.date_taken.map(|dt| dt.to_string());

        html! {
            <PhotoExifProperty title="Taken on" values={vec![date_taken]}/>
        }
    };

    let location = {
        let map_link = {
            if let (Some(lat), Some(lon)) = (props.exif.gps_latitude, props.exif.gps_longitude) {
                Some(html! {
                    <a target="_blank"
                        href={format!("https://www.openstreetmap.org/#map=18/{lat}/{lon}")}
                        rel="noreferrer">
                        {format!("{lat}, {lon}")}
                    </a>
                })
            } else {
                None
            }
        };

        html! {
            if let Some(map_link) = map_link {
                <PhotoExifProperty title="Location" element={map_link}/>
            }
        }
    };

    html! {
        <div class="photo-exif">
            {model}
            {exposure}
            {focal_length}
            {taken_on}
            {location}
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct PhotoExifPropertyProps {
    pub title: AttrValue,
    #[prop_or_default]
    pub values: Vec<Option<String>>,
    pub element: Option<Children>,
}

#[function_component(PhotoExifProperty)]
fn photo_exif_property(props: &PhotoExifPropertyProps) -> Html {
    let any_value = props.element.is_some() || props.values.iter().any(|f| f.is_some());

    if any_value {
        let values: Vec<String> = props.values.clone().into_iter().flatten().collect();
        let value = values.join(" ");

        let value_or_element = if let Some(element) = props.element.clone() {
            html! { <>{element}</> }
        } else {
            html! { {&value} }
        };

        html! {
            <div class="photo-exif-property">
                <span class="title">{&props.title}</span>
                <span class="value">{value_or_element}</span>
            </div>
        }
    } else {
        html! {}
    }
}
