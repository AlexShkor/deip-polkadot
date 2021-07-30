use std::fmt::Debug;

use substrate_subxt::{RawEvent, Event};
use codec::Decode;

use super::RuntimeT;
use super::frame::{
    deip_proposal::{self, DeipProposal},
    deip::{self, Deip},
    deip_org::{self, DeipOrg}
};

pub use KnownEvents::*;

#[derive(Debug)]
pub enum KnownEvents<T: DeipProposal + Deip + DeipOrg> {
    // DeipProposal:
    ProposalProposed(deip_proposal::ProposedEvent<T>),
    ProposalApproved(deip_proposal::ApprovedEvent<T>),
    ProposalRevokedApproval(deip_proposal::RevokedApprovalEvent<T>),
    ProposalResolved(deip_proposal::ResolvedEvent<T>),
    ProposalExpired(deip_proposal::ExpiredEvent<T>),
    // Deip:
    ProjectCreated(deip::ProjectCreatedEvent<T>),
    ProjectRemoved(deip::ProjectRemovedEvent<T>),
    ProjectUpdated(deip::ProjectUpdatedEvent<T>),
    ProjectContentCreated(deip::ProjectContentCreatedEvent<T>),
    NdaCreated(deip::NdaCreatedEvent<T>),
    NdaAccessRequestCreated(deip::NdaAccessRequestCreatedEvent<T>),
    NdaAccessRequestFulfilled(deip::NdaAccessRequestFulfilledEvent<T>),
    NdaAccessRequestRejected(deip::NdaAccessRequestRejectedEvent<T>),
    DomainAdded(deip::DomainAddedEvent<T>),
    ReviewCreated(deip::ReviewCreatedEvent<T>),
    // DeipOrg:
    OrgCreate(deip_org::OrgCreateEvent<T>),
    OrgTransferOwnership(deip_org::OrgTransferOwnershipEvent<T>),
}

pub fn known_events<T>(e: &RawEvent) -> Option<KnownEvents<T>> where T: DeipProposal + Deip + DeipOrg + Debug {
    let event = match (e.module.as_str(), e.variant.as_str()) {
        // =========== DeipProposal:
        (
            deip_proposal::ProposedEvent::<T>::MODULE,
            deip_proposal::ProposedEvent::<T>::EVENT
        ) => { 
            decode_event_data(e).map(ProposalProposed)
        },
        (
            deip_proposal::ApprovedEvent::<T>::MODULE,
            deip_proposal::ApprovedEvent::<T>::EVENT
        ) => {
            decode_event_data(e).map(ProposalApproved)
        },
        (
            deip_proposal::RevokedApprovalEvent::<T>::MODULE,
            deip_proposal::RevokedApprovalEvent::<T>::EVENT
        ) => {
            decode_event_data(e).map(ProposalRevokedApproval)
        },
        (
            deip_proposal::ResolvedEvent::<T>::MODULE,
            deip_proposal::ResolvedEvent::<T>::EVENT
        ) => {
            decode_event_data(e).map(ProposalResolved)
        },
        (
            deip_proposal::ExpiredEvent::<T>::MODULE,
            deip_proposal::ExpiredEvent::<T>::EVENT
        ) => {
            decode_event_data(e).map(ProposalExpired)
        },
        // =========== Deip:
        (
            deip::ProjectCreatedEvent::<T>::MODULE,
            deip::ProjectCreatedEvent::<T>::EVENT
        ) => {                          
            decode_event_data(e).map(ProjectCreated)
        },                              
        (                               
            deip::ProjectRemovedEvent::<T>::MODULE,
            deip::ProjectRemovedEvent::<T>::EVENT
        ) => {                          
            decode_event_data(e).map(ProjectRemoved)
        },
        (                               
            deip::ProjectUpdatedEvent::<T>::MODULE,
            deip::ProjectUpdatedEvent::<T>::EVENT
        ) => {                          
            decode_event_data(e).map(ProjectUpdated)
        },
        (                               
            deip::ProjectContentCreatedEvent::<T>::MODULE,
            deip::ProjectContentCreatedEvent::<T>::EVENT
        ) => {                          
            decode_event_data(e).map(ProjectContentCreated)
        },
        (                               
            deip::NdaCreatedEvent::<T>::MODULE,
            deip::NdaCreatedEvent::<T>::EVENT
        ) => {                          
            decode_event_data(e).map(NdaCreated)
        },
        (                               
            deip::NdaAccessRequestCreatedEvent::<T>::MODULE,
            deip::NdaAccessRequestCreatedEvent::<T>::EVENT
        ) => {                          
            decode_event_data(e).map(NdaAccessRequestCreated)
        },
        (                               
            deip::NdaAccessRequestRejectedEvent::<T>::MODULE,
            deip::NdaAccessRequestRejectedEvent::<T>::EVENT
        ) => {                          
            decode_event_data(e).map(NdaAccessRequestRejected)
        },
        (                               
            deip::DomainAddedEvent::<T>::MODULE,
            deip::DomainAddedEvent::<T>::EVENT
        ) => {                          
            decode_event_data(e).map(DomainAdded)
        },
        (                               
            deip::ReviewCreatedEvent::<T>::MODULE,
            deip::ReviewCreatedEvent::<T>::EVENT
        ) => {                          
            decode_event_data(e).map(ReviewCreated)
        },
        // =========== DeipOrg:
        (                               
            deip_org::OrgCreateEvent::<T>::MODULE,
            deip_org::OrgCreateEvent::<T>::EVENT
        ) => {                          
            decode_event_data(e).map(OrgCreate)
        },
        (                               
            deip_org::OrgTransferOwnershipEvent::<T>::MODULE,
            deip_org::OrgTransferOwnershipEvent::<T>::EVENT
        ) => {                          
            decode_event_data(e).map(OrgTransferOwnership)
        },
        _ => return None,
    };
    if let Err(err) = event {
        log::error!("{}", err);
        return None
    }
    log::debug!("{:?}", event.as_ref().unwrap());
    event.ok()
}

fn decode_event_data<T: Decode>(e: &RawEvent) -> Result<T, codec::Error> {
    T::decode(&mut &e.data[..])
}
