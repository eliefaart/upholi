use std::error::Error;

/// A short alias for Result<T, Box<dyn std::error::Error>>, allows writing Result<T> instead
pub type Result<T, E = Box<dyn Error>> = std::result::Result<T, E>;