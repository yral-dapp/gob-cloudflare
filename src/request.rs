//! Request traits for Cloudflare API requests
//! each request must implement [CfReqMeta] and one of [CfReq] or [CfReqAuth]
use reqwest::Method;
use serde::de::DeserializeOwned;

/// Response type for a Cloudflare API request
pub trait CfRes: DeserializeOwned {
    /// Whether the response is wrapped in a success object
    /// i.e `{"result": ...}`
    const IS_SUCCESS_WRAPPED: bool;
}

/// Metadata for a Cloudflare API JSON request
pub trait CfReqMeta: Sized + Send {
    /// HTTP method for the request
    const METHOD: Method;
    /// Expected JSON response type
    type JsonResponse: CfRes;
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
