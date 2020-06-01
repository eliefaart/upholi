extern crate base64;

use image::{GenericImageView, DynamicImage, ImageFormat};

/*
	TODO
	This can be optimized if I can make an image class, with get_image_dimensions and resize_image
	as its functions. Then I only need to load load image from bytes once.
*/

pub fn get_image_dimensions(image_bytes: &Vec<u8>) -> (u32, u32) {
	let image = get_image_from_bytes(image_bytes);

	// Get current dimensions
	let photo_width = image.width();
	let photo_height = image.height();

	(photo_width, photo_height)
}

/// Creates a new image with desires size (dimensions)
pub fn resize_image(image_bytes: &Vec<u8>, to_size: u32) -> Vec<u8> {
	let image = get_image_from_bytes(image_bytes);

	// Get current dimensions
	let photo_width = image.width();
	let photo_height = image.height();
	
	if photo_width > to_size && photo_height > to_size {
		let image_thumbnail = image.thumbnail(to_size, to_size);
		get_image_bytes(&image_thumbnail)
	}
	else {
		// Return bytes as-is from original image. Due to how image is loaded into memory, 
		// the size is usually reduced. Possibly because of 'more' or different compression.
		get_image_bytes(&image)
	}
}

pub fn rotate_image_upright(image_bytes: &Vec<u8>, cur_exif_orientation: u8) -> Option<Vec<u8>> {
	let image = get_image_from_bytes(image_bytes);

	if cur_exif_orientation == 8 {
		Some(get_image_bytes(&image.rotate270()))
	} else {
		// Already correct orientation
		None
	}
}

/// Convert image bytes to image object
fn get_image_from_bytes(image_bytes: &Vec<u8>) -> DynamicImage {
	let image_result = image::load_from_memory(&image_bytes[0..]);
	let image = image_result.unwrap();
	image
}

/// Get bytes from image in Jpeg format
fn get_image_bytes(image: &DynamicImage) -> Vec<u8> {
	let mut buffer: Vec<u8> = Vec::new();
	let write_result = image.write_to(&mut buffer, ImageFormat::Jpeg);
	match write_result {
		Ok(_x) => (),
		Err(e) => println!("{}", e)
	}
	buffer
}
