use crate::database;
use crate::database::{DatabaseEntity, DatabaseExt};
use crate::entities::user::User;
use crate::error::*;
use crate::ids::create_unique_id;
use async_trait::async_trait;
use serde::Deserialize;

const IDENTITY_PROVIDER_NAME: &str = "github";
const USER_AGENT: &str = "localhost";

pub struct Github {}

/// User info data as returned by Github, but only those I need.
/// See https://developer.github.com/v3/users/#get-a-user for all available fields
#[derive(Deserialize, Debug)]
struct UserInfo {
	pub id: i64,
}

#[async_trait]
impl super::OAuth2Provider for Github {
	fn get_provider_id<'a>() -> &'a str {
		IDENTITY_PROVIDER_NAME
	}

	async fn get_user_info(&self, access_token: &str) -> Result<User> {
		let provider_id = Self::get_provider_id();
		match crate::SETTINGS.get_oauth_provider_settings(provider_id) {
			Some(oauth_settings) => {
				let client = reqwest::Client::new();
				let request = client
					.get(&oauth_settings.userinfo_url)
					.header(
						reqwest::header::AUTHORIZATION,
						format!("Bearer {}", access_token),
					)
					.header(reqwest::header::USER_AGENT, USER_AGENT);
				let response = request.send().await?;

				let user_info = response.json::<UserInfo>().await?;
				let user_opt = database::get_database().get_user_for_identity_provider(IDENTITY_PROVIDER_NAME,&user_info.id.to_string())?;

				// Take the user in the Option, or create a new one
				let user = match user_opt {
					Some(user) => user,
					None => User::create(IDENTITY_PROVIDER_NAME.to_string(), user_info.id.to_string()).await?
				};

				Ok(user)
			}
			None => Err(Box::from(format!(
				"No settings found for OAuth provider {}",
				provider_id
			))),
		}
	}
}
