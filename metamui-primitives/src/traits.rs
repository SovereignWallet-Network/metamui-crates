use super::*;
use codec::{Decode, Encode};
use sp_core::hexdisplay::HexDisplay;
use sp_runtime::RuntimeDebug;
use scale_info::TypeInfo;
use frame_support::{traits::{ConstU32}, BoundedVec, sp_runtime::DispatchError};
use sp_core::sr25519::{Signature as SRSignature};
use sp_std::{prelude::*};


// DID

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
    fn did_exists(_: MultiAddress<AccountId>) -> bool {
        false
    }
    /// convert accountId to DID
    fn get_did(_: &AccountId) -> Option<Did> {
        None
    }
    /// convert accountId to DID
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

/// VC Property max length
pub type VCPropertyLimit = ConstU32<32>;
/// VC Property type
pub type VCProperty = BoundedVec<u8, VCPropertyLimit>;

/// Type of VCs
#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum VCType {
    /// VC to create a Token
    TokenVC,
    /// VC to slash token
    SlashTokens,
    /// VC to mint token
    MintTokens,
    /// VC to transfer token
    TokenTransferVC,
    /// VC for generic purpose
    GenericVC,
}


/// Struct for VC
#[derive(Clone, PartialEq, Eq, RuntimeDebug, Encode, Decode, TypeInfo)]
pub struct VC<Hash> {
    /// Hash of the data in VC
    pub hash: Hash,
    /// Owner of VC
    pub owner: Did,
    /// Issuers of VC
    pub issuers: Vec<Did>,
    /// Signatures of Issuers on hash
    pub signatures: Vec<SRSignature>,
    /// If VC is used or not
    pub is_vc_used: bool,
    /// Type of VC
    pub vc_type: VCType,
    /// VC payload
    pub vc_property: VCProperty,
}

/// Trait to get VC details
pub trait VCResolve<Hash> {
    /// Get VC from VC Id
    fn get_vc(vc_id: &VCid) -> Option<VC<Hash>>;
    /// Get if VC is used
    fn is_vc_used(vc_id: &VCid) -> bool;
    /// Set VC used
    fn set_vc_used(vc_id: &VCid, is_vc_used: bool);
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
    fn set_vc_used(_vc_id: &VCid, _is_vc_used: bool) {
        ()
    }
    /// Decode VC
    fn decode_vc<E: Decode>(_vc_bytes: &[u8]) -> Result<E, DispatchError> {
        Err("Not Implemented".into())
    }
}

pub trait IsMember {
    fn is_caller_council_member(_: AccountId) -> bool;
}

impl IsMember for () {
    fn is_caller_council_member(_: AccountId) -> bool{
        false
    }
}