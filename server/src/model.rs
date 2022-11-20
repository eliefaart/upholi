use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub trait DbItem: Serialize + DeserializeOwned + Sync + Send + Unpin {
    /// Get the name of the collection this item will be stored in in the database.
    fn collection_name() -> &'static str;
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password_phc: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Session {
    pub id: String,
    pub user_id: Option<String>,
    pub shares: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Share {
    pub id: String,
    pub user_id: String,
    pub password_phc: String,
    #[serde(flatten)]
    pub data: EncryptedData,
}

#[derive(Serialize, Deserialize)]
pub struct EncryptedData {
    pub base64: String,
    pub nonce: String,
}

impl DbItem for EncryptedData {
    fn collection_name() -> &'static str {
        "items"
    }
}

#[derive(Serialize, Deserialize)]
pub struct File {
    pub file_id: String,
    /// Path, container ID/name, something that indicates where this file is stores.
    pub container: String,
}

impl DbItem for File {
    fn collection_name() -> &'static str {
        "files"
    }
}
