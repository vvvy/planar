use planar_core::*;
pub use planar_core::apdu::*;
use async_trait::async_trait;
use crate::metadata::*;

#[async_trait]
pub trait Context: Send + Sync {
    async fn invoke_sqlp(&self, q: &SQLPQ) -> Result<SQLPR>;
    async fn submit_disp(&self, m: &DispM) -> Result<()>;
    async fn table_metadata(&self) -> &dyn TableMetadataApi;
}

#[async_trait]
pub trait Function<Q: serde::de::DeserializeOwned, R: serde::Serialize>: Send + Sync {
    async fn invoke(&self, cx: &dyn Context, q: Q) -> Result<R>;
}

