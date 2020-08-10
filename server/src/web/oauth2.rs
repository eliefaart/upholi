//use oauth2::prelude::*;
use oauth2::{AuthUrl, ClientId, ClientSecret, CsrfToken, TokenUrl, TokenResponse, AuthorizationCode, PkceCodeChallenge};
use oauth2::basic::BasicClient;
use oauth2::reqwest::http_client;
use serde::Deserialize;
use lazy_static::lazy_static;
use crate::error::*;
use crate::database;
use crate::database::{DatabaseExt,DatabaseEntity};
use crate::entities::user::User;
use crate::ids::create_unique_id;

const IDENTITY_PROVIDER_NAME: &str = "github";
const USER_AGENT: &str = "localhost";

lazy_static! {
	#[derive(Debug)]
	pub static ref OAUTH_CLIENT: oauth2::basic::BasicClient = create_client();
}

#[derive(Deserialize, Debug)]
struct UserInfo {
	pub id: i64
}

/// Generate a full authorization URL to redirect the user to
/// Return-type tuple is (auth url, state token, pkce verifier)
/// TODO: Refactor this to return a proper type instead of a 3-string-tuple.
pub fn get_auth_url() -> (String, String, String) {
	// Generate a PKCE challenge.
	let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

	// Generate the full authorization URL
	let (auth_url, csrf_token) = OAUTH_CLIENT
		.authorize_url(CsrfToken::new_random)
		.set_pkce_challenge(pkce_challenge)
		.url();

	(auth_url.to_string(), csrf_token.secret().to_string(), pkce_verifier.secret().to_string())
}

/// Get access token for the authorization code received from oauth provider 
pub fn get_access_token(auth_code: &str, pkce_verifier: &str) -> Result<String> {
	let token_result = OAUTH_CLIENT
		.exchange_code(AuthorizationCode::new(auth_code.to_string()))
		.set_pkce_verifier(oauth2::PkceCodeVerifier::new(pkce_verifier.to_string()))
		.request(http_client);

	match token_result {
		Ok(token) => {
			Ok(token.access_token().secret().to_string())
		},
		Err(error) => {
			println!("{}", error);
			Err(Box::from(Oauth2Error::GetAccessTokenFailed))
		}
	}
}

/// Get user info for access_token
pub async fn get_user_info(access_token: &str) -> Result<User, Box<dyn std::error::Error>> {
	let client = reqwest::Client::new();
	let request = client
		.get(&crate::SETTINGS.oauth.userinfo_url)
		.header(reqwest::header::AUTHORIZATION, format!("Bearer {}", access_token))
		.header(reqwest::header::USER_AGENT, USER_AGENT);
	let response = request.send().await?;

	let user_info = response.json::<UserInfo>().await?;
	let user_opt = database::get_database().get_user_for_identity_provider(IDENTITY_PROVIDER_NAME, &user_info.id.to_string())?;

	// Take the user in the Option, or create a new one
	let user = match user_opt {
		Some(user) => user,
		None => {
			let user = User{
				id: create_unique_id(),
				identity_provider: IDENTITY_PROVIDER_NAME.to_string(),
				identity_provider_user_id: user_info.id.to_string()
			};
	
			user.insert()?;
			user
		}
	};
	
	Ok(user)
}

/// Create an oauth2 client
fn create_client() -> oauth2::basic::BasicClient {
	let client_id = ClientId::new(crate::SETTINGS.oauth.client_id.to_string());
	let client_secret = ClientSecret::new(crate::SETTINGS.oauth.client_secret.to_string());

	let auth_url = AuthUrl::new(crate::SETTINGS.oauth.auth_url.to_string()).expect("Invalid authorization endpoint URL");
	let token_url = TokenUrl::new(crate::SETTINGS.oauth.token_url.to_string()).expect("Invalid token endpoint URL");
	
	BasicClient::new(
		client_id,
		Some(client_secret),
		auth_url,
		Some(token_url)
	)
}