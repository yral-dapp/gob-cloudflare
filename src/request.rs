//! Request traits for Cloudflare API requests
//! each request must implement [CfReqMeta] and one of [CfReq] or [CfReqAuth]
use bytes::Bytes;
use reqwest::Method;
use serde::de::DeserializeOwned;

use crate::Result;

/// Metadata for a Cloudflare API JSON request
pub trait CfReqMeta: Sized + Send {
    /// HTTP method for the request
    const METHOD: Method;
    /// Expected Response type
    type Response: DeserializeOwned;

    /// Deserialize the response from the API
    /// The default implementation that assumes the response is JSON encoded [crate::CfSuccessRes]
    /// and extracts the `result` field
    fn deserialize_response(body: Bytes) -> Result<Self::Response> {
        let res: crate::CfSuccessRes<Self::Response> = serde_json::from_slice(&body)?;
        Ok(res.result)
    }
}

/// A Cloudflare API request that does not require authentication
pub trait CfReq: CfReqMeta {
    /// Path for the request relative to the base URL(i.e [crate::consts::CF_BASE_URL])
    const PATH: &'static str;
}

/// A Cloudflare API request that requires authentication
pub trait CfReqAuth: CfReqMeta {
    /// Url kind (usually [String])
    type Url: AsRef<str>;

    /// Path for the request relative to the base URL(i.e [crate::consts::CF_BASE_URL])
    fn path(&self, account_id: &str) -> Self::Url;
}
