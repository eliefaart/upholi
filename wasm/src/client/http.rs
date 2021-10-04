use upholi_lib::result::Result;
use upholi_lib::http::*;

use crate::entities::EntityWithProof;

pub async fn get_photos_using_key_access_proof(base_url: &str, entities: &Vec<EntityWithProof>) -> Result<Vec<response::PhotoMinimal>> {
	let url = format!("{}/api/photos/find", &base_url);
	let client = reqwest::Client::new();

	let response = client.post(&url)
		.json(&entities)
		.send().await?;

	let photos = response.json::<Vec<response::PhotoMinimal>>().await?;
	Ok(photos)
}

pub async fn get_albums(base_url: &str) -> Result<Vec<response::Album>> {
	let url = format!("{}/api/albums", &base_url);
	let response = reqwest::get(url).await?;
	let albums = response.json::<Vec<response::Album>>().await?;
	Ok(albums)
}

pub async fn get_share(base_url: &str, id: &str) -> Result<response::Share> {
	let url = format!("{}/api/share/{}", base_url, id);
	let response = reqwest::get(url).await?;
	let share = response.json::<response::Share>().await?;
	Ok(share)
}