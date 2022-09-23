#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use pallet_did::types::*;
	use cumulus_pallet_xcm::{ensure_sibling_para, Origin as CumulusOrigin};
	use metamui_primitives::Did;
	use frame_system::Config as SystemConfig;
	use cumulus_primitives_core::ParaId;


	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		/// Origin for parachain
		type Origin: From<<Self as SystemConfig>::Origin>
			+ Into<Result<CumulusOrigin, <Self as Config>::Origin>>;
		/// On Update Did
		type OnUpdateDid: UpdateDid;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// New private did synced from a parachain
		NewPrivateDidSynced { did: Did, para_id: ParaId },

		NewPublicDidSynced { did: Did, para_id: ParaId },

		DidRemovalSynced { did: Did, para_id: ParaId },

		DidKeyUpdateSynced { did: Did, para_id: ParaId },

		DidMetadataUpdateSynced { did: Did, para_id: ParaId },

	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {

		/// Adds a DID on chain, where
		/// origin - the origin of the transaction
		/// sign_key - public signing key of the DID
		/// identifier - public unique identifier for the DID
		/// metadata - optional metadata to the DID - meant for bank nodes to display URL
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn create_private(
			origin: OriginFor<T>,
			public_key: PublicKey,
			identifier: Did,
			metadata: Metadata,
		) -> DispatchResult {
			// Check if origin is a from a parachain
			let para_id: ParaId = ensure_sibling_para(<T as Config>::Origin::from(origin))?;
			
			T::OnUpdateDid::add_private_did(
				public_key,
				identifier,
				metadata,
			)?;

			Self::deposit_event(Event::NewPrivateDidSynced { did: identifier, para_id });
			
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// Adds a DID on chain, where
		/// origin - the origin of the transaction
		/// sign_key - public signing key of the DID
		/// identifier - public unique identifier for the DID
		/// metadata - optional metadata to the DID - meant for bank nodes to display URL
		/// registration_number - Company registration number
		/// company_name - Company Name
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn create_public(
			origin: OriginFor<T>,
			public_key: PublicKey,
			identifier: Did,
			metadata: Metadata,
			registration_number: RegistrationNumber,
			company_name: CompanyName,
		) -> DispatchResult {
			// Check if origin is a from a parachain
			let para_id: ParaId = ensure_sibling_para(<T as Config>::Origin::from(origin))?;
			
			T::OnUpdateDid::add_public_did(
				public_key,
				identifier,
				metadata,
				registration_number,
				company_name,
			)?;

			// Emit an event.
			Self::deposit_event(Event::NewPublicDidSynced { did: identifier, para_id });

			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// Removes a DID from chain storage, where
		/// origin - the origin of the transaction
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn remove(origin: OriginFor<T>, identifier: Did) -> DispatchResult {
			// Check if origin is a from a parachain
			let para_id: ParaId = ensure_sibling_para(<T as Config>::Origin::from(origin))?;

			T::OnUpdateDid::remove_did(
				identifier,
			)?;
			
			// deposit an event that the DID has been removed
			Self::deposit_event(Event::DidRemovalSynced { did: identifier, para_id });

			Ok(())
		}

		/// Updates a DID public key on the chain
		/// origin - the origin of the transaction
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn rotate_key(
			origin: OriginFor<T>,
			identifier: Did,
			public_key: PublicKey,
		) -> DispatchResult {
			// Check if origin is a from a parachain
			let para_id: ParaId = ensure_sibling_para(<T as Config>::Origin::from(origin))?;

			T::OnUpdateDid::rotate_key(
				identifier,
				public_key,
			)?;

			// create key updated event
			Self::deposit_event(Event::DidKeyUpdateSynced { did: identifier, para_id });

			Ok(())
		}

		/// Updates DID metadata on the chain
		/// origin - the origin of the transaction
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn update_metadata(
			origin: OriginFor<T>,
			identifier: Did,
			metadata: Metadata,
		) -> DispatchResult {
			// Check if origin is a from a parachain
			let para_id: ParaId = ensure_sibling_para(<T as Config>::Origin::from(origin))?;

			T::OnUpdateDid::update_metadata(
				identifier,
				metadata,
			)?;

			// create metadata updated event
			Self::deposit_event(Event::DidMetadataUpdateSynced{ did: identifier, para_id });

			Ok(())
		}
	}
}
