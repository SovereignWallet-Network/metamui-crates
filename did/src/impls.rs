use super::pallet::*;
use crate::types::*;
use codec::Codec;
use metamui_primitives::traits::{MultiAddress, DidResolve};
use sp_runtime::traits::{LookupError, StaticLookup};
use frame_support::pallet_prelude::DispatchResult;

impl<T: Config> DidResolve<T::AccountId> for Pallet<T> {

  /// Check if Did exists
  fn did_exists(address: MultiAddress<T::AccountId>) -> bool {
    match address {
      // Return if the source is accountId
      MultiAddress::Id(id) => RLookup::<T>::contains_key(id),
      // Fetch the accountId from storage if did is passed
      MultiAddress::Did(did) => Lookup::<T>::contains_key(did),
    }
  }

  /// Get did from account id 
  fn get_did(account_id: &T::AccountId) -> Option<Did> {
    RLookup::<T>::get(account_id)
  }

  /// Get accountId from did
  fn get_account_id(did: &Did) -> Option<T::AccountId> {
    Lookup::<T>::get(did)
  }

  /// Get public_key from accountId
  fn get_public_key(did: &Did) -> Option<PublicKey> {
    Self::get_pub_key(did)
  }

  fn is_did_public(did: &Did) -> bool {
    Self::check_did_public(did)
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

/// Implement update did
impl<T: Config> UpdateDid for Pallet<T> {
  fn add_private_did(
      public_key: PublicKey,
      did: Did,
  ) -> DispatchResult {
    // Validate did
    Self::can_add_did(public_key, did)?;

    // Insert Did
    Self::do_create_private_did(public_key, did)?;

    Ok(())
  }

  fn add_public_did(
      public_key: PublicKey,
      did: Did,
      registration_number: RegistrationNumber,
      company_name: CompanyName,
  ) -> DispatchResult {
    // Validate did
    Self::can_add_did(public_key, did)?;

    Self::do_create_public_did(public_key, did, registration_number, company_name)?;
    
    Ok(())
  }

  fn remove_did(did: Did) -> DispatchResult {
    Self::do_remove(&did)
  }

  fn rotate_key(
      did: Did,
      public_key: PublicKey,
  ) -> DispatchResult {
    Self::do_rotate_key(&did, &public_key)
  }

  fn update_metadata(
      did: Did,
      metadata: Metadata,
  ) -> DispatchResult {
    Self::do_update_metadata(&did, &metadata)
  }
}
