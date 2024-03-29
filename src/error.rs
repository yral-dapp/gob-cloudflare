use std::str::Utf8Error;

use super::CfApiErr;
use thiserror::Error;

/// Error type for the Cloudflare API client
#[allow(missing_docs)]
#[derive(Error, Debug)] // Errors are self explanatory
pub enum Error {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error("invalid url: {0}")]
    InvalidUrl(#[from] url::ParseError),
    #[error("json ser/de error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("error(s) from cloudflare: {0:?}")]
    Cloudflare(Vec<CfApiErr>),
    #[error("invalid utf8 string: {0}")]
    Utf8(#[from] Utf8Error),
}

/// Result type for the Cloudflare API client
pub type Result<T, E = Error> = std::result::Result<T, E>;
