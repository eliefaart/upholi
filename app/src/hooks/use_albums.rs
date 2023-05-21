use regex::Regex;
use yew::prelude::*;

#[hook]
pub fn use_albums() -> (UseStateHandle<Vec<crate::models::Album>>, Callback<()>) {
    let albums = use_state(Vec::new);

    let refresh_albums = {
        let albums_state = albums.clone();
        Callback::from(move |_| {
            let albums_state = albums_state.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let mut albums = crate::WASM_CLIENT.get_albums().await.unwrap();

                let regex_contnains_year = Regex::new(r".?(?<year>[0-9]{4}).?").unwrap();

                // Sort albums.
                // Basically albums are sorted by title alphabetically.
                // However, albums with a year (eg 2022) in the title are moved to the front and are sorted by year descending.
                // It's a bit magical, but that is fine for now. :D
                albums.sort_by(|a, b| {
                    let a_year = regex_contnains_year
                        .captures(&a.title)
                        .and_then(|cap| cap.name("year").map(|year| year.as_str().to_string()));
                    let b_year = regex_contnains_year
                        .captures(&b.title)
                        .and_then(|cap| cap.name("year").map(|year| year.as_str().to_string()));

                    if a_year.is_none() && b_year.is_none() {
                        String::cmp(&a.title, &b.title)
                    } else if a_year.is_some() && b_year.is_none() {
                        std::cmp::Ordering::Less
                    } else if a_year.is_none() && b_year.is_some() {
                        std::cmp::Ordering::Greater
                    } else {
                        // They both have values now
                        let a_year = a_year.unwrap();
                        let b_year = b_year.unwrap();
                        // order desc
                        String::cmp(&b_year, &a_year)
                    }
                });

                albums_state.set(albums);
            });
        })
    };

    {
        let refresh_albums = refresh_albums.clone();
        use_effect_with_deps(move |_| refresh_albums.emit(()), ());
    }

    (albums, refresh_albums)
}
