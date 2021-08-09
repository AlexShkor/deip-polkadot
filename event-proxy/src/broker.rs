use rdkafka::producer::FutureRecord;
use rdkafka::message::ToBytes;
use rdkafka::util::Timeout;
use tokio::sync::mpsc;

pub use BrokerDirectives::*;

pub const BOOTSTRAP_SERVERS: &str = "127.0.0.1:9092";
pub const TOPIC: &str = "blockchain";
pub const EVENTS_KEY: &str = "events";

pub enum BrokerDirectives<Payload> {
    Send(Payload),
    ExitLoop,
}

pub async fn broker_loop<P: ToBytes>(
    mut control: mpsc::Receiver<BrokerDirectives<P>>,
) {
    let mut config = rdkafka::ClientConfig::new();
    config.set("bootstrap.servers", BOOTSTRAP_SERVERS);
    let producer = config.create::<rdkafka::producer::FutureProducer>().unwrap();
    while let Some(directive) = control.recv().await {
        match directive {
            BrokerDirectives::ExitLoop => break,
            BrokerDirectives::Send(payload) => {
                let record = FutureRecord::to(TOPIC)
                    .key(EVENTS_KEY)
                    .payload(&payload);
                let (partition, offset) = producer
                    .send(record, Timeout::Never)
                    .await.unwrap();
            }
        }
    }
}
