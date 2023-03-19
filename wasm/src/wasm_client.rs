use crate::api_client::{ApiClient, File};
use crate::encryption::symmetric::{decrypt_slice, derive_key_from_string, generate_key};
use crate::exif::Exif;
use crate::images::Image;
use crate::keys::{get_key_from_user_credentials, get_master_key, get_share_key, set_master_key, set_share_key};
use crate::models::Photo;
use crate::models::{
    Album, AlbumHydrated, AlbumPhoto, AlbumShareData, AlbumShareDataPhoto, Library, LibraryAlbum, LibraryPhoto, LibraryShare, Share,
    ShareData,
};
use crate::repository;
use crate::repository::ItemVariant;
use crate::{encryption, hashing};
use anyhow::{anyhow, Result};
use serde::Serialize;
use upholi_lib::http::request::{CreateUserRequest, UpsertShareRequest};
use upholi_lib::ids::id;
use upholi_lib::PhotoVariant;
use wasm_bindgen::UnwrapThrowExt;

pub const KEY_MASTER_KEY: &str = "master-key";
pub const KEY_LIBRARY: &str = "library";

/// Wrapper struct containing info about bytes to upload.
pub struct PhotoUploadInfo {
    pub image: Image,
    pub exif: Option<Exif>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PhotoUploadResult {
    skipped: bool,
    photo_id: String,
}

impl PhotoUploadInfo {
    /// Try to construct an object from image file bytes
    pub fn try_from_slice(bytes: &[u8]) -> Result<Self> {
        let exif = Exif::parse_from_photo_bytes(bytes)?;
        let exif_orientation = match &exif {
            Some(exif) => exif.orientation.unwrap_or(1),
            None => 1,
        };

        let image = Image::from_buffer(bytes, exif_orientation as u8)?;
        Ok(Self { image, exif })
    }
}

/// Helper functions for UpholiClient, which essentially wraps this object.
/// This object itself is not exposed outside the wasm.
pub struct WasmClient<'a> {
    api_client: &'a ApiClient,
}

impl<'a> WasmClient<'a> {
    pub fn new(api_client: &'a ApiClient) -> Self {
        Self { api_client }
    }

    pub async fn register(&self, username: &str, password: &str) -> Result<()> {
        let password_derived_key = get_key_from_user_credentials(username, password)?;

        // This will be the master encryption key of the user.
        // We encrypt it using the key derived from the user's password,
        // and the encrypted master key is stored server-side.
        let master_key = encryption::symmetric::generate_key();

        let body = CreateUserRequest {
            username: username.into(),
            password: password.into(),
        };

        self.api_client.register(&body).await?;
        set_master_key(&master_key);
        repository::set(KEY_MASTER_KEY, &password_derived_key, ItemVariant::MasterKey(master_key.clone())).await?;
        repository::set(KEY_LIBRARY, &master_key, ItemVariant::Library(Library::default())).await?;

        Ok(())
    }

    /// Returns the user's master encryption key when login was succesful
    pub async fn login(&self, username: &str, password: &str) -> Result<()> {
        self.api_client.login(username, password).await?;

        let password_derived_key = get_key_from_user_credentials(username, password)?;
        let master_key = repository::get(KEY_MASTER_KEY, &password_derived_key)
            .await?
            .ok_or_else(|| anyhow!("Master key missing"))?
            .try_into()?;

        set_master_key(&master_key);
        Ok(())
    }

    pub async fn get_library_photos(&self) -> Result<Vec<LibraryPhoto>> {
        let library = self.get_library().await?;
        Ok(library.photos.into_iter().rev().collect())
    }

    pub async fn get_photo(&self, id: &str) -> Result<Photo> {
        let photo_encryption_key = self.determine_photo_key(id).await?;
        let photo_item = repository::get(id, &photo_encryption_key).await?;
        let photo = photo_item.ok_or_else(|| anyhow!("Photo '{id}' not found"))?.try_into()?;

        Ok(photo)
    }

    pub async fn get_albums(&self) -> Result<Vec<Album>> {
        let library = self.get_library().await?;
        let album_ids = library.albums.into_iter().map(|album| album.id);

        let mut albums = vec![];
        for album_id in album_ids {
            let album = self.get_album(&album_id).await?;
            if let Some(album) = album {
                albums.push(album);
            }
        }

        Ok(albums)
    }

    async fn get_album(&self, id: &str) -> Result<Option<Album>> {
        let library = self.get_library().await?;
        let encryption_key = self.get_item_encryption_key(&library, id)?;
        self.get_album_using_key(id, encryption_key).await
    }

    async fn get_album_using_key(&self, id: &str, album_encryption_key: &[u8]) -> Result<Option<Album>> {
        let item = repository::get(id, album_encryption_key).await?;
        match item {
            Some(item) => Ok(Some(item.try_into()?)),
            None => Ok(None),
        }
    }

    pub async fn create_album(&self, title: &str, initial_photo_ids: Vec<String>) -> Result<String> {
        let album_id = id();
        let album_key = generate_key();

        let album = Album {
            id: album_id.clone(),
            key: album_key.clone(),
            title: title.into(),
            thumbnail_photo_id: initial_photo_ids.first().map(|s| s.to_owned()),
            tags: vec![],
            photos: initial_photo_ids,
        };

        self.update_library(&mut |library: &mut Library| {
            library.albums.push(LibraryAlbum {
                id: album_id.clone(),
                key: album_key.clone(),
            });
            Ok(())
        })
        .await?;

        repository::set(&album_id, &album_key, album.into()).await?;

        Ok(album_id)
    }

    pub async fn get_album_full(&self, id: &str) -> Result<AlbumHydrated> {
        let album = self.get_album(id).await?.ok_or_else(|| anyhow!("Album '{id}' not found"))?;
        let photos = self.get_library_photos().await?;
        let album_photos = photos.into_iter().map(|p| p.into()).collect();
        self.inflate_album(album, album_photos)
    }

    pub async fn delete_album(&self, id: &str) -> Result<()> {
        let shares = self.get_shares().await?;
        let shares_for_album = shares.iter().filter(|share| share.album_id == id);
        for share in shares_for_album {
            self.delete_share(&share.id).await?;
        }

        repository::delete(id).await?;

        self.update_library(&mut |library: &mut Library| {
            library.albums.retain(|ik| ik.id != id);
            Ok(())
        })
        .await?;

        Ok(())
    }

    fn inflate_album(&self, album: Album, photos: Vec<AlbumPhoto>) -> Result<AlbumHydrated> {
        let mut photos_in_album: Vec<AlbumPhoto> = vec![];
        for photo in &photos {
            if album.photos.contains(&photo.id) {
                photos_in_album.push(photo.clone());
            }
        }

        let album = AlbumHydrated {
            id: album.id.clone(),
            title: album.title.clone(),
            tags: album.tags.clone(),
            photos: photos_in_album,
            thumbnail_photo: match album.thumbnail_photo_id.clone() {
                Some(thumbnail_photo_id) => {
                    let photo = photos
                        .into_iter()
                        .find(|photo| photo.id == thumbnail_photo_id)
                        .ok_or_else(|| anyhow!("Photo not found for thumbnail of album {}", &album.id))?;
                    Some(photo)
                }
                None => None,
            },
        };

        Ok(album)
    }

    pub async fn upload_photo(&self, bytes: &[u8]) -> Result<PhotoUploadResult> {
        let photo_hash = hashing::compute_sha256_hash(bytes)?;
        let library = self.get_library().await?;
        let existing_photo = library.photos.iter().find(|photo| photo.hash == photo_hash);
        if let Some(existing_photo) = existing_photo {
            // No error, but no need to upload.
            Ok(PhotoUploadResult {
                skipped: true,
                photo_id: existing_photo.id.clone(),
            })
        } else {
            let upload_info = PhotoUploadInfo::try_from_slice(bytes).unwrap_throw();
            let photo_key = generate_key();
            let photo_id = id();

            // Compute the timestamp to store for this photo
            let now = chrono::Utc::now().timestamp();
            let timestamp = if let Some(exif) = &upload_info.exif {
                exif.date_taken.map_or(now, |dt| dt.timestamp())
            } else {
                now
            };

            let thumbnail_encrypted = crate::encryption::symmetric::encrypt_slice(&photo_key, &upload_info.image.bytes_thumbnail)?;
            let preview_encrypted = crate::encryption::symmetric::encrypt_slice(&photo_key, &upload_info.image.bytes_preview)?;
            let original_encrypted = crate::encryption::symmetric::encrypt_slice(&photo_key, &upload_info.image.bytes_original)?;

            let photo = &Photo {
                id: photo_id.clone(),
                hash: photo_hash,
                width: upload_info.image.width,
                height: upload_info.image.height,
                timestamp,
                content_type: "image/jpeg".to_string(), // TODO
                exif: upload_info.exif.clone(),
                nonce_thumbnail: thumbnail_encrypted.nonce,
                nonce_preview: preview_encrypted.nonce,
                nonce_original: original_encrypted.nonce,
            };

            let files: Vec<File> = vec![
                File {
                    id: format!("{photo_id}-thumbnail"),
                    bytes: thumbnail_encrypted.bytes,
                },
                File {
                    id: format!("{photo_id}-preview"),
                    bytes: preview_encrypted.bytes,
                },
                File {
                    id: format!("{photo_id}-original"),
                    bytes: original_encrypted.bytes,
                },
            ];
            self.api_client.set_files(&files).await?;
            repository::set(&photo_id, &photo_key, ItemVariant::Photo(photo.to_owned())).await?;

            self.update_library(&mut |library: &mut Library| {
                library.photos.push(LibraryPhoto::from(photo, photo_key.to_vec()));
                Ok(())
            })
            .await?;

            Ok(PhotoUploadResult { skipped: false, photo_id })
        }
    }

    pub async fn get_photo_image_src(&self, photo_id: &str, photo_variant: PhotoVariant) -> Result<String> {
        if photo_id.is_empty() {
            Ok(String::new())
        } else {
            let encryption_key = self.determine_photo_key(photo_id).await?;
            let photo = self.get_photo(photo_id).await?;
            let nonce = match photo_variant {
                PhotoVariant::Thumbnail => photo.nonce_thumbnail,
                PhotoVariant::Preview => photo.nonce_preview,
                PhotoVariant::Original => photo.nonce_original,
            };

            let file_id = format!("{photo_id}-{photo_variant}");
            let encrypted_bytes = self
                .api_client
                .get_file(&file_id)
                .await?
                .ok_or_else(|| anyhow!("File '{file_id}' not found"))?;
            let photo_bytes = decrypt_slice(&encryption_key, nonce.as_bytes(), &encrypted_bytes)?;
            let photo_base64 = base64::encode_config(photo_bytes, base64::STANDARD);

            let src = format!("data:{};base64,{}", photo.content_type, photo_base64);
            Ok(src)
        }
    }

    pub async fn delete_photos(&self, ids: &[String]) -> Result<()> {
        let library = self.get_library().await?;
        let albums = self.get_albums().await?;

        for mut album in albums {
            album.photos.retain(|photo_id| !ids.contains(photo_id));

            if let Some(id) = &album.thumbnail_photo_id {
                if ids.contains(id) {
                    album.thumbnail_photo_id = None;
                }
            }

            let album_key = self.get_item_encryption_key(&library, &album.id)?;
            repository::set(&album.id.clone(), album_key, album.into()).await?;
        }

        self.update_library(&mut |library: &mut Library| {
            library.photos.retain(|photo| !ids.contains(&photo.id));
            Ok(())
        })
        .await?;

        let file_ids = ids
            .iter()
            .flat_map(|id| {
                vec![
                    format!("{id}-{}", PhotoVariant::Thumbnail),
                    format!("{id}-{}", PhotoVariant::Preview),
                    format!("{id}-{}", PhotoVariant::Original),
                ]
            })
            .collect();

        repository::delete_many(ids).await?;
        self.api_client.delete_files(file_ids).await?;

        Ok(())
    }

    pub async fn update_album_title_tags(&self, id: &str, title: &str, tags: Vec<String>) -> Result<()> {
        self.update_album(id, &mut |album: &mut Album| {
            album.title = title.to_string();
            album.tags = tags.clone();
        })
        .await?;
        Ok(())
    }

    pub async fn update_album_cover(&self, id: &str, thumbnail_photo_id: &str) -> Result<()> {
        self.update_album(id, &mut |album: &mut Album| {
            album.thumbnail_photo_id = Some(thumbnail_photo_id.into());
        })
        .await?;
        Ok(())
    }

    pub async fn add_photos_to_album(&self, id: &str, photo_ids: &[String]) -> Result<()> {
        self.update_album(id, &mut |album: &mut Album| {
            for id in photo_ids {
                if !album.photos.contains(id) {
                    album.photos.push(id.to_owned());
                }
            }
        })
        .await?;
        Ok(())
    }

    /// Remove given photo IDs from album.
    /// Unsets the album's thumbnail if the current thumbnail is one of the photos to remove from album.
    pub async fn remove_photos_from_album(&self, id: &str, photos: &[String]) -> Result<()> {
        self.update_album(id, &mut |album: &mut Album| {
            album.photos.retain(|id| !photos.contains(id));

            if let Some(thumb_photo_id) = &album.thumbnail_photo_id {
                if photos.contains(thumb_photo_id) {
                    album.thumbnail_photo_id = None;
                }
            }
        })
        .await?;
        Ok(())
    }

    /// Creates or updates a share.
    ///
    /// * `item_id` - ID of the item (e.g. an album) to create a share for.
    pub async fn upsert_share(&self, item_id: &str, password: &str) -> anyhow::Result<String> {
        let library = self.get_library().await?;
        let existing_share = library.shares.iter().find(|s| s.album_id == item_id);

        let album = self.get_album(item_id).await?.ok_or_else(|| anyhow!("Album not found"))?;
        let share_id = match existing_share {
            Some(existing_share) => existing_share.id.clone(),
            None => id(),
        };
        let share_key = derive_key_from_string(password, &share_id)?;
        let share = Share {
            data: ShareData::Album(AlbumShareData {
                album_id: item_id.into(),
                album_key: album.key,
                photos: album
                    .photos
                    .iter()
                    .map(|photo_id| {
                        let photo = library
                            .photos
                            .iter()
                            .find(|p| &p.id == photo_id)
                            .ok_or_else(|| anyhow!("Photo with ID '{photo_id}' not found."))
                            .unwrap_throw();
                        let key = self.get_item_encryption_key(&library, photo_id).unwrap_throw();

                        AlbumShareDataPhoto {
                            id: photo_id.to_string(),
                            key: key.to_vec(),
                            width: photo.width,
                            height: photo.height,
                        }
                    })
                    .collect(),
            }),
        };

        let mut share_item_ids = vec![album.id.clone()];
        share_item_ids.extend(album.photos.iter().flat_map(|id| {
            vec![
                format!("{id}"),
                format!("{id}-{}", PhotoVariant::Thumbnail),
                format!("{id}-{}", PhotoVariant::Preview),
                format!("{id}-{}", PhotoVariant::Original),
            ]
        }));

        self.update_library(&mut |library: &mut Library| {
            library.shares.retain(|s| s.id != share_id);
            library.shares.push(LibraryShare {
                id: share_id.clone(),
                key: share_key.clone(),
                password: password.into(),
                album_id: album.id.clone(),
            });
            Ok(())
        })
        .await?;

        repository::set(&share_id, &share_key, ItemVariant::Share(share)).await?;

        self.api_client
            .upsert_share(UpsertShareRequest {
                id: share_id.clone(),
                password: password.into(),
                items: share_item_ids,
            })
            .await?;

        Ok(share_id)
    }

    /// Get a share by decrypting it using owner's key.
    pub async fn get_shares(&self) -> Result<Vec<LibraryShare>> {
        let library = self.get_library().await?;
        Ok(library.shares)
    }

    /// Get the share that belongs to given album_id, if any share exists for that album
    pub async fn get_share_for_album(&self, album_id: &str) -> Result<Option<LibraryShare>> {
        let library = self.get_library().await?;
        match library.shares.iter().find(|share| share.album_id == album_id) {
            Some(album) => Ok(Some(album.to_owned())),
            None => Ok(None),
        }
    }

    /// Get the album for given share_id.
    pub async fn get_share_album(&self, share_id: &str) -> Result<AlbumHydrated> {
        let share_key = &get_share_key(share_id)?.ok_or_else(|| anyhow!("No key found for share '{share_id}'."))?;
        let share: Share = repository::get(share_id, share_key)
            .await?
            .ok_or_else(|| anyhow!("Share '{share_id}' not found."))?
            .try_into()?;

        let ShareData::Album(album_data) = share.data;
        let album_id = album_data.album_id;
        let album = self
            .get_album_using_key(&album_id, &album_data.album_key)
            .await?
            .ok_or_else(|| anyhow!("Album '{album_id}' not found."))?;
        let album = self.inflate_album(album, album_data.photos.into_iter().map(|p| p.into()).collect())?;
        Ok(album)
    }

    pub async fn delete_share(&self, id: &str) -> Result<()> {
        self.api_client.delete_share(id).await?;

        repository::delete(id).await?;

        self.update_library(&mut |library: &mut Library| {
            library.shares.retain(|share| share.id != id);
            Ok(())
        })
        .await
    }

    pub async fn is_authorized_for_share(&self, id: &str) -> Result<bool> {
        let already_authorized = self.api_client.is_authorized_for_share(id).await?;

        if already_authorized {
            Ok(true)
        } else {
            // Not yet authorized, but check if share is publicly accessible without requiring a password:
            self.authorize_share(id, "").await
        }
    }

    pub async fn authorize_share(&self, id: &str, password: &str) -> Result<bool> {
        let authorized = self.api_client.authorize_share(id, password).await?;

        if authorized {
            let share_key = derive_key_from_string(password, id)?;
            set_share_key(id, &share_key)?
        }

        Ok(authorized)
    }

    /// Determine encryption key to use for given photo ID.
    ///
    /// * `photo_id` - ID of photo to determine encryption key for.
    async fn determine_photo_key(&self, photo_id: &str) -> Result<Vec<u8>> {
        // TODO: properly determine if we need to check library or shares,
        //  not it relies on an error.

        match &self.get_library().await {
            Ok(library) => Ok(self.get_item_encryption_key(library, photo_id)?.clone()),
            Err(_) => {
                let shares = repository::get_cached_shares()?;
                let photos: Vec<AlbumShareDataPhoto> = shares
                    .into_iter()
                    .map(|s| match s.data {
                        ShareData::Album(share) => share,
                    })
                    .flat_map(|f| f.photos)
                    .collect();
                let photo = photos
                    .iter()
                    .find(|photo| photo.id == photo_id)
                    .ok_or_else(|| anyhow!("Photo '{photo_id}' not available in any of the cached shares."))?;
                Ok(photo.key.clone())
            }
        }
    }

    async fn get_library(&self) -> Result<Library> {
        let master_key = get_master_key();
        let library = repository::get_or(KEY_LIBRARY, &master_key, &|| Library::default().into()).await?;

        library.try_into()
    }

    async fn update_library(&self, modify_library: &mut dyn FnMut(&mut Library) -> Result<()>) -> Result<()> {
        let mut library = self.get_library().await?;
        modify_library(&mut library)?;

        let master_key = get_master_key();
        repository::set(KEY_LIBRARY, &master_key, library.into()).await?;

        Ok(())
    }

    async fn update_album(&self, id: &str, modify_album: &mut dyn FnMut(&mut Album)) -> Result<()> {
        let library = self.get_library().await?;
        let mut album = self.get_album(id).await?.ok_or_else(|| anyhow!("Album not found"))?;

        modify_album(&mut album);

        let album_key = self.get_item_encryption_key(&library, &album.id)?;
        repository::set(id, album_key, album.into()).await?;

        // If a share exists for this album, then update it.
        let share_for_album = self.get_share_for_album(id).await?;
        if let Some(share) = share_for_album {
            // TODO: This can be optimized. This only needs to update the item representing the share,
            // not the share.holding the authentication info.
            self.upsert_share(id, &share.password).await?;
        }

        Ok(())
    }

    fn get_item_encryption_key<'s>(&'s self, library: &'s Library, item_id: &str) -> Result<&'s Vec<u8>> {
        library
            .find_encryption_key(item_id)
            .ok_or_else(|| anyhow!("No key found for item '{}'", item_id))
    }
}
