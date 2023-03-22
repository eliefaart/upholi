use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Share {
    pub data: ShareData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ShareData {
    Album(AlbumShareData),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AlbumShareData {
    pub album_id: String,
    pub album_key: Vec<u8>,
    pub photos: Vec<AlbumShareDataPhoto>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AlbumShareDataPhoto {
    pub id: String,
    pub key: Vec<u8>,
    pub width: u32,
    pub height: u32,
}
