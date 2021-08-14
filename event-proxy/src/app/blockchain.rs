use crate::actor::*;
use super::actor_io::*;

use substrate_subxt::{Client, ChainBlock};
use substrate_subxt::System;
use jsonrpsee_ws_client::Subscription;

use crate::RuntimeT;

pub struct BlockchainActor {
    client: Option<Client<RuntimeT>>,
}
impl BlockchainActor {
    pub fn new() -> Self {
        Self { client: None }
    }
}

pub enum BlockchainActorInputData {
    SubscribeFinalizedBlocks,
    GetBlockHash(<RuntimeT as System>::BlockNumber),
    GetBlock(<RuntimeT as System>::Hash),
    SetClient(Client<RuntimeT>)
}
impl BlockchainActorInputData {
    pub fn subscribe_finalized_blocks() -> BlockchainActorInput {
        ActorDirective::Input(Self::SubscribeFinalizedBlocks)
    }
    pub fn get_block_hash(number: <RuntimeT as System>::Header) -> BlockchainActorInput {
        ActorDirective::Input(Self::GetBlockHash(number.number))
    }
    pub fn get_block(hash: <RuntimeT as System>::Hash) -> BlockchainActorInput {
        ActorDirective::Input(Self::GetBlock(hash))
    }
    pub fn set_client(client: Client<RuntimeT>) -> BlockchainActorInput {
        ActorDirective::Input(Self::SetClient(client))
    }
}
pub type BlockchainActorInput = ActorDirective<BlockchainActorInputData>;
pub type FinalizedBlocksSubscription = Subscription<<RuntimeT as System>::Header>;
pub enum BlockchainActorOutput {
    NoClient,
    Ok(BlockchainActorOutputData)
}
pub enum BlockchainActorOutputData {
    SubscribeFinalizedBlocks(Result<FinalizedBlocksSubscription, substrate_subxt::Error>),
    GetBlockHash(Result<Option<<RuntimeT as System>::Hash>, substrate_subxt::Error>),
    GetBlock(Result<Option<ChainBlock<RuntimeT>>, substrate_subxt::Error>),
    SetClient,
}
pub type BlockchainActorIO = ActorJack<BlockchainActorInput, BlockchainActorOutput>;
pub type BlockchainActorIOPair = ActorJackPair<BlockchainActorIO, BlockchainActorInput, BlockchainActorOutput>;

#[async_trait::async_trait]
impl Actor
<
    BlockchainActorInputData,
    BlockchainActorInput,
    BlockchainActorOutput,
    BlockchainActorIO
>
for BlockchainActor
{
    async fn on_input(&mut self, data: BlockchainActorInputData) -> BlockchainActorOutput {
        if self.client.is_none() {
            return if let BlockchainActorInputData::SetClient(c) = data {
                let _ = self.client.replace(c);
                BlockchainActorOutput::Ok(BlockchainActorOutputData::SetClient)
            } else { 
                BlockchainActorOutput::NoClient
            };
        }
        let client = self.client.as_mut().unwrap();
        let output = match data {
            BlockchainActorInputData::SubscribeFinalizedBlocks => {
                BlockchainActorOutputData::SubscribeFinalizedBlocks(
                    client.subscribe_finalized_blocks().await)
            },
            BlockchainActorInputData::GetBlockHash(number) => {
                BlockchainActorOutputData::GetBlockHash(
                    client.block_hash(Some(number.into())).await)
            },
            BlockchainActorInputData::GetBlock(hash) => {
                BlockchainActorOutputData::GetBlock(
                    client.block(Some(hash)).await)
            },
            BlockchainActorInputData::SetClient(c) => {
                let _ = std::mem::replace(client, c);
                BlockchainActorOutputData::SetClient
            },
        };
        BlockchainActorOutput::Ok(output)
    }
}
