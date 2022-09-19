use super::pallet::*;
use crate::types::*;
use codec::{Codec};
use metamui_primitives::traits::{MultiAddress, DidResolve};
use sp_runtime::traits::{LookupError, StaticLookup};

impl<T: Config> DidResolve<T::AccountId> for Pallet<T> {

  /// Check if Did exists
  fn did_exists(x: MultiAddress<T::AccountId>) -> bool {
    match x {
      // Return if the source is accountId
      MultiAddress::Id(id) => RLookup::<T>::contains_key(id),
      // Fetch the accountId from storage if did is passed
      MultiAddress::Did(did) => Lookup::<T>::contains_key(did),
    }
  }

  /// Get did from account id 
  fn get_did(k: &T::AccountId) -> Option<Did> {
    RLookup::<T>::get(k)
  }

  fn get_account_id(k: &Did) -> Option<T::AccountId> {
    Lookup::<T>::get(k)
  }
}

/// implement the lookup trait to fetch the accountid of the
/// did from storage
impl<T: Config> StaticLookup for Pallet<T>
where
  MultiAddress<T::AccountId>: Codec,
{
  type Source = MultiAddress<T::AccountId>;
  type Target = T::AccountId;

  fn lookup(x: Self::Source) -> Result<Self::Target, LookupError> {
    match x {
      // Return if the source is accountId
      MultiAddress::Id(id) => Ok(id),
      // Fetch the accountId from storage if did is passed
      MultiAddress::Did(did) => Lookup::<T>::get(did).ok_or(LookupError),
    }
  }

  fn unlookup(x: Self::Target) -> Self::Source {
    MultiAddress::Id(x)
  }
}


impl<T: Config> UpdateDid for Pallet<T> {
	fn add_private_did(
			public_key: PublicKey,
			identifier: Did,
			metadata: Metadata,
	) {
    let _ = Self::do_create_private_did(public_key, identifier, metadata);
  }

	fn add_public_did(
      public_key: PublicKey,
			identifier: Did,
			metadata: Metadata,
			registration_number: RegistrationNumber,
			company_name: CompanyName,
	) {
    let _ = Self::do_create_public_did(public_key, identifier, metadata, registration_number, company_name);
  }

	fn remove_did(identifier: Did) {
    let _ = Self::do_remove(&identifier);
  }

	fn rotate_key(
      identifier: Did,
			public_key: PublicKey,
	) {
    let _ = Self::do_rotate_key(&identifier, &public_key);
  }

	fn update_metadata(
      identifier: Did,
			metadata: Metadata,
	) {
    let _ = Self::do_update_metadata(&identifier, &metadata);
  }
}
