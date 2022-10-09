#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

// #[cfg(test)]
// mod mock;
// #[cfg(test)]
// mod tests;

#[cfg(feature = "std")]
pub use serde;
#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
// #[cfg(test)]
// mod tests;
pub mod types;

mod impls;
pub use crate::impls::*;

#[frame_support::pallet]
pub mod pallet {
	use codec::Decode;
	use frame_support::{ pallet_prelude::{ *, DispatchResult }, BoundedVec };
	use frame_system::{ self, pallet_prelude::*} ;
	use sp_std::vec::Vec;
	use crate::types::*;

	use metamui_primitives::{ VCid, types::PublicDidVC, traits::VCResolve, };

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		/// Validator Origin
		type ValidatorOrigin: EnsureOrigin<Self::Origin>;
		/// Maximum number of key changes by an account
		type MaxKeyChanges: Get<u32>;
		/// On Did update
		type OnDidUpdate: DidUpdated;
		/// Trait to resolve VC
		type VCResolution: VCResolve<Self::Hash>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// the map for storing did information
	#[pallet::storage]
	pub type DIDs<T: Config> =
		StorageMap<_, Blake2_128Concat, Did, (DIDType, T::BlockNumber), OptionQuery>;

	// map to enable lookup from did to account id
	#[pallet::storage]
	pub type Lookup<T: Config> = StorageMap<_, Blake2_128Concat, Did, T::AccountId, OptionQuery>;

	// map to enable reverse lookup
	#[pallet::storage]
	pub type RLookup<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, Did, OptionQuery>;

	// map to store history of key rotation
	#[pallet::storage]
	pub type PrevKeys<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		Did,
		BoundedVec<(T::AccountId, T::BlockNumber), T::MaxKeyChanges>,
		OptionQuery,
	>;

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub initial_dids: Vec<DIDType>,
		pub phantom: PhantomData<T>,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self {
				initial_dids: Default::default(),
				phantom: Default::default(),
			}
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			Pallet::<T>::initialize_dids(&self.initial_dids);
		}
	}


	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A DID has been created
		DidCreated { did: Did },
		/// A DID has been removed
		DidRemoved { did: Did },
		/// DID key have been rotated
		DidKeyUpdated { did: Did },
		/// DID Metadata has been updated
		DidMetadataUpdated { did: Did },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		/// The given DID already exists on chain
		DIDAlreadyExists,
		/// Invalid DID, either format or length is wrong
		InvalidDid,
		/// PublicKey already linked to another DID on chain
		PublicKeyRegistered,
		/// The given DID does not exist on chain
		DIDDoesNotExist,
		/// The operation is restricted to the validator only
		NotAValidator,
    /// The given VCId does not exist on chain
		VCIdDoesNotExist,
		/// The entered VCId is not eligible to create Did
		InvalidVC
	}

	#[pallet::call]
	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	impl<T: Config> Pallet<T> {
		/// Adds a DID on chain, where
		/// _origin - the origin of the transaction
		/// public_key - public signing key of the DID
		/// vc_id - The id of the VC that is authorized to create this DID
		/// identifier - public unique identifier for the DID
		/// metadata - optional metadata to the DID - meant for bank nodes to display URL
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn create_private(
			_origin: OriginFor<T>,
			public_key: PublicKey,
			vc_id: VCid,
			identifier: Did,
			metadata: Metadata,
		) -> DispatchResult {
			// Check if the VCId exists on chain
			let vcs_details_option = T::VCResolution::get_vc(&vc_id);
			ensure!(vcs_details_option == None, Error::<T>::VCIdDoesNotExist);
			let vcs_details = vcs_details_option.unwrap();
			// Verify if the vc is valid
			ensure!(!Self::verify_did_vc(vcs_details, VCType::PrivateDidVC), Error::<T>::InvalidVC);
			// Create the did
			Self::do_create_private_did(public_key, identifier, metadata.clone())?;
			// Emit an event.
			Self::deposit_event(Event::DidCreated { did: identifier });

			T::OnDidUpdate::on_new_private_did(
				public_key,
				identifier,
				metadata,
			);
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
			_origin: OriginFor<T>,
			public_key: PublicKey,
			vc_id: VCid,
			identifier: Did,
			metadata: Metadata,
			// registration_number: RegistrationNumber,
			// company_name: CompanyName,
		) -> DispatchResult {
			// Verify if the vc is valid
			let vcs_details_option = T::VCResolution::get_vc(&vc_id);
			ensure!(vcs_details_option == None, Error::<T>::VCIdDoesNotExist);
			let vcs_details = vcs_details_option.unwrap();
			// Decode the VC for getting the registration number and company name
			let did_vc_property = T::VCResolution::decode_vc::<PublicDidVC>(&vcs_details.vc_property)?;
			// Create the did
			Self::do_create_public_did(public_key, identifier, metadata.clone(), did_vc_property.registration_number.clone(), did_vc_property.company_name.clone())?;
			// Emit an event.
			Self::deposit_event(Event::DidCreated { did: identifier });

			T::OnDidUpdate::on_new_public_did(
				public_key,
				identifier,
				metadata,
				did_vc_property.registration_number,
				did_vc_property.company_name,
			);
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// Removes a DID from chain storage, where
		/// origin - the origin of the transaction
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn remove(origin: OriginFor<T>, identifier: Did) -> DispatchResult {
			// Check if origin is a from a validator
			T::ValidatorOrigin::ensure_origin(origin)?;

			Self::do_remove(&identifier)?;

			// deposit an event that the DID has been removed
			Self::deposit_event(Event::DidRemoved{ did: identifier });

			T::OnDidUpdate::on_did_removal(
				identifier,
			);

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
			// Check if origin is a from a validator
			T::ValidatorOrigin::ensure_origin(origin)?;

			Self::do_rotate_key(&identifier, &public_key)?;

			// create key updated event
			Self::deposit_event(Event::DidKeyUpdated{ did: identifier });

			T::OnDidUpdate::on_key_rotation(
				identifier,
				public_key,
			);

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
			// Check if origin is a from a validator
			T::ValidatorOrigin::ensure_origin(origin)?;

			Self::do_update_metadata(&identifier, &metadata)?;

			// create metadata updated event
			Self::deposit_event(Event::DidMetadataUpdated{ did: identifier });

			T::OnDidUpdate::on_metadata_updation(
				identifier,
				metadata,
			);

			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		/// Function to check vc when trying to create dids
		fn verify_did_vc(vcs_details: VC<T::Hash>, did_vc_type: VCType) -> bool {
			if vcs_details.vc_type == did_vc_type && vcs_details.is_vc_active && !vcs_details.is_vc_used {
				true
			} else {
				false
			}
		}

		/// Function to check if did which is going to be created is valid or not
		pub fn is_did_valid(identifier: Did) -> bool {
			let did_colon: [u8; 4] = [100, 105, 100, 58];
			let did_all_zeros: [u8; 32] = [0; 32];
			let did_four_zeros: [u8; 4] = [0; 4];
			let mut did_four_seg = identifier.chunks_exact(4);
			!identifier.is_empty() &&
				identifier.ne(&did_all_zeros) &&
				did_four_seg.next().eq(&Some(&did_colon)) &&
				!did_four_seg.next().eq(&Some(&did_four_zeros))
		}

		/// get the details of the pubkey attached to the DID
		pub fn get_did_details(
			identifier: Did,
		) -> Result<(DIDType, T::BlockNumber), DispatchError> {
			// fetch did details and last updated block
			if let Some((did_doc, last_updated_block)) = DIDs::<T>::get(identifier) {
				Ok((did_doc, last_updated_block))
			} else {
				frame_support::fail!(Error::<T>::DIDDoesNotExist)
			}
		}

		/// get the details of the previous keys attached to the DID
		pub fn get_prev_key_details(
			identifier: Did,
		) -> Result<BoundedVec<(T::AccountId, T::BlockNumber), T::MaxKeyChanges>, DispatchError> {
			// fetch did details and last updated block
			if let Some(prev_key_list) = PrevKeys::<T>::get(identifier) {
				Ok(prev_key_list)
			} else {
				let my_vec: BoundedVec<(T::AccountId, T::BlockNumber), T::MaxKeyChanges> = Default::default();
				Ok(my_vec)
			}
		}

		/// Simple type conversion between sr25519::Public and AccountId
		/// Should not panic for any valid sr25519 - need to make more robust to check for valid
		/// publicKey
		pub fn get_accountid_from_pubkey(pk: &PublicKey) -> T::AccountId {
			//convert a publickey to an accountId
			// TODO : Need a better way to handle the option failing?
			T::AccountId::decode(&mut &pk[..]).unwrap()
		}

		/// Initialize did during genesis
		fn initialize_dids(dids: &Vec<DIDType>) {
			for did in dids.iter() {
				// This is called only in genesis, hence 0
				let block_no: T::BlockNumber = 0u32.into();

				// Did could be either public or private
				let (identifier, public_key): (Did, PublicKey) = match did {
					// Private Did
					DIDType::Private(private_did) => {
						// Add Private DID to the storage
						DIDs::<T>::insert(
							private_did.identifier.clone(),
							(
								DIDType::Private(PrivateDid {
									identifier: private_did.identifier.clone(),
									public_key: private_did.public_key,
									metadata: private_did.metadata.clone(),
								}),
								block_no,
							),
						);
						(private_did.identifier, private_did.public_key)
					},
					// Public Did
					DIDType::Public(public_did) => {
						// Add Public DID to the storage
						DIDs::<T>::insert(
							public_did.identifier.clone(),
							(
								DIDType::Public(PublicDid {
									identifier: public_did.identifier.clone(),
									public_key: public_did.public_key,
									metadata: public_did.metadata.clone(),
									registration_number: public_did
										.registration_number
										.clone(),
									company_name: public_did.company_name.clone(),
								}),
								block_no,
							),
						);
						(public_did.identifier, public_did.public_key)
					},
				};
				Lookup::<T>::insert(
					identifier.clone(),
					Self::get_accountid_from_pubkey(&public_key),
				);
				RLookup::<T>::insert(
					Self::get_accountid_from_pubkey(&public_key),
					identifier,
				);
			}
		}

		/// Create Private Did
		pub fn do_create_private_did(
			public_key: PublicKey,
			identifier: Did,
			metadata: Metadata,
		) -> DispatchResult {

			// ensure did is valid
			ensure!(Self::is_did_valid(identifier.clone()), Error::<T>::InvalidDid);

			// ensure did is not already taken
			ensure!(!DIDs::<T>::contains_key(identifier.clone()), Error::<T>::DIDAlreadyExists);

			// ensure the public key is not already linked to a DID
			ensure!(
				!RLookup::<T>::contains_key(Self::get_accountid_from_pubkey(&public_key)),
				Error::<T>::PublicKeyRegistered
			);

			let current_block_no = <frame_system::Pallet<T>>::block_number();

			// add DID to the storage
			DIDs::<T>::insert(
				identifier.clone(),
				(
					DIDType::Private(PrivateDid {
						identifier: identifier.clone(),
						public_key,
						metadata,
					}),
					current_block_no,
				),
			);

			Lookup::<T>::insert(identifier.clone(), Self::get_accountid_from_pubkey(&public_key));
			RLookup::<T>::insert(Self::get_accountid_from_pubkey(&public_key), identifier.clone());

			Ok(())
		}

		/// Create Public Did
		pub fn do_create_public_did(
			public_key: PublicKey,
			identifier: Did,
			metadata: Metadata,
			registration_number: RegistrationNumber,
			company_name: CompanyName,
		) -> DispatchResult {

			// ensure did is valid
			ensure!(Self::is_did_valid(identifier.clone()), Error::<T>::InvalidDid);

			// ensure did is not already taken
			ensure!(!DIDs::<T>::contains_key(identifier.clone()), Error::<T>::DIDAlreadyExists);

			// ensure the public key is not already linked to a DID
			ensure!(
				!RLookup::<T>::contains_key(Self::get_accountid_from_pubkey(&public_key)),
				Error::<T>::PublicKeyRegistered
			);

			let current_block_no = <frame_system::Pallet<T>>::block_number();

			// add DID to the storage
			DIDs::<T>::insert(
				identifier.clone(),
				(
					DIDType::Public(PublicDid {
						identifier: identifier.clone(),
						public_key,
						metadata,
						registration_number,
						company_name,
					}),
					current_block_no,
				),
			);

			Lookup::<T>::insert(identifier.clone(), Self::get_accountid_from_pubkey(&public_key));
			RLookup::<T>::insert(Self::get_accountid_from_pubkey(&public_key), identifier.clone());

			Ok(())
		}
	
		/// Update metadata of public and private did
		pub fn do_update_metadata(identifier: &Did, metadata: &Metadata) -> DispatchResult {

			// reject if the user does not already have DID registered
			ensure!(DIDs::<T>::contains_key(&identifier), Error::<T>::DIDDoesNotExist);

			// fetch the existing DID document
			let (did_doc, block_number) = Self::get_did_details(identifier.clone())?;

			// modify the public_key of the did doc
			match did_doc {
				DIDType::Public(public_did) => {
					DIDs::<T>::insert(
						identifier.clone(),
						(
							DIDType::Public(PublicDid { metadata: metadata.clone(), ..public_did }),
							block_number,
						),
					);
				},
				DIDType::Private(private_did) => {
					DIDs::<T>::insert(
						identifier.clone(),
						(
							DIDType::Private(PrivateDid { metadata: metadata.clone(), ..private_did }),
							block_number,
						),
					);
				},
			}

			Ok(())
		}
	
		/// Rotate key of public and private did
		pub fn do_rotate_key(identifier: &Did, public_key: &PublicKey) -> DispatchResult {

			//reject if the user does not already have DID registered
			ensure!(DIDs::<T>::contains_key(&identifier), Error::<T>::DIDDoesNotExist);

			// ensure the public key is not already linked to a DID
			ensure!(
				!RLookup::<T>::contains_key(Self::get_accountid_from_pubkey(&public_key)),
				Error::<T>::PublicKeyRegistered
			);

			// fetch the existing DID document
			let (did_doc, last_updated_block) = Self::get_did_details(identifier.clone())?;
			// Get block number
			let current_block_no = <frame_system::Pallet<T>>::block_number();

			let prev_public_key: PublicKey = match did_doc {
				DIDType::Public(public_did) => {
					DIDs::<T>::insert(
						identifier.clone(),
						(
							DIDType::Public(PublicDid {
								identifier: identifier.clone(),
								public_key: public_key.clone(),
								metadata: public_did.metadata.clone(),
								registration_number: public_did.registration_number.clone(),
								company_name: public_did.company_name.clone(),
							}),
							current_block_no,
						),
					);
					public_did.public_key
				},

				DIDType::Private(private_did) => {
					DIDs::<T>::insert(
						identifier.clone(),
						(
							DIDType::Private(PrivateDid {
								identifier: identifier.clone(),
								public_key: public_key.clone(),
								metadata: private_did.metadata.clone(),
							}),
							current_block_no,
						),
					);
					private_did.public_key
				},
			};

			// Remove previous lookup of pubkey to DID
			RLookup::<T>::remove(Self::get_accountid_from_pubkey(
				&prev_public_key,
			));

			// Store the previous key to history
			let mut prev_keys = Self::get_prev_key_details(identifier.clone())?;
			prev_keys
				.try_push((
					Self::get_accountid_from_pubkey(&prev_public_key),
					last_updated_block,
				))
				.ok();

			PrevKeys::<T>::insert(identifier.clone(), prev_keys);

			Lookup::<T>::insert(
				identifier.clone(),
				Self::get_accountid_from_pubkey(&public_key),
			);

			RLookup::<T>::insert(
				Self::get_accountid_from_pubkey(&public_key),
				identifier.clone(),
			);

			Ok(())
		}
	
		/// Remove Did 
		pub fn do_remove(identifier: &Did) -> DispatchResult {
			
			let (did_doc, _) = Self::get_did_details(identifier.clone())?;

			// remove DID from storage
			DIDs::<T>::remove(&identifier);

			Lookup::<T>::remove(identifier.clone());
			match did_doc {
				DIDType::Public(public_did) => {
					RLookup::<T>::remove(Self::get_accountid_from_pubkey(
						&public_did.public_key,
					));
				},
				DIDType::Private(private_did) => {
					RLookup::<T>::remove(Self::get_accountid_from_pubkey(
						&private_did.public_key,
					));
				},
			}

			Ok(())
		}
	
	}

}