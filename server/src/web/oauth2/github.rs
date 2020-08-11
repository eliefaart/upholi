use serde::Deserialize;
use lazy_static::lazy_static;
use async_trait::async_trait;
use crate::error::*;
use crate::database;
use crate::database::{DatabaseExt,DatabaseEntity};
use crate::entities::user::User;
use crate::ids::create_unique_id;

const IDENTITY_PROVIDER_NAME: &str = "github";
const USER_AGENT: &str = "localhost";

lazy_static! {
    #[derive(Debug)]
    pub static ref OAUTH_CLIENT: oauth2::basic::BasicClient = super::create_client(&crate::SETTINGS.oauth);
}

pub struct Github {}

/// User info data as returned by Github, but only those I need.
/// See https://developer.github.com/v3/users/#get-a-user for all available fields
#[derive(Deserialize, Debug)]
struct UserInfo {
    pub id: i64
}

#[async_trait]
impl super::OAuth2Provider for Github {
    fn get_oauth_client() -> &'static oauth2::basic::BasicClient {
        &OAUTH_CLIENT
    }
    
    async fn get_user_info(&self, access_token: &str) -> Result<User> {
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
}