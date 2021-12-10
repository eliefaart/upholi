use chrono::prelude::*;
use rexif::{ExifTag, TagValue};
use serde::{Deserialize, Serialize};
use upholi_lib::result::Result;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Exif {
	pub manufactorer: Option<String>,
	pub model: Option<String>,
	pub aperture: Option<String>,
	pub exposure_time: Option<String>,
	pub iso: Option<i32>,
	pub focal_length: Option<i32>,
	pub focal_length_35mm_equiv: Option<i32>,
	pub orientation: Option<i32>,
	pub date_taken: Option<chrono::DateTime<Utc>>,
	pub gps_latitude: Option<f32>,
	pub gps_longitude: Option<f32>,
}

impl Exif {
	/// Parse EXIF data from photo bytes. Bytes can represent a .jpg or .tiff file.
	pub fn parse_from_photo_bytes(photo_bytes: &[u8]) -> Result<Exif> {
		let result = rexif::parse_buffer(photo_bytes);
		match result {
			Ok(exif) => {
				let closure_get_exif_data_as_string =
					|tag: ExifTag| -> Option<String> { Self::get_exif_data(&exif, tag, Self::convert_exif_to_string) };

				let closure_get_exif_data_as_i32 =
					|tag: ExifTag| -> Option<i32> { Self::get_exif_data(&exif, tag, Self::convert_exif_to_i32) };

				let closure_get_exif_data_as_datetime = |tag: ExifTag| -> Option<chrono::DateTime<Utc>> {
					Self::get_exif_data(&exif, tag, Self::convert_exif_to_datetime)
				};

				let closure_get_exif_data_as_coord =
					|tag: ExifTag| -> Option<f32> { Self::get_exif_data(&exif, tag, Self::convert_exif_to_gps_coord) };

				// Date taken can be in various EXIF fields.
				let date_taken = closure_get_exif_data_as_datetime(ExifTag::DateTimeOriginal)
					.or_else(|| closure_get_exif_data_as_datetime(ExifTag::DateTime))
					.or_else(|| closure_get_exif_data_as_datetime(ExifTag::DateTimeDigitized));

				Ok(Self {
					manufactorer: closure_get_exif_data_as_string(ExifTag::Make),
					model: closure_get_exif_data_as_string(ExifTag::Model),
					aperture: closure_get_exif_data_as_string(ExifTag::FNumber),
					exposure_time: Self::remove_spaces(&closure_get_exif_data_as_string(ExifTag::ExposureTime)),
					iso: closure_get_exif_data_as_i32(ExifTag::ISOSpeedRatings),
					focal_length: closure_get_exif_data_as_i32(ExifTag::FocalLength),
					focal_length_35mm_equiv: closure_get_exif_data_as_i32(ExifTag::FocalLengthIn35mmFilm),
					orientation: closure_get_exif_data_as_i32(ExifTag::Orientation),
					date_taken,
					gps_latitude: closure_get_exif_data_as_coord(ExifTag::GPSLatitude),
					gps_longitude: closure_get_exif_data_as_coord(ExifTag::GPSLongitude),
				})
			}
			Err(error) => {
				match error {
					// Some errors are fine, we just return default Exif for these cases,
					// For others we still return error
					rexif::ExifError::JpegWithoutExif(_) => Ok(Self::default()),
					rexif::ExifError::FileTypeUnknown => Ok(Self::default()),
					rexif::ExifError::UnsupportedNamespace => Ok(Self::default()),
					_ => Err(Box::from(format!("{:?}", error))),
				}
			}
		}
	}

	/// Gets the value of given exif field
	fn get_exif_data<T>(exif: &rexif::ExifData, tag: ExifTag, convert_value: fn(&rexif::ExifEntry) -> Option<T>) -> Option<T> {
		let result = exif.entries.iter().find(|entry| entry.tag == tag);
		if let Some(entry) = result {
			convert_value(entry)
		} else {
			None
		}
	}

	/// Convert exif field to String
	fn convert_exif_to_string(entry: &rexif::ExifEntry) -> Option<String> {
		match &entry.value {
			TagValue::Ascii(val) => Some(val.to_string()),
			TagValue::URational(_) => Some(entry.value_more_readable.to_string()),
			_ => {
				println!("convert_exif_to_string: {:?}", entry.value);
				None
			}
		}
	}

	/// Convert exif field to i32
	fn convert_exif_to_i32(entry: &rexif::ExifEntry) -> Option<i32> {
		match &entry.value {
			TagValue::URational(rat) => Some(rat[0].numerator as i32 / rat[0].denominator as i32),
			TagValue::U16(val) => Some(val[0] as i32),
			_ => {
				println!("convert_exif_to_i32: {:?}", entry.value);
				None
			}
		}
	}

	/// Convert exif field to chrono::DateTime<Utc>
	fn convert_exif_to_datetime(entry: &rexif::ExifEntry) -> Option<chrono::DateTime<Utc>> {
		match &entry.value {
			TagValue::Ascii(val) => {
				let result = Utc.datetime_from_str(val, "%Y:%m:%d %H:%M:%S");
				if let Ok(datetime) = result {
					Some(datetime)
				} else {
					None
				}
			}
			_ => None,
		}
	}

	/// Convert exif field to f32 representing a coordinate
	fn convert_exif_to_gps_coord(entry: &rexif::ExifEntry) -> Option<f32> {
		match &entry.value {
			TagValue::URational(rat) => {
				// rat should have 3 entries; degrees, minutes, seconds
				if rat.len() != 3 {
					None
				} else {
					let degrees = rat[0].numerator as f32 / rat[0].denominator as f32;
					let minutes = rat[1].numerator as f32 / rat[1].denominator as f32;
					let seconds = rat[2].numerator as f32 / rat[2].denominator as f32;

					let coord = degrees + (minutes / 60f32) + (seconds / 3600f32);
					Some(coord)
				}
			}
			_ => None,
		}
	}

	/// Remove the spaces from the string-value contained within the option.
	/// If the option contains None, this function will return None.
	fn remove_spaces(option: &Option<String>) -> Option<String> {
		option.as_ref().map(|string| string.replace(" ", ""))
	}
}
