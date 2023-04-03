use super::Photo;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Library {
    pub photos: Vec<LibraryPhoto>,
    pub albums: Vec<LibraryAlbum>,
    pub shares: Vec<LibraryShare>,
}

impl Library {
    /// Find the encryption key for given item ID.
    pub fn find_encryption_key(&self, item_id: &str) -> Option<&Vec<u8>> {
        let find_as_album = || self.albums.iter().find(|i| i.id == item_id).map(|i| &i.key);
        let find_as_photo = || self.photos.iter().find(|i| i.id == item_id).map(|i| &i.key);
        let find_as_share = || self.shares.iter().find(|i| i.id == item_id).map(|i| &i.key);

        None.or_else(find_as_album).or_else(find_as_photo).or_else(find_as_share)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LibraryPhoto {
    pub id: String,
    pub key: Vec<u8>,
    pub hash: String,
    pub width: u32,
    pub height: u32,
}

impl LibraryPhoto {
    pub fn from(photo: &Photo, key: Vec<u8>) -> Self {
        Self {
            id: photo.id.clone(),
            key,
            hash: photo.hash.clone(),
            width: photo.width,
            height: photo.height,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LibraryAlbum {
    pub id: String,
    pub key: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LibraryShare {
    pub id: String,
    pub key: Vec<u8>,
    pub password: String,
    pub album_id: String,
}

#[cfg(test)]
mod tests {
    use super::{Library, LibraryAlbum, LibraryPhoto, LibraryShare};

    #[test]
    fn library_find_encryption_key() {
        let mut library = Library::default();

        let album_id = "album";
        let photo_id = "photo";
        let share_id = "share";
        let album_key = b"album".to_vec();
        let photo_key = b"photo".to_vec();
        let share_key = b"share".to_vec();

        library.albums.push(LibraryAlbum {
            id: album_id.into(),
            key: album_key.clone(),
        });
        library.photos.push(LibraryPhoto {
            id: photo_id.into(),
            key: photo_key.clone(),
            hash: String::new(),
            height: 0,
            width: 0,
        });
        library.shares.push(LibraryShare {
            id: share_id.into(),
            key: share_key.clone(),
            album_id: String::new(),
            password: String::new(),
        });

        assert_eq!(library.find_encryption_key(album_id).unwrap().to_owned(), album_key);
        assert_eq!(library.find_encryption_key(photo_id).unwrap().to_owned(), photo_key);
        assert_eq!(library.find_encryption_key(share_id).unwrap().to_owned(), share_key);
        assert_eq!(library.find_encryption_key("does-not-exist"), None);
    }
}
