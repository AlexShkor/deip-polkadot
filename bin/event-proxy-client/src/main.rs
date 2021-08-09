use rdkafka::consumer::Consumer;
use futures::stream::{StreamExt};
use rdkafka::Message;

pub const BOOTSTRAP_SERVERS: &str = "127.0.0.1:9092";
pub const TOPIC: &str = "blockchain";
pub const EVENTS_KEY: &str = "events";
pub const GROUP_ID: &str = "offchain";

#[tokio::main]
async fn main() {
    let mut config = rdkafka::ClientConfig::new();
    config.set("bootstrap.servers", BOOTSTRAP_SERVERS);
    config.set("group.id", GROUP_ID);
    let consumer = config.create::<rdkafka::consumer::StreamConsumer>().unwrap();
    consumer.subscribe(&[TOPIC]).unwrap();
    let mut stream = consumer.stream();
    while let Some(m) = stream.next().await {
        println!("{:?}", m);
        if let Ok(ok) = m {
            let owned = ok.detach();
            println!(
                "key: {:?} ; topic: {:?} ; offset {:?} ; payload: {:?}",
                owned.key().map(String::from_utf8_lossy),
                owned.topic(),
                owned.offset(),
                owned.payload().map(String::from_utf8_lossy)
            );
        }
    }
}
