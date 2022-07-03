use planar_core::*;
use pal::{function::*, metadata::{TableMetadataApi, TableMetadataApiImpl}};
use crate::{function::*, queue::*, metastore::*};
use async_trait::async_trait;
use tokio::task::JoinHandle;
use std::sync::Arc;


const SQLP_PORT: u16 = 7001;
const DISP_Q: &'static str = "disp";

struct LocalContext { 
    sqlp: WebClient<SQLPQ, SQLPR>,
    disp: Publisher<DispM>,
    tmd: TableMetadataApiImpl<MetadataClient>
}

impl LocalContext {
    async fn new() -> Result<Self> { Ok(Self { 
        sqlp: WebClient::new(SQLP_PORT),
        disp: Publisher::new(DISP_Q.into()).await?,
        tmd: TableMetadataApiImpl::new(MetadataClient {  })
    })}
}

#[async_trait]
impl Context for LocalContext {

    async fn invoke_sqlp(&self, q: &SQLPQ) -> Result<SQLPR> {
        self.sqlp.invoke(q).await
    }

    async fn submit_disp(&self, m: &DispM) -> Result<()> {
        self.disp.publish(m).await
    }

    async fn table_metadata(&self) -> &dyn TableMetadataApi {
        &self.tmd
    }

}


pub struct Runtime {
    cx: Arc<LocalContext>
}

impl Runtime {
    pub async fn new() -> Result<Self> { Ok(Self { cx: Arc::new(LocalContext::new().await?) }) }

    pub fn context(&self) -> Arc<dyn Context> { return self.cx.clone() } 

    pub fn run_sqlp(&self, svc: Arc<dyn Function<SQLPQ, SQLPR>>) -> JoinHandle<Result<()>> {
        launch_web_server(SQLP_PORT, self.cx.clone(), svc) 
    }

    pub fn run_disp(&self, svc: Arc<dyn Function<DispM, ()>>) -> JoinHandle<Result<()>> {
        Consumer::spawn_consumer(DISP_Q.into(), self.cx.clone(), svc)
    }
}