/// API HTTP request models
pub mod request {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct CreateUserRequest {
        pub username: String,
        pub password: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct AuthenticateUserRequest {
        pub username: String,
        pub password: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct AuthorizeShareRequest {
        pub password: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct UpsertShareRequest {
        pub id: String,
        pub password: String,
        pub base64: String,
        pub nonce: String,
        /// List of item ID that this share includes
        pub items: Vec<String>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct DeleteManyRequest {
        pub ids: Vec<String>,
    }
}

/// API HTTP response models
pub mod response {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct CreatedResult {
        pub id: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct GetShareResult {
        pub base64: String,
        pub nonce: String,
    }
}
