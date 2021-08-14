use crate::actor::*;
use super::actor_io::*;

use substrate_subxt::{Client, ClientBuilder};

use crate::RuntimeT;
use crate::types::register_types;
use crate::URL;


pub struct RpcClientBuilderActor;

pub type RpcClientBuilderActorInputData = ();
pub type RpcClientBuilderActorInput = ActorDirective<RpcClientBuilderActorInputData>;
pub type RpcClientBuilderActorOutput = Result<Client<RuntimeT>, substrate_subxt::Error>;
pub type RpcClientBuilderActorIO = ActorJack<RpcClientBuilderActorInput, RpcClientBuilderActorOutput>;
pub type RpcClientBuilderActorIOPair = ActorJackPair<RpcClientBuilderActorIO, RpcClientBuilderActorInput, RpcClientBuilderActorOutput>;

#[async_trait::async_trait]
impl Actor
<
    RpcClientBuilderActorInputData,
    RpcClientBuilderActorInput,
    RpcClientBuilderActorOutput,
    RpcClientBuilderActorIO
>
for RpcClientBuilderActor
{
    async fn on_input(&mut self, _data: RpcClientBuilderActorInputData) -> RpcClientBuilderActorOutput {
        register_types(ClientBuilder::<RuntimeT>::new())
            .set_url(URL)
            // .skip_type_sizes_check()
            .build()
            .await
            .map_err(|e| { log::error!("{:?}", &e); e })
    }
}
