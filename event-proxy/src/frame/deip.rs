
use substrate_subxt::system::System;
use substrate_subxt::{module, Event};

use sp_std::prelude::*;
use codec::{Encode, Decode};
use frame_support::{Parameter};
use sp_runtime::traits::Member;

use serde::{Serialize, ser::{Serializer, SerializeStruct}};

#[module]
pub trait Deip: System {
    type DomainId: Parameter + Member + Serialize;
    type ProjectId: Parameter + Member + Serialize;
    type Project: Parameter + Member + Serialize;
    type Review: Parameter + Member + Serialize;
    type NdaId: Parameter + Member + Serialize;
    type NdaAccessRequestId: Parameter + Member + Serialize;
    type ProjectContentId: Parameter + Member + Serialize;
    type ProjectTokenSaleId: Parameter + Member + Serialize;
    type InvestmentId: Parameter + Member + Serialize;
    type ProjectTokenSale: Parameter + Member + Serialize;
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct ProjectCreatedEvent<T: Deip>(T::AccountId, T::Project);
impl<T: Deip> Serialize for ProjectCreatedEvent<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
        where S: Serializer
    {
        let mut s = serializer.serialize_struct("ProjectCreatedEvent", 2)?;
        s.serialize_field("account_id", &self.0)?;
        s.serialize_field("project", &self.1)?;
        s.end()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct ProjectRemovedEvent<T: Deip>(T::AccountId, T::Project);
impl<T: Deip> Serialize for ProjectRemovedEvent<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
        where S: Serializer
    {
        let mut s = serializer.serialize_struct("ProjectRemovedEvent", 2)?;
        s.serialize_field("account_id", &self.0)?;
        s.serialize_field("project", &self.1)?;
        s.end()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct ProjectUpdatedEvent<T: Deip>(T::AccountId, T::ProjectId);
impl<T: Deip> Serialize for ProjectUpdatedEvent<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
        where S: Serializer
    {
        let mut s = serializer.serialize_struct("ProjectUpdatedEvent", 2)?;
        s.serialize_field("account_id", &self.0)?;
        s.serialize_field("project_id", &self.1)?;
        s.end()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct ProjectContentCreatedEvent<T: Deip>(T::AccountId, T::ProjectContentId);
impl<T: Deip> Serialize for ProjectContentCreatedEvent<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
        where S: Serializer
    {
        let mut s = serializer.serialize_struct("ProjectContentCreatedEvent", 2)?;
        s.serialize_field("account_id", &self.0)?;
        s.serialize_field("content_id", &self.1)?;
        s.end()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct NdaCreatedEvent<T: Deip>(T::AccountId, T::NdaId);
impl<T: Deip> Serialize for NdaCreatedEvent<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
        where S: Serializer
    {
        let mut s = serializer.serialize_struct("NdaCreatedEvent", 2)?;
        s.serialize_field("account_id", &self.0)?;
        s.serialize_field("nda_id", &self.1)?;
        s.end()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct NdaAccessRequestCreatedEvent<T: Deip>(T::AccountId, T::NdaAccessRequestId);
impl<T: Deip> Serialize for NdaAccessRequestCreatedEvent<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
        where S: Serializer
    {
        let mut s = serializer.serialize_struct("NdaAccessRequestCreatedEvent", 2)?;
        s.serialize_field("account_id", &self.0)?;
        s.serialize_field("access_request_id", &self.1)?;
        s.end()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct NdaAccessRequestFulfilledEvent<T: Deip>(T::AccountId, T::NdaAccessRequestId);
impl<T: Deip> Serialize for NdaAccessRequestFulfilledEvent<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
        where S: Serializer
    {
        let mut s = serializer.serialize_struct("NdaAccessRequestFulfilledEvent", 2)?;
        s.serialize_field("account_id", &self.0)?;
        s.serialize_field("access_request_id", &self.1)?;
        s.end()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct NdaAccessRequestRejectedEvent<T: Deip>(T::AccountId, T::NdaAccessRequestId);
impl<T: Deip> Serialize for NdaAccessRequestRejectedEvent<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
        where S: Serializer
    {
        let mut s = serializer.serialize_struct("NdaAccessRequestRejectedEvent", 2)?;
        s.serialize_field("account_id", &self.0)?;
        s.serialize_field("access_request_id", &self.1)?;
        s.end()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct DomainAddedEvent<T: Deip>(T::AccountId, T::DomainId);
impl<T: Deip> Serialize for DomainAddedEvent<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
        where S: Serializer
    {
        let mut s = serializer.serialize_struct("DomainAddedEvent", 2)?;
        s.serialize_field("account_id", &self.0)?;
        s.serialize_field("domain_id", &self.1)?;
        s.end()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct ReviewCreatedEvent<T: Deip>(T::AccountId, T::Review);
impl<T: Deip> Serialize for ReviewCreatedEvent<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
        where S: Serializer
    {
        let mut s = serializer.serialize_struct("ReviewCreatedEvent", 2)?;
        s.serialize_field("account_id", &self.0)?;
        s.serialize_field("review", &self.1)?;
        s.end()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct ProjectTokenSaleCreatedEvent<T: Deip>(T::ProjectId, T::ProjectTokenSale);
impl<T: Deip> Serialize for ProjectTokenSaleCreatedEvent<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
        where S: Serializer
    {
        let mut s = serializer.serialize_struct("ProjectTokenSaleCreatedEvent", 2)?;
        s.serialize_field("project_id", &self.0)?;
        s.serialize_field("project_token_sale", &self.1)?;
        s.end()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct ProjectTokenSaleActivatedEvent<T: Deip>(T::ProjectId, T::InvestmentId);
impl<T: Deip> Serialize for ProjectTokenSaleActivatedEvent<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
        where S: Serializer
    {
        let mut s = serializer.serialize_struct("ProjectTokenSaleActivatedEvent", 2)?;
        s.serialize_field("project_id", &self.0)?;
        s.serialize_field("investment_id", &self.1)?;
        s.end()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct ProjectTokenSaleFinishedEvent<T: Deip>(T::ProjectId, T::InvestmentId);
impl<T: Deip> Serialize for ProjectTokenSaleFinishedEvent<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
        where S: Serializer
    {
        let mut s = serializer.serialize_struct("ProjectTokenSaleFinishedEvent", 2)?;
        s.serialize_field("project_id", &self.0)?;
        s.serialize_field("investment_id", &self.1)?;
        s.end()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct ProjectTokenSaleExpiredEvent<T: Deip>(T::ProjectId, T::InvestmentId);
impl<T: Deip> Serialize for ProjectTokenSaleExpiredEvent<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
        where S: Serializer
    {
        let mut s = serializer.serialize_struct("ProjectTokenSaleExpiredEvent", 2)?;
        s.serialize_field("project_id", &self.0)?;
        s.serialize_field("investment_id", &self.1)?;
        s.end()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct ProjectTokenSaleContributedEvent<T: Deip>(T::InvestmentId, T::AccountId);
impl<T: Deip> Serialize for ProjectTokenSaleContributedEvent<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
        where S: Serializer
    {
        let mut s = serializer.serialize_struct("ProjectTokenSaleContributedEvent", 2)?;
        s.serialize_field("investment_id", &self.0)?;
        s.serialize_field("account_id", &self.1)?;
        s.end()
    }
}
