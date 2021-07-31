
use substrate_subxt::system::System;
use substrate_subxt::{module, Event};

use sp_std::prelude::*;
use codec::{Encode, Decode};
use frame_support::{Parameter};
use sp_runtime::traits::Member;

use serde::{Serialize, ser::{Serializer, SerializeStruct}};

#[module]
pub trait DeipOrg: System {
    type Org: Parameter + Member + Serialize;
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct OrgCreateEvent<T: DeipOrg>(T::Org);
impl<T: DeipOrg> Serialize for OrgCreateEvent<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
        where S: Serializer
    {
        let mut s = serializer.serialize_struct("OrgCreateEvent", 1)?;
        s.serialize_field("dao", &self.0)?;
        s.end()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct OrgTransferOwnershipEvent<T: DeipOrg>(T::Org);
impl<T: DeipOrg> Serialize for OrgTransferOwnershipEvent<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
        where S: Serializer
    {
        let mut s = serializer.serialize_struct("OrgTransferOwnershipEvent", 1)?;
        s.serialize_field("dao", &self.0)?;
        s.end()
    }
}
