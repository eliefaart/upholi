use crate::{api_client::ApiClient, wasm_client::WasmClient};
use js_sys::{Array, JsString, Promise};
use once_cell::sync::Lazy;
use serde::Serialize;
use upholi_lib::PhotoVariant;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

/*
 * Info on async functions within struct implementations:
 * https://github.com/rustwasm/wasm-bindgen/issues/1858
 *
 * https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm
 *
 * One time needed in ../app/:
 * npm install --save ..\wasm\pkg\
 */

mod api_client;
mod encryption;
mod exif;
mod hashing;
mod images;
mod keys;
mod models;
mod multipart;
mod repository;
mod wasm_client;

static ORIGIN: Lazy<String> = Lazy::new(|| {
	let window = web_sys::window().expect_throw("Could not find global 'window'.");
	let location = window.location();
	location.origin().expect_throw("could not determine 'origin'.")
});
static WASM_CLIENT: Lazy<WasmClient> = Lazy::new(|| WasmClient::new(&ORIGIN));
static API_CLIENT: Lazy<ApiClient> = Lazy::new(|| ApiClient::new(&ORIGIN));

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(js_namespace = console)]
	fn log(s: &str);

	#[wasm_bindgen(js_namespace = console)]
	fn error(s: &str);
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
	Ok(())
}

/// Convert given Vec of item to a JsValue that represents a JavaScript array.
fn to_js_array<T: Serialize>(items: Vec<T>) -> JsValue {
	let js_array = items
		.into_iter()
		.map(|photo| serde_wasm_bindgen::to_value(&photo).unwrap_throw())
		.collect::<Array>();
	JsValue::from(js_array)
}

/// The JavaScript interface that frontend uses to call functions in the wasm.
/// This struct wraps WasmClient which contains the actual implementation.
/// By proxying the calls to WasmClient, only this struct needs to deal with JS types.
#[wasm_bindgen]
pub struct UpholiClient {}

#[wasm_bindgen]
impl UpholiClient {
	#[wasm_bindgen(constructor)]
	#[allow(clippy::new_without_default)]
	pub fn new() -> UpholiClient {
		UpholiClient {}
	}

	#[wasm_bindgen(js_name = register)]
	pub fn register(&self, username: String, password: String) -> Promise {
		future_to_promise(async move {
			WASM_CLIENT.register(&username, &password).await.unwrap_throw();
			Ok(JsValue::UNDEFINED)
		})
	}

	#[wasm_bindgen(js_name = login)]
	pub fn login(&self, username: String, password: String) -> Promise {
		future_to_promise(async move {
			WASM_CLIENT.login(&username, &password).await.unwrap_throw();
			Ok(JsValue::UNDEFINED)
		})
	}

	/// Get all photos of current user.
	#[wasm_bindgen(js_name = getPhotos)]
	pub fn get_photos(&self) -> Promise {
		future_to_promise(async move {
			let photos = WASM_CLIENT.get_library_photos().await.unwrap_throw();
			Ok(to_js_array(photos))
		})
	}

	/// Get photo data
	#[wasm_bindgen(js_name = getPhoto)]
	pub fn get_photo(&self, id: String) -> Promise {
		future_to_promise(async move {
			let photo = WASM_CLIENT.get_photo(&id, None).await.unwrap_throw();
			Ok(serde_wasm_bindgen::to_value(&photo).unwrap_throw())
		})
	}

	/// Get photo data
	#[wasm_bindgen(js_name = getPhotoWithProof)]
	pub fn get_photo_with_proof(&self, id: String, key: String) -> Promise {
		future_to_promise(async move {
			let key = base64::decode_config(key, base64::STANDARD).unwrap_throw();
			let photo = WASM_CLIENT.get_photo(&id, Some(key)).await.unwrap_throw();
			Ok(serde_wasm_bindgen::to_value(&photo).unwrap_throw())
		})
	}

	/// Upload/Create a photo
	#[wasm_bindgen(js_name = uploadPhoto)]
	pub fn upload_photo(&self, bytes: Vec<u8>) -> Promise {
		future_to_promise(async move {
			let result = WASM_CLIENT.upload_photo(&bytes).await.unwrap_throw();
			Ok(serde_wasm_bindgen::to_value(&result).unwrap_throw())
		})
	}

	/// Get a base64 string of a photo's thumbnail image
	#[wasm_bindgen(js_name = getPhotoThumbnailImageSrc)]
	pub fn get_photo_thumbnail_image_src(&self, id: String) -> Promise {
		Self::get_photo_image_src(self, id, PhotoVariant::Thumbnail, None)
	}

	/// Get a base64 string of a photo's preview image
	#[wasm_bindgen(js_name = getPhotoPreviewImageSrc)]
	pub fn get_photo_preview_image_src(&self, id: String) -> Promise {
		Self::get_photo_image_src(self, id, PhotoVariant::Preview, None)
	}

	/// Get a base64 string of photo's original file
	#[wasm_bindgen(js_name = getPhotoOriginalImageSrc)]
	pub fn get_photo_original_image_src(&self, id: String) -> Promise {
		Self::get_photo_image_src(self, id, PhotoVariant::Original, None)
	}

	/// Get a base64 string of a photo's thumbnail image
	#[wasm_bindgen(js_name = getPhotoThumbnailImageSrcWithProof)]
	pub fn get_photo_thumbnail_image_src_with_proof(&self, id: String, key: String) -> Promise {
		Self::get_photo_image_src(self, id, PhotoVariant::Thumbnail, Some(key))
	}

	/// Get a base64 string of a photo's preview image
	#[wasm_bindgen(js_name = getPhotoPreviewImageSrcWithProof)]
	pub fn get_photo_preview_image_src_with_proof(&self, id: String, key: String) -> Promise {
		Self::get_photo_image_src(self, id, PhotoVariant::Preview, Some(key))
	}

	/// Get a base64 string of photo's original file
	#[wasm_bindgen(js_name = getPhotoOriginalImageSrcWithProof)]
	pub fn get_photo_original_image_src_with_proof(&self, id: String, key: String) -> Promise {
		Self::get_photo_image_src(self, id, PhotoVariant::Original, Some(key))
	}

	/// Get a string of a photo variant that can be used within an HTML image element's src attribute
	fn get_photo_image_src(&self, id: String, photo_variant: PhotoVariant, key: Option<String>) -> Promise {
		future_to_promise(async move {
			let key = key.map(|key_str| base64::decode_config(&key_str, base64::STANDARD).unwrap_throw());
			let base64 = WASM_CLIENT.get_photo_image_src(&id, photo_variant, key).await.unwrap_throw();
			Ok(JsValue::from(base64))
		})
	}

	/// Permanently delete a photo
	#[wasm_bindgen(js_name = deletePhotos)]
	pub fn delete_photos(&self, photo_ids: Box<[JsString]>) -> Promise {
		future_to_promise(async move {
			let photo_ids = photo_ids.iter().map(|id| id.into()).collect::<Vec<String>>();
			WASM_CLIENT.delete_photos(&photo_ids).await.unwrap_throw();
			Ok(JsValue::UNDEFINED)
		})
	}

	#[wasm_bindgen(js_name = getAlbums)]
	pub fn get_albums(&mut self) -> Promise {
		future_to_promise(async move {
			let albums = WASM_CLIENT.get_albums().await.unwrap_throw();
			Ok(to_js_array(albums))
		})
	}

	#[wasm_bindgen(js_name = getAlbum)]
	pub fn get_album(&mut self, id: String) -> Promise {
		future_to_promise(async move {
			let album = WASM_CLIENT.get_album_full(&id).await.unwrap_throw();
			Ok(serde_wasm_bindgen::to_value(&album).unwrap_throw())
		})
	}

	#[wasm_bindgen(js_name = createAlbum)]
	pub fn create_album(&mut self, title: String, initial_photo_ids: Box<[JsString]>) -> Promise {
		future_to_promise(async move {
			let initial_photo_ids = initial_photo_ids.iter().map(|id| id.into()).collect();
			let id = WASM_CLIENT.create_album(&title, initial_photo_ids).await.unwrap_throw();
			Ok(JsValue::from(id))
		})
	}

	#[wasm_bindgen(js_name = deleteAlbum)]
	pub fn delete_album(&mut self, id: String) -> Promise {
		future_to_promise(async move {
			WASM_CLIENT.delete_album(&id).await.unwrap_throw();
			Ok(JsValue::UNDEFINED)
		})
	}

	#[wasm_bindgen(js_name = updateAlbumTitleTags)]
	pub fn update_album_title_tags(&mut self, id: String, title: String, tags: Box<[JsString]>) -> Promise {
		future_to_promise(async move {
			let tags = tags.iter().map(|tag| tag.into()).collect();
			WASM_CLIENT.update_album_title_tags(&id, &title, tags).await.unwrap_throw();
			Ok(JsValue::UNDEFINED)
		})
	}

	#[wasm_bindgen(js_name = updateAlbumCover)]
	pub fn update_album_cover(&mut self, id: String, cover_photo_id: String) -> Promise {
		future_to_promise(async move {
			WASM_CLIENT.update_album_cover(&id, &cover_photo_id).await.unwrap_throw();
			Ok(JsValue::UNDEFINED)
		})
	}

	#[wasm_bindgen(js_name = addPhotosToAlbum)]
	pub fn add_photos_to_album(&mut self, id: String, photos: Box<[JsString]>) -> Promise {
		future_to_promise(async move {
			let photo_ids = photos.iter().map(|photo| photo.into()).collect::<Vec<String>>();
			WASM_CLIENT.add_photos_to_album(&id, &photo_ids).await.unwrap_throw();
			Ok(JsValue::UNDEFINED)
		})
	}

	#[wasm_bindgen(js_name = removePhotosFromAlbum)]
	pub fn remove_photos_from_album(&mut self, id: String, photos: Box<[JsString]>) -> Promise {
		future_to_promise(async move {
			let photo_ids = photos.iter().map(|photo| photo.into()).collect::<Vec<String>>();
			WASM_CLIENT.remove_photos_from_album(&id, &photo_ids).await.unwrap_throw();
			Ok(JsValue::UNDEFINED)
		})
	}

	/// Creates or updates a share for an album
	#[wasm_bindgen(js_name = upsertAlbumShare)]
	pub fn upsert_album_share(&self, album_id: String, password: String) -> Promise {
		future_to_promise(async move {
			let share_id = WASM_CLIENT.upsert_share(&album_id, &password).await.unwrap_throw();
			Ok(JsValue::from_str(&share_id))
		})
	}

	/// Creates or updates a share for an album
	#[wasm_bindgen(js_name = getShares)]
	pub fn get_shares(&self) -> Promise {
		future_to_promise(async move {
			let shares = WASM_CLIENT.get_shares().await.unwrap_throw();
			Ok(to_js_array(shares))
		})
	}

	/// Find an album share
	#[wasm_bindgen(js_name = getAlbumShare)]
	pub fn get_album_share(&self, id: String) -> Promise {
		future_to_promise(async move {
			let share = WASM_CLIENT.get_album_share(&id).await.unwrap_throw();
			Ok(serde_wasm_bindgen::to_value(&share).unwrap_throw())
		})
	}

	#[wasm_bindgen(js_name = isAuthorizedForShare)]
	pub fn is_authorized_for_share(&self, share_id: String) -> Promise {
		future_to_promise(async move {
			let authorized = WASM_CLIENT.is_authorized_for_share(&share_id).await.unwrap_throw();
			Ok(JsValue::from_bool(authorized))
		})
	}

	#[wasm_bindgen(js_name = authorizeShare)]
	pub fn authorize_share(&self, share_id: String, password: String) -> Promise {
		future_to_promise(async move {
			let authorized = WASM_CLIENT.authorize_share(&share_id, &password).await.unwrap_throw();
			Ok(JsValue::from_bool(authorized))
		})
	}

	/// Get the album for given share
	#[wasm_bindgen(js_name = getShareAlbum)]
	pub fn get_share_album(&self, share_id: String) -> Promise {
		future_to_promise(async move {
			let album = WASM_CLIENT.get_share_album(&share_id).await.unwrap_throw();
			Ok(serde_wasm_bindgen::to_value(&album).unwrap_throw())
		})
	}

	/// Deletes a share for an album
	#[wasm_bindgen(js_name = deleteShare)]
	pub fn delete_share(&self, id: String) -> Promise {
		future_to_promise(async move {
			WASM_CLIENT.delete_share(&id).await.unwrap_throw();
			Ok(JsValue::UNDEFINED)
		})
	}
}
