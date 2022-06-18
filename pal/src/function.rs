use std::sync::Arc;
use planar_core::*;
pub use planar_core::apdu::*;

pub trait FunctionServer<Q: serde::de::DeserializeOwned, R: serde::Serialize>: Send + Sync {
    fn invoke(&self, q: Q) -> Result<R>;
}

pub trait FunctionClient<Q: serde::Serialize, R: serde::de::DeserializeOwned> {
    fn invoke(&self, q: Q) -> Result<R>;
}

pub trait FunctionRuntime {
    fn register_sqlp(&self, svc: Arc<dyn FunctionServer<SQLPQ, SQLPR>>);
    fn get_sqlp(&self) -> Box<dyn FunctionClient<SQLPQ, SQLPR>>;
}


