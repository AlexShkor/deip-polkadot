mod frame;
mod events;
mod types;
mod runtime;
mod call_serializer;

use substrate_subxt::ClientBuilder;
use substrate_subxt::NodeTemplateRuntime;
use substrate_subxt::{EventSubscription};

use events::*;
use types::register_types;

const URL: &str = "ws://localhost:9944/";

type RuntimeT = NodeTemplateRuntime;

#[tokio::main]
async fn main() {
    
    flexi_logger::Logger::try_with_env().unwrap().start().unwrap();
    
    let client = register_types(ClientBuilder::<RuntimeT>::new())
        .set_url(URL)
        // .skip_type_sizes_check()
        .build()
        .await.unwrap();
    let sub = client.subscribe_finalized_events().await.unwrap();
    let events_decoder = client.events_decoder();
    let mut sub = EventSubscription::<RuntimeT>::new(
        sub,
        events_decoder
    );
    loop {
        while let Some(Ok(e)) = sub.next().await {
            log::debug!("{:?} ; {:?} ; {:?}", e.variant, e.module, e.data);
            let k = known_events::<RuntimeT>(&e);
            println!("{}", serde_json::to_string_pretty(&k).unwrap());
        }
    }
}
