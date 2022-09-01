use super::*;

/// Trait to resolve Did
pub trait DidResolve<AccountId> {
  /// return if an accountId is mapped to a DID
  fn did_exists(x: &AccountId) -> bool;
  /// convert accountId to DID
  fn get_did_from_account_id(k: &AccountId) -> Did;
}