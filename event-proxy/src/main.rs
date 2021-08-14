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
    let mut client_builder_actor = RpcClientBuilderActor;
    let (cb_io, mut cb_io2)
        = RpcClientBuilderActorIO::pair();
    tokio::spawn(async move { client_builder_actor.actor_loop(cb_io).await });
    
    // Init rpc-client-status-actor:
    // let mut rpc_client_status = RpcClientStatusActor::new(client.rpc_client().clone());
    // let (cs_io, mut cs_io2) 
    //     = RpcClientStatusActorIO::pair();
    // tokio::spawn(async move {
    //     rpc_client_status.actor_loop(cs_io).await
    // });
    
    // Init blockchain-actor:
    let mut blockchain = BlockchainActor::new();
    let (b_io, mut b_io2) = BlockchainActorIO::pair();
    tokio::spawn(async move { blockchain.actor_loop(b_io).await });
    
    // Init message-broker-actor:
    let mut message_broker = MessageBrokerActor::new();
    let (mb_io, mb_io2) = MessageBrokerActorIO::pair();
    tokio::spawn(async move { message_broker.actor_loop(mb_io).await });
    
    let mut subscription_task_queue = FuturesOrdered::new();
    
    let mut blockchain_actor_task_queue = FuturesOrdered::new();
    let mut message_broker_actor_task_queue = FuturesOrdered::new();
    let mut rpc_client_builder_actor_task_queue = FuturesOrdered::new();
    
    let mut released_blockchain_actor_queue = released_actor_queue::<BlockchainActorIOPair>();
    let mut released_message_broker_actor_queue = released_actor_queue::<MessageBrokerActorIOPair>();
    let mut released_rpc_client_builder_actor_queue = released_actor_queue::<RpcClientBuilderActorIOPair>();
    
    release_actor(mb_io2, &mut released_message_broker_actor_queue).await;
    release_actor(cb_io2, &mut released_rpc_client_builder_actor_queue).await;
    
    blockchain_actor_task_queue.push(
        actor_task::<
            BlockchainActorInput,
            BlockchainActorOutput,
            BlockchainActorIO
        >(BlockchainActorInputData::subscribe_finalized_blocks(), b_io2));
    
    loop {
        tokio::select! {
            Some(subscription_task_result) = subscription_task_queue.next() => {
                let (maybe_finalized_block_header, subscription) = subscription_task_result;
                // println!("!!!!!!!!!!!!!!!!, {:?}", maybe_finalized_block_header);
                match maybe_finalized_block_header {
                    Ok(Some(finalized_block_header)) => {
                        let blockchain_actor_io = wait_released_actor(&mut released_blockchain_actor_queue).await;
                        blockchain_actor_task_queue.push(
                            actor_task::<
                                BlockchainActorInput,
                                BlockchainActorOutput,
                                BlockchainActorIO
                            >(BlockchainActorInputData::get_block_hash(finalized_block_header), blockchain_actor_io));
                    },
                    Ok(None) => {
                        // Subscription terminated
                        unimplemented!();
                    },
                    Err(e) => { unimplemented!(); },
                }
                subscription_task_queue.push(subscription_task(subscription));
            },
            Some(blockchain_actor_task_result) = blockchain_actor_task_queue.next() => {
                let (output, io) = blockchain_actor_task_result;
                release_actor(io, &mut released_blockchain_actor_queue).await;
                match output {
                    None => { unimplemented!(); },
                    Some(BlockchainActorOutput::NoClient) => {
                        let rpc_client_builder_actor_io = wait_released_actor(&mut released_rpc_client_builder_actor_queue).await;
                        rpc_client_builder_actor_task_queue.push(
                            actor_task::<
                                RpcClientBuilderActorInput,
                                RpcClientBuilderActorOutput,
                                RpcClientBuilderActorIO
                            >(RpcClientBuilderActorInput::Input(()), rpc_client_builder_actor_io)
                        );
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
                                let blockchain_actor_io = wait_released_actor(&mut released_blockchain_actor_queue).await;
                                blockchain_actor_task_queue.push(
                                    actor_task::<
                                        BlockchainActorInput,
                                        BlockchainActorOutput,
                                        BlockchainActorIO
                                    >(BlockchainActorInputData::get_block(hash), blockchain_actor_io));
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
                                let message_broker_actor_io = wait_released_actor(&mut released_message_broker_actor_queue).await;
                                message_broker_actor_task_queue.push(
                                    actor_task::<
                                        MessageBrokerActorInput,
                                        MessageBrokerActorOutput,
                                        MessageBrokerActorIO
                                    >(MessageBrokerActorInput::Input(payload), message_broker_actor_io)
                                );
                            },
                            Err(e) => { unimplemented!(); }
                        }
                    },
                    Some(BlockchainActorOutput::Ok(BlockchainActorOutputData::SetClient)) => {
                        let blockchain_actor_io = wait_released_actor(&mut released_blockchain_actor_queue).await;
                        blockchain_actor_task_queue.push(
                            actor_task::<
                                BlockchainActorInput,
                                BlockchainActorOutput,
                                BlockchainActorIO
                            >(BlockchainActorInputData::subscribe_finalized_blocks(), blockchain_actor_io)
                        );
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
                        let blockchain_actor_io = wait_released_actor(&mut released_blockchain_actor_queue).await; 
                        blockchain_actor_task_queue.push(
                            actor_task::<
                                BlockchainActorInput,
                                BlockchainActorOutput,
                                BlockchainActorIO
                            >(BlockchainActorInputData::set_client(client), blockchain_actor_io)
                        );
                    },
                    Some(Err(e)) => { unimplemented!(); },
                    None => { unimplemented!(); }
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

fn released_actor_queue<T>() -> ReleasedActorQueue<T> { mpsc::channel::<T>(1) }

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

async fn actor_task<I: Send, O: Send, IO>(input: I, mut io: IO::Pair) -> (Option<O>, IO::Pair)
    where 
        IO: ActorIO<I, O, ActorJackI<I>, ActorJackO<O>, ActorJackI<O>, ActorJackO<I>>
{
    if io.send(input).await.is_err() { return (None, io) }
    (io.recv().await, io)
}
