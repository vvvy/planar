
use planar_core::*;
use pal::function::*;
use std::{sync::Arc, marker::PhantomData};
use std::convert::Infallible;
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use async_trait::async_trait;

fn launch_web_server<Q, R>(port: u16, cx: Arc<dyn Context>, svc: Arc<dyn Function<Q, R>>) where 
    Q: serde::de::DeserializeOwned + Send + 'static, 
    R: serde::Serialize + 'static {
        let addr = ([127, 0, 0, 1], port).into();
        
        let make_svc = make_service_fn(move |_| {
            let svc = svc.clone();
            let cx = cx.clone();
            async move {
                Ok::<_, Infallible>(service_fn(move |mut q: Request<Body>|{ 
                    let svc = svc.clone();
                    let cx = cx.clone();
                    async move {
                        let qraw = hyper::body::to_bytes(q.body_mut()).await?;
                        let q: Q = serde_json::from_slice(qraw.as_ref())?;
                        let r = svc.invoke(cx.as_ref(), q).await?;
                        let rraw = serde_json::to_vec(&r)?;
                        Ok::<_, Error>(Response::new(Body::from(rraw)))
                    }
                }))
            }
        });
        
        // Then bind and serve...
        let server = Server::bind(&addr)
            .serve(make_svc);
        
        tokio::spawn(server);
}

use hyper::{client::connect::HttpConnector, Method, Client};

struct WebClient<Q, R> {
    client: Client<HttpConnector, Body>,
    url: String,
    _q: PhantomData<Q>,
    _r: PhantomData<R>
}

impl<Q: serde::Serialize, R: serde::de::DeserializeOwned> WebClient<Q, R> {
    fn new(target_port: u16) -> Self {
        Self { 
            url: format!("http://127.0.0.1:{}/invoke", target_port),
            client: Client::new(),
            _q: PhantomData, _r: PhantomData 
        }
    }

    async fn invoke(&self, q: &Q) -> Result<R> {

        let qraw = serde_json::to_vec(q)?;

        let req = Request::builder()
            .method(Method::POST)
            .uri(&self.url)
            .body(Body::from(qraw))?;

        let mut resp = self.client.request(req).await?;
        let rraw = hyper::body::to_bytes(resp.body_mut()).await?;
        let r: R = serde_json::from_slice(rraw.as_ref())?;
        Ok(r)
    }
}


const SQLP_PORT: u16 = 7001;

struct LocalContext { 
    sqlp: WebClient<SQLPQ, SQLPR>
}

impl LocalContext {
    fn new() -> Self { Self { sqlp: WebClient::new(SQLP_PORT)  }}
}

#[async_trait]
impl Context for LocalContext {

    async fn invoke_sqlp(&self, q: &SQLPQ) -> Result<SQLPR> {
        self.sqlp.invoke(q).await
    }
}

pub struct Runtime {
    cx: Arc<LocalContext>
}

impl Runtime {
    pub fn new() -> Self { Self { cx: Arc::new(LocalContext::new()) } }

    pub fn run_sqlp(&self, svc: Arc<dyn Function<SQLPQ, SQLPR>>) {
        launch_web_server(SQLP_PORT, self.cx.clone(), svc)
    }
}