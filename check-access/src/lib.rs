#![cfg_attr(not(feature = "std"), no_std)]
use codec::{ Decode, Encode };
use frame_support::{ weights::DispatchInfo, traits::GetCallMetadata };
use sp_runtime::{
  traits::{ DispatchInfoOf, Dispatchable, SignedExtension },
  transaction_validity::{
    InvalidTransaction, TransactionLongevity, TransactionPriority, 
    TransactionValidityError, ValidTransaction,
  },
};

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

use sp_std::fmt::Debug;
use sp_std::marker::PhantomData;
use sp_std::prelude::*;
use scale_info::TypeInfo;
use metamui_primitives::{ Did, traits::{ DidResolve, MultiAddress } };
pub mod types;
use crate::types::*;
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {

use super::*;
  use frame_support::pallet_prelude::*;
  use frame_system::pallet_prelude::*;

  #[pallet::config]
  pub trait Config: frame_system::Config{
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		/// Trait to resolve did
		type DidResolution: DidResolve<Self::AccountId>;
    /// Sudo Origin
		type CallOrigin: EnsureOrigin<Self::Origin>;
	}
  
  #[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub initial_extrinsics: Vec<InitialExtrinsics>,
    pub blacklisted_dids: Vec<(Did, BlacklistReason)>,
    pub blacklisting_reasons: Vec<(ReasonCode, BlacklistReason)>,
    pub reasons_count: CurrentReasonCode,
		pub phantom: PhantomData<T>,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self {
				initial_extrinsics: Default::default(),
        blacklisted_dids: Default::default(),
        blacklisting_reasons: Default::default(),
        reasons_count: Default::default(),
				phantom: Default::default(),
			}
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			Pallet::<T>::initialise_extrinsics(&self.initial_extrinsics, &self.blacklisted_dids, &self.blacklisting_reasons, &self.reasons_count);
		}
	}

  #[pallet::pallet]
  #[pallet::generate_store(pub(super) trait Store)]
  pub struct Pallet<T>(_);
  
  #[pallet::storage]
  pub(super) type WhitelistedPallets<T> =  StorageDoubleMap<_, Blake2_128Concat, PalletName, Blake2_128Concat, FunctionName, (), ValueQuery>;

  #[pallet::storage]
  pub(super) type BlacklistedDids<T> =  StorageMap<_, Blake2_128Concat, Did, BlacklistReason, ValueQuery>;

  #[pallet::storage]
  pub(super) type BlacklistingReasons<T> =  StorageMap<_, Blake2_128Concat, ReasonCode, BlacklistReason, ValueQuery>;
  
  #[pallet::storage]
  pub(super) type BlacklistingReasonsRLookup<T> =  StorageMap<_, Blake2_128Concat, BlacklistReason, ReasonCode, ValueQuery>;

  #[pallet::storage]
  pub(super) type ReasonsCounter<T> =  StorageValue<_, CurrentReasonCode, ValueQuery>;


  #[pallet::hooks]
  impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}
  
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// An Extrinsic has been added to Whitelist
		ExtrinsicAdded { pallet_name: PalletName, function_name: FunctionName },
		/// An Extrinsic has been removed from Whitelist
		ExtrinsicRemoved { pallet_name: PalletName, function_name: FunctionName },
    /// A DID has been added to Blacklist
		DidBlacklisted { identifier: Did, reason_name: BlacklistReason},
    /// A DID has been removed from Blacklist
		DidWhitelisted { identifier: Did },
    /// A Blacklisting reason has been added
		ReasonAdded { reason_code: ReasonCode, reason_name: BlacklistReason},
	  /// A Blacklisting reason has been removed
    ReasonRemoved { reason_code: ReasonCode },
  }

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// The entered extrinsic is already added
		ExtrinsicAlreadyExists,
		/// The entered extrinsic is never added
		ExtrinsicDoesNotExist,
    /// The entered did does not exist on chain
		DidDoesNotExist,
    /// The entered did is not blacklisted
		DidIsNotBlacklisted,
		/// The entered did is already blacklisted
		DidAlreadyBlacklisted,
    /// The entered reason_code is invalid
		InvalidReasonCode,
    /// The entered reason is already added
    ReasonAlreadyAdded,
    /// The entered reason is not added
		ReasonIsNotAdded,
    /// The maximum custom reasons have been added
    MaximumReasonsAdded
	}

  #[pallet::call]
  impl<T: Config> Pallet<T> { 
    #[pallet::weight(1)]
    pub fn add_allowed_extrinsic(origin: OriginFor<T>, pallet_name: PalletName, function_name: FunctionName) -> DispatchResultWithPostInfo {
      T::CallOrigin::ensure_origin(origin)?;
			// ensure extrinsic is not already added
			ensure!(!WhitelistedPallets::<T>::contains_key(pallet_name, function_name), Error::<T>::ExtrinsicAlreadyExists);

      WhitelistedPallets::<T>::insert(pallet_name, function_name, ());
			Self::deposit_event(Event::ExtrinsicAdded{ pallet_name, function_name });
      Ok(().into())
    }
          
    #[pallet::weight(1)]
    pub fn remove_allowed_extrinsic(origin: OriginFor<T>, pallet_name: PalletName, function_name: FunctionName) -> DispatchResultWithPostInfo {
      T::CallOrigin::ensure_origin(origin)?;

			// ensure extrinsic exists on chain
			ensure!(WhitelistedPallets::<T>::contains_key(pallet_name, function_name), Error::<T>::ExtrinsicDoesNotExist);

      WhitelistedPallets::<T>::remove(pallet_name, function_name);
			Self::deposit_event(Event::ExtrinsicRemoved{ pallet_name, function_name });
      Ok(().into())
		}

    #[pallet::weight(1)]
    pub fn add_blacklisted_did(origin: OriginFor<T>, identifier: Did, reason_code: Option<u8>) -> DispatchResultWithPostInfo {
      T::CallOrigin::ensure_origin(origin)?;

      // ensure did exists on chain
      ensure!(T::DidResolution::did_exists(MultiAddress::Did(identifier)), Error::<T>::DidDoesNotExist);

			// ensure did is not already blacklisted
			ensure!(!BlacklistedDids::<T>::contains_key(identifier), Error::<T>::DidAlreadyBlacklisted);

      // fetch reason_name from reason_code
      let reason_name = match reason_code {
        Some(reason_code) => {
          // ensure reason_code is valid
          ensure!(BlacklistingReasons::<T>::contains_key(reason_code), Error::<T>::InvalidReasonCode);
          BlacklistingReasons::<T>::get(reason_code)
        }
        None => *b"Other\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"
      };

      // blacklist the did with fetched reason_name
      BlacklistedDids::<T>::insert(identifier, reason_name);

      // deposit event that the did has been blacklisted
			Self::deposit_event(Event::DidBlacklisted{ identifier, reason_name });
      Ok(().into())
    }
          
    #[pallet::weight(1)]
    pub fn remove_blacklisted_did(origin: OriginFor<T>, identifier: Did) -> DispatchResultWithPostInfo {
      T::CallOrigin::ensure_origin(origin)?;

      // ensure did exists on chain
      ensure!(T::DidResolution::did_exists(MultiAddress::Did(identifier)), Error::<T>::DidDoesNotExist);

			// ensure did is actually blacklisted
			ensure!(BlacklistedDids::<T>::contains_key(identifier), Error::<T>::DidIsNotBlacklisted);

      BlacklistedDids::<T>::remove(identifier);
			Self::deposit_event(Event::DidWhitelisted{ identifier });
      Ok(().into())
		}

    #[pallet::weight(1)]
    pub fn add_blacklisting_reason(origin: OriginFor<T>, reason_name: [u8; 32]) -> DispatchResultWithPostInfo {
      T::CallOrigin::ensure_origin(origin)?;

			// ensure reason is not already listed
			ensure!(!BlacklistingReasonsRLookup::<T>::contains_key(reason_name), Error::<T>::ReasonAlreadyAdded);

      // fetch and increment current number_of_reasons
      let number_of_reasons = ReasonsCounter::<T>::get();
      ensure!(number_of_reasons < 255, Error::<T>::MaximumReasonsAdded);
      ReasonsCounter::<T>::put(number_of_reasons+1);

      BlacklistingReasons::<T>::insert(number_of_reasons+1, reason_name);
      BlacklistingReasonsRLookup::<T>::insert(reason_name, number_of_reasons+1);
			Self::deposit_event(Event::ReasonAdded{ reason_code: number_of_reasons+1, reason_name });
      Ok(().into())
    }
          
    #[pallet::weight(1)]
    pub fn remove_blacklisting_reason(origin: OriginFor<T>, reason_code: u8) -> DispatchResultWithPostInfo {
      T::CallOrigin::ensure_origin(origin)?;

      // ensure reason is already listed
			ensure!(BlacklistingReasons::<T>::contains_key(reason_code), Error::<T>::ReasonIsNotAdded);

      let reason_name = BlacklistingReasons::<T>::get(reason_code);
      BlacklistingReasons::<T>::remove(reason_code);
      BlacklistingReasonsRLookup::<T>::remove(reason_name);
			Self::deposit_event(Event::ReasonRemoved{ reason_code });
      Ok(().into())
		}
  }
}

impl<T: Config> Pallet<T> { 
  fn check_pallet(pallet_name: PalletName, function_name: FunctionName) -> bool {
    <WhitelistedPallets<T>>::contains_key(pallet_name, function_name)
  }

  fn is_did_blacklisted(identifier: Did) -> bool {
    <BlacklistedDids<T>>::contains_key(identifier)
  }

  fn adjust_null_padding(name: &mut Vec<u8>) -> Vec<u8> {
    let len = 32;
    let diff = len - name.len();
    name.extend(sp_std::iter::repeat(0).take(diff));
    name.clone()
	}

	fn convert_to_array(name: Vec<u8>) -> [u8; 32] {
		(&name[..]).try_into().unwrap_or_default()
	}

  fn initialise_extrinsics(
    extrinsics: &Vec<InitialExtrinsics>, 
    blacklisted_dids: &Vec<(Did, BlacklistReason)>, 
    blacklisting_reasons: &Vec<(ReasonCode, BlacklistReason)>,
    current_reason_code: &CurrentReasonCode
  ) {
    for extrinsic in extrinsics.iter() {
      let pallet_name = extrinsic.pallet_name;
      let function_name = extrinsic.function_name;
      <WhitelistedPallets<T>>::insert(pallet_name, function_name, ());
    }

    for did_and_reason in blacklisted_dids.iter() {
      <BlacklistedDids<T>>::insert(did_and_reason.0, did_and_reason.1);
    }

    for code_and_reason in blacklisting_reasons.iter(){
      <BlacklistingReasons<T>>::insert(code_and_reason.0, code_and_reason.1);
      <BlacklistingReasonsRLookup<T>>::insert(code_and_reason.1, code_and_reason.0);
    }

    <ReasonsCounter<T>>::put(current_reason_code);
  }
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, Default, TypeInfo)]
pub struct AccessValid<T: Config + Send + Sync>(PhantomData<T>);

impl<T: Config + Send + Sync> AccessValid<T> {
  pub fn new() -> Self {
    Self(PhantomData)
	}
}

/// Debug impl for the `AccessValid` struct.
impl<T: Config + Send + Sync> Debug for AccessValid<T> {
  #[cfg(feature = "std")]
  fn fmt(&self, f: &mut sp_std::fmt::Formatter) -> sp_std::fmt::Result {
    write!(f, "AllowAccount")
  }
  
  #[cfg(not(feature = "std"))]
  fn fmt(&self, _: &mut sp_std::fmt::Formatter) -> sp_std::fmt::Result {
    Ok(())
  }
}

impl<T: Config + Send + Sync + scale_info::TypeInfo> SignedExtension for AccessValid<T>
where
  T::Call: Dispatchable<Info = DispatchInfo> + GetCallMetadata,
{
  type AccountId = T::AccountId;
  type Call = T::Call;
  type AdditionalSigned = ();
  type Pre = ();
  const IDENTIFIER: &'static str = "AllowAccount";

  fn additional_signed(&self) -> sp_std::result::Result<(), TransactionValidityError> {
		Ok(())
  }

  fn validate(
    &self,
    who: &Self::AccountId,
    call: &Self::Call,
    info: &DispatchInfoOf<Self::Call>,
    _len: usize,
  ) -> Result<ValidTransaction, TransactionValidityError> {
  
    let vec_pallet_name = <Pallet<T>>::adjust_null_padding(&mut call.get_call_metadata().pallet_name.as_bytes().to_vec());
    let vec_function_name = <Pallet<T>>::adjust_null_padding(&mut call.get_call_metadata().function_name.as_bytes().to_vec());

		let pallet_name = <Pallet<T>>::convert_to_array(vec_pallet_name);
		let function_name = <Pallet<T>>::convert_to_array(vec_function_name);

    let did_exists: bool = <T>::DidResolution::did_exists(MultiAddress::Id(who.clone()));

    if did_exists && <Pallet<T>>::is_did_blacklisted(<T>::DidResolution::get_did(who).unwrap()) {
			Err(InvalidTransaction::Custom(100).into())
    }

		else if <Pallet<T>>::check_pallet(pallet_name, function_name) || did_exists {
			Ok(ValidTransaction {
				priority: info.weight as TransactionPriority,
				longevity: TransactionLongevity::max_value(),
				propagate: true,
				..Default::default()
			})
		}

		else{
			Err(InvalidTransaction::Custom(230).into())
		}
  }

	fn pre_dispatch(
		self,
		who: &Self::AccountId,
	  call: &Self::Call,
		info: &DispatchInfoOf<Self::Call>,
		len: usize,
	) -> Result<Self::Pre, TransactionValidityError> {
    Self::validate(&self, who, call, info, len)?;
    Ok(())
  }
}