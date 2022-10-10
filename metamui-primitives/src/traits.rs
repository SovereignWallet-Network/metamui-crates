use super::*;
use codec::{Decode, Encode};
use sp_core::hexdisplay::HexDisplay;
use sp_runtime::RuntimeDebug;
use scale_info::TypeInfo;
use frame_support::{ sp_runtime::DispatchError };
use sp_std::{prelude::*};

// DID

/// Trait to resolve Did
pub trait DidResolve<AccountId> {
  /// return if an accountId is mapped to a DID
  fn did_exists(x: MultiAddress<AccountId>) -> bool;
  /// convert accountId to DID
  fn get_did(k: &AccountId) -> Option<Did>;
  /// convert DID to accountId
  fn get_account_id(k: &Did) -> Option<AccountId>;
}

impl<AccountId> DidResolve<AccountId> for () {
    /// return if an accountId is mapped to a DID
    fn did_exists(_: MultiAddress<AccountId>) -> bool {
        false
    }
    /// convert accountId to DID
    fn get_did(_: &AccountId) -> Option<Did> {
        None
    }
    /// convert DID to accountId
    fn get_account_id(_: &Did) -> Option<AccountId> {
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
// VC
/// Trait to get VC details
pub trait VCResolve<Hash> {
    /// Get VC from VC Id
    fn get_vc(vc_id: &VCid) -> Option<VC<Hash>>;
    /// Get if VC is used
    fn is_vc_used(vc_id: &VCid) -> bool;
    /// Set VC used
    fn set_is_vc_used(vc_id: &VCid, is_vc_used: bool);
    /// Decode VC
    fn decode_vc<E: Decode>(vc_bytes: &[u8]) -> Result<E, DispatchError>;
  }
  
impl<Hash> VCResolve<Hash> for () {
    /// Get VC from VC Id
    fn get_vc(_vc_id: &VCid) -> Option<VC<Hash>> {
        None
    }
    /// Get if VC is used
    fn is_vc_used(_vc_id: &VCid) -> bool {
        true
    }
    /// Set VC used
    fn set_is_vc_used(_vc_id: &VCid, _is_vc_used: bool) {
        ()
    }
    /// Decode VC
    fn decode_vc<E: Decode>(_vc_bytes: &[u8]) -> Result<E, DispatchError> {
        Err("Not Implemented".into())
    }
}

/// Trait to give back the VCid
pub trait HasVCId {
    /// Function to return the VCid
    fn vc_id(&self) -> VCid;
}

/// Implementing HasVCId for SlashMintTokens
impl HasVCId for SlashMintTokens {
    /// Function to return the VCid
    fn vc_id(&self) -> VCid {
        self.vc_id
    }
}

/// Implementing HasVCId for TokenTransferVC
impl HasVCId for TokenTransferVC {
    /// Function to return the VCid
    fn vc_id(&self) -> VCid {
        self.vc_id
    }
}

/// Trait to check if a Did is a council member
pub trait IsMember {
    /// Function to check council membership
    fn is_member(_: &Did) -> bool;
}

impl IsMember for () {
    /// Function to check council membership
    fn is_member(_: &Did) -> bool{
        false
    }
}