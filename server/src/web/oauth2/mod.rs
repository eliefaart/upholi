//use oauth2::prelude::*;
use oauth2::{AuthUrl, ClientId, ClientSecret, TokenUrl, CsrfToken, PkceCodeChallenge, AuthorizationCode, TokenResponse};
use oauth2::basic::BasicClient;
use oauth2::reqwest::http_client;
use async_trait::async_trait;
use crate::error::*;
use crate::entities::user::User;

mod github;

/// Get an OAuth2 provider
pub fn get_provider() -> impl OAuth2Provider {
	github::Github{}
}

pub struct AuthUrlInfo {
    pub auth_url: String,
    pub csrf_token: String,
    pub pkce_verifier: String
}

#[async_trait]
pub trait OAuth2Provider {
	/// Get an OAauth 2 client
	/// 
	/// TODO: I don't want this fn to be public, 
	/// but get_auth_url and get_access_token depend on it and its implementation is oauth-provider specific
	/// I havn't figured out yet how to best model this in rust.
	fn get_oauth_client() -> &'static oauth2::basic::BasicClient;

	/// Get user info for access_token
	async fn get_user_info(&self, access_token: &str) -> Result<User>;

	/// Generate a full authorization URL to redirect the user to
	fn get_auth_url(&self) -> AuthUrlInfo {
		// Generate a PKCE challenge.
		let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

		// Generate the full authorization URL
		let (auth_url, csrf_token) = Self::get_oauth_client()
			.authorize_url(CsrfToken::new_random)
			.set_pkce_challenge(pkce_challenge)
            .url();
            
        AuthUrlInfo {
            auth_url: auth_url.to_string(),
            csrf_token: csrf_token.secret().to_string(),
            pkce_verifier: pkce_verifier.secret().to_string()
        }
	}

	/// Get access token for the authorization code received from oauth provider 
	fn get_access_token(&self, auth_code: &str, pkce_verifier: &str) -> Result<String> {
		let token_result = Self::get_oauth_client()
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
}

/// Create an oauth2 client
fn create_client(oauth_settings: &crate::settings::OAuth) -> oauth2::basic::BasicClient {
	let client_id = ClientId::new(oauth_settings.client_id.to_string());
	let client_secret = ClientSecret::new(oauth_settings.client_secret.to_string());

	let auth_url = AuthUrl::new(oauth_settings.auth_url.to_string()).expect("Invalid authorization endpoint URL");
	let token_url = TokenUrl::new(oauth_settings.token_url.to_string()).expect("Invalid token endpoint URL");
	
	BasicClient::new(
		client_id,
		Some(client_secret),
		auth_url,
		Some(token_url)
	)
}