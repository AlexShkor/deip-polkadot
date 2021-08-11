use super::*;

/// Unique Review reference
pub type Id = H160;

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct Vote<AccountId, Moment> {
    dao: AccountId,
    review_id: ReviewId,
    domain_id: DomainId,
    voting_time: Moment,
}

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct Review<Hash, AccountId> {
    /// Reference for external world and uniques control
    external_id: Id,
    /// Reference to the Team
    author: AccountId,
    /// Hash of content
    content: Hash,
    /// List of Domains aka tags Project matches
    domains: Vec<DomainId>,
    /// Model number by which the evaluation is carried out
    assessment_model: u32,
    /// percent in "50.00 %" format
    weight: Vec<u8>,
    /// Reference to Project Content
    project_content_external_id: ProjectContentId,
}

impl<T: Config> Module<T> {
    pub(super) fn create_review_impl(
        account: T::AccountId,
        external_id: Id,
        author: T::DeipAccountId,
        content: T::Hash,
        domains: Vec<DomainId>,
        assessment_model: u32,
        weight: Vec<u8>,
        project_content_external_id: ProjectContentId,
    ) -> DispatchResult {
        let review = Review {
            external_id,
            author: author.into(),
            content,
            domains,
            assessment_model,
            weight,
            project_content_external_id,
        };

        let mut reviews = Reviews::<T>::get();
        let index_to_insert_review = reviews
            .binary_search_by_key(&review.external_id, |&(a, _)| a)
            .err()
            .ok_or(Error::<T>::ReviewAlreadyExists)?;

        ProjectsContent::<T>::get()
            .iter()
            .find(|(id, ..)| id == &review.project_content_external_id)
            .ok_or(Error::<T>::NoSuchProjectContent)?;

        for domain in &review.domains {
            ensure!(Domains::contains_key(&domain), Error::<T>::DomainNotExists);
        }

        reviews.insert(
            index_to_insert_review,
            (review.external_id, review.author.clone()),
        );
        Reviews::<T>::put(reviews);

        ReviewMap::<T>::insert(review.external_id, review.clone());

        Self::deposit_event(RawEvent::ReviewCreated(account, review));

        Ok(())
    }

    pub(super) fn vote_for_review_impl(
        account: T::AccountId,
        review_id: ReviewId,
        domain_id: DomainId,
    ) -> DispatchResult {
        ensure!(
            Domains::contains_key(domain_id),
            Error::<T>::ReviewVoteNoSuchDomain
        );

        let review =
            ReviewMap::<T>::try_get(review_id).map_err(|_| Error::<T>::ReviewVoteNoSuchReview)?;
        ensure!(
            review.domains.contains(&domain_id),
            Error::<T>::ReviewVoteUnrelatedDomain
        );

        ensure!(
            !ReviewVoteMap::<T>::contains_key((review_id, account.clone(), domain_id)),
            Error::<T>::ReviewAlreadyVotedWithDomain
        );

        let vote = Vote {
            dao: account.clone(),
            review_id,
            domain_id,
            voting_time: pallet_timestamp::Module::<T>::get(),
        };

        ReviewVoteMap::<T>::insert((review_id, account.clone(), domain_id), vote);
        Self::deposit_event(RawEvent::ReviewVoted(review_id, account, domain_id));

        Ok(())
    }
}
