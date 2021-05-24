use wasm_bindgen::prelude::*;

// https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm

// One time needed in ../app/:
// npm install --save ..\hello-wasm\pkg\

#[wasm_bindgen]
extern {
	pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
	alert(&format!("Hello, :D {}....!", name));
}

#[wasm_bindgen]
pub fn hello(name: &str) -> String {
	format!("Hello, {}", name)
}