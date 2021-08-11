use super::*;

/// Unique Review reference
pub type Id = H160;

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
}
