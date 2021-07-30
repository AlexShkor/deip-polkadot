
use substrate_subxt::system::System;
use substrate_subxt::{module, Event};

use sp_std::prelude::*;
use codec::{Encode, Decode};
use frame_support::{Parameter};
use sp_runtime::traits::Member;

#[module]
pub trait DeipOrg: System {
    type Org: Parameter + Member;
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct OrgCreateEvent<T: DeipOrg>(T::Org);

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct OrgTransferOwnershipEvent<T: DeipOrg>(T::Org);
