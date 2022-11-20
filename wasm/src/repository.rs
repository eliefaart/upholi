use crate::{models::*, API_CLIENT};
use anyhow::{anyhow, Result};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::RwLock};

static CACHE: Lazy<RwLock<HashMap<String, ItemVariant>>> = Lazy::new(|| RwLock::new(HashMap::new()));

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ItemVariant {
    MasterKey(Vec<u8>),
    Library(Library),
    Photo(Photo),
    Album(Album),
}

impl TryFrom<ItemVariant> for Vec<u8> {
    type Error = anyhow::Error;

    fn try_from(value: ItemVariant) -> Result<Self, Self::Error> {
        if let ItemVariant::MasterKey(key) = value {
            Ok(key)
        } else {
            Err(anyhow!("ItemVariant is not a master key"))
        }
    }
}

impl TryFrom<ItemVariant> for Library {
    type Error = anyhow::Error;

    fn try_from(value: ItemVariant) -> Result<Self, Self::Error> {
        if let ItemVariant::Library(library) = value {
            Ok(library)
        } else {
            Err(anyhow!("ItemVariant is not a library"))
        }
    }
}

impl TryFrom<ItemVariant> for Photo {
    type Error = anyhow::Error;

    fn try_from(value: ItemVariant) -> Result<Self, Self::Error> {
        if let ItemVariant::Photo(photo) = value {
            Ok(photo)
        } else {
            Err(anyhow!("ItemVariant is not a photo"))
        }
    }
}

impl TryFrom<ItemVariant> for Album {
    type Error = anyhow::Error;

    fn try_from(value: ItemVariant) -> Result<Self, Self::Error> {
        if let ItemVariant::Album(album) = value {
            Ok(album)
        } else {
            Err(anyhow!("ItemVariant is not an album"))
        }
    }
}

impl From<Vec<u8>> for ItemVariant {
    fn from(value: Vec<u8>) -> Self {
        ItemVariant::MasterKey(value)
    }
}

impl From<Library> for ItemVariant {
    fn from(value: Library) -> Self {
        ItemVariant::Library(value)
    }
}

impl From<Photo> for ItemVariant {
    fn from(value: Photo) -> Self {
        ItemVariant::Photo(value)
    }
}

impl From<Album> for ItemVariant {
    fn from(value: Album) -> Self {
        ItemVariant::Album(value)
    }
}

pub async fn get(item_id: &str, key: &[u8]) -> Result<Option<ItemVariant>> {
    let is_cached = CACHE.read().unwrap().contains_key(item_id);

    // Try to fetch it from API if it is not in the cache
    if !is_cached {
        if let Some(item) = API_CLIENT.get_item(item_id).await? {
            let item = item.decrypt(key)?;
            let mut cache = CACHE.write().unwrap();
            cache.insert(item_id.to_string(), item);
        }
    }

    // Get the item from cache and return it
    let cache = CACHE.read().unwrap();
    let item = cache.get(item_id);
    match item {
        Some(item) => Ok(Some(item.to_owned())),
        None => Ok(None),
    }
}

pub async fn get_or(item_id: &str, key: &[u8], create: &dyn Fn() -> ItemVariant) -> Result<ItemVariant> {
    let item = get(item_id, key).await?;

    let item = match item {
        Some(item) => item,
        None => {
            let item = create();
            set(item_id, key, item.clone()).await?;
            item
        }
    };

    Ok(item)
}

pub async fn set(item_id: &str, key: &[u8], item: ItemVariant) -> Result<()> {
    let text_item = EncryptedItem::from(key, &item)?;
    API_CLIENT.set_item(item_id, &text_item).await?;

    let mut cache = CACHE.write().unwrap();
    cache.insert(item_id.to_string(), item);

    Ok(())
}

pub async fn delete(item_id: &str) -> Result<()> {
    let item_exists = CACHE.write().unwrap().remove(item_id).is_some();
    if item_exists {
        API_CLIENT.delete_item(item_id).await?;
    }

    Ok(())
}
