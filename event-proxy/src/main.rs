mod frame;
mod events;
mod types;
mod runtime;
mod call_serializer;
mod actor;
mod app;

use std::time::Duration;

use substrate_subxt::{ClientBuilder, Client};
use substrate_subxt::NodeTemplateRuntime;
use substrate_subxt::{EventSubscription};

use tokio::sync::mpsc;

use events::*;
use types::register_types;


const URL: &str = "ws://localhost:9944/";

type RuntimeT = NodeTemplateRuntime;

use app::{
    Actor, ActorI, ActorO, ActorIO, ActorDirective,
    RpcClientBuilderActor, RpcClientBuilderActorIO,
    RpcClientStatusActor, RpcClientStatusActorIO, RpcClientStatusActorInputData, RpcClientStatusActorOutput,
    MessageBrokerActor, MessageBrokerActorIO,
};

#[tokio::main]
async fn main() {
    
    flexi_logger::Logger::try_with_env().unwrap().start().unwrap();
    
    let mut client_builder_actor = RpcClientBuilderActor;
    let (client_builder_io1, mut client_builder_io2)
        = RpcClientBuilderActorIO::pair();
    let _client_builder_task = tokio::spawn(async move {
        client_builder_actor.actor_loop(client_builder_io1).await
    });
    client_builder_io2.send(ActorDirective::Input(())).await.unwrap();
    let client = client_builder_io2.recv().await.unwrap().unwrap();
    
    let mut rpc_client_status = RpcClientStatusActor::new(client.rpc_client().clone());
    let (cs_io1, mut cs_io2) 
        = RpcClientStatusActorIO::pair();
    tokio::spawn(async move { rpc_client_status.actor_loop(cs_io1).await });
    
    let (mut cs_i2, mut cs_o2) = cs_io2.split();
    
    tokio::spawn(async move {
        loop {
            let x = cs_o2.send(RpcClientStatusActorInputData::check_disconnect()).await;
            if x.is_err() { break }
            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    });
    
    let sub = client.subscribe_finalized_events().await.unwrap();
    let events_decoder = client.events_decoder();
    let mut sub = EventSubscription::<RuntimeT>::new(
        sub,
        events_decoder
    );
    
    let mut message_broker = MessageBrokerActor::new();
    let (mb_io1, mb_io2) = MessageBrokerActorIO::pair();
    tokio::spawn(async move { message_broker.actor_loop(mb_io1).await });
    let (mut mb_io2_i, mut mb_io2_o) = mb_io2.split();
    tokio::spawn(async move {
        while let Some(delivery_status) = mb_io2_i.recv().await {
            log::debug!("{:?}", delivery_status);
        }
    });
    
    loop {
        tokio::select! {
            Some(RpcClientStatusActorOutput::Disconnected(true)) = cs_i2.recv() => { println!("DISCONNECTED"); }
            event = sub.next() => {
                match event {
                    Some(Ok(e)) => {
                        println!("EVENT");
                        log::debug!("{:?} ; {:?} ; {:?}", e.variant, e.module, e.data);
                        let k = known_events::<RuntimeT>(&e);
                        let payload = serde_json::to_string_pretty(&k).unwrap();
                        println!("{}", &payload);
                        mb_io2_o.send(ActorDirective::Input(payload)).await.unwrap();
                    },
                    Some(Err(err)) => {
                        log::error!("{}", err);
                    },
                    None => {
                        // println!("DISCONNECTED 2");
                    },
                }
            }
        };
    }
}
