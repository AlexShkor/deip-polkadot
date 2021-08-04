use codec::{Encode, Decode};
use super::{frame, RuntimeT};
use crate::frame::DeipProposal;

#[derive(Clone, Debug, Eq, PartialEq, Decode, Encode)]
pub struct WrappedCall<T>(pub T);

impl frame::deip_proposal::DeipProposal for RuntimeT {
    type ProposalBatch = pallet_deip_proposal::proposal::ProposalBatch<node_template_runtime::Runtime>;
    type InputProposalBatch = pallet_deip_proposal::proposal::InputProposalBatch<node_template_runtime::Runtime>;
    type ProposalId = pallet_deip_proposal::proposal::ProposalId;
    type Call = node_template_runtime::Call;
    type BatchItem = pallet_deip_proposal::proposal::ProposalBatchItemOf<node_template_runtime::Runtime>;
    type ProposalState = pallet_deip_proposal::proposal::ProposalState;
    type WrappedBatch = Vec<pallet_deip_proposal::proposal::BatchItem<
        node_template_runtime::AccountId, Self::WrappedCall>>;
    type WrappedInputBatch = Vec<pallet_deip_proposal::proposal::BatchItem<
        node_template_runtime::deip_account::DeipAccountId<
            node_template_runtime::AccountId>, Self::WrappedCall>>;
    type WrappedCall = WrappedCall<Self::Call>;

    fn wrap_batch<T: From<Self::WrappedBatch>>(batch: &Self::ProposalBatch) -> T {
        batch.iter().map(|x| {
            pallet_deip_proposal::proposal::BatchItem {
                account: x.account.clone(),
                call: Self::wrap_call(&x.call)
            }
        }).collect::<Self::WrappedBatch>().into()
    }

    fn wrap_input_batch(batch: &Self::InputProposalBatch) -> Self::WrappedInputBatch {
        batch.iter().map(|x| {
            pallet_deip_proposal::proposal::BatchItem {
                account: x.account.clone(),
                call: Self::wrap_call(&x.call)
            }
        }).collect()
    }
    
    fn wrap_call(call: &Self::Call) -> Self::WrappedCall {
        WrappedCall(call.clone())
    }
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
    type InvestmentId = Self::ProjectTokenSaleId;
    type ProjectTokenSale = pallet_deip::ProjectTokenSaleOf<node_template_runtime::Runtime>;
}

impl frame::deip_org::DeipOrg for RuntimeT {
    type Org = pallet_deip_org::org::OrgOf<node_template_runtime::Runtime>;
}
