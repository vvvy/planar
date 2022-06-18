
use planar_core::*;
use pal::function::*;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::json;
use warp::{Filter, body, reply, path};
use std::{sync::Arc, marker::PhantomData};


fn launch_server<Q, R>(port: u16, svc: Arc<dyn FunctionServer<Q, R>>) where 
    Q: serde::de::DeserializeOwned + Send + 'static, 
    R: serde::Serialize + 'static {
    let route = path("invoke")
        .and(body::json())
        .map(move |q| match svc.invoke(q) { 
            Ok(w) => reply::json(&w),
            Err(e) => reply::json(&json!({"error": e.to_string()}))
        });
    let server = warp::serve(route);
    tokio::spawn(async move { server.run(([127, 0, 0, 1], port)) });
}

struct LocalFunctionClient<Q, R> {
    url: String,
    _q: PhantomData<Q>,
    _r: PhantomData<R>
}

impl<Q, R> LocalFunctionClient<Q, R> {
    fn new(target_port: u16) -> Self {
        Self { 
            url: format!("http://127.0.0.1:{}/invoke", target_port),
            _q: PhantomData, _r: PhantomData 
        }
    }
}

impl<Q: Serialize, R: DeserializeOwned> FunctionClient<Q, R> for LocalFunctionClient<Q, R> {
    fn invoke(&self, q: Q) -> Result<R> {
        let w = reqwest::blocking::Client::new()
            .post(&self.url)
            .json(&q)
            .send()?;
        let w = w.json()?;
        Ok(w)
    }
}


pub struct FunctionRuntimeImpl;

impl FunctionRuntime for FunctionRuntimeImpl {
    fn register_sqlp(&self, svc: Arc<dyn FunctionServer<SQLPQ, SQLPR>>) {
        launch_server(7001, svc)
    }

    fn get_sqlp(&self) -> Box<dyn FunctionClient<SQLPQ, SQLPR>> {
        Box::new(LocalFunctionClient::new(7001))
    }
}