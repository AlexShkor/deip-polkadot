
use substrate_subxt::system::System;
use substrate_subxt::{module, Event};

use sp_std::prelude::*;
use codec::{Encode, Decode};
use frame_support::{Parameter};
use sp_runtime::traits::Member;

#[module]
pub trait Deip: System {
    type DomainId: Parameter + Member;
    type ProjectId: Parameter + Member;
    type Project: Parameter + Member;
    type Review: Parameter + Member;
    type NdaId: Parameter + Member;
    type NdaAccessRequestId: Parameter + Member;
    type ProjectContentId: Parameter + Member;
    type ProjectTokenSaleId: Parameter + Member;
    type ProjectTokenSale: Parameter + Member;
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct ProjectCreatedEvent<T: Deip>(T::AccountId, T::Project);

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct ProjectRemovedEvent<T: Deip>(T::AccountId, T::Project);

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct ProjectUpdatedEvent<T: Deip>(T::AccountId, T::ProjectId);

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct ProjectContentCreatedEvent<T: Deip>(T::AccountId, T::ProjectContentId);

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct NdaCreatedEvent<T: Deip>(T::AccountId, T::NdaId);

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct NdaAccessRequestCreatedEvent<T: Deip>(T::AccountId, T::NdaAccessRequestId);

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct NdaAccessRequestFulfilledEvent<T: Deip>(T::AccountId, T::NdaAccessRequestId);

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct NdaAccessRequestRejectedEvent<T: Deip>(T::AccountId, T::NdaAccessRequestId);

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct DomainAddedEvent<T: Deip>(T::AccountId, T::DomainId);

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct ReviewCreatedEvent<T: Deip>(T::AccountId, T::Review);
