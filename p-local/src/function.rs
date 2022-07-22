
use planar_core::*;
use pal::function::*;
use std::{sync::Arc, marker::PhantomData};
use std::convert::Infallible;
use std::fmt::Debug;

use hyper::{Body, Request, Response, Server, Method, Client, client::connect::HttpConnector};
use hyper::service::{make_service_fn, service_fn};
use tracing::{info, trace, info_span, Instrument};

pub (crate) fn launch_web_server<Q, R>(port: u16, cx: Arc<dyn Context>, svc: Arc<dyn Function<Q, R>>)
-> tokio::task::JoinHandle<Result<()>> 
where 
    Q: serde::de::DeserializeOwned + Debug + Send + 'static, 
    R: serde::Serialize + Debug + 'static {
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
                        trace!(what="q", data=?q);
                        let r = svc.invoke(cx.as_ref(), q).await?;
                        trace!(what="r", data=?r);
                        let rraw = serde_json::to_vec(&r)?;
                        Ok::<_, Error>(Response::new(Body::from(rraw)))
                    }
                }))
            }
        });
        
        // Then bind and serve...
        let server = Server::bind(&addr)
            .serve(make_svc)
            .instrument(info_span!("webserver", port=%port))
            ;

        //convert error type
        let server = async { match server.await {
                Ok(()) => Ok(()),
                Err(e) => Err(e.into())
            }
        };

        info!(msg="web server startup", port=%port);
        
        tokio::spawn(server)
}

pub(crate) struct WebClient<Q, R> {
    client: Client<HttpConnector, Body>,
    url: String,
    _q: PhantomData<Q>,
    _r: PhantomData<R>
}

impl<Q: serde::Serialize, R: serde::de::DeserializeOwned> WebClient<Q, R> {
    pub(crate) fn new(target_port: u16) -> Self {
        Self { 
            url: format!("http://127.0.0.1:{}/invoke", target_port),
            client: Client::new(),
            _q: PhantomData, _r: PhantomData 
        }
    }

    pub(crate) async fn invoke(&self, q: &Q) -> Result<R> {

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


