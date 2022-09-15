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


/// Trait for type to 
pub trait UpdateDid {
	fn add_private_did(
			public_key: PublicKey,
			identifier: Did,
			metadata: Metadata,
	);

	fn add_public_did(
			public_key: PublicKey,
			identifier: Did,
			metadata: Metadata,
			registration_number: RegistrationNumber,
			company_name: CompanyName,
	);

	fn remove_did(identifier: Did);

	fn rotate_key(
			identifier: Did,
			public_key: PublicKey,
	);

	fn update_metadata(
			identifier: Did,
			metadata: Metadata,
	);
}

impl UpdateDid for () {
	fn add_private_did(
			_: PublicKey,
			_: Did,
			_: Metadata,
	) {}

	fn add_public_did(
			_: PublicKey,
			_: Did,
			_: Metadata,
			_: RegistrationNumber,
			_: CompanyName,
	) {}

	fn remove_did(_: Did) {}

	fn rotate_key(
			_: Did,
			_: PublicKey,
	) {}

	fn update_metadata(
			_: Did,
			_: Metadata,
	) {}
}


/// Trait for type that can handle changes to Dids.
pub trait DidUpdated {
	fn on_new_private_did(
			public_key: PublicKey,
			identifier: Did,
			metadata: Metadata,
	);

	fn on_new_public_did(
			public_key: PublicKey,
			identifier: Did,
			metadata: Metadata,
			registration_number: RegistrationNumber,
			company_name: CompanyName,
	);

	fn on_did_removal(identifier: Did);

	fn on_key_rotation(
			identifier: Did,
			public_key: PublicKey,
	);

	fn on_metadata_updation(
			identifier: Did,
			metadata: Metadata,
	);
}

impl DidUpdated for () {
	fn on_new_private_did(
			_: PublicKey,
			_: Did,
			_: Metadata,
	) {}

	fn on_new_public_did(
			_: PublicKey,
			_: Did,
			_: Metadata,
			_: RegistrationNumber,
			_: CompanyName,
	) {}

	fn on_did_removal(_: Did) {}

	fn on_key_rotation(
			_: Did,
			_: PublicKey,
	) {}

	fn on_metadata_updation(
			_: Did,
			_: Metadata,
	) {}
}