use crate::traits::DeipAssetSystem;
use crate::*;

use sp_runtime::{
    traits::{Saturating, Zero},
    SaturatedConversion,
};

/// Unique ProjectTokenSale ID reference
pub type Id = H160;

#[derive(Encode, Decode, Clone, Copy, RuntimeDebug, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub enum Status {
    Active,
    Finished,
    Expired,
    Inactive,
}

impl Default for Status {
    fn default() -> Self {
        Status::Inactive
    }
}

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub enum InvestmentOpportunity<Moment, AssetId, AssetBalance> {
    ProjectTokenSale {
        /// a moment when the sale starts. Must be later than current moment.
        start_time: Moment,
        /// a moment when the sale ends. Must be later than `start_time`.
        end_time: Moment,
        /// id of the asset intended to raise.
        asset_id: AssetId,
        /// amount of units to raise.
        soft_cap: AssetBalance,
        /// amount upper limit of units to raise. Must be greater or equal to `soft_cap`.
        hard_cap: AssetBalance,
        /// specifies how many tokens of the project are intended to sale.
        security_tokens_on_sale: Vec<(AssetId, AssetBalance)>,
    },
}

/// The object represents a sale of project's tokens with
/// various parameters.
/// It is connected to the specific project.
#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct Info<Moment, AssetId, AssetBalance> {
    /// Reference for external world and uniques control
    pub external_id: Id,
    /// Reference to the Project
    pub project_id: ProjectId,
    /// When the sale starts
    pub start_time: Moment,
    /// When it supposed to end
    pub end_time: Moment,
    pub status: Status,
    pub asset_id: AssetId,
    /// How many contributions already reserved
    pub total_amount: AssetBalance,
    pub soft_cap: AssetBalance,
    pub hard_cap: AssetBalance,
    /// How many tokens supposed to sale
    pub security_tokens_on_sale: Vec<(AssetId, AssetBalance)>,
}

impl<T: Config> Module<T> {
    pub(super) fn create_investment_opportunity_impl(
        account: AccountIdOf<T>,
        external_id: Id,
        project_id: ProjectId,
        investment_type: InvestmentOpportunity<T::Moment, DeipAssetIdOf<T>, DeipAssetBalanceOf<T>>,
    ) -> DispatchResult {
        match investment_type {
            InvestmentOpportunity::ProjectTokenSale {
                start_time,
                end_time,
                asset_id,
                soft_cap,
                hard_cap,
                security_tokens_on_sale,
            } => Self::create_project_token_sale_impl(
                account,
                external_id,
                project_id,
                start_time,
                end_time,
                asset_id,
                soft_cap,
                hard_cap,
                security_tokens_on_sale,
            ),
        }
    }

    pub(super) fn create_project_token_sale_impl(
        account: T::AccountId,
        external_id: Id,
        project_id: ProjectId,
        start_time: T::Moment,
        end_time: T::Moment,
        asset_id: DeipAssetIdOf<T>,
        soft_cap: DeipAssetBalanceOf<T>,
        hard_cap: DeipAssetBalanceOf<T>,
        security_tokens_on_sale: Vec<(DeipAssetIdOf<T>, DeipAssetBalanceOf<T>)>,
    ) -> DispatchResult {
        ensure!(
            !ProjectTokenSaleMap::<T>::contains_key(external_id),
            Error::<T>::TokenSaleAlreadyExists
        );

        let timestamp = pallet_timestamp::Module::<T>::get();
        ensure!(
            start_time >= timestamp,
            Error::<T>::TokenSaleStartTimeMustBeLaterOrEqualCurrentMoment
        );
        ensure!(
            end_time > start_time,
            Error::<T>::TokenSaleEndTimeMustBeLaterStartTime
        );

        ensure!(
            !soft_cap.is_zero(),
            Error::<T>::TokenSaleSoftCapMustBeGreaterOrEqualMinimum
        );
        ensure!(
            hard_cap >= soft_cap,
            Error::<T>::TokenSaleHardCapShouldBeGreaterOrEqualSoftCap
        );

        ensure!(
            !security_tokens_on_sale.is_empty(),
            Error::<T>::TokenSaleSecurityTokenNotSpecified
        );
        for (security_token_id, asset_amount) in &security_tokens_on_sale {
            ensure!(
                *security_token_id != asset_id,
                Error::<T>::TokenSaleWrongAssetId
            );

            match T::AssetSystem::try_get_tokenized_project(&security_token_id) {
                None => return Err(Error::<T>::TokenSaleAssetIsNotSecurityToken.into()),
                Some(id) => ensure!(
                    id == project_id,
                    Error::<T>::TokenSaleProjectNotTokenizedWithSecurityToken
                ),
            };

            ensure!(
                !asset_amount.is_zero(),
                Error::<T>::TokenSaleAssetAmountMustBePositive
            );
        }

        let projects = Projects::<T>::get();
        match projects.binary_search_by_key(&project_id, |&(p, _)| p) {
            Ok(index) => {
                if projects[index].1 != account {
                    return Err(Error::<T>::ProjectNotBelongToTeam.into());
                }
            }
            Err(_) => return Err(Error::<T>::NoSuchProject.into()),
        }

        let mut token_sales = ProjectTokenSaleByProjectIdStatus::get();
        if let Ok(_) = token_sales.binary_search_by_key(
            &(project_id, ProjectTokenSaleStatus::Active),
            |&(p, s, _)| (p, s),
        ) {
            return Err(Error::<T>::TokenSaleScheduledAlready.into());
        }

        let index = match token_sales.binary_search_by_key(
            &(project_id, ProjectTokenSaleStatus::Inactive),
            |&(p, s, _)| (p, s),
        ) {
            Ok(_) => return Err(Error::<T>::TokenSaleScheduledAlready.into()),
            Err(i) => i,
        };

        if let Err(_) =
            T::AssetSystem::transactionally_reserve(&account, project_id, &security_tokens_on_sale)
        {
            return Err(Error::<T>::TokenSaleBalanceIsNotEnough.into());
        }

        let new_project_token_sale = ProjectTokenSale {
            external_id: external_id,
            project_id: project_id,
            start_time: start_time,
            end_time: end_time,
            asset_id,
            soft_cap: soft_cap,
            hard_cap: hard_cap,
            security_tokens_on_sale: security_tokens_on_sale,
            ..Default::default()
        };

        token_sales.insert(index, (project_id, Status::Inactive, external_id));
        ProjectTokenSaleByProjectIdStatus::put(token_sales);
        ProjectTokenSaleMap::<T>::insert(external_id, new_project_token_sale.clone());

        Self::deposit_event(RawEvent::ProjectTokenSaleCreated(
            project_id,
            new_project_token_sale,
        ));

        Ok(())
    }

    pub(super) fn collect_funds(sale_id: Id, amount: DeipAssetBalanceOf<T>) -> Result<(), ()> {
        ProjectTokenSaleMap::<T>::mutate_exists(sale_id, |sale| -> Result<(), ()> {
            match sale.as_mut() {
                Some(s) => s.total_amount = amount.saturating_add(s.total_amount),
                None => return Err(()),
            }
            Ok(())
        })
    }

    pub(super) fn finish_project_token_sale_by_id(sale_id: Id) -> Result<(), ()> {
        match ProjectTokenSaleMap::<T>::try_get(sale_id) {
            Err(_) => Err(()),
            Ok(sale) => {
                Self::update_status(&sale, Status::Finished);
                Self::process_project_token_sale_contributions(&sale);
                Ok(())
            }
        }
    }

    pub(super) fn activate_project_token_sale_impl(sale_id: Id) -> DispatchResult {
        ProjectTokenSaleMap::<T>::mutate_exists(sale_id, |maybe_sale| -> DispatchResult {
            let sale = match maybe_sale.as_mut() {
                None => return Err(Error::<T>::TokenSaleNotFound.into()),
                Some(s) => s,
            };

            match sale.status {
                Status::Active => return Ok(()),
                Status::Inactive => ensure!(
                    pallet_timestamp::Module::<T>::get() >= sale.start_time,
                    Error::<T>::TokenSaleShouldBeStarted
                ),
                _ => return Err(Error::<T>::TokenSaleShouldBeInactive.into()),
            };

            Self::update_status_index(sale, Status::Active);
            sale.status = Status::Active;
            Self::deposit_event(RawEvent::ProjectTokenSaleActivated(
                sale.project_id,
                sale_id,
            ));

            Ok(())
        })
    }

    pub(super) fn expire_project_token_sale_impl(sale_id: Id) -> DispatchResult {
        ProjectTokenSaleMap::<T>::mutate_exists(sale_id, |maybe_sale| -> DispatchResult {
            let sale = match maybe_sale.as_mut() {
                None => return Err(Error::<T>::TokenSaleNotFound.into()),
                Some(s) => s,
            };

            match sale.status {
                Status::Expired => return Ok(()),
                Status::Active => ensure!(
                    pallet_timestamp::Module::<T>::get() >= sale.end_time,
                    Error::<T>::TokenSaleExpirationWrongState
                ),
                _ => return Err(Error::<T>::TokenSaleShouldBeActive.into()),
            };

            Self::update_status_index(sale, Status::Expired);
            sale.status = Status::Expired;

            Self::refund_project_token_sale(sale);

            Ok(())
        })
    }

    pub(super) fn finish_project_token_sale_impl(sale_id: Id) -> DispatchResult {
        ProjectTokenSaleMap::<T>::mutate_exists(sale_id, |maybe_sale| -> DispatchResult {
            let sale = match maybe_sale.as_mut() {
                None => return Err(Error::<T>::TokenSaleNotFound.into()),
                Some(s) => s,
            };

            match sale.status {
                Status::Finished => return Ok(()),
                Status::Active => (),
                _ => return Err(Error::<T>::TokenSaleShouldBeActive.into()),
            };

            Self::update_status_index(sale, Status::Finished);
            sale.status = Status::Finished;

            Self::process_project_token_sale_contributions(sale);

            Ok(())
        })
    }

    pub(super) fn process_project_token_sales_offchain() {
        let now = pallet_timestamp::Module::<T>::get();
        for (id, sale) in ProjectTokenSaleMap::<T>::iter() {
            if sale.end_time <= now && matches!(sale.status, Status::Active) {
                if sale.total_amount < sale.soft_cap {
                    let call = Call::expire_project_token_sale(id);
                    let submit =
                        SubmitTransaction::<T, Call<T>>::submit_unsigned_transaction(call.into());
                    debug!("submit expire_project_token_sale: {}", submit.is_ok());
                } else if sale.total_amount >= sale.soft_cap {
                    let call = Call::finish_project_token_sale(id);
                    let submit =
                        SubmitTransaction::<T, Call<T>>::submit_unsigned_transaction(call.into());
                    debug!("submit finish_project_token_sale: {}", submit.is_ok());
                }
            } else if sale.end_time > now {
                if now >= sale.start_time && matches!(sale.status, Status::Inactive) {
                    let call = Call::activate_project_token_sale(id);
                    let submit =
                        SubmitTransaction::<T, Call<T>>::submit_unsigned_transaction(call.into());
                    debug!("submit activate_project_token_sale: {}", submit.is_ok());
                }
            }
        }
    }

    fn update_status(sale: &ProjectTokenSaleOf<T>, new_status: Status) {
        Self::update_status_index(sale, new_status);

        ProjectTokenSaleMap::<T>::mutate_exists(sale.external_id, |maybe_sale| -> () {
            let sale = maybe_sale.as_mut().expect("we keep collections in sync");
            sale.status = new_status;
        });
    }

    fn update_status_index(sale: &ProjectTokenSaleOf<T>, new_status: Status) {
        let mut token_sales = ProjectTokenSaleByProjectIdStatus::get();
        match new_status {
            Status::Inactive => (),
            Status::Finished | Status::Expired | Status::Active => {
                let old_index = token_sales
                    .binary_search_by_key(&(sale.project_id, sale.status), |&(p, t, _)| (p, t))
                    .expect("we keep collections in sync");
                token_sales.remove(old_index);

                let index = match token_sales
                    .binary_search_by_key(&(sale.project_id, new_status), |&(p, t, _)| (p, t))
                {
                    Ok(i) => i,
                    Err(i) => i,
                };

                token_sales.insert(index, (sale.project_id, new_status, sale.external_id));
                ProjectTokenSaleByProjectIdStatus::put(token_sales);
            }
        }
    }

    fn refund_project_token_sale(sale: &ProjectTokenSaleOf<T>) {
        let team_id = &ProjectMap::<T>::try_get(sale.project_id)
            .expect("checked in create method")
            .team_id;

        if let Ok(ref c) = ProjectTokenSaleContributions::<T>::try_get(sale.external_id) {
            for (_, ref contribution) in c {
                T::AssetSystem::transfer(
                    sale.project_id,
                    &contribution.owner,
                    sale.asset_id,
                    contribution.amount,
                )
                .expect("user's asset should be reserved earlier");
            }
            ProjectTokenSaleContributions::<T>::remove(sale.external_id);
        }

        T::AssetSystem::transactionally_unreserve(sale.project_id, team_id)
            .expect("assets should be reserved earlier");

        Self::deposit_event(RawEvent::ProjectTokenSaleExpired(
            sale.project_id,
            sale.external_id,
        ));
    }

    fn process_project_token_sale_contributions(sale: &ProjectTokenSaleOf<T>) {
        T::AssetSystem::transfer(
            sale.project_id,
            &ProjectMap::<T>::get(sale.project_id).team_id,
            sale.asset_id,
            sale.total_amount,
        )
        .expect("total_amount");

        let contributions = ProjectTokenSaleContributions::<T>::try_get(sale.external_id)
            .expect("Token sale is about to finish, but there are no contributions?");
        let mut iter = contributions.iter();

        let (_, ref first_contribution) = iter
            .next()
            .expect("Token sale is about to finish, but there are no contributors?");

        for (_, ref contribution) in iter {
            for (asset_id, asset_amount) in &sale.security_tokens_on_sale {
                // similiar to frame_support::traits::Imbalance::ration
                let token_amount = contribution
                    .amount
                    .saturated_into::<u128>()
                    .saturating_mul(asset_amount.clone().saturated_into())
                    / sale.total_amount.saturated_into::<u128>();
                let token_amount: DeipAssetBalanceOf<T> = token_amount.saturated_into();
                if token_amount.is_zero() {
                    continue;
                }

                T::AssetSystem::transfer(
                    sale.project_id,
                    &contribution.owner,
                    *asset_id,
                    token_amount,
                )
                .expect("Required token_amount should be reserved");
            }
        }

        // process the remainder
        T::AssetSystem::transactionally_unreserve(sale.project_id, &first_contribution.owner)
            .expect("remaining assets should be reserved earlier");

        ProjectTokenSaleContributions::<T>::remove(sale.external_id);

        Self::deposit_event(RawEvent::ProjectTokenSaleFinished(
            sale.project_id,
            sale.external_id,
        ));
    }
}
