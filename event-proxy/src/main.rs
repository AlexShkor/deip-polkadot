mod frame;
mod events;
mod types;

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

use codec::{Decode, Encode};
use node_template_runtime::ProposalExpirePeriod;

use events::*;
use types::register_types;

use serde::{Serialize, ser::{Serializer, SerializeMap}};

const URL: &str = "ws://localhost:9944/";

type RuntimeT = NodeTemplateRuntime;

#[tokio::main]
async fn main() {
    
    flexi_logger::Logger::try_with_env().unwrap().start().unwrap();
    
    let client = register_types(ClientBuilder::<RuntimeT>::new())
        .set_url(URL)
        .skip_type_sizes_check()
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
            // match known_events::<RuntimeT>(&e) {
            //     Some(ProposalProposed(e)) => {},
            //     Some(ProposalApproved(e)) => {},
            //     Some(ProposalRevokedApproval(e)) => {},
            //     Some(ProposalResolved(e)) => {},
            //     Some(ProposalExpired(e)) => {},
            //     None | _ => {}
            // }
        }
    }
}

impl frame::deip_proposal::DeipProposal for RuntimeT {
    type ProposalBatch = pallet_deip_proposal::proposal::ProposalBatch<node_template_runtime::Runtime>;
    type ProposalId = pallet_deip_proposal::proposal::ProposalId;
    type Call = node_template_runtime::Call;
    type BatchItem = pallet_deip_proposal::proposal::ProposalBatchItemOf<node_template_runtime::Runtime>;
    type ProposalState = pallet_deip_proposal::proposal::ProposalState;
    type WrappedBatch = Vec<pallet_deip_proposal::proposal::BatchItem<node_template_runtime::AccountId, WrappedCall>>;

    fn wrap_batch(batch: &Self::ProposalBatch) -> Self::WrappedBatch {
        batch.iter().map(|x| {
            pallet_deip_proposal::proposal::BatchItem {
                account: x.account.clone(),
                call: WrappedCall(x.call.clone())
            }
        }).collect()
    }
}

use node_template_runtime::Call as RuntimeCall;

#[derive(Clone, Debug, Eq, PartialEq, Decode, Encode)]
pub struct WrappedCall(RuntimeCall);

impl Serialize for WrappedCall {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
        where S: Serializer
    {
        match &self.0 {
            RuntimeCall::Deip(pallet_deip::Call::create_project(
                                  is_private,
                                  external_id,
                                  team_id,
                                  description,
                                  domains)) => {
                CallObject {
                    module: "deip",
                    call: "create_project",
                    args: &DeipCreateProjectCallArgs {
                        is_private,
                        external_id,
                        team_id,
                        description,
                        domains,
                    },
                }.serialize(serializer)
            },
            RuntimeCall::Deip(pallet_deip::Call::create_investment_opportunity(
                                  external_id,
                                  project_id,
                                  investment_type)) => {
                CallObject {
                    module: "deip",
                    call: "create_investment_opportunity",
                    args: &DeipCreateInvestmentOpportunityCallArgs {
                        external_id,
                        project_id,
                        investment_type
                    },
                }.serialize(serializer)
            },
            RuntimeCall::Deip(pallet_deip::Call::invest(
                                  id,
                                  amount)) => {
                CallObject {
                    module: "deip",
                    call: "invest",
                    args: &DeipInvestCallArgs {
                        id,
                        amount
                    },
                }.serialize(serializer)
            },
            RuntimeCall::Deip(pallet_deip::Call::create_project_content(
                                  external_id,
                                  project_external_id,
                                  team_id,
                                  content_type,
                                  description,
                                  content,
                                  authors,
                                  references)) => {
                CallObject {
                    module: "deip",
                    call: "create_project_content",
                    args: &DeipCreateProjectContentCallArgs {
                        external_id,
                        project_external_id,
                        team_id,
                        content_type,
                        description,
                        content,
                        authors,
                        references,
                    },
                }.serialize(serializer)
            },
            RuntimeCall::Deip(pallet_deip::Call::create_project_nda(
                                  external_id,
                                  end_date,
                                  contract_hash,
                                  maybe_start_date,
                                  parties,
                                  projects)) => {
                CallObject {
                    module: "deip",
                    call: "create_project_nda",
                    args: &DeipCreateProjectNdaCallArgs {
                        external_id,
                        end_date,
                        contract_hash,
                        maybe_start_date,
                        parties,
                        projects,
                    },
                }.serialize(serializer)
            },
            RuntimeCall::Deip(pallet_deip::Call::create_nda_content_access_request(
                                  external_id,
                                  nda_external_id,
                                  encrypted_payload_hash,
                                  encrypted_payload_iv)) => {
                CallObject {
                    module: "deip",
                    call: "create_nda_content_access_request",
                    args: &DeipCreateProjectNdaAccessRequestCallArgs {
                        external_id,
                        nda_external_id,
                        encrypted_payload_hash,
                        encrypted_payload_iv
                    },
                }.serialize(serializer)
            },
            RuntimeCall::Deip(pallet_deip::Call::fulfill_nda_content_access_request(
                                  external_id,
                                  encrypted_payload_encryption_key,
                                  proof_of_encrypted_payload_encryption_key)) => {
                CallObject {
                    module: "deip",
                    call: "fulfill_nda_content_access_request",
                    args: &DeipFulfillNdaAccessRequestCallArgs {
                        external_id,
                        encrypted_payload_encryption_key,
                        proof_of_encrypted_payload_encryption_key
                    },
                }.serialize(serializer)
            },
            RuntimeCall::Deip(pallet_deip::Call::reject_nda_content_access_request(
                                  external_id)) => {
                CallObject {
                    module: "deip",
                    call: "reject_nda_content_access_request",
                    args: &DeipRejectNdaAccessRequestCallArgs {
                        external_id,
                    },
                }.serialize(serializer)
            },
            RuntimeCall::Deip(pallet_deip::Call::create_review(
                                  external_id,
                                  author,
                                  content,
                                  domains,
                                  assessment_model,
                                  weight,
                                  project_content_external_id)) => {
                CallObject {
                    module: "deip",
                    call: "create_review",
                    args: &DeipCreateReviewCallArgs {
                        external_id,
                        author,
                        content,
                        domains,
                        assessment_model,
                        weight,
                        project_content_external_id,
                    },
                }.serialize(serializer)
            },
            RuntimeCall::Deip(pallet_deip::Call::add_domain(
                                  domain)) => {
                CallObject {
                    module: "deip",
                    call: "add_domain",
                    args: &DeipAddDomainCallArgs {
                        domain,
                    },
                }.serialize(serializer)
            },
            _ =>  serializer.serialize_u8(0)
        }
    }
}

#[derive(Serialize)]
struct DeipAddDomainCallArgs<A> {
    domain: A,
}

#[derive(Serialize)]
struct DeipCreateReviewCallArgs<A, B, C, D, E, F, G> {
    external_id: A,
    author: B,
    content: C,
    domains: D,
    assessment_model: E,
    weight: F,
    project_content_external_id: G,
}

#[derive(Serialize)]
struct DeipRejectNdaAccessRequestCallArgs<A> {
    external_id: A,
}

#[derive(Serialize)]
struct DeipFulfillNdaAccessRequestCallArgs<A, B, C> {
    external_id: A,
    encrypted_payload_encryption_key: B,
    proof_of_encrypted_payload_encryption_key: C,
}

#[derive(Serialize)]
struct DeipCreateProjectNdaAccessRequestCallArgs<A, B, C, D> {
    external_id: A,
    nda_external_id: B,
    encrypted_payload_hash: C,
    encrypted_payload_iv: D,
}

#[derive(Serialize)]
struct DeipCreateProjectNdaCallArgs<A, B, C, D, E, F> {
    external_id: A,
    end_date: B,
    contract_hash: C,
    maybe_start_date: D,
    parties: E,
    projects: F,
}

#[derive(Serialize)]
struct DeipCreateProjectContentCallArgs<A, B, C, D, E, F, G, H> {
    external_id: A,
    project_external_id: B,
    team_id: C,
    content_type: D,
    description: E,
    content: F,
    authors: G,
    references: H,
}

#[derive(Serialize)]
struct DeipInvestCallArgs<A, B> {
    id: A,
    amount: B,
}

#[derive(Serialize)]
struct DeipCreateInvestmentOpportunityCallArgs<A, B, C> {
    external_id: A,
    project_id: B,
    investment_type: C,
}

#[derive(Serialize)]
struct DeipCreateProjectCallArgs<A, B, C, D, E> {
    is_private: A,
    external_id: B,
    team_id: C,
    description: D,
    domains: E
}

#[derive(Serialize)]
struct CallObject<A, B, C> {
    module: A,
    call: B,
    args: C
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
