use serde::{Deserialize, Serialize};
use std::fmt;

pub mod http;
pub mod ids;
pub mod passwords;

pub enum PhotoVariant {
	Original,
	Preview,
	Thumbnail,
}

impl fmt::Display for PhotoVariant {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			PhotoVariant::Thumbnail => write!(f, "thumbnail"),
			PhotoVariant::Preview => write!(f, "preview"),
			PhotoVariant::Original => write!(f, "original"),
		}
	}
}

impl From<PhotoVariant> for String {
	fn from(photo_variant: PhotoVariant) -> String {
		photo_variant.to_string()
	}
}

impl From<&PhotoVariant> for String {
	fn from(photo_variant: &PhotoVariant) -> String {
		photo_variant.to_string()
	}
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ShareType {
	Album,
}

impl Clone for ShareType {
	fn clone(&self) -> Self {
		match *self {
			Self::Album => Self::Album,
		}
	}
}
