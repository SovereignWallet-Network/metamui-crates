use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{traits::ConstU32, BoundedVec};
use sp_core::sr25519;
use scale_info::TypeInfo;
pub use metamui_primitives::Did;


pub type PublicKey = sr25519::Public;
pub type MaxMetadata = ConstU32<32>;
pub type MaxRegNumLen = ConstU32<32>;
pub type MaxCompNameLen = ConstU32<32>;
pub type Metadata = BoundedVec<u8, MaxMetadata>;
pub type RegistrationNumber = BoundedVec<u8, MaxMetadata>;
pub type CompanyName = BoundedVec<u8, MaxCompNameLen>;


#[derive(Decode, Encode, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PrivateDid {
	pub identifier: Did,
	pub public_key: PublicKey,
	pub metadata: Metadata,
}

#[derive(Decode, Encode, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PublicDid {
	pub identifier: Did,
	pub public_key: PublicKey,
	pub metadata: Metadata,
	pub registration_number: RegistrationNumber,
	pub company_name: CompanyName,
}

#[derive(Decode, Encode, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DIDType {
	Public(PublicDid),
	Private(PrivateDid),
}
