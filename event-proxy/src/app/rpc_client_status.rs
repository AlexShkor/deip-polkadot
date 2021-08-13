use crate::actor::*;
use super::actor_io::*;

use substrate_subxt::RpcClient;


pub struct RpcClientStatusActor(RpcClient);
impl RpcClientStatusActor {
    pub fn new(client: RpcClient) -> Self {
        Self(client)
    }
}

pub enum RpcClientStatusActorInputData {
    SetClient(RpcClient),
    CheckDisconnect
}
impl RpcClientStatusActorInputData {
    pub fn check_disconnect() -> RpcClientStatusActorInput {
        ActorDirective::Input(Self::CheckDisconnect)
    }
    pub fn set_client(client: RpcClient) -> RpcClientStatusActorInput {
        ActorDirective::Input(Self::SetClient(client))
    }
}
pub type RpcClientStatusActorInput = ActorDirective<RpcClientStatusActorInputData>;
pub enum RpcClientStatusActorOutput {
    SetClientOk,
    Disconnected(bool)
}
pub type RpcClientStatusActorIO = ActorJack<RpcClientStatusActorInput, RpcClientStatusActorOutput>;

#[async_trait::async_trait]
impl Actor
<
    RpcClientStatusActorInputData,
    RpcClientStatusActorInput,
    RpcClientStatusActorOutput,
    RpcClientStatusActorIO
>
for RpcClientStatusActor
{
    async fn on_input(&mut self, data: RpcClientStatusActorInputData) -> RpcClientStatusActorOutput {
        match data {
            RpcClientStatusActorInputData::SetClient(client) => {
                self.0 = client;
                return RpcClientStatusActorOutput::SetClientOk
            },
            RpcClientStatusActorInputData::CheckDisconnect => (),
        }
        let connected = match self.0 {
            RpcClient::WebSocket(ref ws_client) => {
                ws_client.is_connected()
            },
            _ => true,
        };
        RpcClientStatusActorOutput::Disconnected(!connected)
    }
}
