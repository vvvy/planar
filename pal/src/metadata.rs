pub use planar_core::*;
pub use planar_core::table_md::*;

use async_trait::async_trait;
use serde::{Serialize, de::DeserializeOwned};

pub enum KeyNamespace {
    Process,
    Table
}

impl From<KeyNamespace> for String {
    fn from(value: KeyNamespace) -> Self {
        match value {
            KeyNamespace::Table => "t".into(),
            KeyNamespace::Process => "p".into(),
        }
    }
}

/// Generic key
pub struct Key<'r> {
    /// Namespace identifier
    pub ns: KeyNamespace,
    /// Effective number of levels
    pub ln: u8,
    /// Level 1 key info
    pub l1: &'r str,
    /// Level 2 key info
    pub l2: &'r str,
    /// Level 3 key info
    pub l3: &'r str,
    /// Level 4 key info
    pub l4: &'r str
}

impl<'r> Key<'r> {
    fn n2(ns: KeyNamespace, l1: &'r str, l2: &'r str) -> Self { Self { ns, ln: 2, l1, l2, l3: "", l4: "" } }
    //fn n3(ns: KeyNamespace, l1: &'r str, l2: &'r str, l3: &'r str) -> Self { Self { ns, l1, l2, l3, l4: "" } }
    //fn n4(ns: KeyNamespace, l1: &'r str, l2: &'r str, l3: &'r str, l4: &'r str) -> Self { Self { ns, l1, l2, l3, l4 } }
}

impl<'r> From<TableDefKey<'r>> for Key<'r> {
    fn from(value: TableDefKey<'r>) -> Self {
        Key::n2(KeyNamespace::Table, value.schema, value.table)
    }
}


#[async_trait]
pub trait MetadataApi {
    async fn put<'r, K: Into<Key<'r>>+Send, D: Serialize+Sync>(&self, key: K, data: &'r D) -> Result<()>;
    async fn get<'r, K: Into<Key<'r>>+Send, D: DeserializeOwned>(&self, key: K) -> Result<Option<D>>;
}

#[async_trait]
pub trait TableMetadataApi {
    async fn put_table_def<'r>(&self, key: TableDefKey<'r>, table_def: &'r TableDef) -> Result<()>;
    async fn get_table_def<'r>(&self, key: TableDefKey<'r>) -> Result<Option<TableDef>>;
}

fn _dyn_tmda_check(_: Box<dyn TableMetadataApi>) { }


pub struct TableMetadataApiImpl<M> {
    m: M
}

impl<M: MetadataApi> TableMetadataApiImpl<M> {
    pub fn new(m: M) -> Self { Self { m } }
}

#[async_trait]
impl<M: MetadataApi + Sync + Send> TableMetadataApi for TableMetadataApiImpl<M> {
    async fn put_table_def<'r>(&self, key: TableDefKey<'r>, table_def: &'r TableDef) -> Result<()> {
        self.m.put(key, table_def).await
    }
    async fn get_table_def<'r>(&self, key: TableDefKey<'r>) -> Result<Option<TableDef>> {
        self.m.get(key).await
    }
}
