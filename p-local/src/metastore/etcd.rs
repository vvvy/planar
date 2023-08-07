use planar_core::*;
use etcd_client::Client;
use serde::{Serialize, de::DeserializeOwned};


pub(crate) struct MetaStoreAsyncClient {
    client: Client
}

impl MetaStoreAsyncClient {
    pub(crate) async fn new() -> Result<Self> {
        let client = Client::connect(["localhost:2379"], None).await?;
        Ok(Self { client })
    }

    pub(crate) async fn put<V: Serialize>(&mut self, key: &str, value: &V) -> Result<()> {
        let v = serde_json::to_vec(value)?;
        self.client.put(key, v, None).await?;
        Ok(())
    }

    pub(crate) async fn get<V: DeserializeOwned>(&mut self, key: &str) -> Result<Option<V>> {
        let w = self.client.get(key, None).await?;
        let kvs = w.kvs();
        let rv = if !kvs.is_empty() { Some(serde_json::from_slice(kvs[0].value())?) } else { None } ;
        Ok(rv)
    }
}

