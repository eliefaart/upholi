use upholi_lib::http::request::FindSharesFilter;
use upholi_lib::http::*;
use upholi_lib::result::Result;

use crate::entities::EntityWithProof;

pub async fn get_photos_using_key_access_proof(base_url: &str, entities: &[EntityWithProof]) -> Result<Vec<response::PhotoMinimal>> {
	let url = format!("{}/api/photos/find", &base_url);
	let client = reqwest::Client::new();

	let response = client.post(&url).json(&entities).send().await?;

	let photos = response.json().await?;
	Ok(photos)
}

pub async fn get_albums(base_url: &str) -> Result<Vec<response::Album>> {
	let url = format!("{}/api/albums", &base_url);
	let response = reqwest::get(url).await?;
	let albums = response.json().await?;
	Ok(albums)
}

pub async fn get_shares(base_url: &str, filters: Option<FindSharesFilter>) -> Result<Vec<response::Share>> {
	let mut url = format!("{}/api/shares", base_url);
	if let Some(filters) = filters {
		if let Some(identifier_hash) = filters.identifier_hash {
			url = format!("{}?identifier_hash={}", url, identifier_hash);
		}
	}

	let response = reqwest::get(url).await?;
	let shares = response.json().await?;
	Ok(shares)
}

pub async fn get_share(base_url: &str, id: &str) -> Result<response::Share> {
	let url = format!("{}/api/share/{}", base_url, id);
	let response = reqwest::get(url).await?;
	let share = response.json().await?;
	Ok(share)
}
