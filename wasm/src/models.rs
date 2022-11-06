use crate::exif::Exif;
use anyhow::Result;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TextItem {
	pub key: String,
	pub base64: String,
	pub nonce: String,
}

impl TextItem {
	pub fn from<T: Serialize>(key: &[u8], item: &T) -> Result<Self> {
		let json = serde_json::to_string(item)?;
		let bytes = json.as_bytes();
		let encrypt_result = crate::encryption::symmetric::encrypt_slice(key, bytes)?;
		let base64 = base64::encode_config(encrypt_result.bytes, base64::STANDARD);
		Ok(Self {
			key: String::new(),
			base64,
			nonce: encrypt_result.nonce,
		})
	}

	pub fn decrypt<TDecrypted: DeserializeOwned>(&self, key: &[u8]) -> Result<TDecrypted> {
		let nonce = self.nonce.as_bytes();
		let bytes = base64::decode_config(&self.base64, base64::STANDARD)?;
		let bytes = crate::encryption::symmetric::decrypt_slice(key, nonce, &bytes)?;
		Ok(serde_json::from_slice(&bytes)?)
	}
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Library {
	pub item_keys: Vec<ItemKey>,
	pub photos: Vec<LibraryPhoto>,
	pub albums: Vec<LibraryAlbum>,
	pub shares: Vec<LibraryShare>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ItemKey {
	pub item_id: String,
	pub key: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LibraryPhoto {
	pub id: String,
	pub hash: String,
	pub width: u32,
	pub height: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LibraryAlbum {
	pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LibraryShare {
	pub id: String,
	pub password: String,
	pub album_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Photo {
	pub id: String,
	/// Hash string of original file bytes
	pub hash: String,
	/// Width of photo
	pub width: u32,
	/// Height of photo
	pub height: u32,
	/// A timestamp of the photo used for sorting purposes.
	/// To be filles with the datetime a photo was taken on, or uploaded on.
	pub timestamp: i64,
	pub content_type: String,
	pub exif: Option<Exif>,
	pub nonce_thumbnail: String,
	pub nonce_preview: String,
	pub nonce_original: String,
}

impl From<&Photo> for LibraryPhoto {
	fn from(photo: &Photo) -> Self {
		Self {
			id: photo.id.clone(),
			hash: photo.hash.clone(),
			width: photo.width,
			height: photo.height,
		}
	}
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Album {
	pub id: String,
	pub key: Vec<u8>,
	pub title: String,
	pub thumbnail_photo_id: Option<String>,
	pub tags: Vec<String>,
	pub photos: Vec<String>,
}

/// Album, but with enriched photo data
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumExpanded {
	pub id: String,
	pub title: String,
	pub tags: Vec<String>,
	pub photos: Vec<AlbumPhoto>,
	pub thumbnail_photo: Option<AlbumPhoto>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AlbumPhoto {
	pub id: String,
	pub width: u32,
	pub height: u32,
	pub key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Share {
	pub data: ShareData,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ShareData {
	Album(AlbumShareData),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AlbumShareData {
	pub album_id: String,
	pub album_key: Vec<u8>,
	pub photos: Vec<AlbumShareDataPhoto>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AlbumShareDataPhoto {
	pub id: String,
	pub key: Vec<u8>,
	pub width: u32,
	pub height: u32,
}

impl From<LibraryPhoto> for AlbumPhoto {
	fn from(source: LibraryPhoto) -> Self {
		Self {
			id: source.id,
			width: source.width,
			height: source.height,
			key: None,
		}
	}
}

impl From<AlbumShareDataPhoto> for AlbumPhoto {
	fn from(source: AlbumShareDataPhoto) -> Self {
		Self {
			id: source.id,
			width: source.width,
			height: source.height,
			key: Some(base64::encode_config(source.key, base64::STANDARD)),
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::{
		encryption::symmetric::generate_key,
		models::{Library, TextItem},
	};

	#[test]
	fn encrypt_decrypt_text_item_bytes() {
		let key = generate_key();
		let item = TextItem::from(&key, &key).unwrap();
		let decrypted: Vec<u8> = item.decrypt(&key).unwrap();

		assert_eq!(key, decrypted);
	}

	#[test]
	fn encrypt_decrypt_text_item_instance() {
		let key = generate_key();
		let library = Library::default();
		let item = TextItem::from(&key, &library).unwrap();
		let decrypted: Library = item.decrypt(&key).unwrap();

		assert_eq!(library.photos.len(), decrypted.photos.len());
		assert_eq!(library.item_keys.len(), decrypted.item_keys.len());
	}
}
