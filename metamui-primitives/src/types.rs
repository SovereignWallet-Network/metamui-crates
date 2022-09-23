use super::*;
use codec::{Decode, Encode};
use sp_runtime::RuntimeDebug;
use scale_info::TypeInfo;
use frame_support::{traits::{ConstU32}, BoundedVec};
use sp_core::sr25519::{Signature as SRSignature};

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

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SlashMintTokens {
  pub vc_id: VCid,
  pub amount: u128,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TokenTransferVC {
  pub vc_id: VCid,
  pub amount: u128,
}