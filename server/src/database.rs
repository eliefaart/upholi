use crate::model::{DbItem, FileItemData, Session, Share, TextItemData, User};
use anyhow::Result;
use async_once::AsyncOnce;
use bson::{doc, Document};
use lazy_static::lazy_static;
use mongodb::{
	options::{ClientOptions, ReplaceOptions},
	Client,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

const COLLECTION_NAME_USERS: &str = "users";
const COLLECTION_NAME_SESSIONS: &str = "sessions";
const COLLECTION_NAME_SHARES: &str = "shares";

#[derive(Serialize, Deserialize)]
struct ItemContainer<TData> {
	pub key: String,
	pub user_id: String,
	pub shares: Vec<String>,
	#[serde(flatten)]
	pub data: TData,
}

lazy_static! {
	/// A reference to the database that can be used to execute queries etc
	static ref DB: AsyncOnce<mongodb::Database> = AsyncOnce::new(async {
		let connection_string = &crate::SETTINGS.database.connection_string;
		let client_options = ClientOptions::parse(connection_string)
			.await
			.expect("Failed to parse database connection string");

		let client =
			Client::with_options(client_options).expect("Failed to initialize database client");

		client
			.default_database()
			.expect("No default database found in connection string")
	});
}

pub async fn insert_user(user: &User) -> Result<()> {
	insert(COLLECTION_NAME_USERS, user).await
}

pub async fn get_user_by_username(username: &str) -> Result<Option<User>> {
	get(COLLECTION_NAME_USERS, "username", username).await
}

pub async fn get_session(id: &str) -> Result<Option<Session>> {
	get(COLLECTION_NAME_SESSIONS, "id", id).await
}

pub async fn upsert_session(session: &Session) -> Result<()> {
	let collection = DB.get().await.collection::<Session>(COLLECTION_NAME_SESSIONS);
	collection
		.replace_one(
			doc! {
				"id": &session.id,
			},
			session,
			ReplaceOptions::builder().upsert(true).build(),
		)
		.await?;

	Ok(())
}

pub async fn get_share(id: &str) -> Result<Option<Share>> {
	get(COLLECTION_NAME_SHARES, "id", id).await
}

pub async fn upsert_share(share: &Share) -> Result<()> {
	let collection = DB.get().await.collection::<Share>(COLLECTION_NAME_SHARES);
	collection
		.replace_one(
			doc! {
				"id": &share.id,
			},
			share,
			ReplaceOptions::builder().upsert(true).build(),
		)
		.await?;

	Ok(())
}

pub async fn delete_share(user_id: &str, id: &str) -> Result<()> {
	let collection = DB.get().await.collection::<Share>(COLLECTION_NAME_SHARES);
	collection
		.delete_one(
			doc! {
				"id": id,
				"user_id": user_id,
			},
			None,
		)
		.await?;

	Ok(())
}

pub async fn set_items_for_share(share_id: &str, item_ids: &[String]) -> Result<()> {
	remove_items_from_share(share_id).await?;

	let collection_names = vec![TextItemData::collection_name(), FileItemData::collection_name()];
	for collection_name in collection_names {
		DB.get()
			.await
			.collection::<Document>(collection_name)
			.update_many(
				doc! {
					"key": {
						"$in": item_ids
					},
				},
				doc! {
					"$addToSet": {
						"shares": share_id
					}
				},
				None,
			)
			.await?;
	}

	Ok(())
}

pub async fn remove_items_from_share(share_id: &str) -> Result<()> {
	let collection_names = vec![TextItemData::collection_name(), FileItemData::collection_name()];
	for collection_name in collection_names {
		DB.get()
			.await
			.collection::<Document>(collection_name)
			.update_many(
				doc! {
					"shares": share_id
				},
				doc! {
					"$pull": {
						"shares": share_id
					}
				},
				None,
			)
			.await?;
	}

	Ok(())
}

/// Removes all authorizations from sessions for given share_id
pub async fn remove_authorizations_for_share(share_id: &str) -> Result<()> {
	DB.get()
		.await
		.collection::<Session>(COLLECTION_NAME_SESSIONS)
		.update_many(
			doc! {"shares": share_id},
			doc! {
				"$pull": {
					"shares": share_id
				}
			},
			None,
		)
		.await?;

	Ok(())
}

/// Get all keys of type.
pub async fn get_item_keys<T: DbItem>(user_id: &str) -> Result<Vec<String>> {
	let collection_name = T::collection_name();
	let collection = DB.get().await.collection::<T>(collection_name);
	let query = collection.aggregate(
		vec![
			doc! {
				"$match": {
					"user_id": user_id,
				}
			},
			doc! {
				"$project": {
					"_id": -1,
					"key": 1
				}
			},
		],
		None,
	);

	let mut cursor = query.await?;

	let mut keys: Vec<String> = vec![];
	while cursor.advance().await? {
		let current = cursor.current();
		let key = current.get_str("key")?;
		keys.push(key.to_string());
	}

	Ok(keys)
}

pub async fn get_item<T: DbItem>(key: &str, session: &Session) -> Result<Option<T>> {
	if session.user_id.is_none() && session.shares.is_empty() {
		return Ok(None);
	}

	let collection = DB.get().await.collection::<ItemContainer<T>>(T::collection_name());
	let mut filter = doc! { "key": key };

	if let Some(user_id) = &session.user_id {
		filter.extend(doc! {"user_id": user_id});
	} else {
		filter.extend(doc! {"shares": { "$in": &session.shares }});
	}

	let result = collection.find_one(filter, None).await?;

	match result {
		Some(item_container) => Ok(Some(item_container.data)),
		None => Ok(None),
	}
}

pub async fn upsert_item<T: DbItem>(key: &str, item: T, user_id: &str) -> Result<()> {
	let item = ItemContainer {
		key: String::from(key),
		user_id: user_id.to_string(),
		data: item,
		shares: vec![],
	};
	let collection = DB.get().await.collection::<ItemContainer<T>>(T::collection_name());
	collection
		.replace_one(
			doc! {
				"key": key,
				"user_id": user_id,
			},
			item,
			ReplaceOptions::builder().upsert(true).build(),
		)
		.await?;

	Ok(())
}

pub async fn delete_item<T: DbItem>(key: &str, user_id: &str) -> Result<()> {
	let collection = DB.get().await.collection::<T>(T::collection_name());
	collection
		.delete_one(
			doc! {
				"key": key,
				"user_id": user_id,
			},
			None,
		)
		.await?;

	Ok(())
}

async fn get<T: DeserializeOwned + Unpin + Send + Sync>(collection_name: &str, id_name: &str, id: &str) -> Result<Option<T>> {
	let collection = DB.get().await.collection::<T>(collection_name);
	let doc = collection
		.find_one(
			doc! {
				id_name: id,
			},
			None,
		)
		.await?;

	Ok(doc)
}

async fn insert<T: Serialize>(collection_name: &str, document: &T) -> Result<()> {
	let collection = DB.get().await.collection::<T>(collection_name);
	collection.insert_one(document, None).await?;

	Ok(())
}
