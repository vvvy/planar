use planar_core::*;
use etcd_client::Client;
use tokio::runtime::Runtime;
use serde::{Serialize, de::DeserializeOwned};


struct MetaStoreAsyncClient {
    client: Client
}

impl MetaStoreAsyncClient {
    async fn new() -> Result<Self> {
        let client = Client::connect(["localhost:2379"], None).await?;
        Ok(Self { client })
    }

    async fn put<V: Serialize>(&mut self, key: &str, value: &V) -> Result<()> {
        let v = serde_json::to_vec(value)?;
        self.client.put(key, v, None).await?;
        Ok(())
    }

    async fn get<V: DeserializeOwned>(&mut self, key: &str) -> Result<Option<V>> {
        let w = self.client.get(key, None).await?;
        let kvs = w.kvs();
        let rv = if kvs.len() > 0 { Some(serde_json::from_slice(kvs[0].value())?) } else { None } ;
        Ok(rv)
    }
}

pub struct MetaStoreClient {
    rt: Runtime
}

impl MetaStoreClient {
    pub fn put<V: Serialize>(&mut self, key: &str, value: &V) -> Result<()> {
        self.rt.block_on(async { MetaStoreAsyncClient::new().await?.put(key, value).await })
    }

    pub fn get<V: DeserializeOwned>(&mut self, key: &str) -> Result<Option<V>> {
        self.rt.block_on(async { MetaStoreAsyncClient::new().await?.get(key).await })
    }
}

