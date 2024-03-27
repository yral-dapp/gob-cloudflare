use serde::Deserialize;

/// Cloudflare API error response
#[derive(Deserialize, Debug)]
pub struct CfApiErr {
    /// Error code
    pub code: u16,
    /// Error message
    pub message: String,
}

#[derive(Deserialize)]
pub(crate) struct CfSuccessRes<T> {
    pub result: T,
}

#[derive(Deserialize)]
pub(crate) struct CfErrRes {
    pub errors: Vec<CfApiErr>,
}
