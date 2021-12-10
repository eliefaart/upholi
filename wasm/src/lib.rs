use wasm_bindgen::prelude::*;

mod client;
mod encryption;
mod entities;
mod exif;
mod hashing;
mod images;
mod multipart;

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(js_namespace = console)]
	fn log(s: &str);

	#[wasm_bindgen(js_namespace = console)]
	fn error(s: &str);
}
