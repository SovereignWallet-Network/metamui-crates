#![cfg_attr(not(feature = "std"), no_std)]
/// The VC pallet issues list of VCs that empowers any user to perfom permitted operations.
use frame_support::{
  codec::{ Decode, Encode },
  ensure, fail,
  traits::EnsureOrigin
};

use frame_system::{self, ensure_signed};
use sp_core::sr25519;
use sp_runtime::{
  traits::{ BlakeTwo256, Hash, Verify },
  DispatchError,
};
use metamui_primitives::{ 
  Did, VCid, VCHex,
  traits::{ DidResolve, IsMember, MultiAddress, IsValidator },
  types::{ VCType, VC, SlashMintTokens, TokenTransferVC, PublicDidVC, PrivateDidVC }
};
use sp_std::{ prelude::*, vec };
use sr25519::Signature;

// #[cfg(test)]
// mod tests;

mod impls;

pub mod types;
pub use crate::types::*;
use serde_big_array::big_array;

pub use pallet::*;
#[frame_support::pallet]
pub mod pallet {
  use super::*;
  use frame_support::pallet_prelude::*;
  use frame_system::pallet_prelude::*;

  /// Configure the pallet by specifying the parameters and types on which it depends.
  #[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    /// Origin from which approvals must come.
    type ApproveOrigin: EnsureOrigin<<Self as frame_system::Config>::Origin>;
    
    /// Ensure Caller Is Council Member
    type IsCouncilMember: IsMember;

    /// Ensure Caller Is Validator
    type IsValidator: IsValidator;

    /// Resolve Did from account Id
    type DidResolution: DidResolve<Self::AccountId>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Given VC is validated
		VCValidated{ vcid: VCid },
		/// Updated VC status flag
		VCStatusUpdated{ vcid: VCid, vcstatus: IsVCActive },
	}

	#[pallet::error]
	pub enum Error<T> {
    /// Unable to decode the VC
    InvalidVC,
    /// VC properties verification failed
    VCPropertiesNotVerified,
    /// The given VCId does not exist on chain
    VCIdDoesNotExist,
    /// The operation is permitted only for issuer & validator
    NotAValidatorNorIssuer,
    /// Linked VC does not exist
    LinkedVCNotFound,
    /// The given type of VC should be signed by the owner of respective TokenVC
    VCNotSignedByTokenVCOwner,
    /// VC Already Exist
    VCAlreadyExists,
    /// Either signature is invalid or signer is not a valid issuer 
    InvalidSignature,
    /// The issuer has already approved the VC
    DuplicateSignature,
    /// Invalid currency code
    InvalidCurrencyCode,
    /// The caller is not a council member
    NotACouncilMember,
    /// The caller is not a validator
    NotAValidator,
    /// Did doesn't exist on chain
    DidDoesNotExist,
    /// Public key in the DidVC is already used
    PublicKeyRegistered
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]

	pub struct Pallet<T>(_);

  /// the map for storing VC information
	#[pallet::storage]
  pub(super) type VCs<T: Config> = StorageMap<_, Blake2_128Concat, VCid, Option<VC<T::Hash>>, ValueQuery>;

  /// map to enable lookup from Did to VCids
	#[pallet::storage]
  pub(super) type Lookup<T: Config> = StorageMap<_, Blake2_128Concat, Did, Vec<VCid>, ValueQuery>;

	/// map to enable reverse lookup from VCid to Did
	#[pallet::storage]
  pub(super) type RLookup<T: Config> = StorageMap<_, Blake2_128Concat, VCid, Did, ValueQuery>;

	/// the map for storing history of VC
	#[pallet::storage]
  pub(super) type VCHistory<T: Config> = StorageMap<_, Blake2_128Concat, VCid, Option<(IsVCActive, T::BlockNumber)>, ValueQuery>;

	/// map for vc id and approvers list
	#[pallet::storage]
  pub(super) type VCApproverList<T: Config> = StorageMap<_, Blake2_128Concat, VCid, Vec<Did>, ValueQuery>;

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

  #[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub initial_vcs: Vec<InitialVCs>,
		pub phantom: PhantomData<T>,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self {
				initial_vcs: Default::default(),
				phantom: Default::default(),
			}
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			Pallet::<T>::initialize_vcs(&self.initial_vcs);
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Adds a member to the membership set
		#[pallet::weight(1)]
		pub fn store(origin: OriginFor<T>, vc_hex: VCHex) -> DispatchResult {

			// Extracting vc from encoded vc byte array
			let mut vc: VC<T::Hash> = Self::decode_vc(&vc_hex)?;
			// Issuer’s Did validity will be checked in the set_approved_issuers() 
			// Check if owner’s did is registered or not
      ensure!(<T as pallet::Config>::DidResolution::did_exists(MultiAddress::Did(vc.owner)), Error::<T>::DidDoesNotExist);
      
			match vc.vc_type {
        VCType::TokenVC => {
          // Check if the origin of the call is approved orgin or not
					<T as Config>::ApproveOrigin::ensure_origin(origin)?;
          Self::validate_currency_code(&vc)?;
				}

				VCType::SlashTokens | VCType::MintTokens | VCType::TokenTransferVC | VCType::PrivateDidVC | VCType::PublicDidVC => {
          let sender = ensure_signed(origin)?;
    
          // Check If Sender's Did Exists
          let sender_did = <T as pallet::Config>::DidResolution::get_did(&sender);
          ensure!(sender_did.is_some(), Error::<T>::DidDoesNotExist);
          let sender_did = sender_did.unwrap();

					// Validating caller of above VC types
					Self::validate_vcs(&vc, &sender_did)?;
				}

				VCType::GenericVC => {
          let sender = ensure_signed(origin)?;

          // Check If Sender's Did Exists
          let sender_did = <T as pallet::Config>::DidResolution::get_did(&sender);
          ensure!(sender_did.is_some(), Error::<T>::DidDoesNotExist);
          let sender_did = sender_did.unwrap();

					// ensure the caller is a council member account
					ensure!(<T as pallet::Config>::IsCouncilMember::is_collective_member(&sender_did), Error::<T>::NotACouncilMember);
				}
      }
		
			// Generating vc_id from vc to emit in the event
			let vc_id: VCid = *BlakeTwo256::hash_of(&vc).as_fixed_bytes();
      // Setting is_vc_active to false
      vc.is_vc_active = false;
			// storing hash
			Self::store_vc(vc.owner, vc, vc_id)?;
			Self::deposit_event(Event::VCValidated{ vcid: vc_id });
			Ok(())
		}
		
		/// Update signature of vc_hash to update status as Active or Inactive
		///
		/// This function will set vc status as Active only if all issuers's signatures are verified
		#[pallet::weight(1)]
		pub fn add_signature(origin: OriginFor<T>, vc_id: VCid, sign: Signature) -> DispatchResult {
			// Ensure caller is signed account
			let senders_acccount_id = ensure_signed(origin)?;
  
			Self::validate_updater(&senders_acccount_id, &vc_id)?;
  
			let mut vc = if let Some(vcs_details)  = VCs::<T>::get(vc_id) {
				vcs_details
			} else {
				fail!(Error::<T>::VCIdDoesNotExist)
			};
  
			Self::validate_sign(&vc, sign.clone(), vc_id)?;
  
			vc.signatures.push(sign);
  
			Self::update_vc_and_status(vc_id, vc)?;
			Ok(())
		}
		
		/// Update status of vc_hash wheather it is active or inactive
		#[pallet::weight(1)]
		pub fn update_status(origin: OriginFor<T>, vc_id: VCid, vc_status: IsVCActive) -> DispatchResult {
			// Ensure caller is signed account
			let senders_acccount_id = ensure_signed(origin)?;
		
			Self::validate_updater(&senders_acccount_id, &vc_id)?;

			Self::update_vc_status(vc_id, vc_status)?;

			Ok(())
		}
	}
}

impl<T: Config> Pallet<T> {
  fn initialize_vcs(init_vcs: &Vec<InitialVCs>) {
    for initial_vc in init_vcs.iter() {
      let vc_id = &initial_vc.vc_id;
      let vc_hex = &initial_vc.vc_hex;
      
      let vc = Self::decode_vc::<VC<T::Hash>>(&vc_hex).unwrap();

      VCs::<T>::insert(vc_id.clone(), Some(vc.clone()));
      let mut vcids = Lookup::<T>::get(vc.owner);
      vcids.push(*vc_id);
      Lookup::<T>::insert(vc.owner, vcids);
      
      RLookup::<T>::insert(vc_id, vc.owner);
    }
  }
  
  /// Decoding VC from encoded bytes
  pub fn decode_vc<E: codec::Decode>(mut vc_bytes: &[u8]) -> Result<E, DispatchError> {
    let vc: E = match Decode::decode(&mut vc_bytes) {
      Ok(vc) => vc,
      Err(_) => fail!(Error::<T>::InvalidVC),
    };
    Ok(vc)
  }

  /// Validate updater
  fn validate_updater(
    senders_acccount_id: &T::AccountId,
    vc_id: &VCid,
  ) -> Result<(), DispatchError> {
    
    // Check if sender's did exists on chain
    let senders_did = <T as pallet::Config>::DidResolution::get_did(&senders_acccount_id);
    ensure!(senders_did == None, Error::<T>::DidDoesNotExist);

    let senders_did = senders_did.unwrap();
    // Ensure either sender is one of the issuer or member of validator set
    if let Some(vc) = VCs::<T>::get(vc_id) {
      if !vc.issuers.contains(&senders_did)
        && !<T as pallet::Config>::IsValidator::is_validator(&senders_did)
      {
        fail!(Error::<T>::NotAValidatorNorIssuer);
      }
    };
    Ok(())
  }

  /// Validate slash/token/did vc
  fn validate_vcs(vc: &VC<T::Hash>, sender_did: &Did) -> Result<(), DispatchError> {
    match vc.vc_type {
      // derive slash/token vc
      VCType::SlashTokens | VCType::MintTokens => {
        let slash_or_mint: SlashMintTokens =
          Self::decode_vc::<SlashMintTokens>(&vc.vc_property)?;
        let token_vc_struct =
          if let Some(vc_struct) = VCs::<T>::get(&slash_or_mint.vc_id) {
            vc_struct
          } else {
            fail!(Error::<T>::LinkedVCNotFound);
          };
        ensure!(
          vc.issuers.contains(&token_vc_struct.owner),
          Error::<T>::VCNotSignedByTokenVCOwner
        );
      },

      VCType::TokenTransferVC => {
        // derive Transfer Tokens
        let transfer_tokens: TokenTransferVC = 
					Self::decode_vc::<TokenTransferVC>(&vc.vc_property)?;
        let token_vc_struct =
          if let Some(vc_struct) = VCs::<T>::get(&transfer_tokens.vc_id) {
            vc_struct
          } else {
            fail!(Error::<T>::LinkedVCNotFound);
          };
        ensure!(
          vc.issuers.contains(&token_vc_struct.owner),
          Error::<T>::VCNotSignedByTokenVCOwner
        );
      },

      VCType::PrivateDidVC => {
        let vc_property = Self::decode_vc::<PrivateDidVC>(&vc.vc_property)?;
        let public_key = vc_property.public_key;
        let account_id = T::AccountId::decode(&mut &public_key[..]).unwrap();

        // ensure the caller is a council member account
        let did_region = <T as pallet::Config>::IsValidator::get_region(vc_property.did);
        ensure!(<T as pallet::Config>::IsValidator::has_regional_permission(sender_did, did_region), Error::<T>::NotAValidator);
        ensure!(!<T as pallet::Config>::DidResolution::did_exists(MultiAddress::Id(account_id)), Error::<T>::PublicKeyRegistered);
      },

      VCType::PublicDidVC => {
        let vc_property = Self::decode_vc::<PublicDidVC>(&vc.vc_property)?;
        let public_key = vc_property.public_key;
        let account_id = T::AccountId::decode(&mut &public_key[..]).unwrap();

        // ensure the caller is a council member account
        let did_region = <T as pallet::Config>::IsValidator::get_region(vc_property.did);
        ensure!(<T as pallet::Config>::IsValidator::has_regional_permission(&sender_did, did_region), Error::<T>::NotAValidator);
        ensure!(!<T as pallet::Config>::DidResolution::did_exists(MultiAddress::Id(account_id)), Error::<T>::PublicKeyRegistered);
      },

      _ => {}
    }
    Ok(())
  }

  fn validate_currency_code(vc: &VC<T::Hash>) -> Result<(), DispatchError>  {
    let mut currency_code: Vec<u8> = Default::default();
    match vc.vc_type {
      VCType::TokenVC => {
        let vc_property: TokenVC =
          Self::decode_vc::<TokenVC>(&vc.vc_property)?;
        currency_code = vc_property.currency_code.into();
      },
      
      _ => { return Ok(()); }
    }
    currency_code.retain(|val| *val != 0);
    ensure!(!currency_code.contains(&0), Error::<T>::InvalidCurrencyCode);
    for &cc in currency_code.iter() {
      ensure!(cc.is_ascii_uppercase(), Error::<T>::InvalidCurrencyCode);
    }
    
    Ok(())
  }

  /// Validating VC
  pub fn is_vc_active(vc: &VC<T::Hash>) -> Result<IsVCActive, DispatchError> {
    if vc.vc_type != VCType::GenericVC {
      let hash = T::Hashing::hash_of(&(&vc.vc_type, &vc.vc_property, &vc.owner, &vc.issuers));
      // ensure the valid hash
      ensure!(vc.hash.eq(&hash), Error::<T>::VCPropertiesNotVerified);
    }

    // checking for duplicate issuers
    let mut issuers = vc.issuers.clone();
    let org_issuer_count = issuers.len();
    issuers.sort();
    issuers.dedup();
    if org_issuer_count != issuers.len() {
      fail!(Error::<T>::DuplicateSignature);
    }

    // checking for duplicate signatures
    let signatures = vc.signatures.clone();
    for i in 0..(signatures.len() - 1) {
      for j in (i + 1)..signatures.len() {
        if signatures[i] == signatures[j] {
          fail!(Error::<T>::DuplicateSignature);
        }
      }
    }

    // ensure the caller has all issuers' signature
    if vc.issuers.len() != vc.signatures.len() {
      return Ok(false);
    } else {
      let mut verified_count: usize = 0;
      for issuer in vc.issuers.iter() {
        ensure!(!<T as pallet::Config>::DidResolution::did_exists(MultiAddress::Did(*issuer)), Error::<T>::DidDoesNotExist);
        let public_key = <T as pallet::Config>::DidResolution::get_public_key(issuer).unwrap();
        
        for signature in vc.signatures.iter() {
          if signature.verify(vc.hash.as_ref(), &public_key) {
            verified_count += 1;
          }
        }
      }
      if verified_count != vc.signatures.len() {
        return Ok(false);
      }
    }
    Ok(true)
  }

  /// Store VC
  fn store_vc(identifier: Did, vc: VC<T::Hash>, vc_id: VCid) -> Result<(), DispatchError> {
    let current_block_no = <frame_system::Pallet<T>>::block_number();
    let vc_status = Self::is_vc_active(&vc)?;

    // Check if vc already exists
    ensure!(!RLookup::<T>::contains_key(&vc_id), Error::<T>::VCAlreadyExists);
        
    Self::set_approved_issuers(vc_id, &vc)?;

    VCs::<T>::insert(vc_id, Some(vc.clone()));
    RLookup::<T>::insert(vc_id, identifier);

    if Lookup::<T>::contains_key(&identifier) {
      let mut vc_ids = Lookup::<T>::get(identifier);
      vc_ids.push(vc_id);
      Lookup::<T>::insert(identifier, vc_ids);
    } else {
      Lookup::<T>::insert(identifier, vec![vc_id]);
    }

    VCHistory::<T>::insert(vc_id, Some((vc_status, current_block_no)));

    Ok(())
  }

  /// Update VC from storage
  fn update_vc_status(vc_id: VCid, status: IsVCActive) -> Result<(), DispatchError> {
    if let Some(vc) = VCs::<T>::get(&vc_id) {
      VCs::<T>::insert(vc_id, Some(vc));
    } else {
      fail!(Error::<T>::VCIdDoesNotExist);
    }

    if let Some(vc_history) = VCHistory::<T>::get(&vc_id) {
      VCHistory::<T>::insert(vc_id, Some((status, vc_history.1)));
    }
    Self::deposit_event(Event::VCStatusUpdated{ vcid: vc_id, vcstatus: status });
    Ok(())
  }

  // Update VC and vc_status from storage
  fn update_vc_and_status(vc_id: VCid, updated_vc: VC<T::Hash>) -> Result<(), DispatchError> {
    let status = Self::is_vc_active(&updated_vc)?;
    VCs::<T>::insert(vc_id, Some(updated_vc));

    if let Some(vc_history) = VCHistory::<T>::get(&vc_id) {
      VCHistory::<T>::insert(vc_id, Some((status, vc_history.1)));
    }

    Self::deposit_event(Event::VCStatusUpdated{ vcid: vc_id, vcstatus: status });
    Ok(())
  }

  /// Update vc's is_used flag to true
  pub fn set_is_used_flag(vc_id: VCid, is_vc_used: Option<bool>) {
    if let Some(mut vc) = VCs::<T>::get(&vc_id) {
      vc.is_vc_used = is_vc_used.unwrap_or(true);
      VCs::<T>::insert(vc_id, Some(vc));
    }
  }

  // Validate sign
  fn validate_sign(vc: &VC<T::Hash>, sign: Signature, vc_id: VCid) -> Result<(), DispatchError> {
    let mut is_sign_valid = false;
    let mut vc_approver_list = VCApproverList::<T>::get(vc_id);
    for issuer in vc.issuers.iter() {
      ensure!(!<T as pallet::Config>::DidResolution::did_exists(MultiAddress::Did(*issuer)), Error::<T>::DidDoesNotExist);
      let public_key = <T as pallet::Config>::DidResolution::get_public_key(&issuer).unwrap();
      
      if sign.verify(vc.hash.as_ref(), &public_key) {
        if vc_approver_list.contains(&issuer) {
          fail!(Error::<T>::DuplicateSignature);
        }
        vc_approver_list.push(*issuer);
        is_sign_valid = true;
      }
    }
    if !is_sign_valid {
      fail!(Error::<T>::InvalidSignature);
    }
    VCApproverList::<T>::insert(vc_id, vc_approver_list);
    Ok(())
  }

  fn set_approved_issuers(vc_id: VCid, vc: &VC<T::Hash>) -> Result<(), DispatchError> {
    let mut vc_approver_list = VCApproverList::<T>::get(vc_id);
    let signatures = vc.signatures.clone();
    // Check approved signatures
    for i in 0..signatures.len() {
      let sign = &signatures[i];
      let mut is_sign_valid = false;
      for issuer in vc.issuers.iter() {
        ensure!(<T as pallet::Config>::DidResolution::did_exists(MultiAddress::Did(*issuer)), Error::<T>::DidDoesNotExist);
        let public_key = <T as pallet::Config>::DidResolution::get_public_key(issuer).unwrap();

        if sign.verify(vc.hash.as_ref(), &public_key) {
          if vc_approver_list.contains(&issuer) {
            fail!(Error::<T>::DuplicateSignature);
          }
          is_sign_valid = true;
          vc_approver_list.push(*issuer);
        }
      }
      if !is_sign_valid {
        fail!(Error::<T>::InvalidSignature);
      }
    }
    VCApproverList::<T>::insert(vc_id, vc_approver_list);
    Ok(())
  }
}
