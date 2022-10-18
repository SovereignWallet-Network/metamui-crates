use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{ pallet_prelude::DispatchResult };
use scale_info::TypeInfo;
pub use metamui_primitives::Did;
pub use metamui_primitives::types::*;
use cumulus_primitives_core::ParaId;

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
    public_key: PublicKey,
    identifier: Did,
  ) -> DispatchResult;

  fn add_public_did(
    public_key: PublicKey,
    identifier: Did,
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
      _: PublicKey,
      _: Did,
  ) -> DispatchResult {
    Err("Not Implemented".into())
  }

  fn add_public_did(
      _: PublicKey,
      _: Did,
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
  fn on_new_did(
      para_id: ParaId,
      public_key: PublicKey,
      identifier: Did,
  );

  fn on_did_removal(
    para_id: ParaId,
    identifier: Did,
  );

  fn on_key_updation(
    para_id: ParaId,
    identifier: Did,
    public_key: PublicKey,
);
}

impl DidUpdated for () {
  fn on_new_did(
      _: ParaId,
      _: PublicKey,
      _: Did,
  ) {
    ()
  }

  fn on_did_removal(
    _: ParaId,
    _: Did,
  ) {
    ()
  }

  fn on_key_updation(
      _: ParaId,
      _: Did,
      _: PublicKey,
  ) {
    ()
  }
}