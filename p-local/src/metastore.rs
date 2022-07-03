mod etcd;

use planar_core::*;
use pal::metadata::{MetadataApi, Key};
use etcd::MetaStoreAsyncClient;
use async_trait::async_trait;
use serde::{Serialize, de::DeserializeOwned};


pub(crate) struct MetadataClient { }

fn hkey<'r, K: Into<Key<'r>>>(key: K) -> String {
    let key: Key<'r> = key.into();
    let mut rv: String = key.ns.into();

    if key.ln < 1 { return rv }
    rv.push('/');
    rv.push_str(key.l1);

    if key.ln < 2 { return rv }
    rv.push('/');
    rv.push_str(key.l2);

    if key.ln < 3 { return rv }
    rv.push('/');
    rv.push_str(key.l3);

    if key.ln < 4 { return rv }
    rv.push('/');
    rv.push_str(key.l4);

    rv
}

#[async_trait]
impl MetadataApi for MetadataClient {
    async fn put<'r, K: Into<Key<'r>> + Send, D: Serialize + Sync>(&self, key: K, data: &'r D) -> Result<()> {
        MetaStoreAsyncClient::new().await?.put(&hkey(key), data).await
    }
    async fn get<'r, K: Into<Key<'r>> + Send, D: DeserializeOwned>(&self, key: K) -> Result<Option<D>> {
        MetaStoreAsyncClient::new().await?.get(&hkey(key)).await
    }
}