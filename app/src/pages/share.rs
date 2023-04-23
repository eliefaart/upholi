use crate::{
    components::{Button, Gallery},
    hooks::{use_is_authorized_for_share, use_share_album},
    WASM_CLIENT,
};
use web_sys::HtmlInputElement;
use yew::{
    function_component, html, use_effect_with_deps, use_node_ref, use_state, AttrValue, Callback, Html, Properties,
};

#[derive(Properties, PartialEq)]
pub struct SharePageProps {
    pub id: AttrValue,
}

#[function_component(SharePage)]
pub fn share_page(props: &SharePageProps) -> Html {
    let (album, refresh_album) = use_share_album(props.id.to_string());
    let authorized = use_is_authorized_for_share(&props.id);
    let auth_attempt_made = use_state(|| false);
    let selected_photos = use_state(Vec::new);

    {
        let refresh_album = refresh_album.clone();
        use_effect_with_deps(
            move |_| {
                refresh_album.emit(());
            },
            authorized.clone(),
        )
    }

    let on_try_authorize = {
        let share_id = props.id.to_string();
        let auth_attempt_made = auth_attempt_made.clone();

        Callback::from(move |password: String| {
            let share_id = share_id.clone();
            let auth_attempt_made = auth_attempt_made.clone();
            let refresh_album = refresh_album.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let authorized = WASM_CLIENT.authorize_share(&share_id, &password).await.unwrap();
                if !*auth_attempt_made {
                    auth_attempt_made.set(true);
                }

                if authorized {
                    refresh_album.emit(());
                }
            });
        })
    };

    if let Some(authorized) = *authorized {
        if let Some(album) = (*album).clone() {
            html! {
                <>
                    <h1>{ &album.title }</h1>
                    <Gallery photos={album.photos} selected_photos={selected_photos}/>
                </>
            }
        } else {
            html! {
                if !authorized {
                    <>
                        <UnlockShare
                            share_id={&props.id}
                            on_try_authorize={on_try_authorize}
                            auth_attempt_made={*auth_attempt_made}
                            />
                    </>
                }
            }
        }
    } else {
        html! {}
    }
}

#[derive(Properties, PartialEq)]
pub struct UnlockShareProps {
    pub share_id: AttrValue,
    pub on_try_authorize: Callback<String>,
    pub auth_attempt_made: bool,
}

#[function_component(UnlockShare)]
pub fn unlock_share(props: &UnlockShareProps) -> Html {
    let password_ref = use_node_ref();

    let on_click_unlock = {
        let on_try_authorize = props.on_try_authorize.clone();
        let password_ref = password_ref.clone();

        move |_| {
            let on_try_authorize = on_try_authorize.clone();
            if let Some(password_input) = password_ref.cast::<HtmlInputElement>() {
                let password = password_input.value();
                on_try_authorize.emit(password);
            }
        }
    };

    html! {
        <div class="unlock-share">
            <h1>{"Locked"}</h1>
            <label>{"Password"}
                <input ref={password_ref} type="password"/>
            </label>
            <div class="footer">
                <span class="status">
                    {html! {
                        if props.auth_attempt_made {
                            <>{"Incorrect password"}</>
                        }
                    }}
                </span>
                <Button label={"Submit"} on_click={on_click_unlock}/>
            </div>
        </div>
    }
}
