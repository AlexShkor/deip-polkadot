use crate::actor::*;
use super::actor_io::*;

use rdkafka::producer::{FutureRecord, FutureProducer, future_producer::OwnedDeliveryResult};
use rdkafka::message::{ToBytes};
use rdkafka::util::Timeout;


pub const BOOTSTRAP_SERVERS: &str = "127.0.0.1:9092";
pub const TOPIC: &str = "blockchain";
pub const EVENTS_KEY: &str = "events";

pub struct MessageBrokerActor {
    producer: FutureProducer
}
impl MessageBrokerActor {
    pub fn new() -> Self {
        let mut config = rdkafka::ClientConfig::new();
        config.set("bootstrap.servers", BOOTSTRAP_SERVERS);
        let producer = config.create::<FutureProducer>().unwrap();
        Self { producer }
    }
}

pub type MessageBrokerActorInputData = String;
pub type MessageBrokerActorInput = ActorDirective<MessageBrokerActorInputData>;
pub type MessageBrokerActorOutput = OwnedDeliveryResult;
pub type MessageBrokerActorIO = ActorJack<MessageBrokerActorInput, MessageBrokerActorOutput>;

#[async_trait::async_trait]
impl Actor
<
    MessageBrokerActorInputData,
    MessageBrokerActorInput,
    MessageBrokerActorOutput,
    MessageBrokerActorIO
>
for MessageBrokerActor
{
    async fn on_input(&mut self, data: MessageBrokerActorInputData) -> MessageBrokerActorOutput {
        let record = FutureRecord::to(TOPIC)
            .key(EVENTS_KEY)
            .payload(&data);
        self.producer
            .send(record, Timeout::After(std::time::Duration::from_secs(5)))
            .await
    }
}
