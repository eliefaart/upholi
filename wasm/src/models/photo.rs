use crate::exif::Exif;
use serde::{Deserialize, Serialize};

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
