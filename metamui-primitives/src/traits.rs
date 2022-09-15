use super::*;
use codec::{Decode, Encode};
use sp_core::hexdisplay::HexDisplay;
use sp_runtime::RuntimeDebug;
use scale_info::TypeInfo;

/// Trait to resolve Did
pub trait DidResolve<AccountId> {
  /// return if an accountId is mapped to a DID
  fn did_exists(x: MultiAddress<AccountId>) -> bool;
  /// convert accountId to DID
  fn get_did(k: &AccountId) -> Option<Did>;
  /// convert accountId to DID
  fn get_account_id(k: &Did) -> Option<AccountId>;
}

impl<AccountId> DidResolve<AccountId> for () {
    /// return if an accountId is mapped to a DID
    fn did_exists(x: MultiAddress<AccountId>) -> bool {
        false
    }
    /// convert accountId to DID
    fn get_did(k: &AccountId) -> Option<Did> {
        None
    }
    /// convert accountId to DID
    fn get_account_id(k: &Did) -> Option<AccountId> {
        None
    }
}


/// Use this struct for the account lookup
/// This struct can have the value of either rawbytes or accountid
/// This is necessary to compile all other pallets that depend on the accountID field
/// Once all pallets have been ported to the custom DID format we can remove the dependency
/// on this struct and lookup trait in general
#[derive(Encode, Decode, PartialEq, Eq, Clone, TypeInfo, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Hash))]
pub enum MultiAddress<AccountId> {
    /// type for regular pubkey accountid
    Id(AccountId),
    /// type for lookup to the did identifier - referencing the did type from the did module
    Did(Did),
}

#[cfg(feature = "std")]
impl<AccountId> std::fmt::Display for MultiAddress<AccountId>
where
    AccountId: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MultiAddress::Did(inner) => write!(f, "{}", HexDisplay::from(inner)),
            MultiAddress::Id(_inner) => write!(f, "{}", self),
        }
    }
}

// Create a MultiAddress object from an accountid passed
impl<AccountId> From<AccountId> for MultiAddress<AccountId> {
    fn from(x: AccountId) -> Self {
        MultiAddress::Id(x)
    }
}

// The default option to select when creating a Multiaddress
// The current default is set to accountid, but once we migrate all pallets
// to use did signing, we can move default to did
impl<AccountId: Default> Default for MultiAddress<AccountId> {
    fn default() -> Self {
        MultiAddress::Id(Default::default())
    }
}
