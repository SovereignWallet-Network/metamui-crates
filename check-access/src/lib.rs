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

use metamui_primitives::traits::{ DidResolve, MultiAddress };
pub mod types;
use crate::types::*;
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
  use super::*;
  use frame_support::pallet_prelude::*;
  use frame_system::pallet_prelude::*;
  
  /// Configure the pallet by specifying the parameters and types on which it depends.
  #[pallet::config]
  pub trait Config: frame_system::Config{
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		/// Trait to resolve did
		type DidResolution: DidResolve<Self::AccountId>;
	}
  
  #[pallet::pallet]
  #[pallet::generate_store(pub(super) trait Store)]
  pub struct Pallet<T>(_);
  
  #[pallet::storage]
  pub(super) type WhitelistedPallets<T: Config> = StorageMap<_, Blake2_128Concat, ExtrinsicsStruct, (), ValueQuery>;

  #[pallet::hooks]
  impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}
  
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// An Extrinsic has been added to WhiteList
		ExtrinsicAdded { extrinsic: ExtrinsicsStruct },
		/// An Extrinsic has been removed from WhiteList
		ExtrinsicRemoved { extrinsic: ExtrinsicsStruct },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// The entered extrinsic already exists on chain
		ExtrinsicAlreadyExists,
		/// The entered extrinsic does not exist on chain
		ExtrinsicDoesNotExist,
	}

  #[pallet::call]
  impl<T: Config> Pallet<T> { 
    #[pallet::weight(1)]
    pub fn add_allowed_extrinsic(origin: OriginFor<T>, pallet_name: PalletName, function_name: FunctionName) -> DispatchResultWithPostInfo {
      ensure_root(origin)?;

			// ensure extrinsic is not already added
			let extrinsic = ExtrinsicsStruct { pallet_name, function_name }; 
			ensure!(!WhitelistedPallets::<T>::contains_key(extrinsic.clone()), Error::<T>::ExtrinsicAlreadyExists);

      WhitelistedPallets::<T>::insert(extrinsic.clone(), ());
			Self::deposit_event(Event::ExtrinsicAdded{ extrinsic });
      Ok(().into())
    }
          
    #[pallet::weight(1)]
    pub fn remove_allowed_extrinsic(origin: OriginFor<T>, pallet_name: PalletName, function_name: FunctionName) -> DispatchResultWithPostInfo {
      ensure_root(origin)?;

			// ensure extrinsic exists on chain
			let extrinsic = ExtrinsicsStruct { pallet_name, function_name };
			ensure!(WhitelistedPallets::<T>::contains_key(extrinsic.clone()), Error::<T>::ExtrinsicDoesNotExist);

      WhitelistedPallets::<T>::remove(extrinsic.clone());
			Self::deposit_event(Event::ExtrinsicRemoved{ extrinsic });
      Ok(().into())
		}
  }
}

impl<T: Config> Pallet<T> { 
  fn check_pallet(pallet_name: PalletName, function_name: FunctionName) -> bool{
		let extrinsic = ExtrinsicsStruct { pallet_name, function_name };
    <WhitelistedPallets<T>>::contains_key(extrinsic)
  }

	fn convert_to_array(name: Vec<u8>) -> [u8; 32] {
		(&name[..]).try_into().unwrap_or_default()
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
  
		let pallet_name = <Pallet<T>>::convert_to_array(
			call.get_call_metadata().pallet_name.as_bytes().to_vec()
		);

		let function_name = <Pallet<T>>::convert_to_array(
			call.get_call_metadata().pallet_name.as_bytes().to_vec()
		);

		if <Pallet<T>>::check_pallet(pallet_name, function_name) || <T>::DidResolution::did_exists(MultiAddress::Id(who.clone())) {
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
    match Self::validate(&self, who, call, info, len){
      Ok(_) => Ok(()),
      Err(_) => Err(InvalidTransaction::Custom(230).into())
    }
  }
}