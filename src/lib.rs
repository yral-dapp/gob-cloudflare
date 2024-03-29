//! A Rust library for accessing the Cloudflare API.
#![deny(missing_docs)]
pub mod api;
pub mod consts;
mod error;
pub mod request;
mod types;

use std::sync::Arc;

use consts::CF_BASE_URL;
pub use error::*;
use request::{CfReq, CfReqAuth, CfReqMeta};
use reqwest::{multipart::Form, IntoUrl, Method, RequestBuilder, Url};
use serde::Serialize;
pub use types::*;

/// Cloudflare API credentials
#[derive(Debug, Clone)]
pub struct Credentials {
    /// API token
    pub token: String,
    /// Account ID
    pub account_id: String,
}

/// Client for accessing the Cloudflare API
/// without any authentication.
#[derive(Clone, Debug)]
pub struct Cloudflare {
    client: reqwest::Client,
    base_url: Arc<Url>,
}

impl Default for Cloudflare {
    fn default() -> Self {
        Self {
            client: Default::default(),
            base_url: Arc::new(CF_BASE_URL.parse().unwrap()),
        }
    }
}

impl Cloudflare {
    /// Create a new client with the given base URL.
    /// Use [Default::default] to use the default base URL ([crate::consts::CF_BASE_URL])
    pub fn new(base_url: Url) -> Self {
        Self {
            client: Default::default(),
            base_url: Arc::new(base_url),
        }
    }

    fn req_builder(
        &self,
        method: Method,
        url: impl IntoUrl,
        auth: Option<&Credentials>,
    ) -> RequestBuilder {
        let reqb = self.client.request(method, url);
        if let Some(creds) = auth {
            reqb.bearer_auth(&creds.token)
        } else {
            reqb
        }
    }

    async fn send_json<Req: CfReqMeta + Serialize>(
        &self,
        req: Req,
        reqb: RequestBuilder,
    ) -> Result<Req::JsonResponse> {
        let reqb = if Req::METHOD == Method::GET {
            reqb.query(&req)
        } else {
            reqb.json(&req)
        };
        self.send_inner::<Req>(reqb).await
    }

    async fn send_inner<Req: CfReqMeta>(&self, reqb: RequestBuilder) -> Result<Req::JsonResponse> {
        let resp = reqb.send().await?;
        let status = resp.status();
        if !status.is_success() {
            let err: CfErrRes = resp.json().await?;
            return Err(Error::Cloudflare(err.errors));
        }
        let res: CfSuccessRes<Req::JsonResponse> = resp.json().await?;
        Ok(res.result)
    }

    /// Send a request to the Cloudflare API.
    pub async fn send<Req: CfReq + Serialize>(&self, req: Req) -> Result<Req::JsonResponse> {
        let reqb = self.req_builder(Req::METHOD, self.base_url.join(Req::PATH)?, None);
        self.send_json(req, reqb).await
    }
}

/// Client for accessing the Cloudflare API
/// with authentication.
#[derive(Clone, Debug)]
pub struct CloudflareAuth {
    inner: Cloudflare,
    creds: Arc<Credentials>,
}

impl CloudflareAuth {
    /// Create a new authenticated client with the given base URL.
    /// use [Self::new] to use the default base URL ([crate::consts::CF_BASE_URL])
    pub fn with_base_url(base_url: Url, creds: Credentials) -> Self {
        Self {
            inner: Cloudflare::new(base_url),
            creds: Arc::new(creds),
        }
    }

    /// Create a new authenticated client with the default base URL.
    pub fn new(creds: Credentials) -> Self {
        Self::with_base_url(CF_BASE_URL.parse().unwrap(), creds)
    }

    /// Send an unauthenticated request to the Cloudflare API.
    pub async fn send<Req: CfReq + Serialize>(&self, req: Req) -> Result<Req::JsonResponse> {
        self.inner.send(req).await
    }

    fn build_url(&self, req: &impl CfReqAuth) -> Result<Url> {
        let path = req.path(&self.creds.account_id);
        let url = self.inner.base_url.join(path.as_ref())?;
        Ok(url)
    }

    /// Send an authenticated request to the Cloudflare API.
    pub async fn send_auth<Req: CfReqAuth + Serialize>(
        &self,
        req: Req,
    ) -> Result<Req::JsonResponse> {
        let url = self.build_url(&req)?;
        let reqb = self.inner.req_builder(Req::METHOD, url, Some(&self.creds));
        self.inner.send_json(req, reqb).await
    }

    /// Send an authenticated multipart request to the Cloudflare API.
    pub async fn send_auth_multipart<Req: CfReqAuth + Into<Form>>(
        &self,
        req: Req,
    ) -> Result<Req::JsonResponse> {
        let url = self.build_url(&req)?;
        let reqb = self.inner.req_builder(Req::METHOD, url, Some(&self.creds));
        let reqb = reqb.multipart(req.into());
        self.inner.send_inner::<Req>(reqb).await
    }
}
