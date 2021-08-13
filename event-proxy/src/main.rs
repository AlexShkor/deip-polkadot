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

use events::*;
use types::register_types;


const URL: &str = "ws://localhost:9944/";

type RuntimeT = NodeTemplateRuntime;

use app::{
    Actor, ActorI, ActorO, ActorIO, ActorDirective,
    ActorJackPair, ActorJackI, ActorJackO,
    RpcClientBuilderActor, RpcClientBuilderActorIO,
    RpcClientStatusActor, RpcClientStatusActorIO, RpcClientStatusActorInputData, RpcClientStatusActorOutput,
    MessageBrokerActor, MessageBrokerActorIO, MessageBrokerActorInput, MessageBrokerActorIOPair, MessageBrokerActorOutput, MessageBrokerActorInputData,
    BlockchainActor, BlockchainActorIO, BlockchainActorInputData, BlockchainActorOutput, BlockchainActorInput, BlockchainActorIOPair, FinalizedBlocksSubscription,
};
use crate::app::ActorJack;

#[tokio::main]
async fn main() {
    
    flexi_logger::Logger::try_with_env().unwrap().start().unwrap();
    
    // Init rpc-client-builder-actor:
    let mut client_builder_actor = RpcClientBuilderActor;
    let (cb_io, mut cb_io2)
        = RpcClientBuilderActorIO::pair();
    tokio::spawn(async move {
        client_builder_actor.actor_loop(cb_io).await
    });
    
    // Get rpc-client:
    cb_io2.send(ActorDirective::Input(())).await.unwrap();
    let client = cb_io2.recv().await.unwrap().unwrap();
    
    // Init rpc-client-status-actor:
    let mut rpc_client_status = RpcClientStatusActor::new(client.rpc_client().clone());
    let (cs_io, mut cs_io2) 
        = RpcClientStatusActorIO::pair();
    tokio::spawn(async move {
        rpc_client_status.actor_loop(cs_io).await
    });
    
    // Spawn check_disconnect periodic for rpc-client-status-actor:
    let (mut cs_i2, mut cs_o2) = cs_io2.split();
    // tokio::spawn(async move {
    //     loop {
    //         let x = cs_o2.send(RpcClientStatusActorInputData::check_disconnect()).await;
    //         if x.is_err() { break }
    //         tokio::time::sleep(Duration::from_secs(5)).await;
    //     }
    // });
    
    // Init blockchain-actor:
    let mut blockchain = BlockchainActor::new(client);
    let (b_io, mut b_io2) = BlockchainActorIO::pair();
    tokio::spawn(async move {
        blockchain.actor_loop(b_io).await
    });
    
    // let (mut b_i2, mut b_o2) = b_io2.split();
    
    // Get block-subscription:
    // b_io2.send(BlockchainActorInputData::subscribe_finalized_blocks()).await.unwrap();
    // let subscription = b_io2.recv().await.unwrap();
    // let mut sub = match subscription {
    //     BlockchainActorOutput::SubscribeFinalizedBlocks(Ok(s)) => s,
    //     _ => unreachable!(),
    // };
    
    // Init message-broker-actor:
    let mut message_broker = MessageBrokerActor::new();
    let (mb_io, mb_io2) = MessageBrokerActorIO::pair();
    tokio::spawn(async move {
        message_broker.actor_loop(mb_io).await
    });
    
    // Spawn delivery_status reader for message-broker-actor:
    // let (mut mb_i2, mut mb_o2) = mb_io2.split();
    // tokio::spawn(async move {
    //     while let Some(delivery_status) = mb_i2.recv().await {
    //         log::debug!("{:?}", delivery_status);
    //     }
    // });
    
    let mut subscription_task_queue = FuturesOrdered::new();
    let mut blockchain_actor_task_queue = FuturesOrdered::new();
    let mut message_broker_actor_task_queue = FuturesOrdered::new();
    
    let mut released_blockchain_actor_queue = mpsc::channel(1);
    let mut released_message_broker_actor_queue = mpsc::channel(1);
    
    blockchain_actor_task_queue.push(
        actor_task::<
            BlockchainActorInput,
            BlockchainActorOutput,
            BlockchainActorIO
        >(BlockchainActorInputData::subscribe_finalized_blocks(), b_io2));
    
    release_actor(mb_io2, &released_message_broker_actor_queue).await;

    loop {
        tokio::select! {
            maybe_client = cb_io2.recv() => {
                match maybe_client {
                    Some(Ok(client)) => {},
                    Some(Err(e)) => {},
                    None => {},
                }
            },
            maybe_send = cs_o2.send(RpcClientStatusActorInputData::check_disconnect()) => {
                match maybe_send {
                    Ok(_) => {},
                    Err(_) => {},
                }
            },
            maybe_status = cs_i2.recv() => {
                match maybe_status {
                    Some(RpcClientStatusActorOutput::Disconnected(true)) => {},
                    Some(_) => {},
                    None => {},
                }
            },
            // maybe_send = b_o2.send(BlockchainActorInputData::subscribe_finalized_blocks()) => {
            //     match maybe_send {
            //         Ok(_) => {},
            //         Err(_) => {},
            //     }
            // },
            // maybe_subscribe = b_i2.recv() => {
            //     match maybe_subscribe {
            //         Some(BlockchainActorOutput::SubscribeFinalizedBlocks(Ok(subscription))) => {
            //             subscription_task_queue.push(subscription_task(subscription));
            //         },
            //         Some(BlockchainActorOutput::SubscribeFinalizedBlocks(Err(e))) => {},
            //         Some(_) => {},
            //         None => {},
            //     }
            // },
            // maybe_delivery = mb_i2.recv() => {
            //     match maybe_delivery {
            //         Some(delivery) => {},
            //         None => {},
            //     }
            // },
            // maybe_send = mb_o2.send(ActorDirective::Input(payload)) => {
            //     match maybe_send {
            //         Ok(_) => {},
            //         Err(_) => {}
            //     }
            // },
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
                release_actor(io, &released_blockchain_actor_queue).await;
                match output {
                    Some(BlockchainActorOutput::SubscribeFinalizedBlocks(maybe_subscription)) => {
                        match maybe_subscription {
                            Ok(subscription) => {
                                subscription_task_queue.push(subscription_task(subscription));
                            },
                            Err(e) => { unimplemented!(); },
                        }
                    },
                    Some(BlockchainActorOutput::GetBlockHash(maybe_hash)) => {
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
                    Some(BlockchainActorOutput::GetBlock(maybe_block)) => {
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
                    None => { unimplemented!(); },
                }
            },
            Some(message_broker_actor_task_result) = message_broker_actor_task_queue.next() => {
                let (output, io) = message_broker_actor_task_result;
                log::debug!("DELIVERY STATUS: {:?}", output);
                release_actor(io, &released_message_broker_actor_queue).await;
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

async fn release_actor<T>(io: T, q: &ReleasedActorQueue<T>) {
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
    io.send(input).await.unwrap();
    (io.recv().await, io)
}
