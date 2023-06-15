use bounce::{use_atom, Atom};
use yew::prelude::*;

#[derive(Atom, PartialEq, Default)]
struct OverlayState(bool);

#[hook]
pub fn use_overlay() -> (bool, Callback<bool>) {
    let state = use_atom::<OverlayState>();

    let set_state = {
        let state = state.clone();

        Callback::from(move |visible: bool| {
            state.set(OverlayState(visible));
        })
    };

    (state.0, set_state)
}
