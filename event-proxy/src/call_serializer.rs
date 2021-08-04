use serde::{Serialize, ser::Serializer};
use super::{DeipProposal, runtime, RuntimeT};
use node_template_runtime::Call;

impl Serialize for runtime::WrappedCall<<RuntimeT as DeipProposal>::Call> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
        where S: Serializer
    {
        match &self.0 {
            // =============== Deip:
            Call::Deip(pallet_deip::Call::create_project(
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
            Call::Deip(pallet_deip::Call::create_investment_opportunity(
                              external_id,
                              project_id,
                              investment_type)) => {
                CallObject {
                    module: "deip",
                    call: "create_investment_opportunity",
                    args: &DeipCreateInvestmentOpportunityCallArgs {
                        external_id,
                        project_id,
                        investment_type,
                    },
                }.serialize(serializer)
            },
            Call::Deip(pallet_deip::Call::invest(
                              id,
                              amount)) => {
                CallObject {
                    module: "deip",
                    call: "invest",
                    args: &DeipInvestCallArgs {
                        id,
                        amount,
                    },
                }.serialize(serializer)
            },
            Call::Deip(pallet_deip::Call::create_project_content(
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
            Call::Deip(pallet_deip::Call::create_project_nda(
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
            Call::Deip(pallet_deip::Call::create_nda_content_access_request(
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
                        encrypted_payload_iv,
                    },
                }.serialize(serializer)
            },
            Call::Deip(pallet_deip::Call::fulfill_nda_content_access_request(
                              external_id,
                              encrypted_payload_encryption_key,
                              proof_of_encrypted_payload_encryption_key)) => {
                CallObject {
                    module: "deip",
                    call: "fulfill_nda_content_access_request",
                    args: &DeipFulfillNdaAccessRequestCallArgs {
                        external_id,
                        encrypted_payload_encryption_key,
                        proof_of_encrypted_payload_encryption_key,
                    },
                }.serialize(serializer)
            },
            Call::Deip(pallet_deip::Call::reject_nda_content_access_request(
                              external_id)) => {
                CallObject {
                    module: "deip",
                    call: "reject_nda_content_access_request",
                    args: &DeipRejectNdaAccessRequestCallArgs {
                        external_id,
                    },
                }.serialize(serializer)
            },
            Call::Deip(pallet_deip::Call::create_review(
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
            Call::Deip(pallet_deip::Call::add_domain(
                              domain)) => {
                CallObject {
                    module: "deip",
                    call: "add_domain",
                    args: &DeipAddDomainCallArgs {
                        domain,
                    },
                }.serialize(serializer)
            },
            // =============== DeipProposal:
            Call::DeipProposal(pallet_deip_proposal::Call::propose(
                                      batch,
                                      external_id)) => {
                CallObject {
                    module: "deip_proposal",
                    call: "propose",
                    args: &DeipProposalProposeCallArgs {
                        batch: &RuntimeT::wrap_input_batch(batch),
                        external_id,
                    },
                }.serialize(serializer)
            }
            Call::DeipProposal(pallet_deip_proposal::Call::decide(
                                      proposal_id,
                                      decision)) => {
                CallObject {
                    module: "deip_proposal",
                    call: "decide",
                    args: &DeipProposalDecideCallArgs {
                        proposal_id,
                        decision,
                    },
                }.serialize(serializer)
            },
            // =============== DeipOrg:
            Call::DeipOrg(pallet_deip_org::Call::create(
                                 name,
                                 key_source)) => {
                CallObject {
                    module: "deip_org",
                    call: "create",
                    args: &DeipOrgCreateCallArgs {
                        name,
                        key_source,
                    },
                }.serialize(serializer)
            },
            Call::DeipOrg(pallet_deip_org::Call::transfer_ownership(
                                 transfer_to,
                                 key_source)) => {
                CallObject {
                    module: "deip_org",
                    call: "transfer_ownership",
                    args: &DeipOrgTransferOwnershipCallArgs {
                        transfer_to,
                        key_source,
                    },
                }.serialize(serializer)
            },
            Call::DeipOrg(pallet_deip_org::Call::on_behalf(
                                 name,
                                 call)) => {
                CallObject {
                    module: "deip_org",
                    call: "on_behalf",
                    args: &DeipOrgOnBehalfCallArgs {
                        name,
                        call: &RuntimeT::wrap_call(call),
                    },
                }.serialize(serializer)
            },
            _ => serializer.serialize_u8(0)
        }
    }
}

#[derive(Serialize)]
struct DeipOrgOnBehalfCallArgs<A, B> {
    name: A,
    call: B,
}

#[derive(Serialize)]
struct DeipOrgTransferOwnershipCallArgs<A, B> {
    transfer_to: A,
    key_source: B,
}

#[derive(Serialize)]
struct DeipOrgCreateCallArgs<A, B> {
    name: A,
    key_source: B,
}

#[derive(Serialize)]
struct DeipProposalDecideCallArgs<A, B> {
    proposal_id: A,
    decision: B,
}

#[derive(Serialize)]
struct DeipProposalProposeCallArgs<A, B> {
    batch: A,
    external_id: B,
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
    domains: E,
}

#[derive(Serialize)]
struct CallObject<A, B, C> {
    module: A,
    call: B,
    args: C,
}
