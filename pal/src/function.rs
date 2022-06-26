use planar_core::*;
pub use planar_core::apdu::*;
use std::sync::Arc;
use async_trait::async_trait;

#[async_trait]
pub trait Context: Send + Sync {
    async fn invoke_sqlp(&self, q: &SQLPQ) -> Result<SQLPR>;
}

#[async_trait]
pub trait Function<Q: serde::de::DeserializeOwned, R: serde::Serialize>: Send + Sync {
    async fn invoke(&self, cx: &dyn Context, q: Q) -> Result<R>;
}

pub trait FunctionRuntime {
    fn register_sqlp(&self, svc: Arc<dyn Function<SQLPQ, SQLPR>>);
}


