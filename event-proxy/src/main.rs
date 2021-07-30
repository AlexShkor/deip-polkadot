mod frame;
mod events;

use substrate_subxt::ClientBuilder;
use substrate_subxt::NodeTemplateRuntime;
use substrate_subxt::{EventSubscription, EventsDecoder, Runtime, Error, Event, RawEvent};

use sp_core::hashing::twox_128;

use frame::deip_proposal::{self, DeipProposal};
use frame::deip::Deip;
use frame::deip_org::DeipOrg;

use frame_support::Parameter;
use frame_support::pallet_prelude::Member;
use substrate_subxt::system::System;

use codec::Decode;
use node_template_runtime::ProposalExpirePeriod;

use events::*;

const URL: &str = "ws://localhost:9944/";

type RuntimeT = NodeTemplateRuntime;

#[tokio::main]
async fn main() {
    
    flexi_logger::Logger::try_with_env().unwrap().start().unwrap();
    
    let client = ClientBuilder::<RuntimeT>::new()
        .set_url(URL)
        // .skip_type_sizes_check()
        // System:
        .register_type_size::<<RuntimeT as System>::AccountId>("T::AccountId")
        // DeipProposal:
        .register_type_size::<<RuntimeT as DeipProposal>::ProposalBatch>("ProposalBatch<T>")
        .register_type_size::<<RuntimeT as DeipProposal>::ProposalId>("ProposalId")
        .register_type_size::<<RuntimeT as DeipProposal>::ProposalState>("ProposalState")
        // Deip:
        .register_type_size::<<RuntimeT as Deip>::DomainId>("DomainId")
        .register_type_size::<<RuntimeT as Deip>::ProjectId>("ProjectId")
        .register_type_size::<<RuntimeT as Deip>::Project>("Project")
        .register_type_size::<<RuntimeT as Deip>::Review>("Review")
        .register_type_size::<<RuntimeT as Deip>::NdaId>("NdaId")
        .register_type_size::<<RuntimeT as Deip>::NdaAccessRequestId>("NdaAccessRequestId")
        .register_type_size::<<RuntimeT as Deip>::ProjectContentId>("ProjectContentId")
        .register_type_size::<<RuntimeT as Deip>::ProjectTokenSaleId>("ProjectTokenSaleId")
        .register_type_size::<<RuntimeT as Deip>::ProjectTokenSale>("ProjectTokenSale")
        // DeipOrg:
        .register_type_size::<<RuntimeT as DeipOrg>::Org>("OrgOf<T>")
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
            match known_events::<RuntimeT>(&e) {
                Some(ProposalProposed(e)) => {},
                Some(ProposalApproved(e)) => {},
                Some(ProposalRevokedApproval(e)) => {},
                Some(ProposalResolved(e)) => {},
                Some(ProposalExpired(e)) => {},
                None | _ => {}
            }
        }
    }
}

impl frame::deip_proposal::DeipProposal for RuntimeT {
    type ProposalBatch = pallet_deip_proposal::proposal::ProposalBatch<node_template_runtime::Runtime>;
    type ProposalId = pallet_deip_proposal::proposal::ProposalId;
    type Call = node_template_runtime::Call;
    type BatchItem = pallet_deip_proposal::proposal::ProposalBatchItemOf<node_template_runtime::Runtime>;
    type ProposalState = pallet_deip_proposal::proposal::ProposalState;
}

impl frame::deip::Deip for RuntimeT {
    type DomainId = pallet_deip::DomainId;
    type ProjectId = pallet_deip::ProjectId;
    type Project = pallet_deip::Project<Self::Hash, Self::AccountId>;
    type Review = pallet_deip::Review<Self::Hash, Self::AccountId>;
    type NdaId = pallet_deip::NdaId;
    type NdaAccessRequestId = pallet_deip::NdaAccessRequestId;
    type ProjectContentId = pallet_deip::ProjectContentId;
    type ProjectTokenSaleId = pallet_deip::InvestmentId;
    type ProjectTokenSale = pallet_deip::ProjectTokenSaleOf<node_template_runtime::Runtime>;
}

impl frame::deip_org::DeipOrg for RuntimeT {
    type Org = pallet_deip_org::org::OrgOf<node_template_runtime::Runtime>;
}
