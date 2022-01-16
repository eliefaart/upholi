use crate::client::helper::PhotoUploadInfo;
use js_sys::{Array, JsString};
use upholi_lib::{PhotoVariant, ShareType};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

use crate::entities::Entity;

mod helper;
mod http;

/*
 * Info on async functions within struct implementations:
 * https://github.com/rustwasm/wasm-bindgen/issues/1858
 *
 * https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm
 *
 * One time needed in ../app/:
 * npm install --save ..\wasm\pkg\
 */

#[wasm_bindgen(start)]
#[allow(dead_code)]
pub fn start() -> Result<(), JsValue> {
	Ok(())
}

/// Client for Upholi server.
/// For requests that require a user to be authenticated.
#[wasm_bindgen]
pub struct UpholiClient {
	/// The master private key of current session
	private_key: String,
}

#[wasm_bindgen]
impl UpholiClient {
	#[wasm_bindgen(constructor)]
	pub fn new(private_key: String) -> UpholiClient {
		UpholiClient { private_key }
	}

	#[wasm_bindgen(js_name = register)]
	pub fn register(&self, username: String, password: String) -> js_sys::Promise {
		future_to_promise(async move {
			match helper::CLIENT.read().unwrap().register(&username, &password).await {
				Ok(_) => Ok(JsValue::NULL),
				Err(error) => Err(format!("{}", error).into()),
			}
		})
	}

	#[wasm_bindgen(js_name = login)]
	pub fn login(&self, username: String, password: String) -> js_sys::Promise {
		future_to_promise(async move {
			match helper::CLIENT.read().unwrap().login(&username, &password).await {
				Ok(key) => match String::from_utf8(key) {
					Ok(key) => Ok(JsValue::from_str(&key)),
					Err(error) => Err(format!("{}", error).into()),
				},
				Err(error) => Err(format!("{}", error).into()),
			}
		})
	}

	#[wasm_bindgen(js_name = getUserInfo)]
	pub fn get_user_info(&self) -> js_sys::Promise {
		future_to_promise(async move {
			match helper::CLIENT.read().unwrap().get_user_info().await {
				Ok(user_info) => Ok(JsValue::from_serde(&user_info).unwrap_throw()),
				Err(error) => Err(format!("{}", error).into()),
			}
		})
	}

	/// Get all photos of current user.
	#[wasm_bindgen(js_name = getPhotos)]
	pub fn get_photos(&self) -> js_sys::Promise {
		future_to_promise(async move {
			match helper::CLIENT.read().unwrap().get_photos().await {
				Ok(photos) => {
					let mut js_array_photos: Vec<JsValue> = Vec::new();

					for photo in photos {
						let photo = JsValue::from_serde(&photo).unwrap_throw();
						js_array_photos.push(photo);
					}

					let js_array_photos = JsValue::from(js_array_photos.iter().collect::<Array>());
					Ok(js_array_photos)
				}
				Err(error) => Err(format!("{}", error).into()),
			}
		})
	}

	/// Get photo data
	#[wasm_bindgen(js_name = getPhoto)]
	pub fn get_photo(&self, id: String) -> js_sys::Promise {
		let private_key = self.private_key.as_bytes().to_owned();

		future_to_promise(async move {
			match helper::CLIENT.read().unwrap().get_photo(&private_key, &id, &None).await {
				Ok(photo) => Ok(JsValue::from_serde(photo.as_js_value()).unwrap_throw()),
				Err(error) => Err(format!("{}", error).into()),
			}
		})
	}

	/// Get photo data
	#[wasm_bindgen(js_name = getPhotoWithProof)]
	pub fn get_photo_with_proof(&self, id: String, key: String) -> js_sys::Promise {
		let private_key = self.private_key.as_bytes().to_owned();

		future_to_promise(async move {
			match helper::CLIENT.read().unwrap().get_photo(&private_key, &id, &Some(key)).await {
				Ok(photo) => Ok(JsValue::from_serde(photo.as_js_value()).unwrap_throw()),
				Err(error) => Err(format!("{}", error).into()),
			}
		})
	}

	/// Upload/Create a photo
	#[wasm_bindgen(js_name = uploadPhoto)]
	pub fn upload_photo(&self, bytes: Vec<u8>) -> js_sys::Promise {
		let private_key = self.private_key.as_bytes().to_owned();

		future_to_promise(async move {
			let upload_info = PhotoUploadInfo::try_from_slice(&bytes).unwrap_throw();
			match helper::CLIENT.read().unwrap().upload_photo(&private_key, &upload_info).await {
				Ok(id) => Ok(JsValue::from_str(&id)),
				Err(error) => Err(format!("{}", error).into()),
			}
		})
	}

	/// Get a base64 string of a photo's thumbnail image
	#[wasm_bindgen(js_name = getPhotoThumbnailImageSrc)]
	pub fn get_photo_thumbnail_image_src(&self, id: String) -> js_sys::Promise {
		Self::get_photo_image_src(self, id, PhotoVariant::Thumbnail, None)
	}

	/// Get a base64 string of a photo's preview image
	#[wasm_bindgen(js_name = getPhotoPreviewImageSrc)]
	pub fn get_photo_preview_image_src(&self, id: String) -> js_sys::Promise {
		Self::get_photo_image_src(self, id, PhotoVariant::Preview, None)
	}

	/// Get a base64 string of photo's original file
	#[wasm_bindgen(js_name = getPhotoOriginalImageSrc)]
	pub fn get_photo_original_image_src(&self, id: String) -> js_sys::Promise {
		Self::get_photo_image_src(self, id, PhotoVariant::Original, None)
	}

	/// Get a base64 string of a photo's thumbnail image
	#[wasm_bindgen(js_name = getPhotoThumbnailImageSrcWithProof)]
	pub fn get_photo_thumbnail_image_src_with_proof(&self, id: String, key: String) -> js_sys::Promise {
		Self::get_photo_image_src(self, id, PhotoVariant::Thumbnail, Some(key))
	}

	/// Get a base64 string of a photo's preview image
	#[wasm_bindgen(js_name = getPhotoPreviewImageSrcWithProof)]
	pub fn get_photo_preview_image_src_with_proof(&self, id: String, key: String) -> js_sys::Promise {
		Self::get_photo_image_src(self, id, PhotoVariant::Preview, Some(key))
	}

	/// Get a base64 string of photo's original file
	#[wasm_bindgen(js_name = getPhotoOriginalImageSrcWithProof)]
	pub fn get_photo_original_image_src_with_proof(&self, id: String, key: String) -> js_sys::Promise {
		Self::get_photo_image_src(self, id, PhotoVariant::Original, Some(key))
	}

	/// Get a string of a photo variant that can be used within an HTML image element's src attribute
	fn get_photo_image_src(&self, id: String, photo_variant: PhotoVariant, key: Option<String>) -> js_sys::Promise {
		let private_key = self.private_key.as_bytes().to_owned();

		future_to_promise(async move {
			match helper::CLIENT
				.read()
				.unwrap()
				.get_photo_image_src(&private_key, &id, photo_variant, &key)
				.await
			{
				Ok(base64) => Ok(JsValue::from(base64)),
				Err(error) => Err(format!("{}", error).into()),
			}
		})
	}

	/// Permanently delete a photo
	#[wasm_bindgen(js_name = deletePhotos)]
	pub fn delete_photos(&self, photo_ids: Box<[JsString]>) -> js_sys::Promise {
		let private_key = self.private_key.as_bytes().to_owned();

		future_to_promise(async move {
			let photo_ids: Vec<String> = photo_ids.iter().map(|id| id.into()).collect();
			match helper::CLIENT.read().unwrap().delete_photos(&private_key, &photo_ids).await {
				Ok(_) => Ok(JsValue::UNDEFINED),
				Err(error) => Err(format!("{}", error).into()),
			}
		})
	}

	#[wasm_bindgen(js_name = getAlbums)]
	pub fn get_albums(&mut self) -> js_sys::Promise {
		let private_key = self.private_key.as_bytes().to_owned();

		future_to_promise(async move {
			match helper::CLIENT.read().unwrap().get_albums(&private_key).await {
				Ok(albums) => {
					let mut js_array: Vec<JsValue> = Vec::new();

					for album in albums {
						let album = JsValue::from_serde(album.as_js_value()).unwrap_throw();
						js_array.push(album);
					}

					let js_array = JsValue::from(js_array.iter().collect::<Array>());
					Ok(js_array)
				}
				Err(error) => Err(format!("{}", error).into()),
			}
		})
	}

	#[wasm_bindgen(js_name = getAlbum)]
	pub fn get_album(&mut self, id: String) -> js_sys::Promise {
		let private_key = self.private_key.as_bytes().to_owned();

		future_to_promise(async move {
			match helper::CLIENT.read().unwrap().get_album_full(&private_key, &id).await {
				Ok(album) => Ok(JsValue::from_serde(&album).unwrap_throw()),
				Err(error) => Err(format!("{}", error).into()),
			}
		})
	}

	#[wasm_bindgen(js_name = createAlbum)]
	pub fn create_album(&mut self, title: String, initial_photo_ids: Box<[JsString]>) -> js_sys::Promise {
		let private_key = self.private_key.as_bytes().to_owned();

		future_to_promise(async move {
			let initial_photo_ids = initial_photo_ids.iter().map(|id| id.into()).collect();
			match helper::CLIENT
				.read()
				.unwrap()
				.create_album(&private_key, &title, initial_photo_ids)
				.await
			{
				Ok(id) => Ok(JsValue::from(id)),
				Err(error) => Err(format!("{}", error).into()),
			}
		})
	}

	#[wasm_bindgen(js_name = deleteAlbum)]
	pub fn delete_album(&mut self, id: String) -> js_sys::Promise {
		future_to_promise(async move {
			match helper::CLIENT.read().unwrap().delete_album(&id).await {
				Ok(_) => Ok(JsValue::NULL),
				Err(error) => Err(format!("{}", error).into()),
			}
		})
	}

	#[wasm_bindgen(js_name = updateAlbumTitleTags)]
	pub fn update_album_title_tags(&mut self, id: String, title: String, tags: Box<[JsString]>) -> js_sys::Promise {
		let private_key = self.private_key.as_bytes().to_owned();

		future_to_promise(async move {
			let tags = tags.iter().map(|tag| tag.into()).collect();
			match helper::CLIENT
				.read()
				.unwrap()
				.update_album_title_tags(&private_key, &id, &title, tags)
				.await
			{
				Ok(_) => Ok(JsValue::NULL),
				Err(error) => Err(format!("{}", error).into()),
			}
		})
	}

	#[wasm_bindgen(js_name = updateAlbumCover)]
	pub fn update_album_cover(&mut self, id: String, cover_photo_id: String) -> js_sys::Promise {
		let private_key = self.private_key.as_bytes().to_owned();

		future_to_promise(async move {
			match helper::CLIENT
				.read()
				.unwrap()
				.update_album_cover(&private_key, &id, &cover_photo_id)
				.await
			{
				Ok(_) => Ok(JsValue::NULL),
				Err(error) => Err(format!("{}", error).into()),
			}
		})
	}

	#[wasm_bindgen(js_name = addPhotosToAlbum)]
	pub fn add_photos_to_album(&mut self, id: String, photos: Box<[JsString]>) -> js_sys::Promise {
		let private_key = self.private_key.as_bytes().to_owned();

		future_to_promise(async move {
			let photo_ids: Vec<String> = photos.iter().map(|photo| photo.into()).collect();
			match helper::CLIENT
				.read()
				.unwrap()
				.add_photos_to_album(&private_key, &id, &photo_ids)
				.await
			{
				Ok(_) => Ok(JsValue::NULL),
				Err(error) => Err(format!("{}", error).into()),
			}
		})
	}

	#[wasm_bindgen(js_name = removePhotosFromAlbum)]
	pub fn remove_photos_from_album(&mut self, id: String, photos: Box<[JsString]>) -> js_sys::Promise {
		let private_key = self.private_key.as_bytes().to_owned();

		future_to_promise(async move {
			let photo_ids: Vec<String> = photos.iter().map(|photo| photo.into()).collect();
			match helper::CLIENT
				.read()
				.unwrap()
				.remove_photos_from_album(&private_key, &id, &photo_ids)
				.await
			{
				Ok(_) => Ok(JsValue::NULL),
				Err(error) => Err(format!("{}", error).into()),
			}
		})
	}

	/// Creates or updates a share for an album
	#[wasm_bindgen(js_name = upsertAlbumShare)]
	pub fn upsert_album_share(&self, album_id: String, password: String) -> js_sys::Promise {
		let private_key = self.private_key.as_bytes().to_owned();

		future_to_promise(async move {
			match helper::CLIENT
				.read()
				.unwrap()
				.upsert_share(&private_key, ShareType::Album, &album_id, &password)
				.await
			{
				Ok(share_id) => Ok(JsValue::from_str(&share_id)),
				Err(error) => Err(format!("{}", error).into()),
			}
		})
	}

	/// Creates or updates a share for an album
	#[wasm_bindgen(js_name = getShares)]
	pub fn get_shares(&self) -> js_sys::Promise {
		let private_key = self.private_key.as_bytes().to_owned();

		future_to_promise(async move {
			let shares = helper::CLIENT.read().unwrap().get_shares(&private_key, None).await.unwrap_throw();
			let mut js_array: Vec<JsValue> = Vec::new();

			for share in shares {
				let share = JsValue::from_serde(share.as_js_value()).unwrap_throw();
				js_array.push(share);
			}

			Ok(JsValue::from(js_array.iter().collect::<Array>()))
		})
	}

	/// Creates or updates a share for an album
	#[wasm_bindgen(js_name = getShare)]
	pub fn get_share(&self, share_id: String) -> js_sys::Promise {
		let private_key = self.private_key.as_bytes().to_owned();

		future_to_promise(async move {
			let share = helper::CLIENT
				.read()
				.unwrap()
				.get_share(&share_id, &private_key)
				.await
				.unwrap_throw();
			Ok(JsValue::from_serde(share.as_js_value()).unwrap_throw())
		})
	}

	/// Find an album share
	#[wasm_bindgen(js_name = findAlbumShare)]
	pub fn find_album_share(&self, id: String) -> js_sys::Promise {
		Self::find_share(self, ShareType::Album, id)
	}

	/// Find a share of given type
	fn find_share(&self, type_: ShareType, id: String) -> js_sys::Promise {
		let private_key = self.private_key.as_bytes().to_owned();

		future_to_promise(async move {
			let share = helper::CLIENT
				.read()
				.unwrap()
				.find_share(&private_key, &type_, &id)
				.await
				.unwrap_throw();
			match share {
				Some(share) => Ok(JsValue::from_serde(share.as_js_value()).unwrap_throw()),
				None => Ok(JsValue::NULL),
			}
		})
	}

	/// Creates or updates a share for an album
	#[wasm_bindgen(js_name = getShareUsingPassword)]
	pub fn get_share_using_password(&self, share_id: String, password: String) -> js_sys::Promise {
		future_to_promise(async move {
			match helper::CLIENT.read().unwrap().get_share_using_password(&share_id, &password).await {
				Ok(share) => Ok(JsValue::from_serde(share.as_js_value()).unwrap_throw()),
				Err(error) => Err(format!("{}", error).into()),
			}
		})
	}

	/// Creates or updates a share for an album
	#[wasm_bindgen(js_name = getAlbumFromShare)]
	pub fn get_album_from_share(&self, share_id: String, password: String) -> js_sys::Promise {
		future_to_promise(async move {
			match helper::CLIENT.read().unwrap().get_album_from_share(&share_id, &password).await {
				Ok(album) => Ok(JsValue::from_serde(&album).unwrap_throw()),
				Err(error) => Err(format!("{}", error).into()),
			}
		})
	}

	/// Deletes a share for an album
	#[wasm_bindgen(js_name = deleteShare)]
	pub fn delete_share(&self, id: String) -> js_sys::Promise {
		future_to_promise(async move {
			// TODO: id -> album_id
			match helper::CLIENT.read().unwrap().delete_share(&id).await {
				Ok(_) => Ok(JsValue::NULL),
				Err(error) => Err(format!("{}", error).into()),
			}
		})
	}
}
