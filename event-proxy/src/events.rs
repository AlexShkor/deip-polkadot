use std::fmt::Debug;

use substrate_subxt::{RawEvent, Event};
use codec::Decode;
use serde::{Serialize, ser::{Serializer, SerializeStruct}};

use super::frame::{
    deip_proposal::{self, DeipProposal},
    deip::{self, Deip},
    deip_org::{self, DeipOrg}
};

struct TypedEvent<'a, T> {
    r#type: &'a str,
    data: &'a T
}
impl<'a, T> TypedEvent<'a, T> {
    fn new(r#type: &'a str, data: &'a T) -> Self {
        TypedEvent { r#type, data }
    }
}
impl<T: Serialize> Serialize for TypedEvent<'_, T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
        where S: Serializer
    {
        let mut s = serializer.serialize_struct("TypedEvent", 2)?;
        s.serialize_field("type", self.r#type)?;
        s.serialize_field("data", self.data)?;
        s.end()
    }
}

impl<T: DeipProposal + Deip + DeipOrg> Serialize for KnownEvents<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
        where S: Serializer
    {
        match self {
            // =============== DeipProposal:
            ProposalProposed(e) => {
                TypedEvent::new("proposal_proposed", e).serialize(serializer)
            },
            ProposalApproved(e) => {
                TypedEvent::new("proposal_approved", e).serialize(serializer)
            },
            ProposalRevokedApproval(e) => {
                TypedEvent::new("proposal_revokedApproval", e).serialize(serializer)
            },
            ProposalResolved(e) => {
                TypedEvent::new("proposal_resolved", e).serialize(serializer)
            },
            ProposalExpired(e) => {
                TypedEvent::new("proposal_expired", e).serialize(serializer)
            },
            // =============== Deip:
            ProjectCreated(e) => {
                TypedEvent::new("project_created", e).serialize(serializer)
            },
            ProjectRemoved(e) => {
                TypedEvent::new("project_removed", e).serialize(serializer)
            },
            ProjectUpdated(e) => {
                TypedEvent::new("project_updated", e).serialize(serializer)
            },
            ProjectContentCreated(e) => {
                TypedEvent::new("project_contentCreated", e).serialize(serializer)
            },
            NdaCreated(e) => {
                TypedEvent::new("project_ndaCreated", e).serialize(serializer)
            },
            NdaAccessRequestCreated(e) => {
                TypedEvent::new("project_ndaAccessRequestCreated", e).serialize(serializer)
            },
            NdaAccessRequestFulfilled(e) => {
                TypedEvent::new("project_ndaAccessRequestFulfilled", e).serialize(serializer)
            },
            NdaAccessRequestRejected(e) => {
                TypedEvent::new("project_ndaAccessRequestRejected", e).serialize(serializer)
            },
            DomainAdded(e) => {
                TypedEvent::new("project_domainAdded", e).serialize(serializer)
            },
            ReviewCreated(e) => {
                TypedEvent::new("project_reviewCreated", e).serialize(serializer)
            },
            ProjectTokenSaleCreated(e) => {
                TypedEvent::new("project_tokenSaleCreated", e).serialize(serializer)
            },
            ProjectTokenSaleActivated(e) => {
                TypedEvent::new("project_tokenSaleActivated", e).serialize(serializer)
            },
            ProjectTokenSaleFinished(e) => {
                TypedEvent::new("project_tokenSaleFinished", e).serialize(serializer)
            },
            ProjectTokenSaleExpired(e) => {
                TypedEvent::new("project_tokenSaleExpired", e).serialize(serializer)
            },
            ProjectTokenSaleContributed(e) => {
                TypedEvent::new("project_tokenSaleContributed", e).serialize(serializer)
            },
            // =============== DeipOrg:
            OrgCreate(e) => {
                TypedEvent::new("dao_create", e).serialize(serializer)
            },
            OrgTransferOwnership(e) => {
                TypedEvent::new("dao_transferOwnership", e).serialize(serializer)
            },
        }
    }
}

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
    ProjectTokenSaleCreated(deip::ProjectTokenSaleCreatedEvent<T>),
    ProjectTokenSaleActivated(deip::ProjectTokenSaleActivatedEvent<T>),
    ProjectTokenSaleFinished(deip::ProjectTokenSaleFinishedEvent<T>),
    ProjectTokenSaleExpired(deip::ProjectTokenSaleExpiredEvent<T>),
    ProjectTokenSaleContributed(deip::ProjectTokenSaleContributedEvent<T>),
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
            decode_event_data(e).map(|x| {
                // println!("HERE: {:?}", &x);
                ProposalProposed(x) })
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
            deip::NdaAccessRequestFulfilledEvent::<T>::MODULE,
            deip::NdaAccessRequestFulfilledEvent::<T>::EVENT
        ) => {                          
            decode_event_data(e).map(NdaAccessRequestFulfilled)
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
        (                               
            deip::ProjectTokenSaleCreatedEvent::<T>::MODULE,
            deip::ProjectTokenSaleCreatedEvent::<T>::EVENT
        ) => {                          
            decode_event_data(e).map(ProjectTokenSaleCreated)
        },
        (                               
            deip::ProjectTokenSaleActivatedEvent::<T>::MODULE,
            deip::ProjectTokenSaleActivatedEvent::<T>::EVENT
        ) => {                          
            decode_event_data(e).map(ProjectTokenSaleActivated)
        },
        (                               
            deip::ProjectTokenSaleFinishedEvent::<T>::MODULE,
            deip::ProjectTokenSaleFinishedEvent::<T>::EVENT
        ) => {                          
            decode_event_data(e).map(ProjectTokenSaleFinished)
        },
        (                               
            deip::ProjectTokenSaleExpiredEvent::<T>::MODULE,
            deip::ProjectTokenSaleExpiredEvent::<T>::EVENT
        ) => {                          
            decode_event_data(e).map(ProjectTokenSaleExpired)
        },
        (                               
            deip::ProjectTokenSaleContributedEvent::<T>::MODULE,
            deip::ProjectTokenSaleContributedEvent::<T>::EVENT
        ) => {                          
            decode_event_data(e).map(ProjectTokenSaleContributed)
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
