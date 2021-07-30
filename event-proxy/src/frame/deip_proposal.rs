
use substrate_subxt::system::System;
use substrate_subxt::{module, Event};

use sp_std::prelude::*;
use codec::{Encode, Decode};
use frame_support::{Parameter};
use sp_runtime::traits::Member;

#[module]
pub trait DeipProposal: System {
    type ProposalBatch: Parameter + Member;
    type ProposalId: Parameter + Member;
    type Call: Parameter + Member;
    type BatchItem: Parameter + Member;
    type ProposalState: Parameter + Member;
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct ProposedEvent<T: DeipProposal> {
    pub author: <T as System>::AccountId,
    pub batch: T::ProposalBatch,
    pub proposal_id: T::ProposalId,
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct ApprovedEvent<T: DeipProposal> {
    pub member: <T as System>::AccountId,
    pub proposal_id: T::ProposalId,
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct RevokedApprovalEvent<T: DeipProposal> {
    pub member: <T as System>::AccountId,
    pub proposal_id: T::ProposalId,
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct ResolvedEvent<T: DeipProposal> {
    pub member: <T as System>::AccountId,
    pub proposal_id: T::ProposalId,
    pub state: T::ProposalState
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct ExpiredEvent<T: DeipProposal> {
    pub proposal_id: T::ProposalId,
}
