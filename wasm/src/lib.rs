use wasm_bindgen::prelude::*;

mod client;
mod entities;
mod images;
mod exif;
mod encryption;
mod multipart;
mod hashing;

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(js_namespace = console)]
	fn log(s: &str);

	#[wasm_bindgen(js_namespace = console)]
	fn error(s: &str);
}

/// Name of the encryption keys created by an entity's owner.
const OWNER_KEY_NAME: &str = "_owner";