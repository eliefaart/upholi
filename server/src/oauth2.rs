//use oauth2::prelude::*;
use oauth2::{AuthUrl, ClientId, ClientSecret, CsrfToken, TokenUrl, TokenResponse, AuthorizationCode};
use oauth2::basic::BasicClient;
use oauth2::reqwest::http_client;
use reqwest;
use serde::{Deserialize};
use lazy_static::lazy_static;

// https://docs.rs/oauth2/3.0.0-alpha.10/oauth2/index.html
// TODO: Add PKCE challenge
//	For this I need to store PKCE verification code for some state

const USER_AGENT: &str = "localhost";
const CONFIG_GET_USERINFO_URI: &str = "https://api.github.com/user";

lazy_static! {
	#[derive(Debug)]
	pub static ref OAUTH_CLIENT: oauth2::basic::BasicClient = create_client();
}

#[derive(Deserialize, Debug)]
pub struct UserInfo {
	id: u64
}

/// Generate a full authorization URL to redirect the user to
pub fn get_auth_url() -> String {
	// Generate the full authorization URL
	let (auth_url, _csrf_token) = OAUTH_CLIENT
		.authorize_url(CsrfToken::new_random)
		.url();

	auth_url.to_string()
}

/// Get access token for the authorization code received from oauth provider 
pub fn get_access_token(auth_code: &str) -> Result<String, String> {
	let token_result = OAUTH_CLIENT
		.exchange_code(AuthorizationCode::new(auth_code.to_string()))
		.request(http_client);

	println!("token_result: {:?}", token_result);

	match token_result {
		Ok(token) => {
			println!("token: {:?}", token);
			println!("access_token: {:?}", token.access_token());
			println!("access_token_secret: {:?}", token.access_token().secret());
			println!("extra_fields: {:?}", token.extra_fields());
			
			Ok(token.access_token().secret().to_string())
		},
		Err(_) => Err("Failed to get access token for auth code".to_string())
	}
}

/// Get user info for access_token
pub async fn get_user_info(access_token: &str) -> Result<UserInfo, Box<dyn std::error::Error>> {
	let client = reqwest::Client::new();
	let request = client
		.get(CONFIG_GET_USERINFO_URI)
		.header(reqwest::header::AUTHORIZATION, format!("Bearer {}", access_token))
		.header(reqwest::header::USER_AGENT, USER_AGENT);
	let response = request.send().await?;

	let user_info = response.json::<UserInfo>().await?;
	Ok(user_info)
}

/// Create an oauth2 client
fn create_client() -> oauth2::basic::BasicClient {
	let config_client_id = "c07b616bdf23782d7ba6";
	let config_client_secret = "970ba9fd011ce9020f05eeb1db2f0ad5b09f3e4f";	// They're localhost secrets..
	let config_auth_url = "https://github.com/login/oauth/authorize";
	let config_token_url = "https://github.com/login/oauth/access_token";

	let client_id = ClientId::new(config_client_id.to_string());
	let client_secret = ClientSecret::new(config_client_secret.to_string());

	let auth_url = AuthUrl::new(config_auth_url.to_string()).expect("Invalid authorization endpoint URL");
	let token_url = TokenUrl::new(config_token_url.to_string()).expect("Invalid token endpoint URL");
	
	BasicClient::new(
		client_id,
		Some(client_secret),
		auth_url,
		Some(token_url)
	)
}