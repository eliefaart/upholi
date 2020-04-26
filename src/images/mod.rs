extern crate base64;

pub fn from_base64(image_base64: &str) -> Vec<u8> {
	let result = base64::decode(image_base64);
	let image_bytes = result.unwrap();

	image_bytes
}