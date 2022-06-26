use planar_core::*;
use futures_lite::StreamExt;

use lapin::{
    options::*, publisher_confirm::Confirmation, types::FieldTable, BasicProperties, Connection,
    ConnectionProperties
};


pub fn publish() -> Result<()> {
    let addr = std::env::var("AMQP_ADDR").unwrap_or_else(|_| "amqp://127.0.0.1:5672/%2f".into());

    let rt = tokio::runtime::Runtime::new()?;

    rt.block_on(async {
        let conn = Connection::connect(
            &addr,
            ConnectionProperties::default(),
        )
        .await?;

        let channel = conn.create_channel().await?;

        let queue = channel
            .queue_declare(
                "hello",
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await?;
        
        let payload = b"Hello world!";

        let confirm = channel
            .basic_publish(
                "",
                "hello",
                BasicPublishOptions::default(),
                payload,
                BasicProperties::default(),
            )
            .await?
            .await?;
        Ok(())
    })

}

pub fn consume() -> Result<()> {
    let addr = std::env::var("AMQP_ADDR").unwrap_or_else(|_| "amqp://127.0.0.1:5672/%2f".into());
    let rt = tokio::runtime::Runtime::new()?;

    rt.block_on(async {
        let conn = Connection::connect(
            &addr,
            ConnectionProperties::default(),
        )
        .await?;

        let channel = conn.create_channel().await?;
        let mut consumer = channel
        .basic_consume(
            "hello",
            "my_consumer",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;
        while let Some(delivery) = consumer.next().await {
            let delivery = delivery.expect("error in consumer");
            println!("{}", String::from_utf8_lossy(&delivery.data));
            delivery
                .ack(BasicAckOptions::default())
                .await
                .expect("ack");
        }
        Ok(())
    })

}
