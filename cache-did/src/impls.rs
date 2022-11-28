use super::*;
use metamui_primitives::{Did, types::PublicKey, traits::{MultiAddress, DidResolve}};

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
  fn get_did(did: &T::AccountId) -> Option<Did> {
    RLookup::<T>::get(did)
  }

  fn get_account_id(did: &Did) -> Option<T::AccountId> {
    Lookup::<T>::get(did)
  }

  fn get_public_key(did: &Did) -> Option<PublicKey> {
    PublicKeyMap::<T>::get(did)
  }

  fn is_did_public(_did: &Did) -> bool {
    false
  }
}
