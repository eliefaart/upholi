use super::{AlbumShareDataPhoto, LibraryPhoto};
use serde::{Deserialize, Serialize};

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
}

impl From<AlbumShareDataPhoto> for AlbumPhoto {
    fn from(source: AlbumShareDataPhoto) -> Self {
        Self {
            id: source.id,
            width: source.width,
            height: source.height,
        }
    }
}

impl From<LibraryPhoto> for AlbumPhoto {
    fn from(source: LibraryPhoto) -> Self {
        Self {
            id: source.id,
            width: source.width,
            height: source.height,
        }
    }
}
