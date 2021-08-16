mod frame;
mod events;
mod types;
mod runtime;
mod call_serializer;
mod actor;
mod app;

use std::time::Duration;

use substrate_subxt::{ClientBuilder, Client, System};
use substrate_subxt::NodeTemplateRuntime;
use substrate_subxt::{EventSubscription};

use tokio::sync::mpsc;
use futures::stream::{FuturesOrdered, StreamExt};
use futures::Future;

use events::*;
use types::register_types;


const URL: &str = "ws://localhost:9944/";

type RuntimeT = NodeTemplateRuntime;

use app::{
    Actor, ActorI, ActorO, ActorIO, ActorDirective,
    ActorJackPair, ActorJackI, ActorJackO,
    RpcClientBuilderActor, RpcClientBuilderActorIO, RpcClientBuilderActorIOPair, RpcClientBuilderActorInput, RpcClientBuilderActorOutput,
    RpcClientStatusActor, RpcClientStatusActorIO, RpcClientStatusActorInputData, RpcClientStatusActorOutput,
    MessageBrokerActor, MessageBrokerActorIO, MessageBrokerActorInput, MessageBrokerActorIOPair, MessageBrokerActorOutput, MessageBrokerActorInputData,
    BlockchainActor, BlockchainActorIO, BlockchainActorInputData, BlockchainActorOutput, BlockchainActorInput, BlockchainActorIOPair, FinalizedBlocksSubscription, BlockchainActorOutputData,
};
use crate::app::ActorJack;

#[tokio::main]
async fn main() {
    
    flexi_logger::Logger::try_with_env().unwrap().start().unwrap();
    
    // Init rpc-client-builder-actor:
    let mut rpc_client_builder_actor = RpcClientBuilderActor;
    let (rpc_client_builder_actor_io, rpc_client_builder_actor_io2) = RpcClientBuilderActorIO::pair();
    tokio::spawn(async move { rpc_client_builder_actor.actor_loop(rpc_client_builder_actor_io).await });
    
    // Init blockchain-actor:
    let mut blockchain_actor = BlockchainActor::new();
    let (blockchain_actor_io, blockchain_actor_io2) = BlockchainActorIO::pair();
    tokio::spawn(async move { blockchain_actor.actor_loop(blockchain_actor_io).await });
    
    // Init message-broker-actor:
    let mut message_broker_actor = MessageBrokerActor::new();
    let (message_broker_actor_io, message_broker_actor_io2) = MessageBrokerActorIO::pair();
    tokio::spawn(async move { message_broker_actor.actor_loop(message_broker_actor_io).await });
    
    let mut subscription_task_queue = FuturesOrdered::new();
    
    let mut blockchain_actor_task_queue = FuturesOrdered::new();
    let mut message_broker_actor_task_queue = FuturesOrdered::new();
    let mut rpc_client_builder_actor_task_queue = FuturesOrdered::new();
    
    let mut released_blockchain_actor_queue = released_actor_queue::<_, _, BlockchainActorIO>();
    let mut released_message_broker_actor_queue = released_actor_queue::<_, _, MessageBrokerActorIO>();
    let mut released_rpc_client_builder_actor_queue = released_actor_queue::<_, _, RpcClientBuilderActorIO>();

    release_actor(blockchain_actor_io2, &mut released_blockchain_actor_queue).await;
    release_actor(message_broker_actor_io2, &mut released_message_broker_actor_queue).await;
    release_actor(rpc_client_builder_actor_io2, &mut released_rpc_client_builder_actor_queue).await;
    
    // Put the initial task to trigger main workflow:
    blockchain_actor_task_queue.push(init_actor_task::<_, _, BlockchainActorIO>(
            BlockchainActorInputData::subscribe_finalized_blocks(),
            &mut released_blockchain_actor_queue
    ).await);
    
    loop {
        tokio::select! {
            Some(subscription_task_result) = subscription_task_queue.next() => {
                let (maybe_finalized_block_header, subscription) = subscription_task_result;
                // println!("!!!!!!!!!!!!!!!!, {:?}", maybe_finalized_block_header);
                match maybe_finalized_block_header {
                    Ok(Some(finalized_block_header)) => {
                        blockchain_actor_task_queue.push(init_actor_task::<_, _, BlockchainActorIO>(
                            BlockchainActorInputData::get_block_hash(finalized_block_header),
                            &mut released_blockchain_actor_queue
                        ).await);
                        subscription_task_queue.push(subscription_task(subscription));
                    },
                    err => {
                        match err {
                            Ok(Some(_)) => unreachable!(),
                            Ok(None) => { log::error!("Subscription termination unexpected"); },
                            Err(e) => { log::error!("{}", e); },
                        }
                        rpc_client_builder_actor_task_queue.push(init_actor_task::<_, _, RpcClientBuilderActorIO>(
                            RpcClientBuilderActorInput::Input(()),
                            &mut released_rpc_client_builder_actor_queue
                        ).await);
                    },
                }
            },
            Some(blockchain_actor_task_result) = blockchain_actor_task_queue.next() => {
                let (output, io) = blockchain_actor_task_result;
                release_actor(io, &mut released_blockchain_actor_queue).await;
                match output {
                    None => { unreachable!(); },
                    Some(BlockchainActorOutput::NoClient) => {
                        rpc_client_builder_actor_task_queue.push(init_actor_task::<_, _, RpcClientBuilderActorIO>(
                            RpcClientBuilderActorInput::Input(()),
                            &mut released_rpc_client_builder_actor_queue
                        ).await);
                    },
                    Some(BlockchainActorOutput::Ok(BlockchainActorOutputData::SubscribeFinalizedBlocks(maybe_subscription))) => {
                        match maybe_subscription {
                            Ok(subscription) => {
                                subscription_task_queue.push(subscription_task(subscription));
                            },
                            Err(e) => { unimplemented!(); },
                        }
                    },
                    Some(BlockchainActorOutput::Ok(BlockchainActorOutputData::GetBlockHash(maybe_hash))) => {
                        match maybe_hash {
                            Ok(maybe_hash) => {
                                let hash = maybe_hash.expect("EXISTENT BLOCK");
                                blockchain_actor_task_queue.push(init_actor_task::<_, _, BlockchainActorIO>(
                                    BlockchainActorInputData::get_block(hash),
                                    &mut released_blockchain_actor_queue
                                ).await);
                            },
                            Err(e) => { unimplemented!(); }
                        }
                    },
                    Some(BlockchainActorOutput::Ok(BlockchainActorOutputData::GetBlock(maybe_block))) => {
                        match maybe_block {
                            Ok(maybe_block) => {
                                let block = maybe_block.expect("EXISTENT BLOCK");
                                println!("BLOCK !!!!!!!!!!!!!!!!, {:?}", &block);
                                let payload = serde_json::to_string_pretty(&block).unwrap();
                                message_broker_actor_task_queue.push(init_actor_task::<_, _, MessageBrokerActorIO>(
                                    MessageBrokerActorInput::Input(payload),
                                    &mut released_message_broker_actor_queue
                                ).await);
                            },
                            Err(e) => { unimplemented!(); }
                        }
                    },
                    Some(BlockchainActorOutput::Ok(BlockchainActorOutputData::SetClient)) => {
                        blockchain_actor_task_queue.push(init_actor_task::<_, _, BlockchainActorIO>(
                            BlockchainActorInputData::subscribe_finalized_blocks(),
                            &mut released_blockchain_actor_queue
                        ).await);
                    },
                }
            },
            Some(message_broker_actor_task_result) = message_broker_actor_task_queue.next() => {
                let (output, io) = message_broker_actor_task_result;
                release_actor(io, &mut released_message_broker_actor_queue).await;
                log::debug!("DELIVERY STATUS: {:?}", output);
            },
            Some(rpc_client_builder_actor_task_result) = rpc_client_builder_actor_task_queue.next() => {
                let (output, io) = rpc_client_builder_actor_task_result;
                release_actor(io, &mut released_rpc_client_builder_actor_queue).await;
                match output {
                    Some(Ok(client)) => {
                        blockchain_actor_task_queue.push(init_actor_task::<_, _, BlockchainActorIO>(
                            BlockchainActorInputData::set_client(client),
                            &mut released_blockchain_actor_queue
                        ).await);
                    },
                    Some(Err(e)) => {
                        log::error!("{}", e);
                        tokio::time::sleep(Duration::from_secs(5)).await;
                        rpc_client_builder_actor_task_queue.push(init_actor_task::<_, _, RpcClientBuilderActorIO>(
                            RpcClientBuilderActorInput::Input(()),
                            &mut released_rpc_client_builder_actor_queue
                        ).await);
                    },
                    None => { unreachable!(); }
                }
            },
        };
    }
    
    
    
    // let header = sub.next().await.unwrap().unwrap();
    // let block = fetch_block(header.number, &mut b_io2).await;
    // println!("BLOCK: {:?}", &block);
    // let payload = serde_json::to_string_pretty(&block).unwrap();
    // println!("{}", &payload);
    // mb_o2.send(ActorDirective::Input(payload)).await.unwrap();
    // 
    // let sub = client.subscribe_finalized_events().await.unwrap();
    // let events_decoder = client.events_decoder();
    // let mut sub = EventSubscription::<RuntimeT>::new(
    //     sub,
    //     events_decoder
    // );
}

type ReleasedActorQueue<T> = (mpsc::Sender<T>, mpsc::Receiver<T>);

fn released_actor_queue<I, O, IO: ActorIO<I, O>>() -> ReleasedActorQueue<IO::Pair> { mpsc::channel(1) }

async fn release_actor<T>(io: T, q: &mut ReleasedActorQueue<T>) {
    if q.0.send(io).await.is_err() {
        panic!("NEVER GONE");
    }
}

async fn wait_released_actor<T>(q: &mut ReleasedActorQueue<T>) -> T {
    match q.1.recv().await {
        Some(x) => x,
        _ => panic!("NEVER GONE"),
    }
} 

type FinalizedBlocksSubscriptionItem = Result<Option<<RuntimeT as System>::Header>, jsonrpsee_ws_client::Error>;

async fn subscription_task(mut subscription: FinalizedBlocksSubscription)
    -> (FinalizedBlocksSubscriptionItem, FinalizedBlocksSubscription)
{
    (subscription.next().await, subscription)
}

async fn actor_task<I, O, IO>(input: I, mut io: IO::Pair) -> (Option<O>, IO::Pair)
    where 
        I: Send, O: Send, IO: ActorIO<I, O>
{
    if io.send(input).await.is_err() { return (None, io) }
    (io.recv().await, io)
}

async fn init_actor_task<I: Send, O: Send, IO: ActorIO<I, O>>(
    input: I,
    io: &mut ReleasedActorQueue<IO::Pair>
)
    -> impl Future<Output = (Option<O>, IO::Pair)>
{
    actor_task::<I, O, IO>(input, wait_released_actor(io).await)
}
