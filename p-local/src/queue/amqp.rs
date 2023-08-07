use planar_core::*;
use pal::function::*;
use futures_lite::StreamExt;
use serde::{Serialize, de::DeserializeOwned};
use std::{sync::Arc, marker::PhantomData};
use tracing::{error, trace, info_span, Instrument};
use lapin::{
    options::*, types::FieldTable, BasicProperties, Connection, Channel,
    message::Delivery, 
    ConnectionProperties
};

struct ClientBase {
    channel: Channel
}

impl ClientBase {
    async fn new(queue: &str) -> Result<Self> {
        let addr = "amqp://127.0.0.1:5672/%2f";
        let conn = Connection::connect(
            addr,
            ConnectionProperties::default(),
        )
        .await?;
        let channel = conn.create_channel().await?;
        let _ = channel
        .queue_declare(
            queue,
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await?;
        Ok(Self { channel })
    }
}


pub struct Publisher<Q> {
    client: ClientBase,
    queue: String,
    _q: PhantomData<Q>
}


impl<Q: Serialize + std::fmt::Debug> Publisher<Q> {
    pub async fn new(queue: String) -> Result<Self> {
        let client = ClientBase::new(&queue).await?;
        let _q = PhantomData;
        Ok(Self { client, queue, _q})
    }

    async fn publish_(&self, q: &Q) -> Result<()> {
        trace!(queue=?self.queue, event="publish", data=?q);
        let payload = serde_json::to_vec(q)?;
        let confirm = self.client.channel
            .basic_publish(
                "",
                &self.queue,
                BasicPublishOptions::default(),
                &payload,
                BasicProperties::default(),
            )
            .await?
            .await?;
        if confirm.is_nack() {
            error!(queue=?self.queue, event="confirm NACK", message=?confirm.take_message())
        }
        Ok(())
    }

    pub async fn publish(&self, q: &Q) -> Result<()> {
        let span = info_span!("publish", queue=?self.queue);
        self.publish_(q).instrument(span).await
    }
}


pub struct Consumer<Q> {
    client: ClientBase,
    queue: String,
    cx: Arc<dyn Context>,
    svc: Arc<dyn Function<Q, ()>>,
    _q: PhantomData<Q>
}

impl<Q: DeserializeOwned + Send + Sync + core::fmt::Debug + 'static> Consumer<Q> {
    pub async fn new(queue: String, cx: Arc<dyn Context>, svc: Arc<dyn Function<Q, ()>>) -> Result<Self> {
        let client = ClientBase::new(&queue).await?;
        let _q = PhantomData;
        Ok(Self { client, queue, cx, svc, _q})
    }
    async fn deliver(&self, delivery: Result<Delivery, lapin::Error>) -> Result<()> {
        let delivery = delivery?;
        delivery
            .ack(BasicAckOptions::default())
            .await?;
        let q: Q = serde_json::from_slice(&delivery.data)?; //TODO: must not return on error?
        trace!(event="deliver", data=?q);
        self.svc.invoke(self.cx.as_ref(), q).await?;
        Ok(())
    }

    pub async fn consume(&self) -> Result<()> {
        let mut consumer = self.client.channel
        .basic_consume(
            &self.queue,
            "my_consumer",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;
        while let Some(delivery) = consumer.next().await {
            match self.deliver(delivery).await {
                Ok(()) => (),
                Err(e) => error!(event="deliver-error", error=?e)
            }
        }
        Ok(())
    }

    pub async fn run_consumer(queue: String, cx: Arc<dyn Context>, svc: Arc<dyn Function<Q, ()>>) -> Result<()> {
        let span = info_span!("run_consumer",queue=%queue);
        Self::new(queue, cx, svc).await?
            .consume()
            .instrument(span)
            .await
    }

    pub fn spawn_consumer(queue: String, cx: Arc<dyn Context>, svc: Arc<dyn Function<Q, ()>>) 
    -> tokio::task::JoinHandle<Result<()>>{
        tokio::spawn(Self::run_consumer(queue, cx, svc))
    }

}
