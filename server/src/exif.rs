use serde::{Serialize, Deserialize};
use chrono::prelude::*;
use chrono::{DateTime};

#[derive(Serialize, Deserialize, Debug)]
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
	pub date_taken: Option<chrono::DateTime<Utc>>
}

impl Exif {
	/// Parse EXIF data from photo bytes. Bytes can represent a .jpg or .tiff file.
	pub fn parse_from_photo_bytes(photo_bytes: &Vec<u8>) -> Result<Exif, String> {

		let result = rexif::parse_buffer(photo_bytes);
		match result {
			Ok(exif) => {

				// These closures are all very similar, except the handling of &entry.value
				// Not sure if I can reduplicate it. Possibly using macros but havn't looked into it yet.
				// I can't make functions instead of closures, because rexif::types is private.

				let closure_get_exif_data_as_string = |tag: rexif::ExifTag| -> Option<String> {
					let result = exif.entries.iter().find(|entry| entry.tag == tag);
					if let Some(entry) = result {
						match &entry.value {
							rexif::TagValue::Ascii(val) => Some(val.to_string()),
							rexif::TagValue::URational(_) => Some(entry.value_more_readable.to_string()),
							_ => {
								println!("{:?}", entry.value);
								None
							}
						}
					} else {
						None
					}
				};

				let closure_get_exif_data_as_i32 = |tag: rexif::ExifTag| -> Option<i32> {
					let result = exif.entries.iter().find(|entry| entry.tag == tag);
					if let Some(entry) = result {
						match &entry.value {
							rexif::TagValue::U32(val) => Some(val[0] as i32),
							rexif::TagValue::U16(val) => Some(val[0] as i32),
							rexif::TagValue::U8(val) => Some(val[0] as i32),
							rexif::TagValue::I32(val) => Some(val[0] as i32),
							rexif::TagValue::I16(val) => Some(val[0] as i32),
							rexif::TagValue::I8(val) => Some(val[0] as i32),
							rexif::TagValue::URational(rat) => Some(rat[0].numerator as i32 / rat[0].denominator as i32),
							_ => {
								println!("{:?}", entry.value);
								None
							}
						}
					} else {
						None
					}
				};

				let closure_get_exif_data_as_datetime = |tag: rexif::ExifTag| -> Option<chrono::DateTime<Utc>> {
					let result = exif.entries.iter().find(|entry| entry.tag == tag);
					if let Some(entry) = result {
						match &entry.value {
							rexif::TagValue::Ascii(val) => {
								let result = Utc.datetime_from_str(val, "%Y:%m:%d %H:%M:%S");
								if let Ok(datetime) = result {
									let dt: DateTime<Utc> = DateTime::from(datetime);
									Some(dt)
								} else {
									None
								}
							},
							_ => None
						}
					} else {
						None
					}
				};

				Ok(Self {
					manufactorer: closure_get_exif_data_as_string(rexif::ExifTag::Make),
					model: closure_get_exif_data_as_string(rexif::ExifTag::Model),
					aperture: closure_get_exif_data_as_string(rexif::ExifTag::FNumber),
					exposure_time: closure_get_exif_data_as_string(rexif::ExifTag::ExposureTime),
					iso: closure_get_exif_data_as_i32(rexif::ExifTag::ISOSpeedRatings),
					focal_length: closure_get_exif_data_as_i32(rexif::ExifTag::FocalLength),
					focal_length_35mm_equiv: closure_get_exif_data_as_i32(rexif::ExifTag::FocalLengthIn35mmFilm),
					orientation: closure_get_exif_data_as_i32(rexif::ExifTag::Orientation),
					date_taken: closure_get_exif_data_as_datetime(rexif::ExifTag::DateTime),
				})
			},
			Err(error) => Err(format!("{:?}", error))
		}
	}
}