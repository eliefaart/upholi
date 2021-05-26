use std::error::Error;
use wasm_bindgen::prelude::*;

mod aes256;

pub type Result<T, E = Box<dyn Error>> = std::result::Result<T, E>;

// https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm

// One time needed in ../app/:
// npm install --save ..\hello-wasm\pkg\

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(js_namespace = console)]
	fn log(s: &str);

	#[wasm_bindgen(js_namespace = console)]
	fn error(s: &str);
}

#[wasm_bindgen]
pub fn get_string(input: &str) -> String {
	format!("{}:{}", input, input)
}

#[wasm_bindgen]
pub fn get_array_u32() -> js_sys::Uint32Array {
	let rust_array = vec!{ 3289, 43434, 23, 4_000_000 };
	js_sys::Uint32Array::from(&rust_array[..])
}

#[wasm_bindgen]
pub fn get_array_u8() -> js_sys::Uint8Array {
	let rust_array = vec!{ 123,192,39,0,255 };
	js_sys::Uint8Array::from(&rust_array[..])
}

#[wasm_bindgen]
pub fn aes256_encrypt(buffer: &[u8]) -> js_sys::Uint8Array {
	let key = b"e0ca4c29d5504e8daa8c52e873e66f71";
	let nonce = b"452b4dd698de";

	let result = aes256::encrypt(key, nonce, buffer);
	match result {
		Ok(bytes) => js_sys::Uint8Array::from(&bytes[..]),
		Err(err) => {
			error(&format!("{}", err));
			js_sys::Uint8Array::from(&buffer[0..0])
		}
	}
}

#[wasm_bindgen]
pub fn aes256_decrypt(buffer: &[u8]) -> js_sys::Uint8Array {
	let key = b"e0ca4c29d5504e8daa8c52e873e66f71";
	let nonce = b"452b4dd698de";

	let result = aes256::decrypt(key, nonce, buffer);
	match result {
		Ok(bytes) => js_sys::Uint8Array::from(&bytes[..]),
		Err(err) => {
			error(&format!("{}", err));
			js_sys::Uint8Array::from(&buffer[0..0])
		}
	}
}

pub struct Photo {
	pub id: String
}
pub fn get_photos() -> Vec<Photo> {
	// NEXT:
	// HTTP REST request to get info
	// then return typed array somehow
}

#[wasm_bindgen]
pub struct SomeData {
	pub id: u32,
	pub something: u32,
	//pub aaa: String
}

#[wasm_bindgen]
pub fn get_struct() -> SomeData {
	SomeData {
		id: 4389,
		something: 2387,
		//aaa: "as".into()
	}
}

// #[js_sys::wasm_bindgen]
// pub struct my_struct {
// 	my_vec: Vec<u32>,
// }

// #[wasm_bindgen]
// impl my_struct {
// 	#[wasm_bindgen(getter)]
// 	pub fn my_vec(&self) -> js_sys::Uint32Array {
// 		return js_sys::Uint32Array::from(&self.my_vec[..]);
// 	}
// }

// #[wasm_bindgen]
// pub fn get_struct2() -> my_struct {
// 	my_struct {
// 		my_vec: vec!{ 3289, 43434, 23, 4_000_000 }
// 	}
// }
