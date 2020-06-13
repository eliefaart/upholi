extern crate base64;

use image::{GenericImageView, DynamicImage, ImageFormat};

const DIMENSIONS_THUMB: u32 = 700;
const DIMENSIONS_PREVIEW: u32 = 2250;

pub struct Image {
	pub width: u32,
	pub height: u32,
	pub bytes_original: Vec<u8>,
	pub bytes_thumbnail: Vec<u8>,
	pub bytes_preview: Vec<u8>
}

impl Image {

	/// Process image from buffer
	pub fn from_buffer(bytes: &[u8], exif_orientation: u8) -> Result<Self, String>  {
		//let image = Self::get_image_from_bytes(bytes);
		match image::load_from_memory(&bytes[0..]) {
			Ok(image) => {
				let (mut width, mut height) = Self::get_image_dimensions(&image);

				// Generate thumbs
				let mut image_preview = Self::resize_image(&image, DIMENSIONS_PREVIEW).unwrap_or_else(|| Self::clone_image(&image));
				let mut image_thumbnail = Self::resize_image(&image_preview, DIMENSIONS_THUMB).unwrap_or_else(|| Self::clone_image(&image_preview));
		
				// Rotate if needed
				if let Some(rotated_image) = Self::rotate_image_upright(&image_thumbnail, exif_orientation) {
					image_thumbnail = rotated_image;
				}
				if let Some(rotated_image) = Self::rotate_image_upright(&image_preview, exif_orientation) {
					image_preview = rotated_image;
				}
		
				// For some orientations, I need to swap the width and height
				if exif_orientation >= 5 && exif_orientation <= 8 {
					std::mem::swap(&mut height, &mut width)
				} 
		
				Ok(Self {
					width,
					height,
					bytes_original: bytes.to_vec(),
					bytes_thumbnail: Self::get_image_bytes(&image_thumbnail),
					bytes_preview: Self::get_image_bytes(&image_preview)
				})
			},
			Err(_) => Err("Corrupt image".to_string())
		}
	}

	/// Get current dimensions
	fn get_image_dimensions(image: &DynamicImage) -> (u32, u32) {
		let photo_width = image.width();
		let photo_height = image.height();
	
		(photo_width, photo_height)
	}

	/// Scale this image down to fit within a specific size. The image's aspect ratio is preserved. The image is scaled to the maximum size possible while neither height nor width exceeding specified 'to_size'.
	fn resize_image(image: &DynamicImage, to_size: u32) -> Option<DynamicImage> {
		// Get current dimensions
		let (width, height) = Self::get_image_dimensions(&image);
		
		// Check if resizing image would make sense based on current dimensions
		// Only resize if target dimensions are smaller than current ones.
		if width > to_size && height > to_size {
			Some(image.thumbnail(to_size, to_size))
		} else {
			None
		}
	}

	/// Rotate image so it is displayed in correct orientation when the oientation exif tag is not present
	fn rotate_image_upright(image: &DynamicImage, cur_exif_orientation: u8) -> Option<DynamicImage> {
		if cur_exif_orientation == 2 {
			Some(image.fliph())
		} else if cur_exif_orientation == 3 {
			Some(image.rotate180())
		} else if cur_exif_orientation == 4 {
			Some(image.flipv())
		} else if cur_exif_orientation == 5 {
			Some(image.rotate90().fliph())
		} else if cur_exif_orientation == 6 {
			Some(image.rotate90())
		} else if cur_exif_orientation == 7 {
			Some(image.rotate90().flipv())
		} else if cur_exif_orientation == 8 {
			Some(image.rotate270())
		} else {
			None	// Orientation is 1 (the desired orientation), or an invalid value
		}
	}

	/// Clone an image
	fn clone_image(image: &DynamicImage) -> DynamicImage {
		Self::get_image_from_bytes(&Self::get_image_bytes(image))
	}

	/// Convert image bytes to image object
	fn get_image_from_bytes(image_bytes: &[u8]) -> DynamicImage {
		let image_result = image::load_from_memory(&image_bytes[0..]);
		image_result.unwrap()
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
}

/* 
	A PNG file in bytes, to use for test cases later.
	let minipng = [137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0, 1,
               0, 0, 0, 1, 8, 0, 0, 0, 0, 58, 126, 155, 85, 0, 0, 0, 10, 73, 68, 65, 84,
               8, 215, 99, 248, 15, 0, 1, 1, 1, 0, 27, 182, 238, 86, 0, 0, 0, 0, 73, 69,
               78, 68, 174, 66, 96, 130];
*/