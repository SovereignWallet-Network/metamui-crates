#![cfg_attr(not(feature = "std"), no_std)]

use crate::types::*;
use frame_support::{
	fail,
	pallet_prelude::*,
	sp_runtime::DispatchError,
	traits::{
		Currency as PalletCurrency, ExistenceRequirement, LockableCurrency, OnKilledAccount,
		OnNewAccount, ReservableCurrency, StoredMap,
	},
};
use frame_system::{pallet_prelude::*, split_inner};
use metamui_primitives::{
	traits::{DidResolve, HasVCId, MultiAddress, VCResolve},
	types::{SlashMintTokens, TokenTransferVC, VCType, VC},
	Did, VCid,
};
pub use pallet::*;

// #[cfg(test)]
// mod mock;
// #[cfg(test)]
// mod tests;
#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

mod types;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	type BalanceOf<T> = <<T as Config>::Currency as PalletCurrency<
		<T as frame_system::Config>::AccountId,
	>>::Balance;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		/// Validator Origin
		type WithdrawOrigin: EnsureOrigin<Self::Origin>;
		/// The staking balance.
		type Currency: LockableCurrency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
		/// Resolve Did from account Id
		type DidResolution: DidResolve<Self::AccountId>;
		/// Resolve VC Data
		type VCResolution: VCResolve<Self::Hash>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn account)]
	pub type Account<T: Config> =
		StorageMap<_, Blake2_128Concat, Did, AccountInfo<T::Index, T::AccountData>, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Withdrawn reserve from account
		ReserveWithdrawn { from: Did, to: Did },
		/// Token amount is slashed
		TokenSlashed { balance: BalanceOf<T>, vc_id: VCid },
		/// Token amount is minted
		TokenMinted { balance: BalanceOf<T>, vc_id: VCid },
		/// Token amount is tranfered
		TransferredWithVC { to: Did, balance: BalanceOf<T>, vc_id: VCid },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Did Not Exists
		DIDDoesNotExist,
		/// Balance too low
		BalanceTooLow,
		/// Unable to decode the VC
		InvalidVC,
		/// VC is not owned by the given DID
		DidNotRegisteredWithVC,
		/// Linked VC does not exist
		LinkedVCNotFound,
		/// The given VCId does not exist on chain
		VCIdDoesNotExist,
		/// VC status is Inactive, cant be use it
		VCIsNotActive,
		/// VC is already used, can't reused
		VCAlreadyUsed,
		/// Recipent DID Not Registered
		RecipentDIDNotRegistered,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		// Transfer to admin from reserved amount for operational costs
		// The dispatch origin for this call must be `Signed` by a validator account.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn withdraw_reserved(
			origin: OriginFor<T>,
			to: Did,
			from: Did,
			amount: BalanceOf<T>,
		) -> DispatchResult {
			let _ = T::WithdrawOrigin::ensure_origin(origin)?;
			ensure!(
				T::DidResolution::did_exists(MultiAddress::Did(to)),
				Error::<T>::DIDDoesNotExist
			);
			ensure!(
				T::DidResolution::did_exists(MultiAddress::Did(from)),
				Error::<T>::DIDDoesNotExist
			);

			let from_acc = T::DidResolution::get_account_id(&from).unwrap();
			let to_acc = T::DidResolution::get_account_id(&to).unwrap();

			// unreserve the mui balance required to issue new token
			T::Currency::unreserve(&from_acc, amount);
			// transfer amount to destination
			T::Currency::transfer(&from_acc, &to_acc, amount, ExistenceRequirement::KeepAlive)?;
			Self::deposit_event(Event::ReserveWithdrawn { from, to });
			Ok(())
		}

		/// Slash the balance from the issuer account
		///
		/// The dispatch origin for this call must be `Signed` by a issuer account.
		#[pallet::weight(1)]
		pub fn slash_token(origin: OriginFor<T>, vc_id: VCid) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			let vc_struct =
				Self::validate_vc(&sender, vc_id, VCType::SlashTokens, Error::<T>::InvalidVC)?;
			let slash_vc: SlashMintTokens =
				T::VCResolution::decode_vc::<SlashMintTokens>(&vc_struct.vc_property)?;
			let amount: BalanceOf<T> = slash_vc.amount.try_into().ok().unwrap_or_default();
			let vc_owner = Self::get_vc_owner::<SlashMintTokens>(vc_struct)?;
			ensure!(T::Currency::can_slash(&vc_owner, amount), Error::<T>::BalanceTooLow);
			T::Currency::slash(&vc_owner, amount);
			// update vc's is_used flag as used
			T::VCResolution::set_is_vc_used(&vc_id, true);
			Self::deposit_event(Event::TokenSlashed { balance: amount, vc_id });
			Ok(().into())
		}

		#[pallet::weight(1)]
		pub fn transfer_token(
			origin: OriginFor<T>,
			vc_id: VCid,
			to: Did,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			ensure!(
				T::DidResolution::did_exists(MultiAddress::Did(to)),
				Error::<T>::RecipentDIDNotRegistered
			);
			let vc_struct =
				Self::validate_vc(&sender, vc_id, VCType::TokenTransferVC, Error::<T>::InvalidVC)?;
			let transfer_vc: TokenTransferVC =
				T::VCResolution::decode_vc::<TokenTransferVC>(&vc_struct.vc_property)?;
			let amount: BalanceOf<T> = transfer_vc.amount.try_into().ok().unwrap_or_default();
			let vc_owner = Self::get_vc_owner::<TokenTransferVC>(vc_struct)?;
			let to_acc = T::DidResolution::get_account_id(&to).unwrap();
			T::Currency::transfer(&vc_owner, &to_acc, amount, ExistenceRequirement::KeepAlive)?;
			// update vc's is_used flag as used
			T::VCResolution::set_is_vc_used(&vc_id, true);
			Self::deposit_event(Event::TransferredWithVC { to, balance: amount, vc_id });
			Ok(().into())
		}

		/// Add amount to the issuer account
		///
		/// The dispatch origin for this call must be `Signed` by a issuer account.
		/// Sender must be part of vc
		#[pallet::weight(1)]
		pub fn mint_token(origin: OriginFor<T>, vc_id: VCid) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			let vc_struct =
				Self::validate_vc(&sender, vc_id, VCType::MintTokens, Error::<T>::InvalidVC)?;
			let mint_vc: SlashMintTokens =
				T::VCResolution::decode_vc::<SlashMintTokens>(&vc_struct.vc_property)?;
			let amount: BalanceOf<T> = mint_vc.amount.try_into().ok().unwrap_or_default();
			let vc_owner = Self::get_vc_owner::<SlashMintTokens>(vc_struct)?;
			T::Currency::deposit_creating(&vc_owner,amount);
			// update vc's is_used flag as used
			T::VCResolution::set_is_vc_used(&vc_id, true);
			Self::deposit_event(Event::TokenMinted { balance: amount, vc_id });
			Ok(().into())
		}
	}

	impl<T: Config> Pallet<T> {
		/// An account is being created.
		pub fn on_created_account(did: Did) {
			let who = T::DidResolution::get_account_id(&did);
			if who.is_some() {
				T::OnNewAccount::on_new_account(&who.unwrap());
			}
		}

		/// Do anything that needs to be done after an account has been killed.
		pub fn on_killed_account(did: Did) {
			let who = T::DidResolution::get_account_id(&did);
			if who.is_some() {
				T::OnKilledAccount::on_killed_account(&who.unwrap());
			}
		}
		// Validate vc
		fn validate_vc(
			senders_acccount_id: &T::AccountId,
			vc_id: VCid,
			vc_type: VCType,
			vc_type_error: Error<T>,
		) -> Result<VC<T::Hash>, DispatchError> {
			let senders_did = T::DidResolution::get_did(&senders_acccount_id);
			let vc_struct = Self::get_vc_struct(vc_id, vc_type, vc_type_error)?;

			// ensure sender has associated vc
			ensure!(senders_did.eq(&Some(vc_struct.owner)), Error::<T>::DidNotRegisteredWithVC);
			Ok(vc_struct)
		}

		/// Get VC Owner
		fn get_vc_owner<G: codec::Decode + HasVCId>(
			vc_struct: VC<T::Hash>,
		) -> Result<T::AccountId, DispatchError> {
			let vc_property: G = T::VCResolution::decode_vc::<G>(&vc_struct.vc_property)?;

			let token_vc_struct =
				if let Some(vc_struct) = T::VCResolution::get_vc(&vc_property.vc_id()) {
					vc_struct
				} else {
					fail!(Error::<T>::LinkedVCNotFound);
				};

			let owners_acc_id = if let Some(owners_acc_id) =
				T::DidResolution::get_account_id(&token_vc_struct.owner)
			{
				owners_acc_id
			} else {
				fail!(Error::<T>::DIDDoesNotExist);
			};
			Ok(owners_acc_id)
		}
		// Get vc struct
		fn get_vc_struct(
			vc_id: VCid,
			vc_type: VCType,
			vc_type_error: Error<T>,
		) -> Result<VC<T::Hash>, DispatchError> {
			// ensure vc exists
			let vc_struct = if let Some(vc_struct) = T::VCResolution::get_vc(&vc_id) {
				vc_struct
			} else {
				fail!(Error::<T>::VCIdDoesNotExist);
			};

			// ensure vc is active
			ensure!(vc_struct.is_vc_active.eq(&true), Error::<T>::VCIsNotActive);

			// ensure vc_type
			ensure!(vc_struct.vc_type.eq(&vc_type), vc_type_error);

			// ensure VC is unused
			ensure!(!vc_struct.is_vc_used, Error::<T>::VCAlreadyUsed);

			Ok(vc_struct)
		}
	}
}

// Implement StoredMap for a simple single-item, kill-account-on-remove system. This works fine for
// storing a single item which is required to not be empty/default for the account to exist.
// Anything more complex will need more sophisticated logic.
impl<T: Config> StoredMap<T::AccountId, T::AccountData> for Pallet<T> {
	fn get(k: &T::AccountId) -> T::AccountData {
		let did = T::DidResolution::get_did(k).unwrap_or_default();
		Account::<T>::get(did).data
	}

	fn try_mutate_exists<R, E: From<DispatchError>>(
		k: &T::AccountId,
		f: impl FnOnce(&mut Option<T::AccountData>) -> Result<R, E>,
	) -> Result<R, E> {
		let did = T::DidResolution::get_did(k).unwrap_or_default();
		Account::<T>::try_mutate_exists(did, |maybe_value| {
			let existed = maybe_value.is_some();
			let (maybe_prefix, mut maybe_data) = split_inner(maybe_value.take(), |account| {
				(
					(account.nonce, account.consumers, account.providers, account.sufficients),
					account.data,
				)
			});
			f(&mut maybe_data).map(|result| {
				*maybe_value = maybe_data.map(|data| {
					let (nonce, consumers, providers, sufficients) =
						maybe_prefix.unwrap_or_default();
					AccountInfo { nonce, consumers, providers, sufficients, data }
				});
				(existed, maybe_value.is_some(), result)
			})
		})
		.map(|(existed, exists, v)| {
			if !existed && exists {
				Self::on_created_account(did.clone());
			} else if existed && !exists {
				Self::on_killed_account(did.clone());
			}
			v
		})
	}
}
