//! Cloudflare KV API
//! See [Cloudflare Docs](https://developers.cloudflare.com/kv/)

use std::marker::PhantomData;

use reqwest::{multipart::Form, Method};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{
    request::{CfReqAuth, CfReqMeta, CfRes},
    Result,
};

/// Namespace helper
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct KvNamespace {
    /// Namespace ID
    namespace_id: String,
}

impl KvNamespace {
    /// Create a new namespace helper
    pub fn new(namespace_id: String) -> Self {
        Self { namespace_id }
    }

    /// Write a KV pair
    pub fn write_kv(&self, key_name: String) -> WriteKVWithMeta {
        WriteKVWithMeta {
            namespace_id: self.namespace_id.clone(),
            key_name,
            metadata: String::new(),
            value: String::new(),
        }
    }

    /// Read a KV pair
    pub fn read_kv(&self, key_name: String) -> ReadKV {
        ReadKV {
            namespace_id: self.namespace_id.clone(),
            key_name,
        }
    }

    /// Read the metadata for a KV pair
    pub fn read_kv_metadata<Meta: DeserializeOwned>(&self, key_name: String) -> ReadKVMeta<Meta> {
        ReadKVMeta {
            namespace_id: self.namespace_id.clone(),
            key_name,
            _meta: PhantomData,
        }
    }
}

/// [Write KV pair with metadata](https://developers.cloudflare.com/api/operations/workers-kv-namespace-write-key-value-pair-with-metadata) API
pub struct WriteKVWithMeta {
    namespace_id: String,
    key_name: String,
    metadata: String,
    value: String,
}

impl WriteKVWithMeta {
    /// Metadata for the KV pair
    pub fn metadata(mut self, metadata: &impl Serialize) -> Result<Self> {
        self.metadata = serde_json::to_string(&metadata)?;
        Ok(self)
    }

    /// Value corresponding to the key
    pub fn value(mut self, value: String) -> Self {
        self.value = value;
        self
    }
}

/// Success response from the [Write KV pair with metadata](https://developers.cloudflare.com/api/operations/workers-kv-namespace-write-key-value-pair-with-metadata#response-body) API
#[derive(Serialize, Deserialize)]
pub struct WriteKVWithMetaRes {}

impl CfRes for WriteKVWithMetaRes {
    const IS_SUCCESS_WRAPPED: bool = true;
}

impl CfReqMeta for WriteKVWithMeta {
    const METHOD: Method = Method::PUT;
    type JsonResponse = WriteKVWithMetaRes;
}

impl CfReqAuth for WriteKVWithMeta {
    type Url = String;

    fn path(&self, account_id: &str) -> String {
        format!(
            "accounts/{account_id}/storage/kv/namespaces/{namespace_id}/values/{key_name}",
            namespace_id = self.namespace_id,
            key_name = self.key_name,
            account_id = account_id
        )
    }
}

impl From<WriteKVWithMeta> for Form {
    fn from(value: WriteKVWithMeta) -> Self {
        Form::new()
            .text("metadata", value.metadata)
            .text("value", value.value)
    }
}

/// [Read KV pair](https://developers.cloudflare.com/api/operations/workers-kv-namespace-read-key-value-pair) API
#[derive(Serialize)]
pub struct ReadKV {
    #[serde(skip)]
    namespace_id: String,
    #[serde(skip)]
    key_name: String,
}

/// Value corresponding to the key
#[derive(Serialize, Deserialize)]
pub struct ReadKVRes(pub String);

impl CfRes for ReadKVRes {
    const IS_SUCCESS_WRAPPED: bool = false;
}

impl CfReqMeta for ReadKV {
    const METHOD: Method = Method::GET;
    type JsonResponse = ReadKVRes;
}

impl CfReqAuth for ReadKV {
    type Url = String;

    fn path(&self, account_id: &str) -> String {
        format!(
            "accounts/{account_id}/storage/kv/namespaces/{namespace_id}/values/{key_name}",
            namespace_id = self.namespace_id,
            key_name = self.key_name,
            account_id = account_id
        )
    }
}

/// [Read KV pair metadata](https://developers.cloudflare.com/api/operations/workers-kv-namespace-read-the-metadata-for-a-key)
#[derive(Serialize)]
pub struct ReadKVMeta<Meta> {
    #[serde(skip)]
    namespace_id: String,
    #[serde(skip)]
    key_name: String,
    #[serde(skip)]
    _meta: PhantomData<Meta>,
}

/// Metadata for the KV pair
#[derive(Deserialize)]
pub struct ReadKVMetaRes<Meta>(pub Meta);

impl<Meta: DeserializeOwned> CfRes for ReadKVMetaRes<Meta> {
    const IS_SUCCESS_WRAPPED: bool = true;
}

impl<Meta: DeserializeOwned + Send> CfReqMeta for ReadKVMeta<Meta> {
    const METHOD: Method = Method::GET;
    type JsonResponse = ReadKVMetaRes<Meta>;
}

impl<Meta: DeserializeOwned + Send> CfReqAuth for ReadKVMeta<Meta> {
    type Url = String;

    fn path(&self, account_id: &str) -> String {
        format!(
            "accounts/{account_id}/storage/kv/namespaces/{namespace_id}/metadata/{key_name}",
            namespace_id = self.namespace_id,
            key_name = self.key_name,
            account_id = account_id
        )
    }
}
