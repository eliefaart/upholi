// cargo build --release --target=wasm32-unknown-unknown

// extern {
// 	fn test() -> String;
// }

// #[no_mangle]
// pub fn test_imported_func() -> usize {
// 	unsafe {
// 		test().len()
// 	}
// }

#[no_mangle]
pub fn get_number() -> u32 {
	53
}

// #[no_mangle]
// pub fn show_string() {
// 	unsafe {
// 		alert("asass");
// 	}
// }

#[no_mangle]
pub fn add(a: u32, b: u32) -> u32 {
	a + b
}