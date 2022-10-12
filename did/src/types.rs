use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{ pallet_prelude::DispatchResult };
use scale_info::TypeInfo;
pub use metamui_primitives::Did;
pub use metamui_primitives::types::*;
use primitives::v2::{Id as ParaId};

pub type Region = [u8; 16];

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
pub enum DIdentity {
  Public(PublicDid),
  Private(PrivateDid),
}

#[derive(Decode, Encode, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DIDRegion {
  Local,
  Tokenchain(ParaId)
}

/// Trait to handle Did Crud operations
pub trait UpdateDid {
  fn add_private_did(
    para_id: ParaId,
    public_key: PublicKey,
    identifier: Did,
    metadata: Metadata,
  ) -> DispatchResult;

  fn add_public_did(
    para_id: ParaId,
    public_key: PublicKey,
    identifier: Did,
    metadata: Metadata,
    registration_number: RegistrationNumber,
    company_name: CompanyName,
  ) -> DispatchResult;

  fn remove_did(identifier: Did) -> DispatchResult;

  fn rotate_key(
    identifier: Did,
    public_key: PublicKey,
  ) -> DispatchResult;

  fn update_metadata(
    identifier: Did,
    metadata: Metadata,
  ) -> DispatchResult;
}

impl UpdateDid for () {
  fn add_private_did(
      _: ParaId,
      _: PublicKey,
      _: Did,
      _: Metadata,
  ) -> DispatchResult {
    Err("Not Implemented".into())
  }

  fn add_public_did(
      _: ParaId,
      _: PublicKey,
      _: Did,
      _: Metadata,
      _: RegistrationNumber,
      _: CompanyName,
  )  -> DispatchResult{
    Err("Not Implemented".into())
  }

  fn remove_did(_: Did) -> DispatchResult {
    Err("Not Implemented".into())
  }

  fn rotate_key(
      _: Did,
      _: PublicKey,
  ) -> DispatchResult {
    Err("Not Implemented".into())
  }

  fn update_metadata(
      _: Did,
      _: Metadata,
  ) -> DispatchResult {
    Err("Not Implemented".into())
  }
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
  ) {
    ()
  }

  fn on_new_public_did(
      _: PublicKey,
      _: Did,
      _: Metadata,
      _: RegistrationNumber,
      _: CompanyName,
  ) {
    ()
  }

  fn on_did_removal(_: Did) {
    ()
  }

  fn on_key_rotation(
      _: Did,
      _: PublicKey,
  ) {
    ()
  }

  fn on_metadata_updation(
      _: Did,
      _: Metadata,
  ) {
    ()
  }
}