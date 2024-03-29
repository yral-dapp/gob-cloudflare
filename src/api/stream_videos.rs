//! Cloudflare Stream Videos related APIs
//! See [Cloudflare Docs](https://developers.cloudflare.com/stream/)
use std::{collections::HashMap, time::Duration};

use crate::{CfReqAuth, CfReqMeta};
use reqwest::Method;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct Watermark {
    uid: String,
}

/// The [Direct Upload API](https://developers.cloudflare.com/api/operations/stream-videos-upload-videos-via-direct-upload-ur-ls)
#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DirectUpload {
    creator: Option<String>,
    max_duration_seconds: Option<u64>,
    meta: HashMap<String, String>,
    watermark: Option<Watermark>,
}

/// Success response from the [Direct Upload API](https://developers.cloudflare.com/api/operations/stream-videos-upload-videos-via-direct-upload-ur-ls#Responses)
#[derive(Serialize, Deserialize)]
pub struct DirectUploadRes {
    /// Unique identifier for the video
    pub uid: String,
    /// Resultant URL for uploading the video
    #[serde(rename = "uploadURL")]
    pub upload_url: String,
}

impl DirectUpload {
    /// Creator of the video
    pub fn creator(mut self, creator: impl Into<String>) -> Self {
        self.creator = Some(creator.into());
        self
    }

    /// Maximum duration of the video
    pub fn max_duration(mut self, max_duration: Duration) -> Self {
        self.max_duration_seconds = Some(max_duration.as_secs());
        self
    }

    /// Add metadata to the video
    pub fn add_meta(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.meta.insert(key.into(), value.into());
        self
    }

    /// Add a watermark by UID to the video
    /// see [Cloudflare docs](https://developers.cloudflare.com/stream/edit-videos/applying-watermarks/)
    pub fn watermark(mut self, uid: impl Into<String>) -> Self {
        self.watermark = Some(Watermark { uid: uid.into() });
        self
    }
}

impl CfReqMeta for DirectUpload {
    const METHOD: Method = Method::POST;
    type Response = DirectUploadRes;
}

impl CfReqAuth for DirectUpload {
    type Url = String;

    fn path(&self, account_id: &str) -> String {
        format!("accounts/{account_id}/stream/direct_upload")
    }
}

/// [Create Download link](https://developers.cloudflare.com/api/operations/stream-m-p-4-downloads-create-downloads) API
#[derive(Serialize)]
pub struct CreateDownloads {
    #[serde(skip)]
    identifier: String,
}

/// Success response from the [Create Download link](https://developers.cloudflare.com/api/operations/stream-m-p-4-downloads-create-downloads#Responses) API
#[derive(Serialize, Deserialize)]
pub struct CreateDownloadsRes {}

impl CreateDownloads {
    /// Create a download link for the video
    /// identifier is the video's uid
    pub fn new(identifier: impl Into<String>) -> Self {
        Self {
            identifier: identifier.into(),
        }
    }
}

impl CfReqMeta for CreateDownloads {
    const METHOD: Method = Method::POST;
    type Response = CreateDownloadsRes;
}

impl CfReqAuth for CreateDownloads {
    type Url = String;

    fn path(&self, account_id: &str) -> String {
        format!("accounts/{account_id}/stream/{}/downloads", self.identifier)
    }
}

/// [Retrieve Video Details](https://developers.cloudflare.com/api/operations/stream-videos-retrieve-video-details) API
#[derive(Serialize)]
pub struct VideoDetails {
    #[serde(skip)]
    identifier: String,
}

/// Specifies the status of the video
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoStatus {
    /// video encode failure code
    pub error_reason_code: Option<String>,
    /// video encode failure message
    pub error_reason_text: Option<String>,
    /// size of the entire upload in bytes
    pub pct_complete: Option<String>,
    /// the processing status for all quality levels for a video
    /// one of `pendingupload`, `downloading`, `queued`, `inprogress`, `ready`, `error`
    pub state: String,
}

/// Success response from the [Retrieve Video Details](https://developers.cloudflare.com/api/operations/stream-videos-retrieve-video-details#Responses) API
/// Note: This response is not complete, only the status is returned
#[derive(Serialize, Deserialize)]
pub struct VideoDetailsRes {
    /// Video status
    pub status: VideoStatus,
}

impl VideoDetails {
    /// Retrieve details of a video
    /// identifier is the video's uid
    pub fn new(identifier: impl Into<String>) -> Self {
        Self {
            identifier: identifier.into(),
        }
    }
}

impl CfReqMeta for VideoDetails {
    const METHOD: Method = Method::GET;
    type Response = VideoDetailsRes;
}

impl CfReqAuth for VideoDetails {
    type Url = String;

    fn path(&self, account_id: &str) -> String {
        format!("accounts/{account_id}/stream/{}", self.identifier)
    }
}
