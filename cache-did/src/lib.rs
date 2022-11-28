#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

// #[cfg(test)]
// mod mock;

// #[cfg(test)]
// mod tests;

mod impls;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
    use cumulus_pallet_xcm::{ensure_relay, Origin as CumulusOrigin};
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use frame_system::Config as SystemConfig;
    use metamui_primitives::{Did, types::DidType};
    use sp_runtime::traits::BlockNumberProvider;
    use pallet_did::types::*;
    use sp_std::vec::Vec;

    #[derive(Encode, Decode, Clone, PartialEq, Eq, Debug)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct DidStruct {
        pub identifier: Did,
        pub public_key: PublicKey,
        pub is_public: bool,
    }

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        type Origin: From<<Self as SystemConfig>::Origin>
            + Into<Result<CumulusOrigin, <Self as Config>::Origin>>;

        /// The overarching call type; we assume sibling chains use the same type.
        type Call: From<Call<Self>> + Encode;

        /// Block number of Relay chain
        type RelayChainBlockNumber: BlockNumberProvider<BlockNumber = Self::BlockNumber>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    // the map for storing did information
    #[pallet::storage]
    pub type PublicKeyMap<T: Config> = StorageMap<_, Blake2_128Concat, Did, PublicKey, OptionQuery>;

    // map to enable lookup from did to account id
    #[pallet::storage]
    pub type Lookup<T: Config> = StorageMap<_, Blake2_128Concat, Did, T::AccountId, OptionQuery>;

    // map to enable reverse lookup
    #[pallet::storage]
    pub type RLookup<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, Did, OptionQuery>;

    // map for Did type
    #[pallet::storage]
    pub type DidTypeMap<T: Config> = StorageMap<_, Blake2_128Concat, Did, DidType, OptionQuery>;

    // Map for Last Updated On
    #[pallet::storage]
    pub type LastUpdatedMap<T: Config> = StorageMap<_, Blake2_128Concat, Did, T::BlockNumber, OptionQuery>;

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub initial_dids: Vec<DidStruct>,
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
        /// New did synced from a parachain
        NewDidSynced { did: Did },

        /// Error on syncing Did
        ErrorSyncingDid { e: DispatchError, did: Did },

        /// Did removal synced from a parachain
        DidRemovalSynced { did: Did },

        /// Error on Did removal parachain
        ErrorRemovingDid { e: DispatchError, did: Did },

        /// Did Key Updated
        DidKeyUpdated { did: Did },

        /// Error updating Key
        ErrorUpdatingKey { e: DispatchError, did: Did },

        /// Removed Did from cache
        DidRemoved { did: Did },
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// The given DID already exists on chain
        DIDDoesNotExists,
        /// PublicKey already linked to another DID on chain
        PublicKeyRegistered,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Adds a DID on chain, where
        /// origin - the origin of the transaction
        /// sign_key - public signing key of the DID
        /// did - public unique did for the DID
        /// metadata - optional metadata to the DID - meant for bank nodes to display URL
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn cache(origin: OriginFor<T>, public_key: PublicKey, did: Did, did_type: DidType) -> DispatchResult {
            // Check if origin is a from a parachain
            ensure_relay(<T as Config>::Origin::from(origin))?;

            let res = Self::do_add_did(&public_key, &did, &did_type);

            match res {
                Ok(()) => Self::deposit_event(Event::NewDidSynced { did }),
                Err(e) => Self::deposit_event(Event::ErrorSyncingDid { e, did }),
            }

            // Return a successful DispatchResultWithPostInfo
            Ok(())
        }

        /// Removes a DID from chain storage, where
        /// origin - the origin of the transaction
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn uncache(origin: OriginFor<T>, did: Did) -> DispatchResult {
            // Check if origin is a from a parachain
            ensure_relay(<T as Config>::Origin::from(origin))?;

            let res = Self::do_remove_did(&did);

            match res {
                Ok(()) => Self::deposit_event(Event::DidRemovalSynced { did }),
                Err(e) => Self::deposit_event(Event::ErrorRemovingDid { e, did }),
            }

            Ok(())
        }

        /// Removes a DID from chain storage, where
        /// origin - the origin of the transaction
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn update_key(origin: OriginFor<T>, did: Did, public_key: PublicKey) -> DispatchResult {
            // Check if origin is a from a parachain
            ensure_relay(<T as Config>::Origin::from(origin))?;

            let res = Self::do_update_did_key(&did, &public_key);

            match res {
                Ok(()) => Self::deposit_event(Event::DidKeyUpdated { did }),
                Err(e) => Self::deposit_event(Event::ErrorUpdatingKey { e, did }),
            }

            Ok(())
        }

        /// Removes a DID from chain storage, where
        /// origin - the origin of the transaction
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn remove(origin: OriginFor<T>, did: Did) -> DispatchResult {
            // Check if origin is a from a validator
            ensure_root(origin)?;

            Self::do_remove_did(&did)?;

            // deposit an event that the DID has been removed
            Self::deposit_event(Event::DidRemoved { did });

            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        /// get the details of the pubkey attached to the DID
        pub fn get_public_key(did: Did) -> Option<PublicKey> {
            PublicKeyMap::<T>::get(did)
        }

        /// Simple type conversion between sr25519::Public and AccountId
        /// Should not panic for any valid sr25519 - need to make more robust to check for valid
        /// publicKey
        pub fn get_accountid_from_pubkey(pk: &PublicKey) -> T::AccountId {
            //convert a publickey to an accountId
            // TODO : Need a better way to handle the option failing?
            T::AccountId::decode(&mut &pk[..]).unwrap()
        }

        /// Check if did is already cached
        pub fn is_already_cached(did: &Did) -> bool {
            if Lookup::<T>::contains_key(did.clone()) {
                return true;
            }
            false
        }

        /// Create Private Did
        pub fn do_add_did(public_key: &PublicKey, did: &Did, did_type: &DidType) -> DispatchResult {

            if Self::is_already_cached(did) {
                Self::do_remove_did(did)?;
            };

            PublicKeyMap::<T>::insert(did.clone(), public_key);
            Lookup::<T>::insert(did.clone(), Self::get_accountid_from_pubkey(public_key));
            RLookup::<T>::insert(Self::get_accountid_from_pubkey(public_key), did.clone());
            DidTypeMap::<T>::insert(did.clone(), did_type);

            let relay_block_number = T::RelayChainBlockNumber::current_block_number();
            LastUpdatedMap::<T>::insert(did.clone(), relay_block_number);

            Ok(())
        }

        /// Remove Did
        pub fn do_remove_did(did: &Did) -> DispatchResult {
            // ensure did is present
            ensure!(
                Lookup::<T>::contains_key(did.clone()),
                Error::<T>::DIDDoesNotExists,
            );

            let public_key = PublicKeyMap::<T>::take(did).unwrap();

            Lookup::<T>::remove(did.clone());
            RLookup::<T>::remove(Self::get_accountid_from_pubkey(&public_key));
            DidTypeMap::<T>::remove(did.clone());
            LastUpdatedMap::<T>::remove(did.clone());

            Ok(())
        }

        /// Create Private Did
        pub fn do_update_did_key(did: &Did, public_key: &PublicKey) -> DispatchResult {
            //reject if the user does not already have DID registered
            ensure!(
                Lookup::<T>::contains_key(did.clone()),
                Error::<T>::DIDDoesNotExists,
            );

            // ensure the public key is not already linked to a DID
            ensure!(
                !RLookup::<T>::contains_key(Self::get_accountid_from_pubkey(public_key)),
                Error::<T>::PublicKeyRegistered,
            );

            // Remove existing public key
            let old_public_key = PublicKeyMap::<T>::take(did).unwrap();
            Lookup::<T>::remove(did.clone());
            RLookup::<T>::remove(Self::get_accountid_from_pubkey(&old_public_key));

            // Update new public key
            Lookup::<T>::insert(did.clone(), Self::get_accountid_from_pubkey(public_key));
            RLookup::<T>::insert(Self::get_accountid_from_pubkey(public_key), did.clone());
            PublicKeyMap::<T>::insert(did.clone(), public_key);
            
            let relay_block_number = T::RelayChainBlockNumber::current_block_number();
            LastUpdatedMap::<T>::insert(did.clone(), relay_block_number);

            Ok(())
        }

        /// Initialize did during genesis
        fn initialize_dids(dids: &Vec<DidStruct>) {
            for did in dids.iter() {
                Lookup::<T>::insert(
                    did.identifier.clone(),
                    Self::get_accountid_from_pubkey(&did.public_key),
                );
                RLookup::<T>::insert(
                    Self::get_accountid_from_pubkey(&did.public_key),
                    did.identifier.clone(),
                );
                PublicKeyMap::<T>::insert(did.identifier.clone(), &did.public_key);

                let did_type = if (&did).is_public == true {
                  DidType::Public
                } else {
                  DidType::Private
                };
                DidTypeMap::<T>::insert(did.identifier.clone(), did_type);

                let relay_block_number = T::RelayChainBlockNumber::current_block_number();
                LastUpdatedMap::<T>::insert(did.identifier.clone(), relay_block_number);
            }
        }
    }
}
