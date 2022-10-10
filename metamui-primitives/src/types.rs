use super::*;
use codec::{Decode, Encode};
use sp_runtime::RuntimeDebug;
use scale_info::TypeInfo;
use frame_support::{traits::ConstU32, BoundedVec};
use sp_core::sr25519::Signature as SRSignature;
use sp_std::prelude::*;
use sp_core::sr25519;

/// VC Property max length
pub type VCPropertyLimit = ConstU32<32>;
/// VC Property type
pub type VCProperty = BoundedVec<u8, VCPropertyLimit>;

/// Public Key TypeI
pub type PublicKey = sr25519::Public;
/// Maximum Size of Metadata
pub type MaxMetadata = ConstU32<32>;
/// Maximum Length of Registration Number
pub type MaxRegNumLen = ConstU32<32>;
/// Maximum Length of Company Name
pub type MaxCompNameLen = ConstU32<32>;
/// Metadata Type
pub type Metadata = BoundedVec<u8, MaxMetadata>;
/// Registration Number Type
pub type RegistrationNumber = BoundedVec<u8, MaxMetadata>;
/// Company Name Type
pub type CompanyName = BoundedVec<u8, MaxCompNameLen>;

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
  /// VC to create public did
  PublicDidVC,
  /// VC to create private did
  PrivateDidVC,
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
  /// If VC is active or not
  pub is_vc_active: bool,
  /// Type of VC
  pub vc_type: VCType,
  /// VC payload
  pub vc_property: VCProperty,
}

/// SlashMintTokens Type VC
#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SlashMintTokens {
  /// VCid field
  pub vc_id: VCid,
  /// Amount field
  pub amount: u128,
}

/// TokenTransfer Type VC
#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TokenTransferVC {
  /// VCid field
  pub vc_id: VCid,
  /// Amount field
  pub amount: u128,
}

/// PublicDidVC Type VC
#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PublicDidVC {
  /// Public Key
	pub public_key: PublicKey,
  /// Metadata
	pub metadata: Metadata,
  /// Registration Number
	pub registration_number: RegistrationNumber,
  /// Name of Company
	pub company_name: CompanyName,
}

/// PrivateDidVC Type VC
#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PrivateDidVC {
  /// Public Key
	pub public_key: PublicKey,
  /// Metadata
	pub metadata: Metadata,
}